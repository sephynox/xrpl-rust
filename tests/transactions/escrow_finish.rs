// Scenarios:
//   - base: create a time-locked XRP escrow then finish it once FinishAfter has passed
//   - with_credential_ids: provision credential + DepositPreauth, finish with credential_ids
//
// NOTE: After EscrowCreate is submitted the test:
//   1. Queries account_objects to confirm the escrow exists on-chain
//   2. Looks up the creating tx to get the validated Sequence (OfferSequence)
//   3. Waits for close_time >= FinishAfter, then one more ledger_accept

use crate::common::{
    generate_funded_wallet, get_escrow_offer_sequence, get_ledger_close_time, ledger_accept,
    provision_credential_for_destination, submit_tx, test_transaction, wait_for_ledger_close_time,
    with_blockchain_lock, SubmitOptions, CREDENTIAL_TYPE_KYC,
};
use xrpl::models::transactions::{
    escrow_create::EscrowCreate, escrow_finish::EscrowFinish, CommonFields, TransactionType,
};

#[tokio::test]
async fn test_escrow_finish_base() {
    with_blockchain_lock(|| async {
        let wallet = generate_funded_wallet().await;
        let destination = generate_funded_wallet().await;

        let close_time = get_ledger_close_time().await;
        let finish_after = (close_time + 2) as u32;

        let mut create_tx = EscrowCreate::new(
            wallet.classic_address.clone().into(),
            None,                                       // account_txn_id
            None,                                       // fee
            None,                                       // last_ledger_sequence
            None,                                       // memos
            None,                                       // sequence
            None,                                       // signers
            None,                                       // source_tag
            None,                                       // ticket_sequence
            "10000".into(),                             // amount: 10 000 drops
            destination.classic_address.clone().into(), // destination
            None,                                       // cancel_after
            None,                                       // condition
            None,                                       // destination_tag
            Some(finish_after),                         // finish_after
            None,                                       // bytecode (XLS-100)
            None,                                       // data (XLS-100)
        );

        // test_transaction signs, submits, asserts tesSUCCESS, and calls ledger_accept.
        test_transaction(&mut create_tx, &wallet).await;

        // Look up the validated Sequence via account_objects → tx query
        // instead of reading the autofilled value from the tx struct.  This confirms the
        // escrow actually exists on-chain before we try to finish it.
        let offer_sequence = get_escrow_offer_sequence(&wallet.classic_address).await;

        // Wait for the validated ledger close_time to reach FinishAfter.
        wait_for_ledger_close_time(finish_after as u64).await;
        // rippled validates a finish using the *previous* ledger's close_time,
        // so one more ledger_accept ensures that previous close_time > FinishAfter.
        ledger_accept().await;

        let mut finish_tx = EscrowFinish::new(
            wallet.classic_address.clone().into(),
            None,                                  // account_txn_id
            None,                                  // fee
            None,                                  // last_ledger_sequence
            None,                                  // memos
            None,                                  // sequence
            None,                                  // signers
            None,                                  // source_tag
            None,                                  // ticket_sequence
            wallet.classic_address.clone().into(), // owner (= EscrowCreate account)
            offer_sequence,                        // offer_sequence
            None,                                  // condition
            None,                                  // fulfillment
            None,                                  // gas (XLS-100)
        );

        test_transaction(&mut finish_tx, &wallet).await;
    })
    .await;
}

// ── with_credential_ids: credential-gated escrow finish ───────────────────────

const CREDENTIAL_TYPE: &str = CREDENTIAL_TYPE_KYC;

#[tokio::test]
async fn test_escrow_finish_with_credential_ids() {
    with_blockchain_lock(|| async {
        let issuer = generate_funded_wallet().await;
        let subject = generate_funded_wallet().await;
        let destination = generate_funded_wallet().await;

        let credential_hash =
            provision_credential_for_destination(&issuer, &subject, &destination, CREDENTIAL_TYPE)
                .await;

        let close_time = get_ledger_close_time().await;
        let finish_after = (close_time + 2) as u32;

        let mut create_tx = EscrowCreate {
            common_fields: CommonFields {
                account: subject.classic_address.clone().into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: "10000".into(),
            destination: destination.classic_address.clone().into(),
            finish_after: Some(finish_after),
            ..Default::default()
        };

        test_transaction(&mut create_tx, &subject).await;

        let offer_sequence = get_escrow_offer_sequence(&subject.classic_address).await;

        wait_for_ledger_close_time(finish_after as u64).await;
        ledger_accept().await;

        // Step 3a: verify gate is enforced — finish WITHOUT credentials must be rejected.
        let mut neg_finish = EscrowFinish {
            common_fields: CommonFields {
                account: subject.classic_address.clone().into(),
                transaction_type: TransactionType::EscrowFinish,
                ..Default::default()
            },
            owner: subject.classic_address.clone().into(),
            offer_sequence,
            ..Default::default()
        };
        let neg_result = submit_tx(
            &mut neg_finish,
            SubmitOptions { wallet: &subject, autofill: true, check_fee: true },
        )
        .await;
        ledger_accept().await;
        assert_eq!(
            neg_result, "tecNO_PERMISSION",
            "escrow finish without credential_ids should be rejected when destination has DepositAuth"
        );

        // Step 3b: finish WITH credential_ids — must succeed.
        let mut finish_tx = EscrowFinish {
            common_fields: CommonFields {
                account: subject.classic_address.clone().into(),
                transaction_type: TransactionType::EscrowFinish,
                ..Default::default()
            },
            owner: subject.classic_address.clone().into(),
            offer_sequence,
            ..Default::default()
        };
        finish_tx.credential_ids = Some(vec![credential_hash.into()]);

        test_transaction(&mut finish_tx, &subject).await;
    })
    .await;
}

// ── XLS-100 Smart Escrows: bytecode-gated finish ────────────────────────────

/// Minimal 46-byte WASM module. Exports `escrow_finish() -> i32` returning 1
/// (always finish). Verified with WebAssembly.compile.
const RETURN_1_WASM_HEX: &str =
    "0061736D010000000105016000017F030201000711010D657363726F775F66696E69736800000A0601040041010B";
/// Same shape as above but returns 0 (always reject → `tecBYTECODE_REJECTED`).
const RETURN_0_WASM_HEX: &str =
    "0061736D010000000105016000017F030201000711010D657363726F775F66696E69736800000A0601040041000B";

/// Happy path: create an escrow with WASM bytecode that returns 1, then
/// finish it with an explicit `Gas` budget.
///
/// Gated on the `SmartEscrow` amendment being enabled on the target rippled.
/// Set `RIPPLED_HAS_SMART_ESCROW=1` when running against a node that has it
/// (e.g. wasm.devnet.rippletest.net or a local `ripple/se/supported` build).
#[tokio::test]
#[ignore = "requires SmartEscrow amendment; set RIPPLED_HAS_SMART_ESCROW=1 to enable"]
async fn test_escrow_finish_with_gas() {
    if std::env::var("RIPPLED_HAS_SMART_ESCROW").ok().as_deref() != Some("1") {
        return;
    }
    with_blockchain_lock(|| async {
        let wallet = generate_funded_wallet().await;
        let destination = generate_funded_wallet().await;
        let close_time = get_ledger_close_time().await;
        let finish_after = (close_time + 2) as u32;

        let mut create_tx = EscrowCreate {
            common_fields: CommonFields {
                account: wallet.classic_address.clone().into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: "10000".into(),
            destination: destination.classic_address.clone().into(),
            finish_after: Some(finish_after),
            bytecode: Some(RETURN_1_WASM_HEX.into()),
            ..Default::default()
        };
        test_transaction(&mut create_tx, &wallet).await;

        let offer_sequence = get_escrow_offer_sequence(&wallet.classic_address).await;
        wait_for_ledger_close_time(finish_after as u64).await;
        ledger_accept().await;

        let mut finish_tx = EscrowFinish {
            common_fields: CommonFields {
                account: wallet.classic_address.clone().into(),
                transaction_type: TransactionType::EscrowFinish,
                ..Default::default()
            },
            owner: wallet.classic_address.clone().into(),
            offer_sequence,
            gas: Some(1_000_000),
            ..Default::default()
        };
        test_transaction(&mut finish_tx, &wallet).await;
    })
    .await;
}

/// Rejection path: create an escrow with WASM bytecode that returns 0, then
/// try to finish it. Rippled should reject the finish with the
/// `tecBYTECODE_REJECTED` result code. Depends on
/// [PR](https://github.com/XRPLF/xrpl-rust/pulls)-`submission-failed-restructure`'s
/// `SubmissionFailed { result_code, .. }` for a clean assertion — falls back
/// to substring matching on the Display output when that PR is not yet
/// merged.
///
/// Gated on the `SmartEscrow` amendment (same env var as above).
#[tokio::test]
#[ignore = "requires SmartEscrow amendment; set RIPPLED_HAS_SMART_ESCROW=1 to enable"]
async fn test_escrow_finish_rejected_by_bytecode() {
    if std::env::var("RIPPLED_HAS_SMART_ESCROW").ok().as_deref() != Some("1") {
        return;
    }
    with_blockchain_lock(|| async {
        let wallet = generate_funded_wallet().await;
        let destination = generate_funded_wallet().await;
        let close_time = get_ledger_close_time().await;
        let finish_after = (close_time + 2) as u32;

        let mut create_tx = EscrowCreate {
            common_fields: CommonFields {
                account: wallet.classic_address.clone().into(),
                transaction_type: TransactionType::EscrowCreate,
                ..Default::default()
            },
            amount: "10000".into(),
            destination: destination.classic_address.clone().into(),
            finish_after: Some(finish_after),
            bytecode: Some(RETURN_0_WASM_HEX.into()),
            ..Default::default()
        };
        test_transaction(&mut create_tx, &wallet).await;

        let offer_sequence = get_escrow_offer_sequence(&wallet.classic_address).await;
        wait_for_ledger_close_time(finish_after as u64).await;
        ledger_accept().await;

        // Use submit_tx (raw engine_result) so we can assert on the specific
        // tec* code without depending on this PR being stacked on the
        // SubmissionFailed-restructure PR.
        let mut finish_tx = EscrowFinish {
            common_fields: CommonFields {
                account: wallet.classic_address.clone().into(),
                transaction_type: TransactionType::EscrowFinish,
                ..Default::default()
            },
            owner: wallet.classic_address.clone().into(),
            offer_sequence,
            gas: Some(1_000_000),
            ..Default::default()
        };
        let result = submit_tx(
            &mut finish_tx,
            SubmitOptions {
                wallet: &wallet,
                autofill: true,
                check_fee: true,
            },
        )
        .await;
        ledger_accept().await;
        assert_eq!(
            result, "tecBYTECODE_REJECTED",
            "escrow finish with return-0 bytecode should hit tecBYTECODE_REJECTED"
        );
    })
    .await;
}

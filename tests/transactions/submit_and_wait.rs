// Direct end-to-end test of asynch::transaction::submit_and_wait.
//
// Other transaction tests use the shared `test_transaction` helper, which calls
// `sign_and_submit` and then `ledger_accept`. submit_and_wait does its own polling
// for ledger validation, so it needs a separate test. Standalone rippled does not
// auto-close ledgers, so a background task drives `ledger_accept` while the poll
// loop runs.

use core::time::Duration;

use crate::common::{
    generate_funded_wallet, get_client, ledger_accept, payment::xrp_payment, with_blockchain_lock,
};
use xrpl::asynch::{
    exceptions::XRPLHelperException,
    transaction::{
        exceptions::{XRPLSubmitAndWaitException, XRPLTransactionHelperException},
        submit_and_wait,
    },
};
use xrpl::wallet::Wallet;

/// Match on the typed `SubmissionFailed { result_code, message }` shape
/// through the wrapped exception layers.
macro_rules! assert_submission_failed_matches {
    ($err:expr, |$code:ident, $msg:ident| $body:block) => {
        match $err {
            XRPLHelperException::XRPLTransactionHelperError(
                XRPLTransactionHelperException::XRPLSubmitAndWaitError(
                    XRPLSubmitAndWaitException::SubmissionFailed {
                        result_code: $code,
                        message: $msg,
                    },
                ),
            ) => $body,
            other => panic!(
                "expected SubmissionFailed {{ result_code, message }}, got {:?}",
                other
            ),
        }
    };
}

#[tokio::test]
async fn test_submit_and_wait_payment() {
    with_blockchain_lock(|| async {
        let client = get_client().await;
        let sender = generate_funded_wallet().await;
        let recipient = Wallet::create(None).expect("recipient wallet");

        // 20 XRP covers the standalone base reserve for the new recipient.
        let mut payment = xrp_payment(
            sender.classic_address.clone(),
            recipient.classic_address.clone(),
            "20000000",
        );

        // Drive ledger closes while submit_and_wait polls for validation.
        let ledger_driver = tokio::spawn(async {
            loop {
                ledger_accept().await;
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });

        let validated_tx =
            submit_and_wait(&mut payment, client, Some(&sender), Some(true), Some(true))
                .await
                .expect("submit_and_wait should return a validated transaction");

        ledger_driver.abort();
        let _ = ledger_driver.await;

        let metadata = validated_tx
            .get_transaction_metadata()
            .expect("validated transaction should have metadata");

        assert_eq!(metadata.transaction_result, "tesSUCCESS");
    })
    .await;
}

/// Prelim `tem*` path: a Payment where `Destination == Account` is rejected
/// by rippled at submit time with `temREDUNDANT`. `submit_and_wait` should
/// return a typed `SubmissionFailed` carrying that code plus the human
/// message.
#[tokio::test]
async fn test_submit_and_wait_prelim_tem_error() {
    with_blockchain_lock(|| async {
        let client = get_client().await;
        let sender = generate_funded_wallet().await;

        // Self-payment — rippled preflight returns temREDUNDANT.
        let mut payment = xrp_payment(
            sender.classic_address.clone(),
            sender.classic_address.clone(),
            "1000000",
        );

        let err = submit_and_wait(&mut payment, client, Some(&sender), Some(true), Some(true))
            .await
            .expect_err("self-payment should fail at prelim tem*");

        assert_submission_failed_matches!(err, |result_code, message| {
            assert!(
                result_code.starts_with("tem"),
                "expected tem* result_code, got {result_code}"
            );
            assert!(
                message.is_some(),
                "prelim tem* path should carry rippled's engine_result_message"
            );
        });
    })
    .await;
}

/// Validated `tec*` path: sending more XRP than the sender holds is accepted
/// by rippled at submit time (fee is consumed), but the transactor rejects it
/// once validated with `tecUNFUNDED_PAYMENT`. `submit_and_wait` should surface
/// that code via the polling branch.
#[tokio::test]
async fn test_submit_and_wait_validated_tec_error() {
    with_blockchain_lock(|| async {
        let client = get_client().await;
        let sender = generate_funded_wallet().await;
        let recipient = Wallet::create(None).expect("recipient wallet");

        // Sender was funded with 400 XRP; asking for 10_000 XRP is well over
        // that even accounting for reserves, so the transactor is guaranteed
        // to reject with tecUNFUNDED_PAYMENT.
        let mut payment = xrp_payment(
            sender.classic_address.clone(),
            recipient.classic_address.clone(),
            "10000000000",
        );

        let ledger_driver = tokio::spawn(async {
            loop {
                ledger_accept().await;
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        });

        let err = submit_and_wait(&mut payment, client, Some(&sender), Some(true), Some(true))
            .await
            .expect_err("over-payment should be rejected once validated");

        ledger_driver.abort();
        let _ = ledger_driver.await;

        assert_submission_failed_matches!(err, |result_code, message| {
            assert_eq!(result_code, "tecUNFUNDED_PAYMENT");
            // Validated tec/tef path only has the code, no separate message.
            assert!(message.is_none(), "tec* path should have message: None");
        });
    })
    .await;
}

/// Polling timeout path: if no ledger closes while `submit_and_wait` polls,
/// the loop exhausts its retries and returns a synthetic
/// `SubmissionFailed { result_code: "submission_timeout", .. }`.
#[tokio::test]
async fn test_submit_and_wait_polling_timeout() {
    with_blockchain_lock(|| async {
        let client = get_client().await;
        let sender = generate_funded_wallet().await;
        let recipient = Wallet::create(None).expect("recipient wallet");

        let mut payment = xrp_payment(
            sender.classic_address.clone(),
            recipient.classic_address.clone(),
            "20000000",
        );

        // No ledger driver spawned → validated_ledger_sequence never advances
        // past last_ledger_sequence, and the poll loop hits its retry cap.
        let err = submit_and_wait(&mut payment, client, Some(&sender), Some(true), Some(true))
            .await
            .expect_err("polling without ledger_accept should time out");

        assert_submission_failed_matches!(err, |result_code, message| {
            assert_eq!(result_code, "submission_timeout");
            assert_eq!(
                message.as_deref(),
                Some("Transaction not included in ledger"),
            );
        });

        // Close the ledger the timed-out tx sat in so subsequent tests
        // (serialized on the blockchain lock) see a clean state.
        ledger_accept().await;
    })
    .await;
}

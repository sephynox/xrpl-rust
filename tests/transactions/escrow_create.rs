// Scenarios:
//   - base: create a time-locked XRP escrow (FinishAfter = close_time + 2) and verify tesSUCCESS
//
// NOTE: FinishAfter is set slightly ahead of the current ledger close_time.
// On testnet ledgers close automatically every ~3-4 s, so EscrowFinish and EscrowCancel
// scenarios (which require waiting for time to advance) live in their own test files.

use crate::common::{
    generate_funded_wallet, get_ledger_close_time, test_transaction, with_blockchain_lock,
};
use xrpl::models::transactions::escrow_create::EscrowCreate;

#[tokio::test]
async fn test_escrow_create_base() {
    with_blockchain_lock(|| async {
        let wallet = generate_funded_wallet().await;
        let destination = generate_funded_wallet().await;

        let close_time = get_ledger_close_time().await;
        let finish_after = (close_time + 2) as u32;

        let mut tx = EscrowCreate::new(
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

        test_transaction(&mut tx, &wallet).await;
    })
    .await;
}

/// Smart-escrow XLS-100 scenario. Creates an escrow with a minimal WebAssembly
/// module (46 bytes) that exports `escrow_finish() -> i32` returning 1
/// (i.e. always finishes). Only asserts the CREATE succeeds — the FINISH-side
/// pairing (with `.with_gas(...)` and a `tecBYTECODE_REJECTED` counter-test)
/// lives in the companion `escrow_finish.rs` file.
///
/// Gated on the `SmartEscrow` amendment being enabled on the target rippled.
/// Set `RIPPLED_HAS_SMART_ESCROW=1` when running against a node that has it
/// (e.g. wasm.devnet.rippletest.net or a local `ripple/se/supported` build);
/// the test is `#[ignore]`d otherwise so `cargo test` on a mainline-only node
/// does not fail spuriously.
#[tokio::test]
#[ignore = "requires SmartEscrow amendment; set RIPPLED_HAS_SMART_ESCROW=1 to enable"]
async fn test_escrow_create_smart_escrow() {
    if std::env::var("RIPPLED_HAS_SMART_ESCROW").ok().as_deref() != Some("1") {
        return;
    }
    with_blockchain_lock(|| async {
        // Minimal WASM: (module (func (export "escrow_finish") (result i32) i32.const 1))
        const RETURN_1_WASM_HEX: &str = "0061736D010000000105016000017F030201000711010D657363726F775F66696E69736800000A0601040041010B";

        let wallet = generate_funded_wallet().await;
        let destination = generate_funded_wallet().await;
        let close_time = get_ledger_close_time().await;
        let finish_after = (close_time + 2) as u32;

        let mut tx = EscrowCreate::new(
            wallet.classic_address.clone().into(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            "10000".into(),
            destination.classic_address.clone().into(),
            None,
            None,
            None,
            Some(finish_after),
            Some(RETURN_1_WASM_HEX.into()), // bytecode (XLS-100)
            None,                           // data
        );

        test_transaction(&mut tx, &wallet).await;
    })
    .await;
}

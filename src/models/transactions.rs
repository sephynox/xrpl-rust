use crate::models::*;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDelete<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::account_delete")]
    transaction_type: TransactionType,
    destination: &'a str,
    destination_tag: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSet<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::account_set")]
    transaction_type: TransactionType,
    clear_flag: Option<u32>,
    domain: Option<&'a str>,
    email_hash: Option<&'a str>,
    message_key: Option<&'a str>,
    set_flag: Option<u32>,
    transfer_rate: Option<u32>,
    tick_size: Option<u32>,
    nftoken_minter: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckCancel<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_cancel")]
    transaction_type: TransactionType,
    check_id: &'a str,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckCash<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_cash")]
    transaction_type: TransactionType,
    check_id: &'a str,
    amount: Option<Currency>,
    deliver_min: Option<Currency>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckCreate<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::check_create")]
    transaction_type: TransactionType,
    destination: &'a str,
    send_max: Currency,
    destination_tag: Option<u32>,
    expiration: Option<u32>,
    invoice_id: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositPreauth<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::deposit_preauth")]
    transaction_type: TransactionType,
    authorize: Option<&'a str>,
    unauthorize: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct EscrowCancel<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_cancel")]
    transaction_type: TransactionType,
    owner: &'a str,
    offer_sequence: u32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct EscrowCreate<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_create")]
    transaction_type: TransactionType,
    amount: Currency,
    destination: &'a str,
    destination_tag: Option<&'a str>,
    cancel_after: Option<u32>,
    finish_after: Option<u32>,
    condition: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct EscrowFinish<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::escrow_finish")]
    transaction_type: TransactionType,
    owner: &'a str,
    offer_sequence: u32,
    condition: Option<&'a str>,
    fulfillment: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NFTokenAcceptOffer<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_accept_offer")]
    transaction_type: TransactionType,
    nftoken_sell_offer: Option<&'a str>,
    nftoken_buy_offer: Option<&'a str>,
    nftoken_broker_fee: Option<Currency>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NFTokenBurn<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_burn")]
    transaction_type: TransactionType,
    account: &'a str,
    nftoken_id: &'a str,
    owner: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NFTokenCancelOffer<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_cancel_offer")]
    transaction_type: TransactionType,
    // Lifetime issue
    #[serde(borrow)]
    nftoken_offers: Vec<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NFTokenCreateOffer<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_create_offer")]
    transaction_type: TransactionType,
    nftoken_id: &'a str,
    amount: Currency,
    owner: Option<&'a str>,
    expiration: Option<u32>,
    destination: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct NFTokenMint<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::nftoken_mint")]
    transaction_type: TransactionType,
    nftoken_taxon: u32,
    issuer: Option<&'a str>,
    transfer_fee: Option<u32>,
    uri: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferCancel {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::offer_cancel")]
    transaction_type: TransactionType,
    offer_sequence: u32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferCreate {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::offer_create")]
    transaction_type: TransactionType,
    taker_gets: Currency,
    taker_pays: Currency,
    expiration: Option<u32>,
    offer_sequence: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Payment<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment")]
    transaction_type: TransactionType,
    amount: Currency,
    destination: &'a str,
    destination_tag: Option<u32>,
    invoice_id: Option<u32>,
    paths: Option<Vec<Vec<PathStep<'a>>>>,
    send_max: Option<Currency>,
    deliver_min: Option<Currency>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentChannelClaim<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_claim")]
    transaction_type: TransactionType,
    channel: &'a str,
    balance: Option<&'a str>,
    amount: Option<&'a str>,
    signature: Option<&'a str>,
    public_key: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentChannelCreate<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_create")]
    transaction_type: TransactionType,
    amount: Currency,
    destination: &'a str,
    settle_delay: u32,
    public_key: &'a str,
    cancel_after: Option<u32>,
    destination_tag: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentChannelFund<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::payment_channel_fund")]
    transaction_type: TransactionType,
    channel: &'a str,
    amount: &'a str,
    expiration: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SetRegularKey<'a> {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::set_regular_key")]
    transaction_type: TransactionType,
    regular_key: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TicketCreate {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::ticket_create")]
    transaction_type: TransactionType,
    ticket_count: u32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct TrustSet {
    #[serde(skip_serializing)]
    #[serde(default = "TransactionType::trust_set")]
    transaction_type: TransactionType,
    limit_amount: Currency,
    quality_in: Option<u32>,
    quality_out: Option<u32>,
}

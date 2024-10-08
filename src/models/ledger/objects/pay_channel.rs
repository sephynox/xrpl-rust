use crate::models::ledger::objects::LedgerEntryType;
use crate::models::FlagCollection;
use crate::models::NoFlags;
use crate::models::{amount::Amount, Model};
use alloc::borrow::Cow;

use serde::{Deserialize, Serialize};

use serde_with::skip_serializing_none;

use super::{CommonFields, LedgerObject};

/// The `PayChannel` object type represents a payment channel. Payment channels enable small,
/// rapid off-ledger payments of XRP that can be later reconciled with the consensus ledger.
///
/// `<https://xrpl.org/paychannel.html#paychannel>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PayChannel<'a> {
    /// The base fields for all ledger object models.
    ///
    /// See Ledger Object Common Fields:
    /// `<https://xrpl.org/ledger-entry-common-fields.html>`
    #[serde(flatten)]
    pub common_fields: CommonFields<'a, NoFlags>,
    // The custom fields for the PayChannel model.
    //
    // See PayChannel fields:
    // `<https://xrpl.org/paychannel.html#paychannel-fields>`
    /// The source address that owns this payment channel.
    pub account: Cow<'a, str>,
    /// Total XRP, in drops, that has been allocated to this channel. This includes XRP
    /// that has been paid to the destination address.
    pub amount: Amount<'a>,
    /// Total XRP, in drops, already paid out by the channel. The difference between
    /// this value and the `Amount` field is how much XRP can still be paid to the destination
    /// address with `PaymentChannelClaim` transactions.
    pub balance: Amount<'a>,
    /// The destination address for this payment channel. While the payment channel is open,
    /// this address is the only one that can receive XRP from the channel.
    pub destination: Cow<'a, str>,
    /// A hint indicating which page of the source address's owner directory links to this
    /// object, in case the directory consists of multiple pages.
    pub owner_node: Cow<'a, str>,
    /// The identifying hash of the transaction that most recently modified this object.
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Cow<'a, str>,
    /// The index of the ledger that contains the transaction that most recently modified
    /// this object.
    pub previous_txn_lgr_seq: u32,
    /// Public key, in hexadecimal, of the key pair that can be used to sign claims against
    /// this channel. This can be any valid secp256k1 or Ed25519 public key.
    pub public_key: Cow<'a, str>,
    /// Number of seconds the source address must wait to close the channel if it still has
    /// any XRP in it.
    pub settle_delay: u32,
    /// The immutable expiration time for this payment channel, in seconds since the Ripple Epoch.
    pub cancel_after: Option<u32>,
    /// An arbitrary tag to further specify the destination for this payment channel, such
    /// as a hosted recipient at the `destination` address.
    pub destination_tag: Option<u32>,
    /// A hint indicating which page of the destination's owner directory links to this object,
    /// in case the directory consists of multiple pages.
    pub destination_node: Option<Cow<'a, str>>,
    /// The mutable expiration time for this payment channel, in seconds since the Ripple Epoch.
    pub expiration: Option<u32>,
    /// An arbitrary tag to further specify the source for this payment channel, such as a
    /// hosted recipient at the owner's address.
    pub source_tag: Option<u32>,
}

impl<'a> Model for PayChannel<'a> {}

impl<'a> LedgerObject<NoFlags> for PayChannel<'a> {
    fn get_ledger_entry_type(&self) -> LedgerEntryType {
        self.common_fields.get_ledger_entry_type()
    }
}

impl<'a> PayChannel<'a> {
    pub fn new(
        index: Option<Cow<'a, str>>,
        ledger_index: Option<Cow<'a, str>>,
        account: Cow<'a, str>,
        amount: Amount<'a>,
        balance: Amount<'a>,
        destination: Cow<'a, str>,
        owner_node: Cow<'a, str>,
        previous_txn_id: Cow<'a, str>,
        previous_txn_lgr_seq: u32,
        public_key: Cow<'a, str>,
        settle_delay: u32,
        cancel_after: Option<u32>,
        destination_tag: Option<u32>,
        destination_node: Option<Cow<'a, str>>,
        expiration: Option<u32>,
        source_tag: Option<u32>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                flags: FlagCollection::default(),
                ledger_entry_type: LedgerEntryType::PayChannel,
                index,
                ledger_index,
            },
            account,
            amount,
            balance,
            destination,
            owner_node,
            previous_txn_id,
            previous_txn_lgr_seq,
            public_key,
            settle_delay,
            cancel_after,
            destination_tag,
            destination_node,
            expiration,
            source_tag,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::Cow;

    #[test]
    fn test_serde() {
        let pay_channel = PayChannel::new(
            Some(Cow::from(
                "96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797",
            )),
            None,
            Cow::from("rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt"),
            Amount::XRPAmount("4325800".into()),
            Amount::XRPAmount("2323423".into()),
            Cow::from("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"),
            Cow::from("0000000000000000"),
            Cow::from("F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF"),
            14524914,
            Cow::from("32D2471DB72B27E3310F355BB33E339BF26F8392D5A93D3BC0FC3B566612DA0F0A"),
            3600,
            Some(536891313),
            Some(1002341),
            Some(Cow::from("0000000000000000")),
            Some(536027313),
            Some(0),
        );
        let serialized = serde_json::to_string(&pay_channel).unwrap();

        let deserialized: PayChannel = serde_json::from_str(&serialized).unwrap();

        assert_eq!(pay_channel, deserialized);
    }
}

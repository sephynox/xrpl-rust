use alloc::{string::ToString, vec::Vec};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    constants::CryptoAlgorithm,
    models::{
        exceptions::{ChannelAuthorizeException, XRPLModelException, XRPLRequestException},
        ChannelAuthorizeError, Model, RequestMethod,
    },
};

/// The channel_authorize method creates a signature that can  be
/// used to redeem a specific amount of XRP from a payment channel.
///
/// Warning: Do not send secret keys to untrusted servers or
/// through unsecured network connections. (This includes the
/// secret, seed, seed_hex, or passphrase fields of this request.)
/// You should only use this method on a secure, encrypted network
/// connection to a server you run or fully trust with your funds.
/// Otherwise, eavesdroppers could use your secret key to sign
/// claims and take all the money from this payment channel and
/// anything else using the same key pair.
///
/// See Set Up Secure Signing:
/// `<https://xrpl.org/set-up-secure-signing.html>`
///
/// See Channel Authorize:
/// `<https://xrpl.org/channel_authorize.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ChannelAuthorize<'a> {
    /// The unique ID of the payment channel to use.
    pub channel_id: &'a str,
    /// Cumulative amount of XRP, in drops, to authorize.
    /// If the destination has already received a lesser amount
    /// of XRP from this channel, the signature created by this
    /// method can be redeemed for the difference.
    pub amount: &'a str,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The secret key to use to sign the claim. This must be
    /// the same key pair as the public key specified in the
    /// channel. Cannot be used with seed, seed_hex, or passphrase.
    pub secret: Option<&'a str>,
    /// The secret seed to use to sign the claim. This must be
    /// the same key pair as the public key specified in the channel.
    /// Must be in the XRP Ledger's base58 format. If provided,
    /// you must also specify the key_type. Cannot be used with
    /// secret, seed_hex, or passphrase.
    pub seed: Option<&'a str>,
    /// The secret seed to use to sign the claim. This must be the
    /// same key pair as the public key specified in the channel.
    /// Must be in hexadecimal format. If provided, you must also
    /// specify the key_type. Cannot be used with secret, seed,
    /// or passphrase.
    pub seed_hex: Option<&'a str>,
    /// A string passphrase to use to sign the claim. This must be
    /// the same key pair as the public key specified in the channel.
    /// The key derived from this passphrase must match the public
    /// key specified in the channel. If provided, you must also
    /// specify the key_type. Cannot be used with secret, seed,
    /// or seed_hex.
    pub passphrase: Option<&'a str>,
    /// The signing algorithm of the cryptographic key pair provided.
    /// Valid types are secp256k1 or ed25519. The default is secp256k1.
    pub key_type: Option<CryptoAlgorithm>,
    /// The request method.
    #[serde(default = "RequestMethod::channel_authorize")]
    pub command: RequestMethod,
}

impl<'a> Default for ChannelAuthorize<'a> {
    fn default() -> Self {
        ChannelAuthorize {
            channel_id: "",
            amount: "",
            id: None,
            secret: None,
            seed: None,
            seed_hex: None,
            passphrase: None,
            key_type: None,
            command: RequestMethod::ChannelAuthorize,
        }
    }
}

impl<'a> Model for ChannelAuthorize<'a> {
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        match self._get_field_error() {
            Err(error) => Err(XRPLModelException::XRPLRequestError(
                XRPLRequestException::ChannelAuthorizeError(error),
            )),
            Ok(_no_error) => Ok(()),
        }
    }
}

impl<'a> ChannelAuthorizeError for ChannelAuthorize<'a> {
    fn _get_field_error(&self) -> Result<(), ChannelAuthorizeException> {
        let mut signing_methods = Vec::new();
        for method in [self.secret, self.seed, self.seed_hex, self.passphrase] {
            if method.is_some() {
                signing_methods.push(method)
            }
        }
        match signing_methods.len() != 1 {
            true => Err(ChannelAuthorizeException::InvalidMustSetExactlyOneOf {
                fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
            }),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod test_channel_authorize_errors {
    use alloc::string::ToString;

    use crate::{
        constants::CryptoAlgorithm,
        models::{
            exceptions::{ChannelAuthorizeException, XRPLModelException, XRPLRequestException},
            Model, RequestMethod,
        },
    };

    use super::ChannelAuthorize;

    #[test]
    fn test_fields_error() {
        let channel_authorize = ChannelAuthorize {
            command: RequestMethod::ChannelAuthorize,
            channel_id: "5DB01B7FFED6B67E6B0414DED11E051D2EE2B7619CE0EAA6286D67A3A4D5BDB3",
            amount: "1000000",
            id: None,
            secret: None,
            seed: Some(""),
            seed_hex: Some(""),
            passphrase: None,
            key_type: Some(CryptoAlgorithm::SECP256K1),
        };
        let expected_error =
            XRPLModelException::XRPLRequestError(XRPLRequestException::ChannelAuthorizeError(
                ChannelAuthorizeException::InvalidMustSetExactlyOneOf {
                    fields: "`secret`, `seed`, `seed_hex`, `passphrase`".to_string(),
                },
            ));
        assert_eq!(channel_authorize.validate(), Err(expected_error))
    }
}

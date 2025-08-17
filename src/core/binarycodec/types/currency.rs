//! Codec for currency property inside an XRPL
//! issued currency amount json.

use super::utils::CURRENCY_CODE_LENGTH;
use super::Hash160;
use super::TryFromParser;
use super::XRPLType;
use crate::core::exceptions::XRPLCoreException;
use crate::core::BinaryParser;
use crate::utils::exceptions::ISOCodeException;
use crate::utils::*;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt::Display;
use exceptions::XRPLUtilsException;
use serde::Serializer;
use serde::{Deserialize, Serialize};

pub const NATIVE_HEX_CODE: &str = "0000000000000000000000000000000000000000";
pub const NATIVE_CODE: &str = "XRP";

/// Codec for serializing and deserializing
/// vectors of Hash256.
#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "&str")]
pub struct Currency(Hash160);

fn _iso_code_from_hex(value: &[u8]) -> Result<Option<String>, XRPLUtilsException> {
    let candidate_iso = alloc::str::from_utf8(&value[12..15])?;

    if candidate_iso == NATIVE_CODE {
        Err(XRPLUtilsException::ISOCodeError(
            ISOCodeException::InvalidXRPBytes,
        ))
    } else if is_iso_code(candidate_iso) {
        Ok(Some(candidate_iso.to_string()))
    } else {
        Ok(None)
    }
}

/// Convert an ISO code to a 160-bit (20 byte) encoded
/// representation.
///
/// See "Currency codes" subheading in Amount Fields:
/// `<https://xrpl.org/serialization.html#amount-fields>`
fn _iso_to_bytes(value: &str) -> Result<[u8; CURRENCY_CODE_LENGTH], ISOCodeException> {
    if !is_iso_code(value) {
        Err(ISOCodeException::InvalidISOCode)
    } else if value == NATIVE_CODE {
        Ok(Default::default())
    } else {
        let value = value.to_uppercase();
        let iso_bytes = value.as_bytes();
        let pad_left: [u8; 12] = Default::default();
        let pad_right: [u8; 5] = Default::default();
        let mut result: Vec<u8> = vec![];

        result.extend_from_slice(&pad_left);
        result.extend_from_slice(iso_bytes);
        result.extend_from_slice(&pad_right);

        Ok(result
            .try_into()
            .or(Err(ISOCodeException::InvalidISOLength))?)
    }
}

impl XRPLType for Currency {
    type Error = XRPLCoreException;

    fn new(buffer: Option<&[u8]>) -> Result<Self, Self::Error> {
        let hash160 = Hash160::new(buffer.or(Some(&[0; CURRENCY_CODE_LENGTH])))?;
        Ok(Currency(hash160))
    }
}

impl TryFromParser for Currency {
    type Error = XRPLCoreException;

    /// Build Currency from a BinaryParser.
    fn from_parser(
        parser: &mut BinaryParser,
        length: Option<usize>,
    ) -> Result<Currency, Self::Error> {
        Ok(Currency(Hash160::from_parser(parser, length)?))
    }
}

impl Serialize for Currency {
    /// Returns the JSON representation of a currency.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<&str> for Currency {
    type Error = XRPLCoreException;

    /// Construct a Currency object from a string
    /// representation of a currency.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if is_iso_code(value) {
            let iso_bytes = _iso_to_bytes(value)?;
            let hash160 = Hash160::new(Some(&iso_bytes))?;
            Ok(Currency(hash160))
        } else if is_iso_hex(value) {
            Ok(Currency(Hash160::new(Some(&hex::decode(value)?))?))
        } else {
            Err(ISOCodeException::UnsupportedCurrencyRepresentation.into())
        }
    }
}

impl Display for Currency {
    /// Get the ISO or hex representation of the Currency bytes.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let buffer = self.0.as_ref();

        if hex::encode_upper(buffer) == NATIVE_HEX_CODE {
            write!(f, "{}", NATIVE_CODE)
        } else {
            let iso = _iso_code_from_hex(buffer);

            if let Ok(code) = iso {
                write!(
                    f,
                    "{}",
                    code.or_else(|| Some(hex::encode_upper(buffer))).unwrap()
                )
            } else {
                write!(f, "{}", hex::encode_upper(buffer))
            }
        }
    }
}

impl AsRef<[u8]> for Currency {
    /// Get a reference of the byte representation.
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Currency {
    pub fn is_xrp(&self) -> bool {
        self.to_string() == NATIVE_CODE
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::format;

    const ILLEGAL_NATIVE_HEX_CODE: &str = "0000000000000000000000005852500000000000";
    const USD_HEX_CODE: &str = "0000000000000000000000005553440000000000";
    const NONSTANDARD_HEX_CODE: &str = "015841551A748AD2C1F76FF6ECB0CCCD00000000";
    const USD_ISO: &str = "USD";

    #[test]
    fn test_iso_to_bytes() {
        // Valid non-XRP
        let usd_iso_bytes = _iso_to_bytes(USD_ISO).unwrap();
        // Valid XRP
        let xrp_iso_bytes = _iso_to_bytes(NATIVE_CODE).unwrap();
        // Error case
        let invalid_iso = "INVALID";

        assert_eq!(USD_HEX_CODE, hex::encode_upper(usd_iso_bytes));
        assert_eq!(NATIVE_HEX_CODE, hex::encode_upper(xrp_iso_bytes));
        assert!(_iso_to_bytes(invalid_iso).is_err());
    }

    #[test]
    fn test_currency_new() {
        let hex = hex::decode(USD_HEX_CODE).expect("");
        let currency = Currency::new(Some(&hex));
        assert_eq!(USD_HEX_CODE, hex::encode_upper(currency.unwrap()))
    }

    #[test]
    fn test_currency_try_from() {
        let from_hex_xrp = Currency::try_from(NATIVE_HEX_CODE).unwrap();
        let from_hex_ic = Currency::try_from(USD_HEX_CODE).unwrap();
        let from_iso_xrp = Currency::try_from(NATIVE_CODE).unwrap();
        let from_iso_ic = Currency::try_from(USD_ISO).unwrap();
        let from_ns = Currency::try_from(NONSTANDARD_HEX_CODE).unwrap();

        assert_eq!(NATIVE_CODE, from_hex_xrp.to_string());
        assert_eq!(USD_ISO, from_hex_ic.to_string());
        assert_eq!(NATIVE_HEX_CODE, hex::encode_upper(from_iso_xrp));
        assert_eq!(USD_HEX_CODE, hex::encode_upper(from_iso_ic));
        assert_eq!(NONSTANDARD_HEX_CODE, hex::encode_upper(from_ns));
    }

    #[test]
    fn accept_currency_serde_encode_decode() {
        let currency = Currency::try_from(USD_HEX_CODE).unwrap();
        let serialize = serde_json::to_string(&currency).unwrap();
        let deserialize: Currency = serde_json::from_str(&serialize).unwrap();

        assert_eq!(format!("\"{USD_ISO}\""), serialize);
        assert_eq!(currency.to_string(), deserialize.to_string());
    }
}

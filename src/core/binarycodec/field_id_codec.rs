//! Encodes and decodes field IDs.
//! Field IDs <https://xrpl.org/serialization.html#field-ids>

use crate::core::binarycodec::definitions::definition_types::load_definition_map;
use crate::core::binarycodec::definitions::definition_types::DefinitionHandler;
use crate::core::binarycodec::definitions::field_header::FieldHeader;
use crate::core::binarycodec::definitions::field_header::CODE_MAX_VALUE;
use crate::core::binarycodec::definitions::field_header::CODE_MIN_VALUE;
use crate::core::binarycodec::exceptions::XRPLBinaryCodecException;
use alloc::vec;
use alloc::vec::Vec;

fn _encode_field_id(field_header: &FieldHeader) -> Result<Vec<u8>, XRPLBinaryCodecException> {
    let type_code = field_header.type_code;
    let field_code = field_header.field_code;
    let range = CODE_MIN_VALUE..CODE_MAX_VALUE;

    if !range.contains(&field_code) {
        Err(XRPLBinaryCodecException::UnexpectedFieldCodeRange {
            min: CODE_MIN_VALUE as usize,
            max: CODE_MAX_VALUE as usize,
        })
    } else if !range.contains(&type_code) {
        Err(XRPLBinaryCodecException::UnexpectedTypeCodeRange {
            min: CODE_MIN_VALUE as usize,
            max: CODE_MAX_VALUE as usize,
        })
    } else if type_code < 16 && field_code < 16 {
        // high 4 bits is the type_code
        // low 4 bits is the field code
        let combined_code = (type_code << 4) | field_code;
        Ok([combined_code as u8].to_vec())
    } else if type_code >= 16 && field_code < 16 {
        // first 4 bits are zeroes
        // next 4 bits is field code
        // next byte is type code
        let mut result = vec![];
        let byte1 = [field_code as u8];
        let byte2 = [type_code as u8];

        result.extend_from_slice(&byte1);
        result.extend_from_slice(&byte2);

        Ok(result)
    } else if type_code < 16 && field_code >= 16 {
        // first 4 bits is type code
        // next 4 bits are zeroes
        // next byte is field code
        let mut result = vec![];
        let byte1 = [(type_code << 4) as u8];
        let byte2 = [field_code as u8];

        result.extend_from_slice(&byte1);
        result.extend_from_slice(&byte2);

        Ok(result)
    } else {
        // both are >= 16
        // first byte is all zeroes
        // second byte is type code
        // third byte is field code
        let mut result = vec![];
        let byte2 = [type_code as u8];
        let byte3 = [field_code as u8];

        result.extend_from_slice(&[0]);
        result.extend_from_slice(&byte2);
        result.extend_from_slice(&byte3);

        Ok(result)
    }
}

fn _decode_field_id(field_id: &str) -> Result<FieldHeader, XRPLBinaryCodecException> {
    let bytes = hex::decode(field_id)?;

    match bytes.len() {
        1 => {
            let type_code = (bytes[0] >> 4) as i16;
            let field_code = (bytes[0] & 0x0F) as i16;

            Ok(FieldHeader {
                type_code,
                field_code,
            })
        }
        2 => {
            let first_byte = bytes[0];
            let second_byte = bytes[1];
            let first_byte_high_bits = first_byte >> 4;
            let first_byte_low_bits = first_byte & 0x0F;

            if first_byte_high_bits == 0 {
                // Next 4 bits are field code, second byte
                // is type code.
                let type_code = second_byte as i16;
                let field_code = first_byte_low_bits as i16;

                Ok(FieldHeader {
                    type_code,
                    field_code,
                })
            } else {
                // Otherwise, next 4 bits are type code,
                // second byte is field code.
                let type_code = first_byte_high_bits as i16;
                let field_code = second_byte as i16;

                Ok(FieldHeader {
                    type_code,
                    field_code,
                })
            }
        }
        3 => {
            let type_code = bytes[1] as i16;
            let field_code = bytes[2] as i16;

            Ok(FieldHeader {
                type_code,
                field_code,
            })
        }
        _ => Err(XRPLBinaryCodecException::UnexpectedFieldIdByteRange { min: 1, max: 3 }),
    }
}

/// Returns the unique field ID for a given field name.
/// This field ID consists of the type code and field
/// code, in 1 to 3 bytes depending on whether those
/// values are "common" (<16) or "uncommon" (>=16)
pub fn encode(field_name: &str) -> Result<Vec<u8>, XRPLBinaryCodecException> {
    let definitions = load_definition_map();
    let field_header = definitions.get_field_header_from_name(field_name);

    if let Some(header) = field_header {
        _encode_field_id(&header)
    } else {
        Err(XRPLBinaryCodecException::UnknownFieldName)
    }
}

/// Returns the field name represented by the given field ID.
pub fn decode(field_id: &str) -> Result<&str, XRPLBinaryCodecException> {
    let definitions = load_definition_map();
    let field_header = _decode_field_id(field_id)?;
    let field_name = definitions.get_field_name_from_header(&field_header);

    if let Some(name) = field_name {
        Ok(name)
    } else {
        Err(XRPLBinaryCodecException::UnknownFieldName)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::binarycodec::test_cases::load_field_tests;

    #[test]
    fn test_encode() {
        for test in load_field_tests() {
            let result = hex::encode_upper(encode(&test.name).unwrap());
            assert_eq!(test.expected_hex, result)
        }
    }

    #[test]
    fn test_decode() {
        for test in load_field_tests() {
            let result = decode(&test.expected_hex).unwrap();
            assert_eq!(test.name, result)
        }
    }
}

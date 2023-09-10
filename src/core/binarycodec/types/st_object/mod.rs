use alloc::{borrow::Cow, vec::Vec};
use anyhow::Result;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_with::skip_serializing_none;

const SOURCE_TAG: &'static str = "SourceTag";
const DESTINATION_TAG: &'static str = "DestinationTag";
const ACCOUNT_FIELD: &'static str = "Account";
const DESTINATION_FIELD: &'static str = "Destination";
const ST_OBJECT: &'static str = "STObject";
const UNL_MODIFY_TXN: &'static str = "0066";
const OBJECT_END_MARKER_BYTE: &'static str = "E1";

use crate::{
    core::{
        addresscodec::{is_valid_xaddress, xaddress_to_classic_address},
        binarycodec::{binary_wrappers::Serialization, exceptions::XRPLBinaryCodecException},
        definitions::{get_field_instance, FieldInstance},
        BinarySerializer,
    },
    Err,
};

pub struct STObject {
    buffer: Vec<u8>,
}

impl STObject {
    fn from_json_value(value: Value, only_signing: bool) -> Result<Self> {
        let mut serializer = BinarySerializer::new();

        if let Value::Object(object) = value.clone() {
            let mut xaddress_decoded = Value::Object(Map::new());
            for (key, val) in object {
                if let Value::String(v) = val.clone() {
                    if is_valid_xaddress(v.as_str()) {
                        let handled = handle_xaddress(key.as_str(), v.as_str()).unwrap(); // TODO: unwrap
                        match handled.account_field_type {
                            AccountFieldType::Account => {
                                if let Some(handled_tag) = handled.tag {
                                    if let Some(value_tag) = value.get(SOURCE_TAG) {
                                        if handled_tag != value_tag.as_u64().unwrap() {
                                            // TODO: unwrap
                                            return Err!(
                                                XRPLBinaryCodecException::XAddressTagMismatch
                                            );
                                        }
                                    }
                                }
                            }
                            AccountFieldType::Destination => {
                                if let Some(handled_tag) = handled.tag {
                                    if let Some(value_tag) = value.get(DESTINATION_TAG) {
                                        if handled_tag != value_tag.as_u64().unwrap() {
                                            // TODO: unwrap
                                            return Err!(
                                                XRPLBinaryCodecException::XAddressTagMismatch
                                            );
                                        }
                                    }
                                }
                            }
                        }

                        let handled_json = serde_json::to_value(handled).unwrap(); // TODO: unwrap
                        for (k, v) in handled_json.as_object().unwrap().into_iter() {
                            xaddress_decoded
                                .as_object_mut()
                                .unwrap() // TODO: unwrap
                                .insert(k.clone(), v.clone());
                        }
                    } else {
                        xaddress_decoded
                            .as_object_mut()
                            .unwrap() // TODO: unwrap
                            .insert(key.clone(), val.clone());
                    }
                } else {
                    xaddress_decoded
                        .as_object_mut()
                        .unwrap() // TODO: unwrap
                        .insert(key.clone(), val.clone());
                }
            }

            let mut sorted_keys: Vec<FieldInstance> = Vec::new();
            for field_name in xaddress_decoded.as_object().unwrap().keys() {
                // TODO: unwrap
                let field_instance = get_field_instance(&field_name);
                if let Some(instance) = field_instance.clone() {
                    if xaddress_decoded.get(instance.name.clone()).is_some()
                        && instance.is_serialized
                    {
                        sorted_keys.push(instance);
                    }
                }
            }
            sorted_keys.sort_by_key(|x| x.ordinal);

            if only_signing {
                sorted_keys.retain(|x| x.is_signing);
            }
            let mut is_unl_modify = false;
            for field_instance in sorted_keys {
                if let Some(associated_value) = xaddress_decoded.get(field_instance.name.clone()) {
                    if field_instance.name == "TransactionType"
                        && associated_value.as_str().unwrap() == UNL_MODIFY_TXN
                    // TODO: unwrap
                    {
                        is_unl_modify = true
                    }

                    let _is_unl_modify_workaround =
                        field_instance.name == "Account" && is_unl_modify;
                    serializer.write_field_and_value(
                        field_instance.clone(),
                        serde_json::to_vec(associated_value).unwrap().as_slice(), // TODO: unwrap and refactor
                                                                                  // is_unl_modify_workaround,
                    );
                }

                if field_instance.associated_type == ST_OBJECT {
                    // TODO: unwrap
                    serializer.append(&mut hex::decode(OBJECT_END_MARKER_BYTE).unwrap());
                }
            }

            Ok(STObject { buffer: serializer })
        } else {
            todo!("error")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
enum AccountFieldType {
    Account,
    Destination,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, new)]
struct AccountTagPair<'a> {
    #[serde(skip_serializing)]
    account_field_type: AccountFieldType,
    account: Cow<'a, str>,
    tag: Option<u64>,
}

fn handle_xaddress<'a>(field: &'a str, xaddress: &'a str) -> Result<AccountTagPair<'a>> {
    let (classic_address, tag, _) = match xaddress_to_classic_address(xaddress) {
        Ok(v) => v,
        Err(error) => return Err!(error),
    };
    if (field == ACCOUNT_FIELD || field == DESTINATION_FIELD) && tag.is_none() {
        return Err!(XRPLBinaryCodecException::FieldHasNoAssiciatedTag);
    }
    let account_field_type = match field {
        ACCOUNT_FIELD => AccountFieldType::Account,
        DESTINATION_FIELD => AccountFieldType::Destination,
        _ => return Err!(XRPLBinaryCodecException::FieldIsNotAccountOrDestination),
    };

    if let Some(tag) = tag {
        Ok(AccountTagPair::new(
            account_field_type,
            classic_address.into(),
            Some(tag),
        ))
    } else {
        Ok(AccountTagPair::new(
            account_field_type,
            classic_address.into(),
            None,
        ))
    }
}

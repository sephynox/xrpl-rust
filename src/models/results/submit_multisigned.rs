use alloc::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response format for the submit_multisigned method, which applies a
/// multi-signed transaction and sends it to the network to be confirmed
/// and included in future ledgers.
///
/// See Submit Multisigned:
/// `<https://xrpl.org/submit_multisigned.html>`
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SubmitMultisigned<'a> {
    /// Code indicating the preliminary result of the transaction,
    /// for example tesSUCCESS
    pub engine_result: Cow<'a, str>,
    /// Numeric code indicating the preliminary result of the transaction,
    /// directly correlated to engine_result
    pub engine_result_code: i32,
    /// Human-readable explanation of the preliminary transaction result
    pub engine_result_message: Cow<'a, str>,
    /// The complete transaction in hex string format
    pub tx_blob: Cow<'a, str>,
    /// The complete transaction in JSON format
    pub tx_json: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_multisigned_deserialization() {
        let json = r#"{
            "engine_result": "tesSUCCESS",
            "engine_result_code": 0,
            "engine_result_message": "The transaction was applied. Only final in a validated ledger.",
            "status": "success",
            "tx_blob": "120014220004000024000000046380000000000000000000000000000000000000005553440000000000B5F762798A53D543A014CAF8B297CFF8F2F937E868400000000000753073008114A3780F5CB5A44D366520FC44055E8ED44D9A2270F3E010732102B3EC4E5DD96029A647CFA20DA07FE1F85296505552CCAC114087E66B46BD77DF74473045022100CC9C56DF51251CB04BB047E5F3B5EF01A0F4A8A549D7A20A7402BF54BA744064022061EF8EF1BCCBF144F480B32508B1D10FD4271831D5303F920DE41C64671CB5B78114204288D2E47F8EF6C99BCC457966320D12409711E1E010732103398A4EDAE8EE009A5879113EAA5BA15C7BB0F612A87F4103E793AC919BD1E3C174473045022100FEE8D8FA2D06CE49E9124567DCA265A21A9F5465F4A9279F075E4CE27E4430DE022042D5305777DA1A7801446780308897699412E4EDF0E1AEFDF3C8A0532BDE4D0881143A4C02EA95AD6AC3BED92FA036E0BBFB712C030CE1F1",
            "tx_json": {
                "Account": "rEuLyBCvcw4CFmzv8RepSiAoNgF8tTGJQC",
                "Fee": "30000",
                "Flags": 262144,
                "LimitAmount": {
                    "currency": "USD",
                    "issuer": "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh",
                    "value": "0"
                },
                "Sequence": 4,
                "Signers": [
                    {
                        "Signer": {
                            "Account": "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
                            "SigningPubKey": "02B3EC4E5DD96029A647CFA20DA07FE1F85296505552CCAC114087E66B46BD77DF",
                            "TxnSignature": "3045022100CC9C56DF51251CB04BB047E5F3B5EF01A0F4A8A549D7A20A7402BF54BA744064022061EF8EF1BCCBF144F480B32508B1D10FD4271831D5303F920DE41C64671CB5B7"
                        }
                    },
                    {
                        "Signer": {
                            "Account": "raKEEVSGnKSD9Zyvxu4z6Pqpm4ABH8FS6n",
                            "SigningPubKey": "03398A4EDAE8EE009A5879113EAA5BA15C7BB0F612A87F4103E793AC919BD1E3C1",
                            "TxnSignature": "3045022100FEE8D8FA2D06CE49E9124567DCA265A21A9F5465F4A9279F075E4CE27E4430DE022042D5305777DA1A7801446780308897699412E4EDF0E1AEFDF3C8A0532BDE4D08"
                        }
                    }
                ],
                "SigningPubKey": "",
                "TransactionType": "TrustSet",
                "hash": "81A477E2A362D171BB16BE17B4120D9F809A327FA00242ABCA867283BEA2F4F8"
            }
        }"#;

        let submit: SubmitMultisigned = serde_json::from_str(json).unwrap();

        assert_eq!(submit.engine_result, "tesSUCCESS");
        assert_eq!(submit.engine_result_code, 0);
        assert_eq!(
            submit.engine_result_message,
            "The transaction was applied. Only final in a validated ledger."
        );

        // Verify tx_json contents
        let tx_json = submit.tx_json.as_object().unwrap();
        assert_eq!(tx_json["Account"], "rEuLyBCvcw4CFmzv8RepSiAoNgF8tTGJQC");
        assert_eq!(tx_json["Fee"], "30000");
        assert_eq!(tx_json["Flags"], 262144);
        assert_eq!(tx_json["Sequence"], 4);
        assert_eq!(tx_json["TransactionType"], "TrustSet");

        // Verify Signers array
        let signers = tx_json["Signers"].as_array().unwrap();
        assert_eq!(signers.len(), 2);

        // Test first signer
        let first_signer = &signers[0]["Signer"];
        assert_eq!(
            first_signer["Account"],
            "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW"
        );

        // Test serialization roundtrip
        let serialized = serde_json::to_string(&submit).unwrap();
        let deserialized: SubmitMultisigned = serde_json::from_str(&serialized).unwrap();
        assert_eq!(submit, deserialized);
    }
}

pub mod account_delete;
pub mod account_set;
pub mod check_cancel;
pub mod check_cash;
pub mod check_create;
pub mod deposit_preauth;
pub mod escrow_cancel;
pub mod escrow_create;
pub mod escrow_finish;
pub mod nftoken_accept_offer;
pub mod nftoken_burn;
pub mod nftoken_cancel_offer;
pub mod nftoken_create_offer;
pub mod nftoken_mint;
pub mod offer_cancel;
pub mod offer_create;
pub mod payment;
pub mod payment_channel_claim;
pub mod payment_channel_create;
pub mod payment_channel_fund;
pub mod pseudo_transactions;
pub mod set_regular_key;
pub mod signer_list_set;
pub mod ticket_create;
pub mod trust_set;

mod flags_serde {
    use core::fmt::Debug;

    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use strum::IntoEnumIterator;

    pub fn serialize<F, S>(flags: &Option<Vec<F>>, s: S) -> Result<S::Ok, S::Error>
    where
        F: Serialize,
        S: Serializer,
    {
        if let Some(f) = flags {
            let flags_as_value = serde_json::to_value(f).unwrap();
            let flag_num_vec: Vec<u32> = serde_json::from_value(flags_as_value).unwrap();
            s.serialize_u32(flag_num_vec.iter().sum())
        } else {
            s.serialize_u32(0)
        }
    }

    pub fn deserialize<'de, F, D>(d: D) -> Result<Option<Vec<F>>, D::Error>
    where
        F: Serialize + IntoEnumIterator + Debug,
        D: Deserializer<'de>,
    {
        let flags_u32 = u32::deserialize(d)?;

        let mut flags_vec = Vec::new();
        for flag in F::iter() {
            let check_flag: u32 = serde_json::to_string(&flag)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            if check_flag & flags_u32 == check_flag {
                flags_vec.push(flag);
            }
        }
        Ok(Some(flags_vec))
    }
}

//! Serde functionalities

use core::hash::BuildHasherDefault;
use fnv::FnvHasher;

pub type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<FnvHasher>>;

pub mod txn_flags {
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

        if flags_vec.is_empty() {
            Ok(None)
        } else {
            Ok(Some(flags_vec))
        }
    }
}

/// Used for tagged variants in an `untagged` enum
pub mod currency_xrp {
    use super::HashMap;
    use serde::de::Error;
    use serde::{ser::SerializeMap, Deserialize};

    pub fn serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let xrp_currency = [("currency", "XRP")];
        let mut map = serializer.serialize_map(Some(xrp_currency.len()))?;
        for (k, v) in xrp_currency {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let xrp_currency: HashMap<&str, &str> = HashMap::deserialize(deserializer)?;

        if xrp_currency.get("currency").unwrap() == &"XRP" {
            return Ok(());
        }

        // TODO: utilize anyhow and thiserror
        Err("Could not deserialize XRP currency.").map_err(D::Error::custom)
    }
}

/// Source: https://github.com/serde-rs/serde/issues/554#issuecomment-249211775
// TODO: Find a way to `#[skip_serializing_none]`
// TODO: Find a more generic way
#[macro_export]
macro_rules! serialize_with_tag {
    (
        $(#[$attr:meta])*
        pub struct $name:ident<$lt:lifetime> {
            $(
                $field:ident : $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        pub struct $name<$lt> {
            $(
                $field: $ty,
            )*
        }

        impl<$lt> ::serde::Serialize for $name<$lt> {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer
            {
                #[derive(Serialize)]
                #[serde(rename_all = "PascalCase")]
                $(#[$attr])*
                pub struct Helper<$lt> {
                    $(
                        $field: $ty,
                    )*
                }

                let helper = Helper {
                    $(
                        $field: self.$field.clone(),
                    )*
                };

                let mut state = serializer.serialize_map(Some(1))?;
                state.serialize_key(stringify!($name))?;
                state.serialize_value(&helper)?;
                state.end()
            }
        }
    };
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(
                $field:ident : $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            $(
                $field: $ty,
            )*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer
            {
                #[derive(Serialize)]
                #[serde(rename_all = "PascalCase")]
                $(#[$attr])*
                pub struct Helper {
                    $(
                        $field: $ty,
                    )*
                }

                let helper = Helper {
                    $(
                        $field: self.$field.clone(),
                    )*
                };

                let mut state = serializer.serialize_map(Some(1))?;
                state.serialize_key(stringify!($name))?;
                state.serialize_value(&helper)?;
                state.end()
            }
        }
    };
}
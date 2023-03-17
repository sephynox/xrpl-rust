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
        Err("Could not deserialize XRP currency.").map_err(Error::custom)
    }
}

/// Source: https://github.com/serde-rs/serde/issues/554#issuecomment-249211775
// TODO: Find a way to `#[skip_serializing_none]`
// TODO: Find a more generic way
#[macro_export]
macro_rules! serde_with_tag {
    (
        $(#[$attr:meta])*
        pub struct $name:ident<$lt:lifetime> {
            $(
                $(#[$doc:meta])*
                $field:ident : $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        pub struct $name<$lt> {
            $(
                $(#[$doc])*
                $field: $ty,
            )*
        }

        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct Helper<$lt> {
            $(
                $field: $ty,
            )*
        }

        impl<$lt> ::serde::Serialize for $name<$lt> {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer
            {
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

        impl<'de: $lt, $lt> ::serde::Deserialize<'de> for $name<$lt> {
            #[allow(non_snake_case)]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let hash_map: HashMap<&$lt str, Helper<$lt>> = HashMap::deserialize(deserializer)?;
                let helper = hash_map.get(stringify!($name)).unwrap();

                Ok(Self {
                    $(
                        $field: helper.$field.into(),
                    )*
                })
            }
        }
    };
    (
        $(#[$attr:meta])*
        pub struct $name:ident {
            $(
                $(#[$doc:meta])*
                $field:ident : $ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        pub struct $name {
            $(
                $(#[$doc])*
                $field: $ty,
            )*
        }

        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct Helper {
            $(
                $field: $ty,
            )*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer
            {
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

        impl<'de> ::serde::Deserialize<'de> for $name {
            #[allow(non_snake_case)]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let hash_map: HashMap<&'de str, Helper> = HashMap::deserialize(deserializer)?;
                let helper = hash_map.get(stringify!($name)).unwrap();

                Ok(Self {
                    $(
                        $field: helper.$field.clone().into(),
                    )*
                })
            }
        }
    };
}

//! Base model

use alloc::string::String;
use serde_json::Value;

use super::exceptions::XRPLModelException;

pub trait Model {
    /// Returns the json representation of a model as a string.
    fn to_json(&self) -> String {
        serde_json::to_string(&self.to_json_value()).expect("Unable to serialize to json string.")
    }
    /// Returns the json representation of a model as a `serde_json::Value`.
    fn to_json_value(&self) -> Value;
    /// Extended in structures to define custom validation logic.
    fn get_errors(&self) -> Result<(), XRPLModelException> {
        Ok(())
    }

    /// Panics if a object is invalid.
    fn validate(&self) -> Result<(), XRPLModelException> {
        match self.get_errors() {
            Ok(_no_error) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Returns whether the structure is valid.
    fn is_valid(&self) -> bool {
        match self.get_errors() {
            Ok(_no_error) => true,
            Err(_error) => false,
        }
    }
}

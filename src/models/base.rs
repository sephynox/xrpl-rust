//! Base model

use alloc::vec::Vec;
use serde_json::Value;

pub trait Base {
    // TODO: handle unknown return types (representing each model)
    // fn from_json(json: &str) {
    //     serde_json::from_str(json).expect("Could not derive struct from json.")
    // }
    // fn from_json_value(&self, json: Value) -> ;
    /// Returns the json representation of a structure as a string.
    // TODO: Issues with referencing.
    // fn to_json(&self) -> &str;
    /// Returns the json representation of a structure as a `serde_json::Value`.
    fn to_json_value(&self) -> Value;
    /// Extended in structures to define custom validation logic.
    fn get_errors(&self) -> Vec<&str> {
        Vec::new()
    }
    // fn validate(&self) {
    //    let errors = self.get_errors();
    //    if !errors.is_empty() {
    // TODO: should raise a `XRPLModelException` with all errors found.
    //    }
    // }
    /// Returns whether the structure is valid.
    fn is_valid(&self) -> bool {
        self.get_errors().is_empty()
    }
}

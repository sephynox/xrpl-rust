//! Base model

use serde_json::Value;

use super::exceptions::XRPLModelException;

pub trait Model {
    // TODO: handle unknown return types (representing each model)
    // fn from_json(json: &str) {
    //     serde_json::from_str(json).expect("Could not derive struct from json.")
    // }
    // fn from_json_value(&self, json: Value) -> ;
    /// Returns the json representation of a structure as a string.
    // TODO: Issues with referencing.
    // fn to_json(&self) -> &str;
    // / Returns the json representation of a structure as a `serde_json::Value`.
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

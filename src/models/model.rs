//! Base model

use super::exceptions::XRPLModelException;

/// A trait that implements basic functions to every model.
pub trait Model {
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

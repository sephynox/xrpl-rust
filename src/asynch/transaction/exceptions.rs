use thiserror_no_std::Error;

use crate::models::amount::XRPAmount;

#[derive(Error, Debug, PartialEq)]
pub enum XRPLTransactionException<'a> {
    #[error("Fee of {0:?} Drops is much higher than a typical XRP transaction fee. This may be a mistake. If intentional, please use `check_fee = false`")]
    FeeUnusuallyHigh(XRPAmount<'a>),
}

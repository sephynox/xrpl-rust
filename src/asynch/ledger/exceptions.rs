use thiserror_no_std::Error;

use crate::asynch::clients::exceptions::XRPLClientException;

pub type XRPLLedgerHelperResult<T> = Result<T, XRPLLedgerHelperException>;

#[derive(Error, Debug)]
pub enum XRPLLedgerHelperException {
    #[error("0:?")]
    ClientException(#[from] XRPLClientException),
}

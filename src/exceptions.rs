use thiserror_no_std::Error;

pub type XRPLResult<T> = Result<T, XRPLException>;

#[derive(Debug, Error)]
pub enum XRPLException {}

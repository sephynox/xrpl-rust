use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XRPLSignTransactionException<'a> {
    #[error("{0:?} value does not match X-Address tag")]
    TagFieldMismatch(&'a str),
}

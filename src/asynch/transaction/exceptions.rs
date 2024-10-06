use core::num::ParseIntError;

use alloc::string::String;
use thiserror_no_std::Error;

use crate::{
    asynch::{
        account::exceptions::XRPLAccountHelperException, clients::exceptions::XRPLClientException,
        ledger::exceptions::XRPLLedgerHelperException,
    },
    core::keypairs::exceptions::XRPLKeypairsException,
    models::XRPAmount,
};

pub type XRPLTransactionHelperResult<T> = Result<T, XRPLTransactionHelperException<'static>>;

#[derive(Error, Debug)]
pub enum XRPLTransactionHelperException<'a> {
    #[error("Fee of {0:?} Drops is much higher than a typical XRP transaction fee. This may be a mistake. If intentional, please use `check_fee = false`")]
    FeeUnusuallyHigh(XRPAmount<'a>),
    #[error("Unable to parse rippled version: {0}")]
    ParseRippledVersionError(ParseIntError),
    #[error("Invalid rippled version: {0}")]
    InvalidRippledVersion(String),
    #[error("0:?")]
    SignTransactionException(#[from] XRPLSignTransactionException),
    #[error("0:?")]
    SubmitAndWaitException(#[from] XRPLSubmitAndWaitException),
    #[error("0:?")]
    FromHexError(#[from] hex::FromHexError),
    #[error("0:?")]
    KeyPairsException(#[from] XRPLKeypairsException),
    #[error("0:?")]
    ClientException(#[from] XRPLClientException),
    #[error("0:?")]
    AccountHelperException(#[from] XRPLAccountHelperException),
    #[error("0:?")]
    LedgerHelperException(#[from] XRPLLedgerHelperException),
    #[error("0:?")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XRPLSignTransactionException {
    #[error("{0:?} value does not match X-Address tag")]
    TagFieldMismatch(String),
    #[error("Fee value of {0:?} is likely entered incorrectly, since it is much larger than the typical XRP transaction cost. If this is intentional, use `check_fee=Some(false)`.")]
    FeeTooHigh(String),
    #[error("Wallet is required to sign transaction")]
    WalletRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XRPLSubmitAndWaitException {
    #[error("Transaction submission failed: {0}")]
    SubmissionFailed(String),
    #[error("The latest validated ledger sequence {validated_ledger_sequence} is greater than the LastLedgerSequence {last_ledger_sequence} in the Transaction. Prelim result: {prelim_result}")]
    SubmissionTimeout {
        last_ledger_sequence: u32,
        validated_ledger_sequence: u32,
        prelim_result: String,
    },
    #[error("Expected field in the transaction metadata: {0}")]
    ExpectedFieldInTxMeta(String),
}

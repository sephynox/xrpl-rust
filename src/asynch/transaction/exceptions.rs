use core::num::ParseIntError;

use alloc::string::String;
use thiserror_no_std::Error;

// XRPLSignTransactionException now lives in `crate::signing::exceptions`.
// Re-exported here for backward compatibility.
pub use crate::signing::exceptions::XRPLSignTransactionException;

#[derive(Error, Debug, PartialEq)]
#[non_exhaustive]
pub enum XRPLTransactionHelperException {
    #[error("Fee of {0:?} Drops is much higher than a typical XRP transaction fee. This may be a mistake. If intentional, please use `check_fee = false`")]
    FeeUnusuallyHigh(String),
    #[error("Unable to parse rippled version: {0}")]
    ParseRippledVersionError(ParseIntError),
    #[error("Invalid rippled version: {0}")]
    InvalidRippledVersion(String),
    #[error("XRPL Sign Transaction error: {0}")]
    XRPLSignTransactionError(#[from] XRPLSignTransactionException),
    #[error("XRPL Submit and Wait error: {0}")]
    XRPLSubmitAndWaitError(#[from] XRPLSubmitAndWaitException),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum XRPLSubmitAndWaitException {
    /// The transaction was rejected before or after it reached the ledger.
    ///
    /// `result_code` is the code rippled returned:
    /// - `tem*` — malformed / rejected at submission time,
    /// - `tec*` / `tef*` — validated by the ledger but rejected by the transactor,
    /// - `"txnNotFound"` / other RPC error names — the ledger did not have the
    ///   transaction on lookup and the polling loop gave up,
    /// - `"submission_timeout"` — the polling loop exhausted its retries
    ///   without seeing the transaction validated.
    ///
    /// `message` is a human-readable explanation (e.g. rippled's
    /// `engine_result_message`) when one is available.
    ///
    /// Callers can pattern-match on `result_code` without substring-parsing
    /// the `Display` output. See the crate-level PR discussion for a possible
    /// future split into `SubmissionFailed` (tem*, network/RPC) and
    /// `TransactionRejected` (tec*/tef* — the ledger accepted the tx and the
    /// transactor rejected it).
    #[error("Transaction submission failed: {result_code}{}", .message.as_deref().map(|m| alloc::format!(" ({m})")).unwrap_or_default())]
    SubmissionFailed {
        result_code: String,
        message: Option<String>,
    },
    #[error("The latest validated ledger sequence {validated_ledger_sequence} is greater than the LastLedgerSequence {last_ledger_sequence} in the Transaction. Prelim result: {prelim_result}")]
    SubmissionTimeout {
        last_ledger_sequence: u32,
        validated_ledger_sequence: u32,
        prelim_result: String,
    },
    #[error("Expected field in the transaction metadata: {0}")]
    ExpectedFieldInTxMeta(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `Display` output includes both the result code and, when present,
    /// the accompanying message in parentheses. This shape is documented in
    /// the variant's rustdoc and is what fallback substring-parsing (if any)
    /// will see.
    #[test]
    fn submission_failed_display_with_message() {
        let err = XRPLSubmitAndWaitException::SubmissionFailed {
            result_code: "temBAD_FEE".to_string(),
            message: Some("Fee is invalid".to_string()),
        };
        assert_eq!(
            err.to_string(),
            "Transaction submission failed: temBAD_FEE (Fee is invalid)"
        );
    }

    /// When there is no message (typical for the tec*/tef* path — the meta
    /// object carries the code but no separate human-readable string), the
    /// parenthesized clause is omitted.
    #[test]
    fn submission_failed_display_without_message() {
        let err = XRPLSubmitAndWaitException::SubmissionFailed {
            result_code: "tecBYTECODE_REJECTED".to_string(),
            message: None,
        };
        assert_eq!(
            err.to_string(),
            "Transaction submission failed: tecBYTECODE_REJECTED"
        );
    }

    /// Callers pattern-matching on the struct-variant shape can pull the
    /// result_code out without touching the Display string.
    #[test]
    fn submission_failed_pattern_match_on_result_code() {
        let err = XRPLSubmitAndWaitException::SubmissionFailed {
            result_code: "tecBYTECODE_REJECTED".to_string(),
            message: None,
        };
        match err {
            XRPLSubmitAndWaitException::SubmissionFailed { result_code, .. } => {
                assert_eq!(result_code, "tecBYTECODE_REJECTED");
                assert!(result_code.starts_with("tec"));
            }
            _ => panic!("expected SubmissionFailed variant"),
        }
    }
}

//! General XRPL Model Exception.

use alloc::string::String;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, PartialEq, Display)]
#[non_exhaustive]
pub enum XRPLModelException {
    InvalidICCannotBeXRP,
    XRPLTransactionError(XRPLTransactionException),
    XRPLRequestError(XRPLRequestException),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum XRPLTransactionException {
    AccountSetError(AccountSetException),
    CheckCashError(CheckCashException),
    DepositPreauthError(DepositPreauthException),
    EscrowCreateError(EscrowCreateException),
    EscrowFinishError(EscrowFinishException),
    NFTokenAcceptOfferError(NFTokenAcceptOfferException),
    NFTokenCancelOfferError(NFTokenCancelOfferException),
    NFTokenCreateOfferError(NFTokenCreateOfferException),
    NFTokenMintError(NFTokenMintException),
    PaymentError(PaymentException),
    SignerListSetError(SignerListSetException),
    UNLModifyError(UNLModifyException),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum AccountSetException {
    InvalidTickSizeTooHigh { max: u32, found: u32 },
    InvalidTickSizeTooLow { min: u32, found: u32 },
    InvalidTransferRateTooHigh { max: u32, found: u32 },
    InvalidTransferRateTooLow { min: u32, found: u32 },
    InvalidDomainIsNotLowercase,
    InvalidDomainTooLong { max: usize, found: usize },
    InvalidClearFlagMustNotEqualSetFlag,
    InvalidMustSetAsfAuthorizedNftokenMinterFlagToSetMinter,
    InvalidNftokenMinterMustBeSetIfAsfAuthorizedNftokenMinterIsSet,
    InvalidNftokenMinterMustNotBeSetIfAsfAuthorizedNftokenMinterIsUnset,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum CheckCashException {
    InvalidMustSetAmountOrDeliverMin,
    InvalidMustNotSetAmountAndDeliverMin,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum DepositPreauthException {
    InvalidMustSetAuthorizeOrUnauthorize,
    InvalidMustNotSetAuthorizeAndUnauthorize,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum EscrowCreateException {
    InvalidCancelAfterMustNotBeBeforeFinishAfter,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum EscrowFinishException {
    InvalidIfOneSetBothConditionAndFulfillmentMustBeSet,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum NFTokenAcceptOfferException {
    InvalidMustSetEitherNftokenBuyOfferOrNftokenSellOffer,
    InvalidBrokerFeeMustBeGreaterZero,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum NFTokenCancelOfferException {
    InvalidMustIncludeOneNFTokenOffer,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum NFTokenCreateOfferException {
    InvalidAmountMustBeGreaterZero,
    InvalidDestinationMustNotEqualAccount,
    InvalidOwnerMustBeSetForBuyOffer,
    InvalidOwnerMustNotBeSetForSellOffer,
    InvalidOwnerMustNotEqualAccount,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum NFTokenMintException {
    InvalidIssuerMustNotEqualAccount,
    InvalidTransferFeeTooHigh { max: u32, found: u32 },
    InvalidURITooLong { max: usize, found: usize },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum PaymentException {
    InvalidXRPtoXRPPaymentsCannotContainPaths,
    InvalidDestinationMustNotEqualAccountForXRPtoXRPPayments,
    InvalidSendMaxMustBeSetForPartialPayments,
    InvalidDeliverMinMustNotBeSetForNonPartialPayments,
    InvalidSendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
    InvalidSendMaxMustBeSetForExchanges,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignerListSetException {
    InvalidMustNotSetSignerEntriesIfSignerListIsBeingDeleted,
    InvalidSignerQuorumMustBeZeroIfSignerListIsBeingDeleted,
    InvalidSignerQuorumMustBeGreaterZero,
    InvalidTooFewSignerEntries { min: usize, found: usize },
    InvalidTooManySignerEntries { max: usize, found: usize },
    InvalidAccountMustNotBeInSignerEntry,
    InvalidMustBeLessOrEqualToSumOfSignerWeightInSignerEntries { max: u32, found: u32 },
    InvalidAnAccountCanNotBeInSignerEntriesTwice,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum UNLModifyException {
    InvalidUNLModifyDisablingMustBeOneOrTwo,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum XRPLRequestException {
    ChannelAuthorizeError(ChannelAuthorizeException),
    SignAndSubmitError(SignAndSubmitException),
    SignForError(SignForException),
    SignError(SignException),
    LedgerEntryError(LedgerEntryException),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ChannelAuthorizeException {
    InvalidMustSetExactlyOneOf { fields: String },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum LedgerEntryException {
    InvalidMustSetExactlyOneOf { fields: String },
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignAndSubmitException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignForException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum SignException {
    InvalidMustSetExactlyOneOf { fields: String },
    InvalidMustOmitKeyTypeIfSecretProvided,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct JSONRPCException {
    code: i32,
    message: String,
}

#[cfg(feature = "std")]
impl alloc::error::Error for XRPLModelException {}

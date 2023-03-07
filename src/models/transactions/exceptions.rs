use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XRPLTransactionException<'a> {
    AccountSetError(XrplAccountSetException<'a>),
    CheckCashError(XrplCheckCashException<'a>),
    DepositPreauthError(XrplDepositPreauthException<'a>),
    EscrowCreateError(XrplEscrowCreateException<'a>),
    EscrowFinishError(XrplEscrowFinishException<'a>),
    NFTokenAcceptOfferError(XrplNFTokenAcceptOfferException<'a>),
    NFTokenCancelOfferError(XrplNFTokenCancelOfferException<'a>),
    NFTokenCreateOfferError(XrplNFTokenCreateOfferException<'a>),
    NFTokenMintError(XrplNFTokenMintException<'a>),
    PaymentError(XrplPaymentException<'a>),
    SignerListSetError(XrplSignerListSetException<'a>),
    UNLModifyError(XrplUNLModifyException<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplAccountSetException<'a> {
    #[error("For more information see: {resource:?}")]
    ValueTooHigh { max: u32, found: u32 },
    #[error("For more information see: {resource:?}")]
    ValueTooLow { min: u32, found: u32 },
    #[error("For more information see: {resource:?}")]
    ValueTooLong,
    #[error("For more information see: {resource:?}")]
    InvalidValueFormat {},
    #[error("For more information see: {resource:?}")]
    FieldRequiresFlag {},
    #[error("For more information see: {resource:?}")]
    FlagRequiresField {},
    #[error("For more information see: {resource:?}")]
    SetAndUnsetSameFlag,
    #[error("For more information see: {resource:?}")]
    SetFieldWhenUnsetRequiredFlag,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplCheckCashException<'a> {
    MustSetAmountOrDeliverMin,
    MustNotSetAmountAndDeliverMin,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplDepositPreauthException<'a> {
    MustSetAuthorizeOrUnauthorize,
    MustNotSetAuthorizeAndUnauthorize,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplEscrowCreateException<'a>{
    CancelAfterMustNotBeBeforeFinishAfter,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplEscrowFinishException<'a> {
    IfOneSetBothConditionAndFulfillmentMustBeSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenAcceptOfferException<'a> {
    MustSetEitherNftokenBuyOfferOrNftokenSellOffer,
    BrokerFeeMustBeGreaterZero,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenCancelOfferException<'a> {
    MustIncludeOneNFTokenOffer,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenCreateOfferException<'a> {
    AmountMustBeGreaterZero,
    DestinationMustNotEqualAccount,
    OwnerMustBeSetForBuyOffer,
    OwnerMustNotBeSetForSellOffer,
    OwnerMustNotEqualAccount,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenMintException<'a> {
    IssuerMustNotEqualAccount,
    TransferFeeTooHigh { max: u32, found: u32 },
    URITooLong { max: usize, found: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplPaymentException<'a> {
    XRPtoXRPPaymentsCannotContainPaths,
    DestinationMustNotEqualAccountForXRPtoXRPPayments,
    SendMaxMustBeSetForPartialPayments,
    DeliverMinMustNotBeSetForNonPartialPayments,
    SendMaxMustNotBeSetForXRPtoXRPNonPartialPayments,
    SendMaxMustBeSetForExchanges,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplSignerListSetException<'a> {
    MustNotSetSignerEntriesIfSignerListIsBeingDeleted,
    SignerQuorumMustBeZeroIfSignerListIsBeingDeleted,
    TooFewSignerEntries { min: usize, found: usize },
    TooManySignerEntries { max: usize, found: usize },
    AccountMustNotBeInSignerEntry,
    MustBeLessOrEqualToSumOfSignerWeightInSignerEntries { max: u32, found: u32 },
    AnAccountCanNotBeInSignerEntriesTwice,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplUNLModifyException<'a> {
    UNLModifyDisablingMustBeOneOrTwo,
}

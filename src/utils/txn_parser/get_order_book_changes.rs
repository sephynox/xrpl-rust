use crate::{models::results::metadata::TransactionMetadata, utils::exceptions::XRPLUtilsResult};

use super::utils::AccountOfferChanges;

/// Parse all order book changes from a transaction's metadata.
pub fn get_order_book_changes<'a: 'b, 'b>(
    metadata: &'a TransactionMetadata<'a>,
) -> XRPLUtilsResult<AccountOfferChanges<'b>> {
    todo!()
}

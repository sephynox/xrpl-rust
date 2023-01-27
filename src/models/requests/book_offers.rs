use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Currency, Model, RequestMethod};

/// The book_offers method retrieves a list of offers, also known
/// as the order book, between two currencies.
///
/// See Book Offers:
/// `<https://xrpl.org/book_offers.html#book_offers>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct BookOffers<'a> {
    /// Specification of which currency the account taking
    /// the offer would receive, as an object with currency
    /// and issuer fields (omit issuer for XRP),
    /// like currency amounts.
    pub taker_gets: Currency,
    /// Specification of which currency the account taking
    /// the offer would pay, as an object with currency and
    /// issuer fields (omit issuer for XRP),
    /// like currency amounts.
    pub taker_pays: Currency,
    /// The unique request id.
    pub id: Option<&'a str>,
    /// A 20-byte hex string for the ledger version to use.
    pub ledger_hash: Option<&'a str>,
    /// The ledger index of the ledger to use, or a shortcut
    /// string to choose a ledger automatically.
    pub ledger_index: Option<&'a str>,
    /// If provided, the server does not provide more than
    /// this many offers in the results. The total number of
    /// results returned may be fewer than the limit,
    /// because the server omits unfunded offers.
    pub limit: Option<u16>,
    /// The Address of an account to use as a perspective.
    /// Unfunded offers placed by this account are always
    /// included in the response. (You can use this to look
    /// up your own orders to cancel them.)
    pub taker: Option<&'a str>,
    /// The request method.
    #[serde(default = "RequestMethod::book_offers")]
    pub command: RequestMethod,
}

impl Default for BookOffers<'static> {
    fn default() -> Self {
        BookOffers {
            taker_gets: Currency::XRP,
            taker_pays: Currency::XRP,
            id: None,
            ledger_hash: None,
            ledger_index: None,
            limit: None,
            taker: None,
            command: RequestMethod::BookOffers,
        }
    }
}

impl Model for BookOffers<'static> {}

#[cfg(test)]
mod test {
    use crate::models::Currency;
    use alloc::borrow::Cow::Borrowed;

    use super::BookOffers;

    #[test]
    fn test_serde() {
        let txn = BookOffers {
            taker_gets: Currency::IssuedCurrency {
                currency: Borrowed("EUR"),
                issuer: Borrowed("rTestIssuer"),
            },
            taker_pays: Currency::XRP,
            ..Default::default()
        };
        let txn_json = serde_json::to_string(&txn).unwrap();
    }
}

use alloc::borrow::Cow;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::currency::Currency;
use crate::models::{requests::RequestMethod, Model, PathStep};

use super::{CommonFields, Request};

/// A path is an array. Each member of a path is an object that specifies a step on that path.
pub type Path<'a> = Vec<PathStep<'a>>;

/// There are three different modes, or sub-commands, of
/// the path_find command. Specify which one you want with
/// the subcommand parameter:
/// * create - Start sending pathfinding information
/// * close - Stop sending pathfinding information
/// * status - Info on the currently-open pathfinding request
///
/// See Path Find:
/// `<https://xrpl.org/path_find.html>`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum PathFindSubcommand {
    #[default]
    Create,
    Close,
    Status,
}

/// WebSocket API only! The path_find method searches for
/// a path along which a transaction can possibly be made,
/// and periodically sends updates when the path changes
/// over time. For a simpl<'a>er version that is supported by
/// JSON-RPC, see the ripple_path_find method. For payments
/// occurring strictly in XRP, it is not necessary to find
/// a path, because XRP can be sent directly to any account.
///
/// Although the rippled server tries to find the cheapest
/// path or combination of paths for making a payment, it is
/// not guaranteed that the paths returned by this method
/// are, in fact, the best paths. Due to server load,
/// pathfinding may not find the best results. Additionally,
/// you should be careful with the pathfinding results from
/// untrusted servers. A server could be modified to return
/// less-than-optimal paths to earn money for its operators.
/// If you do not have your own server that you can trust
/// with pathfinding, you should compare the results of
/// pathfinding from multiple servers run by different
/// parties, to minimize the risk of a single server
/// returning poor results. (Note: A server returning
/// less-than-optimal results is not necessarily proof of
/// malicious behavior; it could also be a symptom of heavy
/// server load.)
///
/// See Path Find:
/// `<https://xrpl.org/path_find.html>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PathFind<'a> {
    /// The common fields shared by all requests.
    #[serde(flatten)]
    pub common_fields: CommonFields<'a>,
    /// Unique address of the account to find a path to.
    /// (In other words, the account that would receive a payment.)
    pub destination_account: Cow<'a, str>,
    /// Currency Amount that the destination account would
    /// receive in a transaction. Special case: New in: rippled 0.30.0
    /// You can specify "-1" (for XRP) or provide -1 as the contents of
    /// the value field (for non-XRP currencies). This requests a path
    /// to deliver as much as possible, while spending no more than
    /// the amount specified in send_max (if provided).
    pub destination_amount: Currency<'a>,
    /// Unique address of the account to find a path
    /// from. (In other words, the account that would
    /// be sending a payment.)
    pub source_account: Cow<'a, str>,
    /// Use "create" to send the create sub-command.
    pub subcommand: PathFindSubcommand,
    /// Array of arrays of objects, representing payment paths to check.
    /// You can use this to keep updated on changes to particular paths
    /// you already know about, or to check the overall cost to make a
    /// payment along a certain path.
    pub paths: Option<Vec<Path<'a>>>,
    /// Currency Amount that would be spent in the transaction.
    /// Not compatible with source_currencies.
    pub send_max: Option<Currency<'a>>,
}

impl<'a> Model for PathFind<'a> {}

impl<'a> Request<'a> for PathFind<'a> {
    fn get_common_fields(&self) -> &CommonFields<'a> {
        &self.common_fields
    }

    fn get_common_fields_mut(&mut self) -> &mut CommonFields<'a> {
        &mut self.common_fields
    }
}

impl<'a> PathFind<'a> {
    pub fn new(
        id: Option<Cow<'a, str>>,
        destination_account: Cow<'a, str>,
        destination_amount: Currency<'a>,
        source_account: Cow<'a, str>,
        subcommand: PathFindSubcommand,
        paths: Option<Vec<Vec<PathStep<'a>>>>,
        send_max: Option<Currency<'a>>,
    ) -> Self {
        Self {
            common_fields: CommonFields {
                command: RequestMethod::PathFind,
                id,
            },
            subcommand,
            source_account,
            destination_account,
            destination_amount,
            send_max,
            paths,
        }
    }
}

use crate::models::Model;
use anyhow::Result;
use serde::Serialize;

/// Interface for all network clients to follow.
// TODO: `T` should implement a trait `Request`
// TODO: `R` should implement a trait `Response`
pub trait Client<'a> {
    async fn request_impl<T: Model + Serialize, R>(&mut self, request: T) -> Result<R>;
}

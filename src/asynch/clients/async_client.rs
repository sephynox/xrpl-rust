use super::client::Client;
use crate::models::Model;
use anyhow::Result;
use serde::Serialize;

/// Interface for all async network clients to follow.
pub trait AsyncClient<'a>: Client<'a> {
    async fn request<T: Model + Serialize, R>(&mut self, request: T) -> Result<R> {
        self.request_impl(request).await
    }
}

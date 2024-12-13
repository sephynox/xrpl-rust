use crate::models::{
    requests::{Request, XRPLRequest},
    results::XRPLResponse,
};
use alloc::borrow::Cow;
use url::Url;

use super::exceptions::XRPLClientResult;

/// A trait to implement request functionality for XRPL clients.
#[allow(async_fn_in_trait)]
pub trait XRPLClient {
    /// The driver for the request implementation. This function must be implemented individually for each client.
    async fn request_impl<'a: 'b, 'b>(
        &self,
        request: XRPLRequest<'a>,
    ) -> XRPLClientResult<XRPLResponse<'b>>;

    /// Get the URL of the host the client is connected to.
    fn get_host(&self) -> Url;

    /// Set the request ID for a request if it is not already set. If the ID is already set, it will not be changed.
    /// If no ID is provided and the `std` feature is enabled, a random ID will be generated. If the `std` feature is not enabled, this function will panic.
    fn set_request_id(&self, request: &mut XRPLRequest<'_>) {
        let common_fields = request.get_common_fields_mut();
        common_fields.id = match &common_fields.id {
            Some(id) => Some(id.clone()),
            None => {
                #[cfg(feature = "std")]
                {
                    Some(self.get_random_id())
                }
                #[cfg(not(feature = "std"))]
                unimplemented!(
                    "Random ID generation is not supported in no_std. Please provide an ID."
                )
            }
        };
    }

    /// Generate a random id for a request if the `std` feature is enabled.
    #[cfg(feature = "std")]
    fn get_random_id<'a>(&self) -> Cow<'a, str> {
        use alloc::string::ToString;

        let random_id = rand::random::<u32>().to_string();

        Cow::Owned(random_id)
    }
}

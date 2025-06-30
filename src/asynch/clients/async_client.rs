use super::{client::XRPLClient, exceptions::XRPLClientResult, CommonFields};
use crate::models::{
    requests::{server_state::ServerState, XRPLRequest},
    results::{server_state::ServerState as ServerStateResult, XRPLResponse},
};
use alloc::string::String;

#[allow(async_fn_in_trait)]
pub trait XRPLAsyncClient: XRPLClient {
    async fn request<'a: 'b, 'b>(&self, request: XRPLRequest<'a>) -> XRPLClientResult<String> {
        self.request_impl(request).await
    }

    async fn get_common_fields(&self) -> XRPLClientResult<CommonFields<'_>> {
        let server_state_raw = self.request(ServerState::new(None).into()).await?;
        let server_state: XRPLResponse<'_, ServerStateResult> =
            serde_json::from_str(&server_state_raw)?;
        let server_state = server_state.result.unwrap(); // TODO
        let common_fields = CommonFields {
            network_id: None, // TODO Server state has no network ID.
            build_version: Some(server_state.state.build_version),
        };

        Ok(common_fields)
    }
}

impl<T: XRPLClient> XRPLAsyncClient for T {}

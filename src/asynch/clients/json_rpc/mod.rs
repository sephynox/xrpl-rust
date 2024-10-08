use alloc::{string::ToString, vec};
use anyhow::Result;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::{models::results::XRPLResponse, Err};

mod exceptions;
pub use exceptions::XRPLJsonRpcException;

use super::client::XRPLClient;

/// Renames the requests field `command` to `method` for JSON-RPC.
fn request_to_json_rpc(request: &impl Serialize) -> Result<Value> {
    let mut json_rpc_request = Map::new();
    let mut request = match serde_json::to_value(request) {
        Ok(request) => match request.as_object().cloned() {
            Some(request) => request,
            None => todo!("Handle non-object requests"),
        },
        Err(error) => return Err!(error),
    };
    if let Some(command) = request.remove("command") {
        json_rpc_request.insert("method".to_string(), command);
        json_rpc_request.insert(
            "params".to_string(),
            serde_json::Value::Array(vec![Value::Object(request)]),
        );
    }

    Ok(Value::Object(json_rpc_request))
}

#[cfg(all(feature = "json-rpc", feature = "std"))]
mod _std {
    use crate::{
        asynch::clients::XRPLFaucet,
        models::requests::{FundFaucet, XRPLRequest},
    };

    use super::*;
    use alloc::string::ToString;
    use reqwest::Client as HttpClient;
    use url::Url;

    pub struct AsyncJsonRpcClient {
        url: Url,
    }

    impl AsyncJsonRpcClient {
        pub fn connect(url: Url) -> Self {
            Self { url }
        }
    }

    impl XRPLClient for AsyncJsonRpcClient {
        async fn request_impl<'a: 'b, 'b>(
            &self,
            request: XRPLRequest<'a>,
        ) -> Result<XRPLResponse<'b>> {
            let client = HttpClient::new();
            let request_json_rpc = request_to_json_rpc(&request)?;
            let response = client
                .post(self.url.as_ref())
                .json(&request_json_rpc)
                .send()
                .await;
            match response {
                Ok(response) => match response.text().await {
                    Ok(response) => {
                        Ok(serde_json::from_str::<XRPLResponse<'b>>(&response).unwrap())
                    }
                    Err(error) => Err!(error),
                },
                Err(error) => Err!(error),
            }
        }

        fn get_host(&self) -> Url {
            self.url.clone()
        }
    }

    impl XRPLFaucet for AsyncJsonRpcClient {
        async fn request_funding(&self, url: Option<Url>, request: FundFaucet<'_>) -> Result<()> {
            let faucet_url = self.get_faucet_url(url)?;
            let client = HttpClient::new();
            let request_json_rpc = serde_json::to_value(&request).unwrap();
            let response = client
                .post(faucet_url.to_string())
                .json(&request_json_rpc)
                .send()
                .await;
            match response {
                Ok(response) => {
                    if response.status().is_success() {
                        Ok(())
                    } else {
                        todo!()
                        // Err!(XRPLJsonRpcException::RequestError())
                    }
                }
                Err(error) => {
                    Err!(error)
                }
            }
        }
    }
}

#[cfg(all(feature = "json-rpc", not(feature = "std")))]
mod _no_std {
    use crate::{
        asynch::clients::{SingleExecutorMutex, XRPLFaucet},
        models::requests::{FundFaucet, XRPLRequest},
    };

    use super::*;
    use alloc::sync::Arc;
    use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
    use embedded_nal_async::{Dns, TcpConnect};
    use reqwless::{
        client::{HttpClient, TlsConfig},
        headers::ContentType,
        request::{Method, RequestBuilder},
    };
    use url::Url;

    pub struct AsyncJsonRpcClient<'a, const BUF: usize, T, D, M = SingleExecutorMutex>
    where
        M: RawMutex,
        T: TcpConnect + 'a,
        D: Dns + 'a,
    {
        url: Url,
        client: Arc<Mutex<M, HttpClient<'a, T, D>>>,
    }

    impl<'a, const BUF: usize, T, D, M> AsyncJsonRpcClient<'a, BUF, T, D, M>
    where
        M: RawMutex,
        T: TcpConnect + 'a,
        D: Dns + 'a,
    {
        pub fn connect(url: Url, tcp: &'a T, dns: &'a D) -> Self {
            Self {
                url,
                client: Arc::new(Mutex::new(HttpClient::new(tcp, dns))),
            }
        }

        pub fn connect_with_tls(url: Url, tcp: &'a T, dns: &'a D, tls: TlsConfig<'a>) -> Self {
            Self {
                url,
                client: Arc::new(Mutex::new(HttpClient::new_with_tls(tcp, dns, tls))),
            }
        }
    }

    impl<const BUF: usize, T, D, M> XRPLClient for AsyncJsonRpcClient<'_, BUF, T, D, M>
    where
        M: RawMutex,
        T: TcpConnect,
        D: Dns,
    {
        async fn request_impl<'a: 'b, 'b>(
            &self,
            request: XRPLRequest<'a>,
        ) -> Result<XRPLResponse<'b>> {
            let request_json_rpc = request_to_json_rpc(&request)?;
            let request_string = request_json_rpc.to_string();
            let request_buf = request_string.as_bytes();
            let mut rx_buffer = [0; BUF];
            let mut client = self.client.lock().await;
            let response = match client.request(Method::POST, self.url.as_str()).await {
                Ok(client) => {
                    if let Err(_error) = client
                        .body(request_buf)
                        .content_type(ContentType::ApplicationJson)
                        .send(&mut rx_buffer)
                        .await
                    {
                        Err!(XRPLJsonRpcException::ReqwlessError)
                    } else {
                        match serde_json::from_slice::<XRPLResponse<'_>>(&rx_buffer) {
                            Ok(response) => Ok(response),
                            Err(error) => Err!(error),
                        }
                    }
                }
                Err(_error) => Err!(XRPLJsonRpcException::ReqwlessError),
            };

            response
        }

        fn get_host(&self) -> Url {
            self.url.clone()
        }
    }

    impl<'a, const BUF: usize, T, D, M> XRPLFaucet for AsyncJsonRpcClient<'a, BUF, T, D, M>
    where
        M: RawMutex,
        T: TcpConnect + 'a,
        D: Dns + 'a,
    {
        async fn request_funding(&self, url: Option<Url>, request: FundFaucet<'_>) -> Result<()> {
            let faucet_url = self.get_faucet_url(url)?;
            let request_json_rpc = serde_json::to_value(&request).unwrap();
            let request_string = request_json_rpc.to_string();
            let request_buf = request_string.as_bytes();
            let mut rx_buffer = [0; BUF];
            let mut client = self.client.lock().await;
            let response = match client.request(Method::POST, faucet_url.as_str()).await {
                Ok(client) => {
                    if let Err(_error) = client
                        .body(request_buf)
                        .content_type(ContentType::ApplicationJson)
                        .send(&mut rx_buffer)
                        .await
                    {
                        Err!(XRPLJsonRpcException::ReqwlessError)
                    } else {
                        if let Ok(response) = serde_json::from_slice::<XRPLResponse<'_>>(&rx_buffer)
                        {
                            if response.is_success() {
                                Ok(())
                            } else {
                                todo!()
                                // Err!(XRPLJsonRpcException::RequestError())
                            }
                        } else {
                            Err!(XRPLJsonRpcException::ReqwlessError)
                        }
                    }
                }
                Err(_error) => Err!(XRPLJsonRpcException::ReqwlessError),
            };

            response
        }
    }
}

#[cfg(all(feature = "json-rpc", not(feature = "std")))]
pub use _no_std::AsyncJsonRpcClient;
#[cfg(all(feature = "json-rpc", feature = "std"))]
pub use _std::AsyncJsonRpcClient;

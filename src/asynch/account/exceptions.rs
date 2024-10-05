use thiserror_no_std::Error;

use crate::{
    asynch::clients::exceptions::XRPLClientException,
    core::addresscodec::exceptions::XRPLAddressCodecException,
};

pub type XRPLAccountHelperResult<T> = Result<T, XRPLAccountHelperException>;

#[derive(Debug, Error)]
pub enum XRPLAccountHelperException {
    #[error("{0}")]
    AddressCodecException(#[from] XRPLAddressCodecException),
    #[error("{0}")]
    ClientException(#[from] XRPLClientException),
}

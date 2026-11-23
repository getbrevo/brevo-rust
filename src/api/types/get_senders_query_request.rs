pub use crate::prelude::*;

/// Query parameters for getSenders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSendersQueryRequest {
    /// Filter your senders for a specific ip. **Available for dedicated IP usage only**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Filter your senders for a specific domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl GetSendersQueryRequest {
    pub fn builder() -> GetSendersQueryRequestBuilder {
        <GetSendersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSendersQueryRequestBuilder {
    ip: Option<String>,
    domain: Option<String>,
}

impl GetSendersQueryRequestBuilder {
    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSendersQueryRequest`].
    pub fn build(self) -> Result<GetSendersQueryRequest, BuildError> {
        Ok(GetSendersQueryRequest {
            ip: self.ip,
            domain: self.domain,
        })
    }
}

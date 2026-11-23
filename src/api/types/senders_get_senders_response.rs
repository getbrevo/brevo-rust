pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSendersResponse {
    /// List of the senders available in your account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senders: Option<Vec<GetSendersResponseSendersItem>>,
}

impl GetSendersResponse {
    pub fn builder() -> GetSendersResponseBuilder {
        <GetSendersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSendersResponseBuilder {
    senders: Option<Vec<GetSendersResponseSendersItem>>,
}

impl GetSendersResponseBuilder {
    pub fn senders(mut self, value: Vec<GetSendersResponseSendersItem>) -> Self {
        self.senders = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSendersResponse`].
    pub fn build(self) -> Result<GetSendersResponse, BuildError> {
        Ok(GetSendersResponse {
            senders: self.senders,
        })
    }
}

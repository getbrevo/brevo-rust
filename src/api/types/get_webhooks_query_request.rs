pub use crate::prelude::*;

/// Query parameters for getWebhooks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWebhooksQueryRequest {
    /// Filter on webhook type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GetWebhooksRequestType>,
    /// Sort the results in the ascending/descending order of webhook creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetWebhooksRequestSort>,
}

impl GetWebhooksQueryRequest {
    pub fn builder() -> GetWebhooksQueryRequestBuilder {
        <GetWebhooksQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWebhooksQueryRequestBuilder {
    r#type: Option<GetWebhooksRequestType>,
    sort: Option<GetWebhooksRequestSort>,
}

impl GetWebhooksQueryRequestBuilder {
    pub fn r#type(mut self, value: GetWebhooksRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn sort(mut self, value: GetWebhooksRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWebhooksQueryRequest`].
    pub fn build(self) -> Result<GetWebhooksQueryRequest, BuildError> {
        Ok(GetWebhooksQueryRequest {
            r#type: self.r#type,
            sort: self.sort,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateSubAccountIpAssociateRequest {
    /// Pass the list of sub-account Ids to be associated with the IP
    /// address
    #[serde(default)]
    pub ids: Vec<i64>,
    /// IP address
    #[serde(default)]
    pub ip: String,
}

impl PostCorporateSubAccountIpAssociateRequest {
    pub fn builder() -> PostCorporateSubAccountIpAssociateRequestBuilder {
        <PostCorporateSubAccountIpAssociateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateSubAccountIpAssociateRequestBuilder {
    ids: Option<Vec<i64>>,
    ip: Option<String>,
}

impl PostCorporateSubAccountIpAssociateRequestBuilder {
    pub fn ids(mut self, value: Vec<i64>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateSubAccountIpAssociateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ids`](PostCorporateSubAccountIpAssociateRequestBuilder::ids)
    /// - [`ip`](PostCorporateSubAccountIpAssociateRequestBuilder::ip)
    pub fn build(self) -> Result<PostCorporateSubAccountIpAssociateRequest, BuildError> {
        Ok(PostCorporateSubAccountIpAssociateRequest {
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
        })
    }
}

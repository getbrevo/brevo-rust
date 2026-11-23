pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateSubAccountIpDissociateRequest {
    /// Pass the list of sub-account Ids to be dissociated from the IP
    /// address
    #[serde(default)]
    pub ids: Vec<i64>,
    /// IP address
    #[serde(default)]
    pub ip: String,
}

impl PutCorporateSubAccountIpDissociateRequest {
    pub fn builder() -> PutCorporateSubAccountIpDissociateRequestBuilder {
        <PutCorporateSubAccountIpDissociateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateSubAccountIpDissociateRequestBuilder {
    ids: Option<Vec<i64>>,
    ip: Option<String>,
}

impl PutCorporateSubAccountIpDissociateRequestBuilder {
    pub fn ids(mut self, value: Vec<i64>) -> Self {
        self.ids = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateSubAccountIpDissociateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ids`](PutCorporateSubAccountIpDissociateRequestBuilder::ids)
    /// - [`ip`](PutCorporateSubAccountIpDissociateRequestBuilder::ip)
    pub fn build(self) -> Result<PutCorporateSubAccountIpDissociateRequest, BuildError> {
        Ok(PutCorporateSubAccountIpDissociateRequest {
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
        })
    }
}

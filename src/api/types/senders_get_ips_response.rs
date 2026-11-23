pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetIpsResponse {
    /// Dedicated IP(s) available on your account. Returns empty array
    /// for standard accounts without dedicated IPs.
    #[serde(default)]
    pub ips: Vec<GetIpsResponseIpsItem>,
}

impl GetIpsResponse {
    pub fn builder() -> GetIpsResponseBuilder {
        <GetIpsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetIpsResponseBuilder {
    ips: Option<Vec<GetIpsResponseIpsItem>>,
}

impl GetIpsResponseBuilder {
    pub fn ips(mut self, value: Vec<GetIpsResponseIpsItem>) -> Self {
        self.ips = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetIpsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ips`](GetIpsResponseBuilder::ips)
    pub fn build(self) -> Result<GetIpsResponse, BuildError> {
        Ok(GetIpsResponse {
            ips: self.ips.ok_or_else(|| BuildError::missing_field("ips"))?,
        })
    }
}

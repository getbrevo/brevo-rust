pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetIpsFromSenderResponse {
    /// Dedicated IP(s) linked to a sender. Returns empty array for standard accounts
    /// without dedicated IPs.
    #[serde(default)]
    pub ips: Vec<GetIpsFromSenderResponseIpsItem>,
}

impl GetIpsFromSenderResponse {
    pub fn builder() -> GetIpsFromSenderResponseBuilder {
        <GetIpsFromSenderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetIpsFromSenderResponseBuilder {
    ips: Option<Vec<GetIpsFromSenderResponseIpsItem>>,
}

impl GetIpsFromSenderResponseBuilder {
    pub fn ips(mut self, value: Vec<GetIpsFromSenderResponseIpsItem>) -> Self {
        self.ips = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetIpsFromSenderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ips`](GetIpsFromSenderResponseBuilder::ips)
    pub fn build(self) -> Result<GetIpsFromSenderResponse, BuildError> {
        Ok(GetIpsFromSenderResponse {
            ips: self.ips.ok_or_else(|| BuildError::missing_field("ips"))?,
        })
    }
}

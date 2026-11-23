pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateIpResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional: Option<bool>,
}

impl GetCorporateIpResponseItem {
    pub fn builder() -> GetCorporateIpResponseItemBuilder {
        <GetCorporateIpResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateIpResponseItemBuilder {
    domain: Option<String>,
    ip: Option<String>,
    transactional: Option<bool>,
}

impl GetCorporateIpResponseItemBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn transactional(mut self, value: bool) -> Self {
        self.transactional = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateIpResponseItem`].
    pub fn build(self) -> Result<GetCorporateIpResponseItem, BuildError> {
        Ok(GetCorporateIpResponseItem {
            domain: self.domain,
            ip: self.ip,
            transactional: self.transactional,
        })
    }
}

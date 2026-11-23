pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSendersResponseSendersItemIpsItem {
    /// Domain of the IP
    #[serde(default)]
    pub domain: String,
    /// Dedicated IP available in your account
    #[serde(default)]
    pub ip: String,
    /// Weight of the IP for this sender
    #[serde(default)]
    pub weight: i64,
}

impl GetSendersResponseSendersItemIpsItem {
    pub fn builder() -> GetSendersResponseSendersItemIpsItemBuilder {
        <GetSendersResponseSendersItemIpsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSendersResponseSendersItemIpsItemBuilder {
    domain: Option<String>,
    ip: Option<String>,
    weight: Option<i64>,
}

impl GetSendersResponseSendersItemIpsItemBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSendersResponseSendersItemIpsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](GetSendersResponseSendersItemIpsItemBuilder::domain)
    /// - [`ip`](GetSendersResponseSendersItemIpsItemBuilder::ip)
    /// - [`weight`](GetSendersResponseSendersItemIpsItemBuilder::weight)
    pub fn build(self) -> Result<GetSendersResponseSendersItemIpsItem, BuildError> {
        Ok(GetSendersResponseSendersItemIpsItem {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
        })
    }
}

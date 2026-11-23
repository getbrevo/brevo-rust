pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetIpsFromSenderResponseIpsItem {
    /// Domain associated to the IP
    #[serde(default)]
    pub domain: String,
    /// ID of the dedicated IP
    #[serde(default)]
    pub id: i64,
    /// Dedicated IP address
    #[serde(default)]
    pub ip: String,
    /// Weight of the IP for this sender
    #[serde(default)]
    pub weight: i64,
}

impl GetIpsFromSenderResponseIpsItem {
    pub fn builder() -> GetIpsFromSenderResponseIpsItemBuilder {
        <GetIpsFromSenderResponseIpsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetIpsFromSenderResponseIpsItemBuilder {
    domain: Option<String>,
    id: Option<i64>,
    ip: Option<String>,
    weight: Option<i64>,
}

impl GetIpsFromSenderResponseIpsItemBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
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

    /// Consumes the builder and constructs a [`GetIpsFromSenderResponseIpsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](GetIpsFromSenderResponseIpsItemBuilder::domain)
    /// - [`id`](GetIpsFromSenderResponseIpsItemBuilder::id)
    /// - [`ip`](GetIpsFromSenderResponseIpsItemBuilder::ip)
    /// - [`weight`](GetIpsFromSenderResponseIpsItemBuilder::weight)
    pub fn build(self) -> Result<GetIpsFromSenderResponseIpsItem, BuildError> {
        Ok(GetIpsFromSenderResponseIpsItem {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
        })
    }
}

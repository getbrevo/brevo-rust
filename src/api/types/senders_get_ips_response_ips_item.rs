pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetIpsResponseIpsItem {
    /// ID of the dedicated IP
    #[serde(default)]
    pub id: i64,
    /// Status of the IP (true=active, false=inactive)
    #[serde(default)]
    pub active: bool,
    /// Domain associated to the IP
    #[serde(default)]
    pub domain: String,
    /// Dedicated IP address
    #[serde(default)]
    pub ip: String,
}

impl GetIpsResponseIpsItem {
    pub fn builder() -> GetIpsResponseIpsItemBuilder {
        <GetIpsResponseIpsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetIpsResponseIpsItemBuilder {
    id: Option<i64>,
    active: Option<bool>,
    domain: Option<String>,
    ip: Option<String>,
}

impl GetIpsResponseIpsItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetIpsResponseIpsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetIpsResponseIpsItemBuilder::id)
    /// - [`active`](GetIpsResponseIpsItemBuilder::active)
    /// - [`domain`](GetIpsResponseIpsItemBuilder::domain)
    /// - [`ip`](GetIpsResponseIpsItemBuilder::ip)
    pub fn build(self) -> Result<GetIpsResponseIpsItem, BuildError> {
        Ok(GetIpsResponseIpsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
        })
    }
}

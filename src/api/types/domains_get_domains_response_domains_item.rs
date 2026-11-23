pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainsResponseDomainsItem {
    /// Unique identifier of the domain
    #[serde(default)]
    pub id: String,
    /// Domain name
    #[serde(default)]
    pub domain_name: String,
    /// Status of domain authentication (true=authenticated,
    /// false=non authenticated)
    #[serde(default)]
    pub authenticated: bool,
    /// Status of domain verification (true=verified, false=non
    /// verified)
    #[serde(default)]
    pub verified: bool,
    /// Dedicated IP associated with domain (null if none)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// DNS provider for the domain
    #[serde(default)]
    pub provider: String,
    /// Information about who created the domain
    #[serde(default)]
    pub creator: GetDomainsResponseDomainsItemCreator,
}

impl GetDomainsResponseDomainsItem {
    pub fn builder() -> GetDomainsResponseDomainsItemBuilder {
        <GetDomainsResponseDomainsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainsResponseDomainsItemBuilder {
    id: Option<String>,
    domain_name: Option<String>,
    authenticated: Option<bool>,
    verified: Option<bool>,
    ip: Option<String>,
    provider: Option<String>,
    creator: Option<GetDomainsResponseDomainsItemCreator>,
}

impl GetDomainsResponseDomainsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn authenticated(mut self, value: bool) -> Self {
        self.authenticated = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn creator(mut self, value: GetDomainsResponseDomainsItemCreator) -> Self {
        self.creator = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDomainsResponseDomainsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetDomainsResponseDomainsItemBuilder::id)
    /// - [`domain_name`](GetDomainsResponseDomainsItemBuilder::domain_name)
    /// - [`authenticated`](GetDomainsResponseDomainsItemBuilder::authenticated)
    /// - [`verified`](GetDomainsResponseDomainsItemBuilder::verified)
    /// - [`provider`](GetDomainsResponseDomainsItemBuilder::provider)
    /// - [`creator`](GetDomainsResponseDomainsItemBuilder::creator)
    pub fn build(self) -> Result<GetDomainsResponseDomainsItem, BuildError> {
        Ok(GetDomainsResponseDomainsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            authenticated: self
                .authenticated
                .ok_or_else(|| BuildError::missing_field("authenticated"))?,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
            ip: self.ip,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            creator: self
                .creator
                .ok_or_else(|| BuildError::missing_field("creator"))?,
        })
    }
}

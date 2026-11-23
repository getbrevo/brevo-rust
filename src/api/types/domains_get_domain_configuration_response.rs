pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainConfigurationResponse {
    /// Domain name
    #[serde(default)]
    pub domain: String,
    /// Status of domain verification (true=verified, false=non
    /// verified)
    #[serde(default)]
    pub verified: bool,
    /// Status of domain authentication (true=authenticated, false=non
    /// authenticated)
    #[serde(default)]
    pub authenticated: bool,
    /// DNS records configuration and status
    #[serde(default)]
    pub dns_records: GetDomainConfigurationResponseDnsRecords,
}

impl GetDomainConfigurationResponse {
    pub fn builder() -> GetDomainConfigurationResponseBuilder {
        <GetDomainConfigurationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainConfigurationResponseBuilder {
    domain: Option<String>,
    verified: Option<bool>,
    authenticated: Option<bool>,
    dns_records: Option<GetDomainConfigurationResponseDnsRecords>,
}

impl GetDomainConfigurationResponseBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn authenticated(mut self, value: bool) -> Self {
        self.authenticated = Some(value);
        self
    }

    pub fn dns_records(mut self, value: GetDomainConfigurationResponseDnsRecords) -> Self {
        self.dns_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDomainConfigurationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](GetDomainConfigurationResponseBuilder::domain)
    /// - [`verified`](GetDomainConfigurationResponseBuilder::verified)
    /// - [`authenticated`](GetDomainConfigurationResponseBuilder::authenticated)
    /// - [`dns_records`](GetDomainConfigurationResponseBuilder::dns_records)
    pub fn build(self) -> Result<GetDomainConfigurationResponse, BuildError> {
        Ok(GetDomainConfigurationResponse {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
            authenticated: self
                .authenticated
                .ok_or_else(|| BuildError::missing_field("authenticated"))?,
            dns_records: self
                .dns_records
                .ok_or_else(|| BuildError::missing_field("dns_records"))?,
        })
    }
}

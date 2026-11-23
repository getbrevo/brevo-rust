pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainResponse {
    /// ID of the Domain created
    #[serde(default)]
    pub id: i64,
    /// Domain name
    #[serde(default)]
    pub domain_name: String,
    /// Detected domain provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_provider: Option<String>,
    /// Success message with next steps
    #[serde(default)]
    pub message: String,
    /// DNS records required for domain authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_records: Option<CreateDomainResponseDnsRecords>,
}

impl CreateDomainResponse {
    pub fn builder() -> CreateDomainResponseBuilder {
        <CreateDomainResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainResponseBuilder {
    id: Option<i64>,
    domain_name: Option<String>,
    domain_provider: Option<String>,
    message: Option<String>,
    dns_records: Option<CreateDomainResponseDnsRecords>,
}

impl CreateDomainResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn domain_name(mut self, value: impl Into<String>) -> Self {
        self.domain_name = Some(value.into());
        self
    }

    pub fn domain_provider(mut self, value: impl Into<String>) -> Self {
        self.domain_provider = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn dns_records(mut self, value: CreateDomainResponseDnsRecords) -> Self {
        self.dns_records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDomainResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateDomainResponseBuilder::id)
    /// - [`domain_name`](CreateDomainResponseBuilder::domain_name)
    /// - [`message`](CreateDomainResponseBuilder::message)
    pub fn build(self) -> Result<CreateDomainResponse, BuildError> {
        Ok(CreateDomainResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            domain_name: self
                .domain_name
                .ok_or_else(|| BuildError::missing_field("domain_name"))?,
            domain_provider: self.domain_provider,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            dns_records: self.dns_records,
        })
    }
}

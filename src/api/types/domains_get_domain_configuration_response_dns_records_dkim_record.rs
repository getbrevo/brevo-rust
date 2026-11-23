pub use crate::prelude::*;

/// DKIM authentication record
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainConfigurationResponseDnsRecordsDkimRecord {
    /// DNS record hostname
    #[serde(default)]
    pub host_name: String,
    /// DNS record type
    #[serde(default)]
    pub r#type: String,
    /// DNS record value
    #[serde(default)]
    pub value: String,
    /// Whether the record is properly configured
    #[serde(default)]
    pub status: bool,
}

impl GetDomainConfigurationResponseDnsRecordsDkimRecord {
    pub fn builder() -> GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder {
        <GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder {
    host_name: Option<String>,
    r#type: Option<String>,
    value: Option<String>,
    status: Option<bool>,
}

impl GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder {
    pub fn host_name(mut self, value: impl Into<String>) -> Self {
        self.host_name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn status(mut self, value: bool) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDomainConfigurationResponseDnsRecordsDkimRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`host_name`](GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder::host_name)
    /// - [`r#type`](GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder::r#type)
    /// - [`value`](GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder::value)
    /// - [`status`](GetDomainConfigurationResponseDnsRecordsDkimRecordBuilder::status)
    pub fn build(self) -> Result<GetDomainConfigurationResponseDnsRecordsDkimRecord, BuildError> {
        Ok(GetDomainConfigurationResponseDnsRecordsDkimRecord {
            host_name: self
                .host_name
                .ok_or_else(|| BuildError::missing_field("host_name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

pub use crate::prelude::*;

/// DKIM authentication record
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainResponseDnsRecordsDkimRecord {
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

impl CreateDomainResponseDnsRecordsDkimRecord {
    pub fn builder() -> CreateDomainResponseDnsRecordsDkimRecordBuilder {
        <CreateDomainResponseDnsRecordsDkimRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainResponseDnsRecordsDkimRecordBuilder {
    host_name: Option<String>,
    r#type: Option<String>,
    value: Option<String>,
    status: Option<bool>,
}

impl CreateDomainResponseDnsRecordsDkimRecordBuilder {
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

    /// Consumes the builder and constructs a [`CreateDomainResponseDnsRecordsDkimRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`host_name`](CreateDomainResponseDnsRecordsDkimRecordBuilder::host_name)
    /// - [`r#type`](CreateDomainResponseDnsRecordsDkimRecordBuilder::r#type)
    /// - [`value`](CreateDomainResponseDnsRecordsDkimRecordBuilder::value)
    /// - [`status`](CreateDomainResponseDnsRecordsDkimRecordBuilder::status)
    pub fn build(self) -> Result<CreateDomainResponseDnsRecordsDkimRecord, BuildError> {
        Ok(CreateDomainResponseDnsRecordsDkimRecord {
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

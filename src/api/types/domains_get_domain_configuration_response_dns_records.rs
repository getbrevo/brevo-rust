pub use crate::prelude::*;

/// DNS records configuration and status
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainConfigurationResponseDnsRecords {
    /// Brevo verification code record
    #[serde(default)]
    pub brevo_code: GetDomainConfigurationResponseDnsRecordsBrevoCode,
    /// DKIM authentication record
    #[serde(default)]
    pub dkim_record: GetDomainConfigurationResponseDnsRecordsDkimRecord,
    /// DMARC policy record
    #[serde(default)]
    pub dmarc_record: GetDomainConfigurationResponseDnsRecordsDmarcRecord,
}

impl GetDomainConfigurationResponseDnsRecords {
    pub fn builder() -> GetDomainConfigurationResponseDnsRecordsBuilder {
        <GetDomainConfigurationResponseDnsRecordsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainConfigurationResponseDnsRecordsBuilder {
    brevo_code: Option<GetDomainConfigurationResponseDnsRecordsBrevoCode>,
    dkim_record: Option<GetDomainConfigurationResponseDnsRecordsDkimRecord>,
    dmarc_record: Option<GetDomainConfigurationResponseDnsRecordsDmarcRecord>,
}

impl GetDomainConfigurationResponseDnsRecordsBuilder {
    pub fn brevo_code(mut self, value: GetDomainConfigurationResponseDnsRecordsBrevoCode) -> Self {
        self.brevo_code = Some(value);
        self
    }

    pub fn dkim_record(
        mut self,
        value: GetDomainConfigurationResponseDnsRecordsDkimRecord,
    ) -> Self {
        self.dkim_record = Some(value);
        self
    }

    pub fn dmarc_record(
        mut self,
        value: GetDomainConfigurationResponseDnsRecordsDmarcRecord,
    ) -> Self {
        self.dmarc_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDomainConfigurationResponseDnsRecords`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brevo_code`](GetDomainConfigurationResponseDnsRecordsBuilder::brevo_code)
    /// - [`dkim_record`](GetDomainConfigurationResponseDnsRecordsBuilder::dkim_record)
    /// - [`dmarc_record`](GetDomainConfigurationResponseDnsRecordsBuilder::dmarc_record)
    pub fn build(self) -> Result<GetDomainConfigurationResponseDnsRecords, BuildError> {
        Ok(GetDomainConfigurationResponseDnsRecords {
            brevo_code: self
                .brevo_code
                .ok_or_else(|| BuildError::missing_field("brevo_code"))?,
            dkim_record: self
                .dkim_record
                .ok_or_else(|| BuildError::missing_field("dkim_record"))?,
            dmarc_record: self
                .dmarc_record
                .ok_or_else(|| BuildError::missing_field("dmarc_record"))?,
        })
    }
}

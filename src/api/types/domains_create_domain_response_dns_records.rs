pub use crate::prelude::*;

/// DNS records required for domain authentication
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainResponseDnsRecords {
    /// Brevo verification code record
    #[serde(default)]
    pub brevo_code: CreateDomainResponseDnsRecordsBrevoCode,
    /// DKIM authentication record
    #[serde(default)]
    pub dkim_record: CreateDomainResponseDnsRecordsDkimRecord,
    /// DMARC policy record
    #[serde(default)]
    pub dmarc_record: CreateDomainResponseDnsRecordsDmarcRecord,
}

impl CreateDomainResponseDnsRecords {
    pub fn builder() -> CreateDomainResponseDnsRecordsBuilder {
        <CreateDomainResponseDnsRecordsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainResponseDnsRecordsBuilder {
    brevo_code: Option<CreateDomainResponseDnsRecordsBrevoCode>,
    dkim_record: Option<CreateDomainResponseDnsRecordsDkimRecord>,
    dmarc_record: Option<CreateDomainResponseDnsRecordsDmarcRecord>,
}

impl CreateDomainResponseDnsRecordsBuilder {
    pub fn brevo_code(mut self, value: CreateDomainResponseDnsRecordsBrevoCode) -> Self {
        self.brevo_code = Some(value);
        self
    }

    pub fn dkim_record(mut self, value: CreateDomainResponseDnsRecordsDkimRecord) -> Self {
        self.dkim_record = Some(value);
        self
    }

    pub fn dmarc_record(mut self, value: CreateDomainResponseDnsRecordsDmarcRecord) -> Self {
        self.dmarc_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDomainResponseDnsRecords`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brevo_code`](CreateDomainResponseDnsRecordsBuilder::brevo_code)
    /// - [`dkim_record`](CreateDomainResponseDnsRecordsBuilder::dkim_record)
    /// - [`dmarc_record`](CreateDomainResponseDnsRecordsBuilder::dmarc_record)
    pub fn build(self) -> Result<CreateDomainResponseDnsRecords, BuildError> {
        Ok(CreateDomainResponseDnsRecords {
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

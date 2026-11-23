pub use crate::prelude::*;

/// Manage US compliance
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsTemplatesResponseTemplatesItemCompliance {
    /// US compliance is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_toggle: Option<bool>,
    /// A recognizable prefix will ensure your audience knows who you are. Recommended by the U.S. carriers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation_prefix: Option<String>,
    /// Instructions to unsubscribe from future communications. Must include STOP keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_keyword: Option<String>,
}

impl GetSmsTemplatesResponseTemplatesItemCompliance {
    pub fn builder() -> GetSmsTemplatesResponseTemplatesItemComplianceBuilder {
        <GetSmsTemplatesResponseTemplatesItemComplianceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsTemplatesResponseTemplatesItemComplianceBuilder {
    compliance_toggle: Option<bool>,
    organisation_prefix: Option<String>,
    stop_keyword: Option<String>,
}

impl GetSmsTemplatesResponseTemplatesItemComplianceBuilder {
    pub fn compliance_toggle(mut self, value: bool) -> Self {
        self.compliance_toggle = Some(value);
        self
    }

    pub fn organisation_prefix(mut self, value: impl Into<String>) -> Self {
        self.organisation_prefix = Some(value.into());
        self
    }

    pub fn stop_keyword(mut self, value: impl Into<String>) -> Self {
        self.stop_keyword = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSmsTemplatesResponseTemplatesItemCompliance`].
    pub fn build(self) -> Result<GetSmsTemplatesResponseTemplatesItemCompliance, BuildError> {
        Ok(GetSmsTemplatesResponseTemplatesItemCompliance {
            compliance_toggle: self.compliance_toggle,
            organisation_prefix: self.organisation_prefix,
            stop_keyword: self.stop_keyword,
        })
    }
}

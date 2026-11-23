pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversionSourceMetricsConversionSource {
    EmailCampaign,
    SmsCampaign,
    AutomationWorkflowEmail,
    AutomationWorkflowSms,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversionSourceMetricsConversionSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EmailCampaign => serializer.serialize_str("email_campaign"),
            Self::SmsCampaign => serializer.serialize_str("sms_campaign"),
            Self::AutomationWorkflowEmail => serializer.serialize_str("automation_workflow_email"),
            Self::AutomationWorkflowSms => serializer.serialize_str("automation_workflow_sms"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversionSourceMetricsConversionSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email_campaign" => Ok(Self::EmailCampaign),
            "sms_campaign" => Ok(Self::SmsCampaign),
            "automation_workflow_email" => Ok(Self::AutomationWorkflowEmail),
            "automation_workflow_sms" => Ok(Self::AutomationWorkflowSms),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversionSourceMetricsConversionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmailCampaign => write!(f, "email_campaign"),
            Self::SmsCampaign => write!(f, "sms_campaign"),
            Self::AutomationWorkflowEmail => write!(f, "automation_workflow_email"),
            Self::AutomationWorkflowSms => write!(f, "automation_workflow_sms"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

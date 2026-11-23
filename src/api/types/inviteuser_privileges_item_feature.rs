pub use crate::prelude::*;

/// Feature name
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InviteuserPrivilegesItemFeature {
    EmailCampaigns,
    SmsCampaigns,
    Contacts,
    Templates,
    Workflows,
    LandingPages,
    TransactionalEmails,
    SmtpApi,
    UserManagement,
    SalesPlatform,
    Phone,
    Conversations,
    SendersDomainsDedicatedIps,
    PushNotifications,
    Companies,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InviteuserPrivilegesItemFeature {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EmailCampaigns => serializer.serialize_str("email_campaigns"),
            Self::SmsCampaigns => serializer.serialize_str("sms_campaigns"),
            Self::Contacts => serializer.serialize_str("contacts"),
            Self::Templates => serializer.serialize_str("templates"),
            Self::Workflows => serializer.serialize_str("workflows"),
            Self::LandingPages => serializer.serialize_str("landing_pages"),
            Self::TransactionalEmails => serializer.serialize_str("transactional_emails"),
            Self::SmtpApi => serializer.serialize_str("smtp_api"),
            Self::UserManagement => serializer.serialize_str("user_management"),
            Self::SalesPlatform => serializer.serialize_str("sales_platform"),
            Self::Phone => serializer.serialize_str("phone"),
            Self::Conversations => serializer.serialize_str("conversations"),
            Self::SendersDomainsDedicatedIps => {
                serializer.serialize_str("senders_domains_dedicated_ips")
            }
            Self::PushNotifications => serializer.serialize_str("push_notifications"),
            Self::Companies => serializer.serialize_str("companies"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InviteuserPrivilegesItemFeature {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email_campaigns" => Ok(Self::EmailCampaigns),
            "sms_campaigns" => Ok(Self::SmsCampaigns),
            "contacts" => Ok(Self::Contacts),
            "templates" => Ok(Self::Templates),
            "workflows" => Ok(Self::Workflows),
            "landing_pages" => Ok(Self::LandingPages),
            "transactional_emails" => Ok(Self::TransactionalEmails),
            "smtp_api" => Ok(Self::SmtpApi),
            "user_management" => Ok(Self::UserManagement),
            "sales_platform" => Ok(Self::SalesPlatform),
            "phone" => Ok(Self::Phone),
            "conversations" => Ok(Self::Conversations),
            "senders_domains_dedicated_ips" => Ok(Self::SendersDomainsDedicatedIps),
            "push_notifications" => Ok(Self::PushNotifications),
            "companies" => Ok(Self::Companies),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InviteuserPrivilegesItemFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmailCampaigns => write!(f, "email_campaigns"),
            Self::SmsCampaigns => write!(f, "sms_campaigns"),
            Self::Contacts => write!(f, "contacts"),
            Self::Templates => write!(f, "templates"),
            Self::Workflows => write!(f, "workflows"),
            Self::LandingPages => write!(f, "landing_pages"),
            Self::TransactionalEmails => write!(f, "transactional_emails"),
            Self::SmtpApi => write!(f, "smtp_api"),
            Self::UserManagement => write!(f, "user_management"),
            Self::SalesPlatform => write!(f, "sales_platform"),
            Self::Phone => write!(f, "phone"),
            Self::Conversations => write!(f, "conversations"),
            Self::SendersDomainsDedicatedIps => write!(f, "senders_domains_dedicated_ips"),
            Self::PushNotifications => write!(f, "push_notifications"),
            Self::Companies => write!(f, "companies"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

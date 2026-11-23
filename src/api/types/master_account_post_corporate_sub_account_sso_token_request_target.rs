pub use crate::prelude::*;

/// **Set target after login success** * **automation** - Redirect
/// to Automation after login * **email_campaign** - Redirect to
/// Email Campaign after login * **contacts** - Redirect to Contacts
/// after login * **landing_pages** - Redirect to Landing Pages
/// after login * **email_transactional** - Redirect to Email
/// Transactional after login * **senders** - Redirect to Senders
/// after login * **sms_campaign** - Redirect to Sms Campaign after
/// login * **sms_transactional** - Redirect to Sms Transactional
/// after login
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostCorporateSubAccountSsoTokenRequestTarget {
    Automation,
    EmailCampaign,
    Contacts,
    LandingPages,
    EmailTransactional,
    Senders,
    SmsCampaign,
    SmsTransactional,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostCorporateSubAccountSsoTokenRequestTarget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Automation => serializer.serialize_str("automation"),
            Self::EmailCampaign => serializer.serialize_str("email_campaign"),
            Self::Contacts => serializer.serialize_str("contacts"),
            Self::LandingPages => serializer.serialize_str("landing_pages"),
            Self::EmailTransactional => serializer.serialize_str("email_transactional"),
            Self::Senders => serializer.serialize_str("senders"),
            Self::SmsCampaign => serializer.serialize_str("sms_campaign"),
            Self::SmsTransactional => serializer.serialize_str("sms_transactional"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostCorporateSubAccountSsoTokenRequestTarget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "automation" => Ok(Self::Automation),
            "email_campaign" => Ok(Self::EmailCampaign),
            "contacts" => Ok(Self::Contacts),
            "landing_pages" => Ok(Self::LandingPages),
            "email_transactional" => Ok(Self::EmailTransactional),
            "senders" => Ok(Self::Senders),
            "sms_campaign" => Ok(Self::SmsCampaign),
            "sms_transactional" => Ok(Self::SmsTransactional),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostCorporateSubAccountSsoTokenRequestTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automation => write!(f, "automation"),
            Self::EmailCampaign => write!(f, "email_campaign"),
            Self::Contacts => write!(f, "contacts"),
            Self::LandingPages => write!(f, "landing_pages"),
            Self::EmailTransactional => write!(f, "email_transactional"),
            Self::Senders => write!(f, "senders"),
            Self::SmsCampaign => write!(f, "sms_campaign"),
            Self::SmsTransactional => write!(f, "sms_transactional"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

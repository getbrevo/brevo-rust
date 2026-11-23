pub use crate::prelude::*;

/// **Mandatory if neither actionForContacts nor actionForEmailCampaigns is passed.** This will export the contacts on the basis of provided action applied on sms campaigns. * **unsubscribed** - Fetch the list of all unsubscribed (blacklisted via any means) contacts for all / particular sms campaigns. * **hardBounces & softBounces** - Fetch the list of hard bounces / soft bounces for all / particular sms campaigns.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestContactExportRequestCustomContactFilterActionForSmsCampaigns {
    HardBounces,
    SoftBounces,
    Unsubscribed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RequestContactExportRequestCustomContactFilterActionForSmsCampaigns {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::HardBounces => serializer.serialize_str("hardBounces"),
            Self::SoftBounces => serializer.serialize_str("softBounces"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RequestContactExportRequestCustomContactFilterActionForSmsCampaigns {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "hardBounces" => Ok(Self::HardBounces),
            "softBounces" => Ok(Self::SoftBounces),
            "unsubscribed" => Ok(Self::Unsubscribed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RequestContactExportRequestCustomContactFilterActionForSmsCampaigns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

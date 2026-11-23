pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetEmailEventReportRequestEvent {
    Bounces,
    HardBounces,
    SoftBounces,
    Delivered,
    Spam,
    Requests,
    Opened,
    Clicks,
    Invalid,
    Deferred,
    Blocked,
    Unsubscribed,
    Error,
    LoadedByProxy,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetEmailEventReportRequestEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bounces => serializer.serialize_str("bounces"),
            Self::HardBounces => serializer.serialize_str("hardBounces"),
            Self::SoftBounces => serializer.serialize_str("softBounces"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Spam => serializer.serialize_str("spam"),
            Self::Requests => serializer.serialize_str("requests"),
            Self::Opened => serializer.serialize_str("opened"),
            Self::Clicks => serializer.serialize_str("clicks"),
            Self::Invalid => serializer.serialize_str("invalid"),
            Self::Deferred => serializer.serialize_str("deferred"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::Error => serializer.serialize_str("error"),
            Self::LoadedByProxy => serializer.serialize_str("loadedByProxy"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetEmailEventReportRequestEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bounces" => Ok(Self::Bounces),
            "hardBounces" => Ok(Self::HardBounces),
            "softBounces" => Ok(Self::SoftBounces),
            "delivered" => Ok(Self::Delivered),
            "spam" => Ok(Self::Spam),
            "requests" => Ok(Self::Requests),
            "opened" => Ok(Self::Opened),
            "clicks" => Ok(Self::Clicks),
            "invalid" => Ok(Self::Invalid),
            "deferred" => Ok(Self::Deferred),
            "blocked" => Ok(Self::Blocked),
            "unsubscribed" => Ok(Self::Unsubscribed),
            "error" => Ok(Self::Error),
            "loadedByProxy" => Ok(Self::LoadedByProxy),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetEmailEventReportRequestEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounces => write!(f, "bounces"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::Delivered => write!(f, "delivered"),
            Self::Spam => write!(f, "spam"),
            Self::Requests => write!(f, "requests"),
            Self::Opened => write!(f, "opened"),
            Self::Clicks => write!(f, "clicks"),
            Self::Invalid => write!(f, "invalid"),
            Self::Deferred => write!(f, "deferred"),
            Self::Blocked => write!(f, "blocked"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::Error => write!(f, "error"),
            Self::LoadedByProxy => write!(f, "loadedByProxy"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

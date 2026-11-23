pub use crate::prelude::*;

/// Filter the history for a specific event type
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExportWebhooksHistoryRequestEvent {
    InvalidParameter,
    MissingParameter,
    HardBounce,
    SoftBounce,
    Delivered,
    Spam,
    Request,
    Opened,
    Click,
    Invalid,
    Deferred,
    Blocked,
    Unsubscribed,
    Error,
    UniqueOpened,
    LoadedByProxy,
    AllEvents,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExportWebhooksHistoryRequestEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InvalidParameter => serializer.serialize_str("invalid_parameter"),
            Self::MissingParameter => serializer.serialize_str("missing_parameter"),
            Self::HardBounce => serializer.serialize_str("hardBounce"),
            Self::SoftBounce => serializer.serialize_str("softBounce"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Spam => serializer.serialize_str("spam"),
            Self::Request => serializer.serialize_str("request"),
            Self::Opened => serializer.serialize_str("opened"),
            Self::Click => serializer.serialize_str("click"),
            Self::Invalid => serializer.serialize_str("invalid"),
            Self::Deferred => serializer.serialize_str("deferred"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::Error => serializer.serialize_str("error"),
            Self::UniqueOpened => serializer.serialize_str("uniqueOpened"),
            Self::LoadedByProxy => serializer.serialize_str("loadedByProxy"),
            Self::AllEvents => serializer.serialize_str("allEvents"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExportWebhooksHistoryRequestEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invalid_parameter" => Ok(Self::InvalidParameter),
            "missing_parameter" => Ok(Self::MissingParameter),
            "hardBounce" => Ok(Self::HardBounce),
            "softBounce" => Ok(Self::SoftBounce),
            "delivered" => Ok(Self::Delivered),
            "spam" => Ok(Self::Spam),
            "request" => Ok(Self::Request),
            "opened" => Ok(Self::Opened),
            "click" => Ok(Self::Click),
            "invalid" => Ok(Self::Invalid),
            "deferred" => Ok(Self::Deferred),
            "blocked" => Ok(Self::Blocked),
            "unsubscribed" => Ok(Self::Unsubscribed),
            "error" => Ok(Self::Error),
            "uniqueOpened" => Ok(Self::UniqueOpened),
            "loadedByProxy" => Ok(Self::LoadedByProxy),
            "allEvents" => Ok(Self::AllEvents),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExportWebhooksHistoryRequestEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter => write!(f, "invalid_parameter"),
            Self::MissingParameter => write!(f, "missing_parameter"),
            Self::HardBounce => write!(f, "hardBounce"),
            Self::SoftBounce => write!(f, "softBounce"),
            Self::Delivered => write!(f, "delivered"),
            Self::Spam => write!(f, "spam"),
            Self::Request => write!(f, "request"),
            Self::Opened => write!(f, "opened"),
            Self::Click => write!(f, "click"),
            Self::Invalid => write!(f, "invalid"),
            Self::Deferred => write!(f, "deferred"),
            Self::Blocked => write!(f, "blocked"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::Error => write!(f, "error"),
            Self::UniqueOpened => write!(f, "uniqueOpened"),
            Self::LoadedByProxy => write!(f, "loadedByProxy"),
            Self::AllEvents => write!(f, "allEvents"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetWhatsappEventReportRequestEvent {
    Sent,
    Delivered,
    Read,
    Error,
    Unsubscribe,
    Reply,
    SoftBounce,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetWhatsappEventReportRequestEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sent => serializer.serialize_str("sent"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Read => serializer.serialize_str("read"),
            Self::Error => serializer.serialize_str("error"),
            Self::Unsubscribe => serializer.serialize_str("unsubscribe"),
            Self::Reply => serializer.serialize_str("reply"),
            Self::SoftBounce => serializer.serialize_str("soft-bounce"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetWhatsappEventReportRequestEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sent" => Ok(Self::Sent),
            "delivered" => Ok(Self::Delivered),
            "read" => Ok(Self::Read),
            "error" => Ok(Self::Error),
            "unsubscribe" => Ok(Self::Unsubscribe),
            "reply" => Ok(Self::Reply),
            "soft-bounce" => Ok(Self::SoftBounce),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetWhatsappEventReportRequestEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::Delivered => write!(f, "delivered"),
            Self::Read => write!(f, "read"),
            Self::Error => write!(f, "error"),
            Self::Unsubscribe => write!(f, "unsubscribe"),
            Self::Reply => write!(f, "reply"),
            Self::SoftBounce => write!(f, "soft-bounce"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

/// Type of the event
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetInboundEmailEventsByUuidResponseLogsItemType {
    Received,
    Processed,
    WebhookFailed,
    WebhookDelivered,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetInboundEmailEventsByUuidResponseLogsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Received => serializer.serialize_str("received"),
            Self::Processed => serializer.serialize_str("processed"),
            Self::WebhookFailed => serializer.serialize_str("webhookFailed"),
            Self::WebhookDelivered => serializer.serialize_str("webhookDelivered"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetInboundEmailEventsByUuidResponseLogsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "received" => Ok(Self::Received),
            "processed" => Ok(Self::Processed),
            "webhookFailed" => Ok(Self::WebhookFailed),
            "webhookDelivered" => Ok(Self::WebhookDelivered),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetInboundEmailEventsByUuidResponseLogsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Received => write!(f, "received"),
            Self::Processed => write!(f, "processed"),
            Self::WebhookFailed => write!(f, "webhookFailed"),
            Self::WebhookDelivered => write!(f, "webhookDelivered"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

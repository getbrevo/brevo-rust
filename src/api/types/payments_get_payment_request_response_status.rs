pub use crate::prelude::*;

/// Status of the payment request.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetPaymentRequestResponseStatus {
    Created,
    Sent,
    ReminderSent,
    Paid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetPaymentRequestResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Created => serializer.serialize_str("created"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::ReminderSent => serializer.serialize_str("reminderSent"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetPaymentRequestResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created" => Ok(Self::Created),
            "sent" => Ok(Self::Sent),
            "reminderSent" => Ok(Self::ReminderSent),
            "paid" => Ok(Self::Paid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetPaymentRequestResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Sent => write!(f, "sent"),
            Self::ReminderSent => write!(f, "reminderSent"),
            Self::Paid => write!(f, "paid"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

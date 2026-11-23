pub use crate::prelude::*;

/// Unit of time before reminder is to be sent
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskReminderUnit {
    Minutes,
    Hours,
    Weeks,
    Days,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskReminderUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Minutes => serializer.serialize_str("minutes"),
            Self::Hours => serializer.serialize_str("hours"),
            Self::Weeks => serializer.serialize_str("weeks"),
            Self::Days => serializer.serialize_str("days"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskReminderUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "minutes" => Ok(Self::Minutes),
            "hours" => Ok(Self::Hours),
            "weeks" => Ok(Self::Weeks),
            "days" => Ok(Self::Days),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskReminderUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minutes => write!(f, "minutes"),
            Self::Hours => write!(f, "hours"),
            Self::Weeks => write!(f, "weeks"),
            Self::Days => write!(f, "days"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

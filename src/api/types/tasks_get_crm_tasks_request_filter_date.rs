pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetCrmTasksRequestFilterDate {
    Overdue,
    Today,
    Tomorrow,
    Week,
    Range,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetCrmTasksRequestFilterDate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Overdue => serializer.serialize_str("overdue"),
            Self::Today => serializer.serialize_str("today"),
            Self::Tomorrow => serializer.serialize_str("tomorrow"),
            Self::Week => serializer.serialize_str("week"),
            Self::Range => serializer.serialize_str("range"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetCrmTasksRequestFilterDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "overdue" => Ok(Self::Overdue),
            "today" => Ok(Self::Today),
            "tomorrow" => Ok(Self::Tomorrow),
            "week" => Ok(Self::Week),
            "range" => Ok(Self::Range),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetCrmTasksRequestFilterDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overdue => write!(f, "overdue"),
            Self::Today => write!(f, "today"),
            Self::Tomorrow => write!(f, "tomorrow"),
            Self::Week => write!(f, "week"),
            Self::Range => write!(f, "range"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

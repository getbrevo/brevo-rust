pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetCrmTasksRequestFilterStatus {
    Done,
    Undone,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetCrmTasksRequestFilterStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Done => serializer.serialize_str("done"),
            Self::Undone => serializer.serialize_str("undone"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetCrmTasksRequestFilterStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "done" => Ok(Self::Done),
            "undone" => Ok(Self::Undone),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetCrmTasksRequestFilterStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done => write!(f, "done"),
            Self::Undone => write!(f, "undone"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

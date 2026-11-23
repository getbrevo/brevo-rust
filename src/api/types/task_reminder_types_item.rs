pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaskReminderTypesItem {
    Email,
    Push,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TaskReminderTypesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Push => serializer.serialize_str("push"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TaskReminderTypesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "push" => Ok(Self::Push),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TaskReminderTypesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Push => write!(f, "push"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

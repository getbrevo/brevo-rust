pub use crate::prelude::*;

/// The type of attribute (must be one of the defined enums)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostCrmAttributesRequestAttributeType {
    Text,
    User,
    Number,
    SingleSelect,
    Date,
    Boolean,
    MultiChoice,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostCrmAttributesRequestAttributeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::User => serializer.serialize_str("user"),
            Self::Number => serializer.serialize_str("number"),
            Self::SingleSelect => serializer.serialize_str("single-select"),
            Self::Date => serializer.serialize_str("date"),
            Self::Boolean => serializer.serialize_str("boolean"),
            Self::MultiChoice => serializer.serialize_str("multi-choice"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PostCrmAttributesRequestAttributeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "user" => Ok(Self::User),
            "number" => Ok(Self::Number),
            "single-select" => Ok(Self::SingleSelect),
            "date" => Ok(Self::Date),
            "boolean" => Ok(Self::Boolean),
            "multi-choice" => Ok(Self::MultiChoice),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostCrmAttributesRequestAttributeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::User => write!(f, "user"),
            Self::Number => write!(f, "number"),
            Self::SingleSelect => write!(f, "single-select"),
            Self::Date => write!(f, "date"),
            Self::Boolean => write!(f, "boolean"),
            Self::MultiChoice => write!(f, "multi-choice"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

/// Type of the attribute
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAttributesResponseAttributesItemType {
    Text,
    Date,
    Float,
    Id,
    Boolean,
    MultipleChoice,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAttributesResponseAttributesItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::Date => serializer.serialize_str("date"),
            Self::Float => serializer.serialize_str("float"),
            Self::Id => serializer.serialize_str("id"),
            Self::Boolean => serializer.serialize_str("boolean"),
            Self::MultipleChoice => serializer.serialize_str("multiple-choice"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAttributesResponseAttributesItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "date" => Ok(Self::Date),
            "float" => Ok(Self::Float),
            "id" => Ok(Self::Id),
            "boolean" => Ok(Self::Boolean),
            "multiple-choice" => Ok(Self::MultipleChoice),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAttributesResponseAttributesItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Date => write!(f, "date"),
            Self::Float => write!(f, "float"),
            Self::Id => write!(f, "id"),
            Self::Boolean => write!(f, "boolean"),
            Self::MultipleChoice => write!(f, "multiple-choice"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

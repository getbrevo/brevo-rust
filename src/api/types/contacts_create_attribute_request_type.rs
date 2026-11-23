pub use crate::prelude::*;

/// Type of the attribute. **Use only if the attribute's category is 'normal', 'category' or 'transactional'** Type **user and multiple-choice** is only available if the category is **normal** attribute Type **id** is only available if the category is **transactional** attribute Type **category** is only available if the category is **category** attribute
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAttributeRequestType {
    Text,
    Date,
    Float,
    Boolean,
    Id,
    Category,
    MultipleChoice,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAttributeRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::Date => serializer.serialize_str("date"),
            Self::Float => serializer.serialize_str("float"),
            Self::Boolean => serializer.serialize_str("boolean"),
            Self::Id => serializer.serialize_str("id"),
            Self::Category => serializer.serialize_str("category"),
            Self::MultipleChoice => serializer.serialize_str("multiple-choice"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAttributeRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "date" => Ok(Self::Date),
            "float" => Ok(Self::Float),
            "boolean" => Ok(Self::Boolean),
            "id" => Ok(Self::Id),
            "category" => Ok(Self::Category),
            "multiple-choice" => Ok(Self::MultipleChoice),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAttributeRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Date => write!(f, "date"),
            Self::Float => write!(f, "float"),
            Self::Boolean => write!(f, "boolean"),
            Self::Id => write!(f, "id"),
            Self::Category => write!(f, "category"),
            Self::MultipleChoice => write!(f, "multiple-choice"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

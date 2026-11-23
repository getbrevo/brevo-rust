pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeleteAttributeRequestAttributeCategory {
    Normal,
    Transactional,
    Category,
    Calculated,
    Global,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeleteAttributeRequestAttributeCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Normal => serializer.serialize_str("normal"),
            Self::Transactional => serializer.serialize_str("transactional"),
            Self::Category => serializer.serialize_str("category"),
            Self::Calculated => serializer.serialize_str("calculated"),
            Self::Global => serializer.serialize_str("global"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeleteAttributeRequestAttributeCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "normal" => Ok(Self::Normal),
            "transactional" => Ok(Self::Transactional),
            "category" => Ok(Self::Category),
            "calculated" => Ok(Self::Calculated),
            "global" => Ok(Self::Global),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeleteAttributeRequestAttributeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Transactional => write!(f, "transactional"),
            Self::Category => write!(f, "category"),
            Self::Calculated => write!(f, "calculated"),
            Self::Global => write!(f, "global"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

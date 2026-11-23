pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAttributeRequestAttributeCategory {
    Category,
    Calculated,
    Global,
    Normal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAttributeRequestAttributeCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Category => serializer.serialize_str("category"),
            Self::Calculated => serializer.serialize_str("calculated"),
            Self::Global => serializer.serialize_str("global"),
            Self::Normal => serializer.serialize_str("normal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAttributeRequestAttributeCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "category" => Ok(Self::Category),
            "calculated" => Ok(Self::Calculated),
            "global" => Ok(Self::Global),
            "normal" => Ok(Self::Normal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAttributeRequestAttributeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Category => write!(f, "category"),
            Self::Calculated => write!(f, "calculated"),
            Self::Global => write!(f, "global"),
            Self::Normal => write!(f, "normal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

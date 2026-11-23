pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetProductsRequestSortByField {
    CreatedAt,
    Name,
    Price,
    Id,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetProductsRequestSortByField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::Name => serializer.serialize_str("name"),
            Self::Price => serializer.serialize_str("price"),
            Self::Id => serializer.serialize_str("id"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetProductsRequestSortByField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "name" => Ok(Self::Name),
            "price" => Ok(Self::Price),
            "id" => Ok(Self::Id),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetProductsRequestSortByField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::Name => write!(f, "name"),
            Self::Price => write!(f, "price"),
            Self::Id => write!(f, "id"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

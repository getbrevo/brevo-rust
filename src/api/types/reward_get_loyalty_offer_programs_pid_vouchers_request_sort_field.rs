pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetLoyaltyOfferProgramsPidVouchersRequestSortField {
    UpdatedAt,
    CreatedAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetLoyaltyOfferProgramsPidVouchersRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UpdatedAt => serializer.serialize_str("updatedAt"),
            Self::CreatedAt => serializer.serialize_str("createdAt"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetLoyaltyOfferProgramsPidVouchersRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "updatedAt" => Ok(Self::UpdatedAt),
            "createdAt" => Ok(Self::CreatedAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetLoyaltyOfferProgramsPidVouchersRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdatedAt => write!(f, "updatedAt"),
            Self::CreatedAt => write!(f, "createdAt"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

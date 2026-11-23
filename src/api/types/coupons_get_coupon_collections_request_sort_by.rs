pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetCouponCollectionsRequestSortBy {
    CreatedAt,
    RemainingCoupons,
    ExpirationDate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetCouponCollectionsRequestSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("createdAt"),
            Self::RemainingCoupons => serializer.serialize_str("remainingCoupons"),
            Self::ExpirationDate => serializer.serialize_str("expirationDate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetCouponCollectionsRequestSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "createdAt" => Ok(Self::CreatedAt),
            "remainingCoupons" => Ok(Self::RemainingCoupons),
            "expirationDate" => Ok(Self::ExpirationDate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetCouponCollectionsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "createdAt"),
            Self::RemainingCoupons => write!(f, "remainingCoupons"),
            Self::ExpirationDate => write!(f, "expirationDate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

/// Displays the plan type of the user
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAccountResponsePlanItemType {
    PayAsYouGo,
    Free,
    Subscription,
    Sms,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAccountResponsePlanItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PayAsYouGo => serializer.serialize_str("payAsYouGo"),
            Self::Free => serializer.serialize_str("free"),
            Self::Subscription => serializer.serialize_str("subscription"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAccountResponsePlanItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payAsYouGo" => Ok(Self::PayAsYouGo),
            "free" => Ok(Self::Free),
            "subscription" => Ok(Self::Subscription),
            "sms" => Ok(Self::Sms),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAccountResponsePlanItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayAsYouGo => write!(f, "payAsYouGo"),
            Self::Free => write!(f, "free"),
            Self::Subscription => write!(f, "subscription"),
            Self::Sms => write!(f, "sms"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

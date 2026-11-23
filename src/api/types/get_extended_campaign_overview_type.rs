pub use crate::prelude::*;

/// Type of campaign
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetExtendedCampaignOverviewType {
    Classic,
    Trigger,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetExtendedCampaignOverviewType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Classic => serializer.serialize_str("classic"),
            Self::Trigger => serializer.serialize_str("trigger"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetExtendedCampaignOverviewType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "classic" => Ok(Self::Classic),
            "trigger" => Ok(Self::Trigger),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetExtendedCampaignOverviewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Classic => write!(f, "classic"),
            Self::Trigger => write!(f, "trigger"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

/// Criteria chosen for winning version (Open/Click)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAbTestCampaignResultResponseWinningCriteria {
    Open,
    Click,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAbTestCampaignResultResponseWinningCriteria {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("Open"),
            Self::Click => serializer.serialize_str("Click"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAbTestCampaignResultResponseWinningCriteria {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Open" => Ok(Self::Open),
            "Click" => Ok(Self::Click),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAbTestCampaignResultResponseWinningCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::Click => write!(f, "Click"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetEmailCampaignRequestStatistics {
    GlobalStats,
    LinksStats,
    StatsByDomain,
    StatsByDevice,
    StatsByBrowser,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetEmailCampaignRequestStatistics {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::GlobalStats => serializer.serialize_str("globalStats"),
            Self::LinksStats => serializer.serialize_str("linksStats"),
            Self::StatsByDomain => serializer.serialize_str("statsByDomain"),
            Self::StatsByDevice => serializer.serialize_str("statsByDevice"),
            Self::StatsByBrowser => serializer.serialize_str("statsByBrowser"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetEmailCampaignRequestStatistics {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "globalStats" => Ok(Self::GlobalStats),
            "linksStats" => Ok(Self::LinksStats),
            "statsByDomain" => Ok(Self::StatsByDomain),
            "statsByDevice" => Ok(Self::StatsByDevice),
            "statsByBrowser" => Ok(Self::StatsByBrowser),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetEmailCampaignRequestStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GlobalStats => write!(f, "globalStats"),
            Self::LinksStats => write!(f, "linksStats"),
            Self::StatsByDomain => write!(f, "statsByDomain"),
            Self::StatsByDevice => write!(f, "statsByDevice"),
            Self::StatsByBrowser => write!(f, "statsByBrowser"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

/// Name/type of the process
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetProcessResponseName {
    Importuser,
    SearchExportUsers,
    TransCalc,
    TransGlobalCalc,
    CampaignProcessing,
    ListExport,
    ContactExport,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetProcessResponseName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Importuser => serializer.serialize_str("IMPORTUSER"),
            Self::SearchExportUsers => serializer.serialize_str("SEARCH_EXPORT_USERS"),
            Self::TransCalc => serializer.serialize_str("TRANS-CALC"),
            Self::TransGlobalCalc => serializer.serialize_str("TRANS-GLOBAL-CALC"),
            Self::CampaignProcessing => serializer.serialize_str("CAMPAIGN_PROCESSING"),
            Self::ListExport => serializer.serialize_str("LIST_EXPORT"),
            Self::ContactExport => serializer.serialize_str("CONTACT_EXPORT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetProcessResponseName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "IMPORTUSER" => Ok(Self::Importuser),
            "SEARCH_EXPORT_USERS" => Ok(Self::SearchExportUsers),
            "TRANS-CALC" => Ok(Self::TransCalc),
            "TRANS-GLOBAL-CALC" => Ok(Self::TransGlobalCalc),
            "CAMPAIGN_PROCESSING" => Ok(Self::CampaignProcessing),
            "LIST_EXPORT" => Ok(Self::ListExport),
            "CONTACT_EXPORT" => Ok(Self::ContactExport),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetProcessResponseName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Importuser => write!(f, "IMPORTUSER"),
            Self::SearchExportUsers => write!(f, "SEARCH_EXPORT_USERS"),
            Self::TransCalc => write!(f, "TRANS-CALC"),
            Self::TransGlobalCalc => write!(f, "TRANS-GLOBAL-CALC"),
            Self::CampaignProcessing => write!(f, "CAMPAIGN_PROCESSING"),
            Self::ListExport => write!(f, "LIST_EXPORT"),
            Self::ContactExport => write!(f, "CONTACT_EXPORT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

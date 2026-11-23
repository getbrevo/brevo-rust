pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetSmsCampaignsRequestStatus {
    Suspended,
    Archive,
    Sent,
    Queued,
    Draft,
    InProcess,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetSmsCampaignsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Suspended => serializer.serialize_str("suspended"),
            Self::Archive => serializer.serialize_str("archive"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::InProcess => serializer.serialize_str("inProcess"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetSmsCampaignsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "suspended" => Ok(Self::Suspended),
            "archive" => Ok(Self::Archive),
            "sent" => Ok(Self::Sent),
            "queued" => Ok(Self::Queued),
            "draft" => Ok(Self::Draft),
            "inProcess" => Ok(Self::InProcess),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetSmsCampaignsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Suspended => write!(f, "suspended"),
            Self::Archive => write!(f, "archive"),
            Self::Sent => write!(f, "sent"),
            Self::Queued => write!(f, "queued"),
            Self::Draft => write!(f, "draft"),
            Self::InProcess => write!(f, "inProcess"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

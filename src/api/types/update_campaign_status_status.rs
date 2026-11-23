pub use crate::prelude::*;

/// Note:- **replicateTemplate** status will be available **only for template type campaigns.**
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateCampaignStatusStatus {
    Suspended,
    Archive,
    Darchive,
    Sent,
    Queued,
    Replicate,
    ReplicateTemplate,
    Cancel,
    Draft,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateCampaignStatusStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Suspended => serializer.serialize_str("suspended"),
            Self::Archive => serializer.serialize_str("archive"),
            Self::Darchive => serializer.serialize_str("darchive"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::Replicate => serializer.serialize_str("replicate"),
            Self::ReplicateTemplate => serializer.serialize_str("replicateTemplate"),
            Self::Cancel => serializer.serialize_str("cancel"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateCampaignStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "suspended" => Ok(Self::Suspended),
            "archive" => Ok(Self::Archive),
            "darchive" => Ok(Self::Darchive),
            "sent" => Ok(Self::Sent),
            "queued" => Ok(Self::Queued),
            "replicate" => Ok(Self::Replicate),
            "replicateTemplate" => Ok(Self::ReplicateTemplate),
            "cancel" => Ok(Self::Cancel),
            "draft" => Ok(Self::Draft),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateCampaignStatusStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Suspended => write!(f, "suspended"),
            Self::Archive => write!(f, "archive"),
            Self::Darchive => write!(f, "darchive"),
            Self::Sent => write!(f, "sent"),
            Self::Queued => write!(f, "queued"),
            Self::Replicate => write!(f, "replicate"),
            Self::ReplicateTemplate => write!(f, "replicateTemplate"),
            Self::Cancel => write!(f, "cancel"),
            Self::Draft => write!(f, "draft"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

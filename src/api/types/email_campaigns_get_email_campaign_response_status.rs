pub use crate::prelude::*;

/// Status of the campaign
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetEmailCampaignResponseStatus {
    Draft,
    Sent,
    Archive,
    Queued,
    Suspended,
    InProcess,
    InReview,
    Cancelling,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetEmailCampaignResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Archive => serializer.serialize_str("archive"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::Suspended => serializer.serialize_str("suspended"),
            Self::InProcess => serializer.serialize_str("in_process"),
            Self::InReview => serializer.serialize_str("in_review"),
            Self::Cancelling => serializer.serialize_str("cancelling"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetEmailCampaignResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "sent" => Ok(Self::Sent),
            "archive" => Ok(Self::Archive),
            "queued" => Ok(Self::Queued),
            "suspended" => Ok(Self::Suspended),
            "in_process" => Ok(Self::InProcess),
            "in_review" => Ok(Self::InReview),
            "cancelling" => Ok(Self::Cancelling),
            "cancelled" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetEmailCampaignResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Sent => write!(f, "sent"),
            Self::Archive => write!(f, "archive"),
            Self::Queued => write!(f, "queued"),
            Self::Suspended => write!(f, "suspended"),
            Self::InProcess => write!(f, "in_process"),
            Self::InReview => write!(f, "in_review"),
            Self::Cancelling => write!(f, "cancelling"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

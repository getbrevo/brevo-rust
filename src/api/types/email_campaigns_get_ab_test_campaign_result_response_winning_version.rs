pub use crate::prelude::*;

/// Winning Campaign Info. pending = Campaign has been picked for sending and winning version is yet to be decided, tie = A tie happened between both the versions, notAvailable = Campaign has not yet been picked for sending.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAbTestCampaignResultResponseWinningVersion {
    NotAvailable,
    Pending,
    Tie,
    A,
    B,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAbTestCampaignResultResponseWinningVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotAvailable => serializer.serialize_str("notAvailable"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Tie => serializer.serialize_str("tie"),
            Self::A => serializer.serialize_str("A"),
            Self::B => serializer.serialize_str("B"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAbTestCampaignResultResponseWinningVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "notAvailable" => Ok(Self::NotAvailable),
            "pending" => Ok(Self::Pending),
            "tie" => Ok(Self::Tie),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAbTestCampaignResultResponseWinningVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAvailable => write!(f, "notAvailable"),
            Self::Pending => write!(f, "pending"),
            Self::Tie => write!(f, "tie"),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

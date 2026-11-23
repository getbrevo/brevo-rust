pub use crate::prelude::*;

/// Reason code for blocking / unsubscribing (This code is safe for comparison)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetTransacBlockedContactsResponseContactsItemReasonCode {
    UnsubscribedViaMa,
    UnsubscribedViaEmail,
    AdminBlocked,
    UnsubscribedViaApi,
    HardBounce,
    ContactFlaggedAsSpam,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetTransacBlockedContactsResponseContactsItemReasonCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UnsubscribedViaMa => serializer.serialize_str("unsubscribedViaMA"),
            Self::UnsubscribedViaEmail => serializer.serialize_str("unsubscribedViaEmail"),
            Self::AdminBlocked => serializer.serialize_str("adminBlocked"),
            Self::UnsubscribedViaApi => serializer.serialize_str("unsubscribedViaApi"),
            Self::HardBounce => serializer.serialize_str("hardBounce"),
            Self::ContactFlaggedAsSpam => serializer.serialize_str("contactFlaggedAsSpam"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetTransacBlockedContactsResponseContactsItemReasonCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unsubscribedViaMA" => Ok(Self::UnsubscribedViaMa),
            "unsubscribedViaEmail" => Ok(Self::UnsubscribedViaEmail),
            "adminBlocked" => Ok(Self::AdminBlocked),
            "unsubscribedViaApi" => Ok(Self::UnsubscribedViaApi),
            "hardBounce" => Ok(Self::HardBounce),
            "contactFlaggedAsSpam" => Ok(Self::ContactFlaggedAsSpam),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetTransacBlockedContactsResponseContactsItemReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsubscribedViaMa => write!(f, "unsubscribedViaMA"),
            Self::UnsubscribedViaEmail => write!(f, "unsubscribedViaEmail"),
            Self::AdminBlocked => write!(f, "adminBlocked"),
            Self::UnsubscribedViaApi => write!(f, "unsubscribedViaApi"),
            Self::HardBounce => write!(f, "hardBounce"),
            Self::ContactFlaggedAsSpam => write!(f, "contactFlaggedAsSpam"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

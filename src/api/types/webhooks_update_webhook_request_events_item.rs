pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateWebhookRequestEventsItem {
    Sent,
    HardBounce,
    SoftBounce,
    Blocked,
    Spam,
    Delivered,
    Request,
    Click,
    Invalid,
    Deferred,
    Opened,
    UniqueOpened,
    Unsubscribed,
    ListAddition,
    ContactUpdated,
    ContactDeleted,
    InboundEmailProcessed,
    Reply,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateWebhookRequestEventsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sent => serializer.serialize_str("sent"),
            Self::HardBounce => serializer.serialize_str("hardBounce"),
            Self::SoftBounce => serializer.serialize_str("softBounce"),
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Spam => serializer.serialize_str("spam"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Request => serializer.serialize_str("request"),
            Self::Click => serializer.serialize_str("click"),
            Self::Invalid => serializer.serialize_str("invalid"),
            Self::Deferred => serializer.serialize_str("deferred"),
            Self::Opened => serializer.serialize_str("opened"),
            Self::UniqueOpened => serializer.serialize_str("uniqueOpened"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::ListAddition => serializer.serialize_str("listAddition"),
            Self::ContactUpdated => serializer.serialize_str("contactUpdated"),
            Self::ContactDeleted => serializer.serialize_str("contactDeleted"),
            Self::InboundEmailProcessed => serializer.serialize_str("inboundEmailProcessed"),
            Self::Reply => serializer.serialize_str("reply"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateWebhookRequestEventsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sent" => Ok(Self::Sent),
            "hardBounce" => Ok(Self::HardBounce),
            "softBounce" => Ok(Self::SoftBounce),
            "blocked" => Ok(Self::Blocked),
            "spam" => Ok(Self::Spam),
            "delivered" => Ok(Self::Delivered),
            "request" => Ok(Self::Request),
            "click" => Ok(Self::Click),
            "invalid" => Ok(Self::Invalid),
            "deferred" => Ok(Self::Deferred),
            "opened" => Ok(Self::Opened),
            "uniqueOpened" => Ok(Self::UniqueOpened),
            "unsubscribed" => Ok(Self::Unsubscribed),
            "listAddition" => Ok(Self::ListAddition),
            "contactUpdated" => Ok(Self::ContactUpdated),
            "contactDeleted" => Ok(Self::ContactDeleted),
            "inboundEmailProcessed" => Ok(Self::InboundEmailProcessed),
            "reply" => Ok(Self::Reply),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateWebhookRequestEventsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::HardBounce => write!(f, "hardBounce"),
            Self::SoftBounce => write!(f, "softBounce"),
            Self::Blocked => write!(f, "blocked"),
            Self::Spam => write!(f, "spam"),
            Self::Delivered => write!(f, "delivered"),
            Self::Request => write!(f, "request"),
            Self::Click => write!(f, "click"),
            Self::Invalid => write!(f, "invalid"),
            Self::Deferred => write!(f, "deferred"),
            Self::Opened => write!(f, "opened"),
            Self::UniqueOpened => write!(f, "uniqueOpened"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::ListAddition => write!(f, "listAddition"),
            Self::ContactUpdated => write!(f, "contactUpdated"),
            Self::ContactDeleted => write!(f, "contactDeleted"),
            Self::InboundEmailProcessed => write!(f, "inboundEmailProcessed"),
            Self::Reply => write!(f, "reply"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

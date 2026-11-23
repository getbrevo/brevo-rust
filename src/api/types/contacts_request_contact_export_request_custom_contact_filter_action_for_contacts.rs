pub use crate::prelude::*;

/// **Mandatory if neither actionForEmailCampaigns nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of the provided action, restricted to the contacts selected by the other filters below. * **allContacts** - Fetch all the selected contacts. * **subscribed & unsubscribed** - Fetch the subscribed / unsubscribed (blacklisted via any means) contacts among the selected contacts. * **unsubscribedPerList** - Fetch the contacts that are unsubscribed from the given listId only; this action requires listId.
///
/// The selection must be narrowed by at least one of **listId**, **segmentId**, **modifiedSince** or **createdSince**. Any of them can be combined with the action, so for example `unsubscribed` together with `modifiedSince` exports the contacts that are currently blocklisted and were modified after the given date-time, with no list or segment involved.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestContactExportRequestCustomContactFilterActionForContacts {
    AllContacts,
    Subscribed,
    Unsubscribed,
    UnsubscribedPerList,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RequestContactExportRequestCustomContactFilterActionForContacts {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AllContacts => serializer.serialize_str("allContacts"),
            Self::Subscribed => serializer.serialize_str("subscribed"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::UnsubscribedPerList => serializer.serialize_str("unsubscribedPerList"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RequestContactExportRequestCustomContactFilterActionForContacts {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allContacts" => Ok(Self::AllContacts),
            "subscribed" => Ok(Self::Subscribed),
            "unsubscribed" => Ok(Self::Unsubscribed),
            "unsubscribedPerList" => Ok(Self::UnsubscribedPerList),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RequestContactExportRequestCustomContactFilterActionForContacts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllContacts => write!(f, "allContacts"),
            Self::Subscribed => write!(f, "subscribed"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::UnsubscribedPerList => write!(f, "unsubscribedPerList"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

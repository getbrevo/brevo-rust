pub use crate::prelude::*;

/// **Mandatory if neither actionForContacts nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on email campaigns. * **openers & nonOpeners** - emailCampaignId is mandatory. Fetch the list of readers / non-readers for a particular email campaign. * **clickers & nonClickers** - emailCampaignId is mandatory. Fetch the list of clickers / non-clickers for a particular email campaign. * **unsubscribed** - emailCampaignId is mandatory. Fetch the list of all unsubscribed (blacklisted via any means) contacts for a particular email campaign. * **hardBounces & softBounces** - emailCampaignId is optional. Fetch the list of hard bounces / soft bounces for a particular / all email campaign(s).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestContactExportRequestCustomContactFilterActionForEmailCampaigns {
    Openers,
    NonOpeners,
    Clickers,
    NonClickers,
    Unsubscribed,
    HardBounces,
    SoftBounces,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RequestContactExportRequestCustomContactFilterActionForEmailCampaigns {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Openers => serializer.serialize_str("openers"),
            Self::NonOpeners => serializer.serialize_str("nonOpeners"),
            Self::Clickers => serializer.serialize_str("clickers"),
            Self::NonClickers => serializer.serialize_str("nonClickers"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::HardBounces => serializer.serialize_str("hardBounces"),
            Self::SoftBounces => serializer.serialize_str("softBounces"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for RequestContactExportRequestCustomContactFilterActionForEmailCampaigns
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "openers" => Ok(Self::Openers),
            "nonOpeners" => Ok(Self::NonOpeners),
            "clickers" => Ok(Self::Clickers),
            "nonClickers" => Ok(Self::NonClickers),
            "unsubscribed" => Ok(Self::Unsubscribed),
            "hardBounces" => Ok(Self::HardBounces),
            "softBounces" => Ok(Self::SoftBounces),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RequestContactExportRequestCustomContactFilterActionForEmailCampaigns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Openers => write!(f, "openers"),
            Self::NonOpeners => write!(f, "nonOpeners"),
            Self::Clickers => write!(f, "clickers"),
            Self::NonClickers => write!(f, "nonClickers"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::HardBounces => write!(f, "hardBounces"),
            Self::SoftBounces => write!(f, "softBounces"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

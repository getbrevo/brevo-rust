pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PutresendcancelinvitationRequestAction {
    Resend,
    Cancel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PutresendcancelinvitationRequestAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Resend => serializer.serialize_str("resend"),
            Self::Cancel => serializer.serialize_str("cancel"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PutresendcancelinvitationRequestAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "resend" => Ok(Self::Resend),
            "cancel" => Ok(Self::Cancel),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PutresendcancelinvitationRequestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resend => write!(f, "resend"),
            Self::Cancel => write!(f, "cancel"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

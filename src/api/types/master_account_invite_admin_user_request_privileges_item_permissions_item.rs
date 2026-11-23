pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InviteAdminUserRequestPrivilegesItemPermissionsItem {
    All,
    None,
    Create,
    EditDelete,
    DownloadData,
    CreateAlerts,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InviteAdminUserRequestPrivilegesItemPermissionsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::None => serializer.serialize_str("none"),
            Self::Create => serializer.serialize_str("create"),
            Self::EditDelete => serializer.serialize_str("edit_delete"),
            Self::DownloadData => serializer.serialize_str("download_data"),
            Self::CreateAlerts => serializer.serialize_str("create_alerts"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InviteAdminUserRequestPrivilegesItemPermissionsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "create" => Ok(Self::Create),
            "edit_delete" => Ok(Self::EditDelete),
            "download_data" => Ok(Self::DownloadData),
            "create_alerts" => Ok(Self::CreateAlerts),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InviteAdminUserRequestPrivilegesItemPermissionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::Create => write!(f, "create"),
            Self::EditDelete => write!(f, "edit_delete"),
            Self::DownloadData => write!(f, "download_data"),
            Self::CreateAlerts => write!(f, "create_alerts"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

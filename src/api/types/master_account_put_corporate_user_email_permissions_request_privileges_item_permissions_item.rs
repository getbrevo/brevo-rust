pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem {
    All,
    None,
    Create,
    EditDelete,
    CreateAlerts,
    DownloadData,
    MyLooks,
    ExploreCreate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::None => serializer.serialize_str("none"),
            Self::Create => serializer.serialize_str("create"),
            Self::EditDelete => serializer.serialize_str("edit_delete"),
            Self::CreateAlerts => serializer.serialize_str("create_alerts"),
            Self::DownloadData => serializer.serialize_str("download_data"),
            Self::MyLooks => serializer.serialize_str("my_looks"),
            Self::ExploreCreate => serializer.serialize_str("explore_create"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "create" => Ok(Self::Create),
            "edit_delete" => Ok(Self::EditDelete),
            "create_alerts" => Ok(Self::CreateAlerts),
            "download_data" => Ok(Self::DownloadData),
            "my_looks" => Ok(Self::MyLooks),
            "explore_create" => Ok(Self::ExploreCreate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::Create => write!(f, "create"),
            Self::EditDelete => write!(f, "edit_delete"),
            Self::CreateAlerts => write!(f, "create_alerts"),
            Self::DownloadData => write!(f, "download_data"),
            Self::MyLooks => write!(f, "my_looks"),
            Self::ExploreCreate => write!(f, "explore_create"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

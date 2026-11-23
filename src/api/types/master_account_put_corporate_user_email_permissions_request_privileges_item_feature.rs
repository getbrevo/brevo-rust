pub use crate::prelude::*;

/// feature name
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature {
    UserManagement,
    Api,
    MyPlan,
    AppsManagement,
    Analytics,
    SubOrganizationGroups,
    CreateSubOrganizations,
    ManageSubOrganizations,
    Security,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UserManagement => serializer.serialize_str("user_management"),
            Self::Api => serializer.serialize_str("api"),
            Self::MyPlan => serializer.serialize_str("my_plan"),
            Self::AppsManagement => serializer.serialize_str("apps_management"),
            Self::Analytics => serializer.serialize_str("analytics"),
            Self::SubOrganizationGroups => serializer.serialize_str("sub_organization_groups"),
            Self::CreateSubOrganizations => serializer.serialize_str("create_sub_organizations"),
            Self::ManageSubOrganizations => serializer.serialize_str("manage_sub_organizations"),
            Self::Security => serializer.serialize_str("security"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user_management" => Ok(Self::UserManagement),
            "api" => Ok(Self::Api),
            "my_plan" => Ok(Self::MyPlan),
            "apps_management" => Ok(Self::AppsManagement),
            "analytics" => Ok(Self::Analytics),
            "sub_organization_groups" => Ok(Self::SubOrganizationGroups),
            "create_sub_organizations" => Ok(Self::CreateSubOrganizations),
            "manage_sub_organizations" => Ok(Self::ManageSubOrganizations),
            "security" => Ok(Self::Security),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserManagement => write!(f, "user_management"),
            Self::Api => write!(f, "api"),
            Self::MyPlan => write!(f, "my_plan"),
            Self::AppsManagement => write!(f, "apps_management"),
            Self::Analytics => write!(f, "analytics"),
            Self::SubOrganizationGroups => write!(f, "sub_organization_groups"),
            Self::CreateSubOrganizations => write!(f, "create_sub_organizations"),
            Self::ManageSubOrganizations => write!(f, "manage_sub_organizations"),
            Self::Security => write!(f, "security"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

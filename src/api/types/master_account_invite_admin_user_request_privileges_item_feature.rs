pub use crate::prelude::*;

/// Feature name
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InviteAdminUserRequestPrivilegesItemFeature {
    MyPlan,
    Api,
    UserManagement,
    AppManagement,
    SubOrganizationGroups,
    CreateSubOrganizations,
    ManageSubOrganizations,
    Analytics,
    Security,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InviteAdminUserRequestPrivilegesItemFeature {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MyPlan => serializer.serialize_str("my_plan"),
            Self::Api => serializer.serialize_str("api"),
            Self::UserManagement => serializer.serialize_str("user_management"),
            Self::AppManagement => serializer.serialize_str("app_management"),
            Self::SubOrganizationGroups => serializer.serialize_str("sub_organization_groups"),
            Self::CreateSubOrganizations => serializer.serialize_str("create_sub_organizations"),
            Self::ManageSubOrganizations => serializer.serialize_str("manage_sub_organizations"),
            Self::Analytics => serializer.serialize_str("analytics"),
            Self::Security => serializer.serialize_str("security"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InviteAdminUserRequestPrivilegesItemFeature {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "my_plan" => Ok(Self::MyPlan),
            "api" => Ok(Self::Api),
            "user_management" => Ok(Self::UserManagement),
            "app_management" => Ok(Self::AppManagement),
            "sub_organization_groups" => Ok(Self::SubOrganizationGroups),
            "create_sub_organizations" => Ok(Self::CreateSubOrganizations),
            "manage_sub_organizations" => Ok(Self::ManageSubOrganizations),
            "analytics" => Ok(Self::Analytics),
            "security" => Ok(Self::Security),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InviteAdminUserRequestPrivilegesItemFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MyPlan => write!(f, "my_plan"),
            Self::Api => write!(f, "api"),
            Self::UserManagement => write!(f, "user_management"),
            Self::AppManagement => write!(f, "app_management"),
            Self::SubOrganizationGroups => write!(f, "sub_organization_groups"),
            Self::CreateSubOrganizations => write!(f, "create_sub_organizations"),
            Self::ManageSubOrganizations => write!(f, "manage_sub_organizations"),
            Self::Analytics => write!(f, "analytics"),
            Self::Security => write!(f, "security"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

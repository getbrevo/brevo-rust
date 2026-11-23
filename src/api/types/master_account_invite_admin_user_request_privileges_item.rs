pub use crate::prelude::*;

/// Privileges given to the user
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteAdminUserRequestPrivilegesItem {
    /// Feature name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<InviteAdminUserRequestPrivilegesItemFeature>,
    /// Permissions for a given feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<InviteAdminUserRequestPrivilegesItemPermissionsItem>>,
}

impl InviteAdminUserRequestPrivilegesItem {
    pub fn builder() -> InviteAdminUserRequestPrivilegesItemBuilder {
        <InviteAdminUserRequestPrivilegesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteAdminUserRequestPrivilegesItemBuilder {
    feature: Option<InviteAdminUserRequestPrivilegesItemFeature>,
    permissions: Option<Vec<InviteAdminUserRequestPrivilegesItemPermissionsItem>>,
}

impl InviteAdminUserRequestPrivilegesItemBuilder {
    pub fn feature(mut self, value: InviteAdminUserRequestPrivilegesItemFeature) -> Self {
        self.feature = Some(value);
        self
    }

    pub fn permissions(
        mut self,
        value: Vec<InviteAdminUserRequestPrivilegesItemPermissionsItem>,
    ) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InviteAdminUserRequestPrivilegesItem`].
    pub fn build(self) -> Result<InviteAdminUserRequestPrivilegesItem, BuildError> {
        Ok(InviteAdminUserRequestPrivilegesItem {
            feature: self.feature,
            permissions: self.permissions,
        })
    }
}

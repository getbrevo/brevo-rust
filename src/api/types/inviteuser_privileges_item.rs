pub use crate::prelude::*;

/// Privileges given to the user
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteuserPrivilegesItem {
    /// Feature name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<InviteuserPrivilegesItemFeature>,
    /// Permissions for a given feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<InviteuserPrivilegesItemPermissionsItem>>,
}

impl InviteuserPrivilegesItem {
    pub fn builder() -> InviteuserPrivilegesItemBuilder {
        <InviteuserPrivilegesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteuserPrivilegesItemBuilder {
    feature: Option<InviteuserPrivilegesItemFeature>,
    permissions: Option<Vec<InviteuserPrivilegesItemPermissionsItem>>,
}

impl InviteuserPrivilegesItemBuilder {
    pub fn feature(mut self, value: InviteuserPrivilegesItemFeature) -> Self {
        self.feature = Some(value);
        self
    }

    pub fn permissions(mut self, value: Vec<InviteuserPrivilegesItemPermissionsItem>) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InviteuserPrivilegesItem`].
    pub fn build(self) -> Result<InviteuserPrivilegesItem, BuildError> {
        Ok(InviteuserPrivilegesItem {
            feature: self.feature,
            permissions: self.permissions,
        })
    }
}

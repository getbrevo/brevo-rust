pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetUserPermissionResponsePrivilegesItem {
    #[serde(default)]
    pub feature: String,
    #[serde(default)]
    pub permissions: Vec<String>,
}

impl GetUserPermissionResponsePrivilegesItem {
    pub fn builder() -> GetUserPermissionResponsePrivilegesItemBuilder {
        <GetUserPermissionResponsePrivilegesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetUserPermissionResponsePrivilegesItemBuilder {
    feature: Option<String>,
    permissions: Option<Vec<String>>,
}

impl GetUserPermissionResponsePrivilegesItemBuilder {
    pub fn feature(mut self, value: impl Into<String>) -> Self {
        self.feature = Some(value.into());
        self
    }

    pub fn permissions(mut self, value: Vec<String>) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetUserPermissionResponsePrivilegesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feature`](GetUserPermissionResponsePrivilegesItemBuilder::feature)
    /// - [`permissions`](GetUserPermissionResponsePrivilegesItemBuilder::permissions)
    pub fn build(self) -> Result<GetUserPermissionResponsePrivilegesItem, BuildError> {
        Ok(GetUserPermissionResponsePrivilegesItem {
            feature: self
                .feature
                .ok_or_else(|| BuildError::missing_field("feature"))?,
            permissions: self
                .permissions
                .ok_or_else(|| BuildError::missing_field("permissions"))?,
        })
    }
}

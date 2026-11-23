pub use crate::prelude::*;

/// Groups details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateUserPermissionResponseGroupsItem {
    /// group identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetCorporateUserPermissionResponseGroupsItem {
    pub fn builder() -> GetCorporateUserPermissionResponseGroupsItemBuilder {
        <GetCorporateUserPermissionResponseGroupsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateUserPermissionResponseGroupsItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl GetCorporateUserPermissionResponseGroupsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateUserPermissionResponseGroupsItem`].
    pub fn build(self) -> Result<GetCorporateUserPermissionResponseGroupsItem, BuildError> {
        Ok(GetCorporateUserPermissionResponseGroupsItem {
            id: self.id,
            name: self.name,
        })
    }
}

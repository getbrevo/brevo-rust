pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponseGroupsItem {
    /// Group id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetCorporateSubAccountIdResponseGroupsItem {
    pub fn builder() -> GetCorporateSubAccountIdResponseGroupsItemBuilder {
        <GetCorporateSubAccountIdResponseGroupsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponseGroupsItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl GetCorporateSubAccountIdResponseGroupsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponseGroupsItem`].
    pub fn build(self) -> Result<GetCorporateSubAccountIdResponseGroupsItem, BuildError> {
        Ok(GetCorporateSubAccountIdResponseGroupsItem {
            id: self.id,
            name: self.name,
        })
    }
}

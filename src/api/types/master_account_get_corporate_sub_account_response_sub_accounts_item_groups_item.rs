pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountResponseSubAccountsItemGroupsItem {
    /// Group identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetCorporateSubAccountResponseSubAccountsItemGroupsItem {
    pub fn builder() -> GetCorporateSubAccountResponseSubAccountsItemGroupsItemBuilder {
        <GetCorporateSubAccountResponseSubAccountsItemGroupsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountResponseSubAccountsItemGroupsItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl GetCorporateSubAccountResponseSubAccountsItemGroupsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountResponseSubAccountsItemGroupsItem`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountResponseSubAccountsItemGroupsItem, BuildError> {
        Ok(GetCorporateSubAccountResponseSubAccountsItemGroupsItem {
            id: self.id,
            name: self.name,
        })
    }
}

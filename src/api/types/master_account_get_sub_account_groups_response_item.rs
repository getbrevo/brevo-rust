pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSubAccountGroupsResponseItem {
    /// The name of the group of sub-accounts
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Unique id of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl GetSubAccountGroupsResponseItem {
    pub fn builder() -> GetSubAccountGroupsResponseItemBuilder {
        <GetSubAccountGroupsResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSubAccountGroupsResponseItemBuilder {
    group_name: Option<String>,
    id: Option<String>,
}

impl GetSubAccountGroupsResponseItemBuilder {
    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSubAccountGroupsResponseItem`].
    pub fn build(self) -> Result<GetSubAccountGroupsResponseItem, BuildError> {
        Ok(GetSubAccountGroupsResponseItem {
            group_name: self.group_name,
            id: self.id,
        })
    }
}

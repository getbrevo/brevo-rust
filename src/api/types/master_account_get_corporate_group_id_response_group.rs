pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateGroupIdResponseGroup {
    /// Group creation date
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Name of the group
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Group id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl GetCorporateGroupIdResponseGroup {
    pub fn builder() -> GetCorporateGroupIdResponseGroupBuilder {
        <GetCorporateGroupIdResponseGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateGroupIdResponseGroupBuilder {
    created_at: Option<String>,
    group_name: Option<String>,
    id: Option<String>,
}

impl GetCorporateGroupIdResponseGroupBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateGroupIdResponseGroup`].
    pub fn build(self) -> Result<GetCorporateGroupIdResponseGroup, BuildError> {
        Ok(GetCorporateGroupIdResponseGroup {
            created_at: self.created_at,
            group_name: self.group_name,
            id: self.id,
        })
    }
}

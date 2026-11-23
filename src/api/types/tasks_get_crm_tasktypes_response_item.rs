pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCrmTasktypesResponseItem {
    /// Id of task type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Title of task type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl GetCrmTasktypesResponseItem {
    pub fn builder() -> GetCrmTasktypesResponseItemBuilder {
        <GetCrmTasktypesResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrmTasktypesResponseItemBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl GetCrmTasktypesResponseItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCrmTasktypesResponseItem`].
    pub fn build(self) -> Result<GetCrmTasktypesResponseItem, BuildError> {
        Ok(GetCrmTasktypesResponseItem {
            id: self.id,
            title: self.title,
        })
    }
}

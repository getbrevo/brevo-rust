pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAssociatedRecordsResponseItemsItem {
    /// The associated record. `ext_id`, `attributes`, `created_at` and `updated_at` are omitted when they are not available for the record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<GetAssociatedRecordsResponseItemsItemObject>,
    /// Object type of the associated record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetAssociatedRecordsResponseItemsItem {
    pub fn builder() -> GetAssociatedRecordsResponseItemsItemBuilder {
        <GetAssociatedRecordsResponseItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAssociatedRecordsResponseItemsItemBuilder {
    object: Option<GetAssociatedRecordsResponseItemsItemObject>,
    r#type: Option<String>,
}

impl GetAssociatedRecordsResponseItemsItemBuilder {
    pub fn object(mut self, value: GetAssociatedRecordsResponseItemsItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAssociatedRecordsResponseItemsItem`].
    pub fn build(self) -> Result<GetAssociatedRecordsResponseItemsItem, BuildError> {
        Ok(GetAssociatedRecordsResponseItemsItem {
            object: self.object,
            r#type: self.r#type,
        })
    }
}

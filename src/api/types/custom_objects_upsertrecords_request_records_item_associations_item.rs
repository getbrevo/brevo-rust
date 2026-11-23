pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertrecordsRequestRecordsItemAssociationsItem {
    /// Type of the associated object
    #[serde(default)]
    pub object_type: String,
    /// Action to perform on the association. - `link`: Create association between records (default if not provided) - `unlink`: Remove association between records. Idempotent — unlinking a non-existing association is a no-op success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<UpsertrecordsRequestRecordsItemAssociationsItemAction>,
    #[serde(default)]
    pub records: Vec<UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem>,
}

impl UpsertrecordsRequestRecordsItemAssociationsItem {
    pub fn builder() -> UpsertrecordsRequestRecordsItemAssociationsItemBuilder {
        <UpsertrecordsRequestRecordsItemAssociationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertrecordsRequestRecordsItemAssociationsItemBuilder {
    object_type: Option<String>,
    action: Option<UpsertrecordsRequestRecordsItemAssociationsItemAction>,
    records: Option<Vec<UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem>>,
}

impl UpsertrecordsRequestRecordsItemAssociationsItemBuilder {
    pub fn object_type(mut self, value: impl Into<String>) -> Self {
        self.object_type = Some(value.into());
        self
    }

    pub fn action(mut self, value: UpsertrecordsRequestRecordsItemAssociationsItemAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn records(
        mut self,
        value: Vec<UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem>,
    ) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertrecordsRequestRecordsItemAssociationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object_type`](UpsertrecordsRequestRecordsItemAssociationsItemBuilder::object_type)
    /// - [`records`](UpsertrecordsRequestRecordsItemAssociationsItemBuilder::records)
    pub fn build(self) -> Result<UpsertrecordsRequestRecordsItemAssociationsItem, BuildError> {
        Ok(UpsertrecordsRequestRecordsItemAssociationsItem {
            object_type: self
                .object_type
                .ok_or_else(|| BuildError::missing_field("object_type"))?,
            action: self.action,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}

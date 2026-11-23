pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem {
    /// Identifiers for the associated object record. Provide either `ext_id` or `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemIdentifiers>,
}

impl UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem {
    pub fn builder() -> UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemBuilder {
        <UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemBuilder {
    identifiers: Option<UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemIdentifiers>,
}

impl UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemBuilder {
    pub fn identifiers(
        mut self,
        value: UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemIdentifiers,
    ) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem`].
    pub fn build(
        self,
    ) -> Result<UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem, BuildError> {
        Ok(UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem {
            identifiers: self.identifiers,
        })
    }
}

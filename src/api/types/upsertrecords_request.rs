pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpsertrecordsRequest {
    /// List of object records to be upsert. Each record can have attributes, identifiers, and associations.
    #[serde(default)]
    pub records: Vec<UpsertrecordsRequestRecordsItem>,
}

impl UpsertrecordsRequest {
    pub fn builder() -> UpsertrecordsRequestBuilder {
        <UpsertrecordsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertrecordsRequestBuilder {
    records: Option<Vec<UpsertrecordsRequestRecordsItem>>,
}

impl UpsertrecordsRequestBuilder {
    pub fn records(mut self, value: Vec<UpsertrecordsRequestRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertrecordsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](UpsertrecordsRequestBuilder::records)
    pub fn build(self) -> Result<UpsertrecordsRequest, BuildError> {
        Ok(UpsertrecordsRequest {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}

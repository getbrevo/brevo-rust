pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct BatchDeleteObjectRecordsRequestIdentifiersExtIds {
    /// List of external record identifiers
    pub ext_ids: Vec<String>,
}

impl BatchDeleteObjectRecordsRequestIdentifiersExtIds {
    pub fn builder() -> BatchDeleteObjectRecordsRequestIdentifiersExtIdsBuilder {
        <BatchDeleteObjectRecordsRequestIdentifiersExtIdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchDeleteObjectRecordsRequestIdentifiersExtIdsBuilder {
    ext_ids: Option<Vec<String>>,
}

impl BatchDeleteObjectRecordsRequestIdentifiersExtIdsBuilder {
    pub fn ext_ids(mut self, value: Vec<String>) -> Self {
        self.ext_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchDeleteObjectRecordsRequestIdentifiersExtIds`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ext_ids`](BatchDeleteObjectRecordsRequestIdentifiersExtIdsBuilder::ext_ids)
    pub fn build(self) -> Result<BatchDeleteObjectRecordsRequestIdentifiersExtIds, BuildError> {
        Ok(BatchDeleteObjectRecordsRequestIdentifiersExtIds {
            ext_ids: self
                .ext_ids
                .ok_or_else(|| BuildError::missing_field("ext_ids"))?,
        })
    }
}

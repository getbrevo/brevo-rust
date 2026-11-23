pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct BatchDeleteObjectRecordsRequestIdentifiersIds {
    /// List of object record IDs
    pub ids: Vec<i64>,
}

impl BatchDeleteObjectRecordsRequestIdentifiersIds {
    pub fn builder() -> BatchDeleteObjectRecordsRequestIdentifiersIdsBuilder {
        <BatchDeleteObjectRecordsRequestIdentifiersIdsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchDeleteObjectRecordsRequestIdentifiersIdsBuilder {
    ids: Option<Vec<i64>>,
}

impl BatchDeleteObjectRecordsRequestIdentifiersIdsBuilder {
    pub fn ids(mut self, value: Vec<i64>) -> Self {
        self.ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchDeleteObjectRecordsRequestIdentifiersIds`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ids`](BatchDeleteObjectRecordsRequestIdentifiersIdsBuilder::ids)
    pub fn build(self) -> Result<BatchDeleteObjectRecordsRequestIdentifiersIds, BuildError> {
        Ok(BatchDeleteObjectRecordsRequestIdentifiersIds {
            ids: self.ids.ok_or_else(|| BuildError::missing_field("ids"))?,
        })
    }
}

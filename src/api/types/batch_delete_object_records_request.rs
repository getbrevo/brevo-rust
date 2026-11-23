pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchDeleteObjectRecordsRequest {
    /// Either `ids` or `ext_ids` must be provided, but not both in the same request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<BatchDeleteObjectRecordsRequestIdentifiers>,
}

impl BatchDeleteObjectRecordsRequest {
    pub fn builder() -> BatchDeleteObjectRecordsRequestBuilder {
        <BatchDeleteObjectRecordsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchDeleteObjectRecordsRequestBuilder {
    identifiers: Option<BatchDeleteObjectRecordsRequestIdentifiers>,
}

impl BatchDeleteObjectRecordsRequestBuilder {
    pub fn identifiers(mut self, value: BatchDeleteObjectRecordsRequestIdentifiers) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchDeleteObjectRecordsRequest`].
    pub fn build(self) -> Result<BatchDeleteObjectRecordsRequest, BuildError> {
        Ok(BatchDeleteObjectRecordsRequest {
            identifiers: self.identifiers,
        })
    }
}

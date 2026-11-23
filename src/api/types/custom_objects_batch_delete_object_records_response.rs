pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchDeleteObjectRecordsResponse {
    /// Identifier for batch process tracking
    #[serde(rename = "processId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BatchDeleteObjectRecordsResponse {
    pub fn builder() -> BatchDeleteObjectRecordsResponseBuilder {
        <BatchDeleteObjectRecordsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchDeleteObjectRecordsResponseBuilder {
    process_id: Option<i64>,
    message: Option<String>,
}

impl BatchDeleteObjectRecordsResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BatchDeleteObjectRecordsResponse`].
    pub fn build(self) -> Result<BatchDeleteObjectRecordsResponse, BuildError> {
        Ok(BatchDeleteObjectRecordsResponse {
            process_id: self.process_id,
            message: self.message,
        })
    }
}

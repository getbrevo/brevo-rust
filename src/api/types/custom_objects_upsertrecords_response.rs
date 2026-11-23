pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertrecordsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Unique Id for the batch process used to track the status of the batch. **How to use this processId:**   Refer to the [Get process status API](https://developers.brevo.com/reference/get-process) to check the execution status of this batch using the returned `processId`.
    #[serde(rename = "processId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
}

impl UpsertrecordsResponse {
    pub fn builder() -> UpsertrecordsResponseBuilder {
        <UpsertrecordsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertrecordsResponseBuilder {
    message: Option<String>,
    process_id: Option<i64>,
}

impl UpsertrecordsResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertrecordsResponse`].
    pub fn build(self) -> Result<UpsertrecordsResponse, BuildError> {
        Ok(UpsertrecordsResponse {
            message: self.message,
            process_id: self.process_id,
        })
    }
}

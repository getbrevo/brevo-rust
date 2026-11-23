pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportWebhooksHistoryResponse {
    /// Id of the process created
    #[serde(rename = "processId")]
    #[serde(default)]
    pub process_id: i64,
}

impl ExportWebhooksHistoryResponse {
    pub fn builder() -> ExportWebhooksHistoryResponseBuilder {
        <ExportWebhooksHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportWebhooksHistoryResponseBuilder {
    process_id: Option<i64>,
}

impl ExportWebhooksHistoryResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportWebhooksHistoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`process_id`](ExportWebhooksHistoryResponseBuilder::process_id)
    pub fn build(self) -> Result<ExportWebhooksHistoryResponse, BuildError> {
        Ok(ExportWebhooksHistoryResponse {
            process_id: self
                .process_id
                .ok_or_else(|| BuildError::missing_field("process_id"))?,
        })
    }
}

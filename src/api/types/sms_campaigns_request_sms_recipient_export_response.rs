pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestSmsRecipientExportResponse {
    /// Id of the process created
    #[serde(rename = "processId")]
    #[serde(default)]
    pub process_id: i64,
}

impl RequestSmsRecipientExportResponse {
    pub fn builder() -> RequestSmsRecipientExportResponseBuilder {
        <RequestSmsRecipientExportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestSmsRecipientExportResponseBuilder {
    process_id: Option<i64>,
}

impl RequestSmsRecipientExportResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestSmsRecipientExportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`process_id`](RequestSmsRecipientExportResponseBuilder::process_id)
    pub fn build(self) -> Result<RequestSmsRecipientExportResponse, BuildError> {
        Ok(RequestSmsRecipientExportResponse {
            process_id: self
                .process_id
                .ok_or_else(|| BuildError::missing_field("process_id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestContactExportResponse {
    /// Id of the process created
    #[serde(rename = "processId")]
    #[serde(default)]
    pub process_id: i64,
}

impl RequestContactExportResponse {
    pub fn builder() -> RequestContactExportResponseBuilder {
        <RequestContactExportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestContactExportResponseBuilder {
    process_id: Option<i64>,
}

impl RequestContactExportResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestContactExportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`process_id`](RequestContactExportResponseBuilder::process_id)
    pub fn build(self) -> Result<RequestContactExportResponse, BuildError> {
        Ok(RequestContactExportResponse {
            process_id: self
                .process_id
                .ok_or_else(|| BuildError::missing_field("process_id"))?,
        })
    }
}

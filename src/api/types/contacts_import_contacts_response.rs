pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportContactsResponse {
    /// Id of the process created
    #[serde(rename = "processId")]
    #[serde(default)]
    pub process_id: i64,
}

impl ImportContactsResponse {
    pub fn builder() -> ImportContactsResponseBuilder {
        <ImportContactsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportContactsResponseBuilder {
    process_id: Option<i64>,
}

impl ImportContactsResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportContactsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`process_id`](ImportContactsResponseBuilder::process_id)
    pub fn build(self) -> Result<ImportContactsResponse, BuildError> {
        Ok(ImportContactsResponse {
            process_id: self
                .process_id
                .ok_or_else(|| BuildError::missing_field("process_id"))?,
        })
    }
}

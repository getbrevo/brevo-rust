pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailExportRecipientsResponse {
    /// Id of the process created
    #[serde(rename = "processId")]
    #[serde(default)]
    pub process_id: i64,
}

impl EmailExportRecipientsResponse {
    pub fn builder() -> EmailExportRecipientsResponseBuilder {
        <EmailExportRecipientsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailExportRecipientsResponseBuilder {
    process_id: Option<i64>,
}

impl EmailExportRecipientsResponseBuilder {
    pub fn process_id(mut self, value: i64) -> Self {
        self.process_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailExportRecipientsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`process_id`](EmailExportRecipientsResponseBuilder::process_id)
    pub fn build(self) -> Result<EmailExportRecipientsResponse, BuildError> {
        Ok(EmailExportRecipientsResponse {
            process_id: self
                .process_id
                .ok_or_else(|| BuildError::missing_field("process_id"))?,
        })
    }
}

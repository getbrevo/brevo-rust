pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactErrorModel {
    /// Error code displayed in case of a failure
    pub code: ContactErrorModelCode,
    /// Readable message associated to the failure
    #[serde(default)]
    pub message: String,
    /// Additional information about the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl ContactErrorModel {
    pub fn builder() -> ContactErrorModelBuilder {
        <ContactErrorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactErrorModelBuilder {
    code: Option<ContactErrorModelCode>,
    message: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl ContactErrorModelBuilder {
    pub fn code(mut self, value: ContactErrorModelCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContactErrorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ContactErrorModelBuilder::code)
    /// - [`message`](ContactErrorModelBuilder::message)
    pub fn build(self) -> Result<ContactErrorModel, BuildError> {
        Ok(ContactErrorModel {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            metadata: self.metadata,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorModel {
    /// Error code displayed in case of a failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Readable message associated to the failure
    #[serde(default)]
    pub message: String,
}

impl ErrorModel {
    pub fn builder() -> ErrorModelBuilder {
        <ErrorModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorModelBuilder {
    code: Option<String>,
    message: Option<String>,
}

impl ErrorModelBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ErrorModelBuilder::message)
    pub fn build(self) -> Result<ErrorModel, BuildError> {
        Ok(ErrorModel {
            code: self.code,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

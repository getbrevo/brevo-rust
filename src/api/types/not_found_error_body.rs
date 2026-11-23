pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotFoundErrorBody {
    /// Error code displayed in case of a failure
    pub code: NotFoundErrorBodyCode,
    /// Readable message associated to the failure
    #[serde(default)]
    pub message: String,
}

impl NotFoundErrorBody {
    pub fn builder() -> NotFoundErrorBodyBuilder {
        <NotFoundErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotFoundErrorBodyBuilder {
    code: Option<NotFoundErrorBodyCode>,
    message: Option<String>,
}

impl NotFoundErrorBodyBuilder {
    pub fn code(mut self, value: NotFoundErrorBodyCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotFoundErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](NotFoundErrorBodyBuilder::code)
    /// - [`message`](NotFoundErrorBodyBuilder::message)
    pub fn build(self) -> Result<NotFoundErrorBody, BuildError> {
        Ok(NotFoundErrorBody {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

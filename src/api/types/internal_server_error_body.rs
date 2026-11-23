pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InternalServerErrorBody {
    /// Error code displayed in case of a failure
    pub code: InternalServerErrorBodyCode,
    /// Readable message associated to the failure
    #[serde(default)]
    pub message: String,
}

impl InternalServerErrorBody {
    pub fn builder() -> InternalServerErrorBodyBuilder {
        <InternalServerErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InternalServerErrorBodyBuilder {
    code: Option<InternalServerErrorBodyCode>,
    message: Option<String>,
}

impl InternalServerErrorBodyBuilder {
    pub fn code(mut self, value: InternalServerErrorBodyCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InternalServerErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](InternalServerErrorBodyBuilder::code)
    /// - [`message`](InternalServerErrorBodyBuilder::message)
    pub fn build(self) -> Result<InternalServerErrorBody, BuildError> {
        Ok(InternalServerErrorBody {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BadRequestErrorBody {
    /// Error code displayed in case of a failure
    pub code: BadRequestErrorBodyCode,
    /// Readable message associated to the failure
    #[serde(default)]
    pub message: String,
}

impl BadRequestErrorBody {
    pub fn builder() -> BadRequestErrorBodyBuilder {
        <BadRequestErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadRequestErrorBodyBuilder {
    code: Option<BadRequestErrorBodyCode>,
    message: Option<String>,
}

impl BadRequestErrorBodyBuilder {
    pub fn code(mut self, value: BadRequestErrorBodyCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BadRequestErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](BadRequestErrorBodyBuilder::code)
    /// - [`message`](BadRequestErrorBodyBuilder::message)
    pub fn build(self) -> Result<BadRequestErrorBody, BuildError> {
        Ok(BadRequestErrorBody {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

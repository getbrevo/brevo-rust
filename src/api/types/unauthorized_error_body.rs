pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnauthorizedErrorBody {
    /// Error code displayed in case of a failure
    pub code: UnauthorizedErrorBodyCode,
    /// Readable message associated to the failure
    #[serde(default)]
    pub message: String,
}

impl UnauthorizedErrorBody {
    pub fn builder() -> UnauthorizedErrorBodyBuilder {
        <UnauthorizedErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnauthorizedErrorBodyBuilder {
    code: Option<UnauthorizedErrorBodyCode>,
    message: Option<String>,
}

impl UnauthorizedErrorBodyBuilder {
    pub fn code(mut self, value: UnauthorizedErrorBodyCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnauthorizedErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](UnauthorizedErrorBodyBuilder::code)
    /// - [`message`](UnauthorizedErrorBodyBuilder::message)
    pub fn build(self) -> Result<UnauthorizedErrorBody, BuildError> {
        Ok(UnauthorizedErrorBody {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

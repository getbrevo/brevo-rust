pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ForbiddenErrorBody {
    /// Error code displayed in case of a failure
    pub code: ForbiddenErrorBodyCode,
    /// Readable message associated to the failure
    #[serde(default)]
    pub message: String,
}

impl ForbiddenErrorBody {
    pub fn builder() -> ForbiddenErrorBodyBuilder {
        <ForbiddenErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForbiddenErrorBodyBuilder {
    code: Option<ForbiddenErrorBodyCode>,
    message: Option<String>,
}

impl ForbiddenErrorBodyBuilder {
    pub fn code(mut self, value: ForbiddenErrorBodyCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForbiddenErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ForbiddenErrorBodyBuilder::code)
    /// - [`message`](ForbiddenErrorBodyBuilder::message)
    pub fn build(self) -> Result<ForbiddenErrorBody, BuildError> {
        Ok(ForbiddenErrorBody {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}

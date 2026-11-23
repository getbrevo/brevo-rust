pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSsoToken {
    /// Session token, it will remain valid for 15 days.
    #[serde(default)]
    pub token: String,
}

impl GetSsoToken {
    pub fn builder() -> GetSsoTokenBuilder {
        <GetSsoTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSsoTokenBuilder {
    token: Option<String>,
}

impl GetSsoTokenBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSsoToken`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](GetSsoTokenBuilder::token)
    pub fn build(self) -> Result<GetSsoToken, BuildError> {
        Ok(GetSsoToken {
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}

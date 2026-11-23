pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateSsoTokenRequest {
    /// User email of admin account
    #[serde(default)]
    pub email: String,
}

impl PostCorporateSsoTokenRequest {
    pub fn builder() -> PostCorporateSsoTokenRequestBuilder {
        <PostCorporateSsoTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateSsoTokenRequestBuilder {
    email: Option<String>,
}

impl PostCorporateSsoTokenRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateSsoTokenRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](PostCorporateSsoTokenRequestBuilder::email)
    pub fn build(self) -> Result<PostCorporateSsoTokenRequest, BuildError> {
        Ok(PostCorporateSsoTokenRequest {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
        })
    }
}

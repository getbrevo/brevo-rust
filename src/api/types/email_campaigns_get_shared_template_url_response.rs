pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSharedTemplateUrlResponse {
    /// A unique URL for the email campaign or transactional template. This URL can be shared with other Brevo users.
    #[serde(rename = "sharedUrl")]
    #[serde(default)]
    pub shared_url: String,
}

impl GetSharedTemplateUrlResponse {
    pub fn builder() -> GetSharedTemplateUrlResponseBuilder {
        <GetSharedTemplateUrlResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSharedTemplateUrlResponseBuilder {
    shared_url: Option<String>,
}

impl GetSharedTemplateUrlResponseBuilder {
    pub fn shared_url(mut self, value: impl Into<String>) -> Self {
        self.shared_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSharedTemplateUrlResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`shared_url`](GetSharedTemplateUrlResponseBuilder::shared_url)
    pub fn build(self) -> Result<GetSharedTemplateUrlResponse, BuildError> {
        Ok(GetSharedTemplateUrlResponse {
            shared_url: self
                .shared_url
                .ok_or_else(|| BuildError::missing_field("shared_url"))?,
        })
    }
}

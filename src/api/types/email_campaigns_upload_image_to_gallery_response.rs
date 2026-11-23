pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadImageToGalleryResponse {
    /// URL of the image uploaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UploadImageToGalleryResponse {
    pub fn builder() -> UploadImageToGalleryResponseBuilder {
        <UploadImageToGalleryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadImageToGalleryResponseBuilder {
    url: Option<String>,
}

impl UploadImageToGalleryResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadImageToGalleryResponse`].
    pub fn build(self) -> Result<UploadImageToGalleryResponse, BuildError> {
        Ok(UploadImageToGalleryResponse { url: self.url })
    }
}

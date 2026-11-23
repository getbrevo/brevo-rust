pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadImageToGalleryRequest {
    /// The absolute url of the image (**no local file**). Maximum allowed size for image is **2MB**. Allowed extensions for images are: #### jpeg, jpg, png, bmp, gif.
    #[serde(rename = "imageUrl")]
    #[serde(default)]
    pub image_url: String,
    /// Name shown in your Brevo image gallery. Include the file extension, e.g. product-banner.png
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UploadImageToGalleryRequest {
    pub fn builder() -> UploadImageToGalleryRequestBuilder {
        <UploadImageToGalleryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadImageToGalleryRequestBuilder {
    image_url: Option<String>,
    name: Option<String>,
}

impl UploadImageToGalleryRequestBuilder {
    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadImageToGalleryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image_url`](UploadImageToGalleryRequestBuilder::image_url)
    pub fn build(self) -> Result<UploadImageToGalleryRequest, BuildError> {
        Ok(UploadImageToGalleryRequest {
            image_url: self
                .image_url
                .ok_or_else(|| BuildError::missing_field("image_url"))?,
            name: self.name,
        })
    }
}

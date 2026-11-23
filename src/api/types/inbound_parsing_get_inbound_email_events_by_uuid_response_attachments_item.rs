pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetInboundEmailEventsByUuidResponseAttachmentsItem {
    /// value of the Content-ID header of the attachment.
    #[serde(rename = "contentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    /// size of the attachment in bytes
    #[serde(rename = "contentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    /// value of the Content-Type header of the attachment
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// filename specified in the Content-Disposition header of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetInboundEmailEventsByUuidResponseAttachmentsItem {
    pub fn builder() -> GetInboundEmailEventsByUuidResponseAttachmentsItemBuilder {
        <GetInboundEmailEventsByUuidResponseAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetInboundEmailEventsByUuidResponseAttachmentsItemBuilder {
    content_id: Option<String>,
    content_length: Option<i64>,
    content_type: Option<String>,
    name: Option<String>,
}

impl GetInboundEmailEventsByUuidResponseAttachmentsItemBuilder {
    pub fn content_id(mut self, value: impl Into<String>) -> Self {
        self.content_id = Some(value.into());
        self
    }

    pub fn content_length(mut self, value: i64) -> Self {
        self.content_length = Some(value);
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetInboundEmailEventsByUuidResponseAttachmentsItem`].
    pub fn build(self) -> Result<GetInboundEmailEventsByUuidResponseAttachmentsItem, BuildError> {
        Ok(GetInboundEmailEventsByUuidResponseAttachmentsItem {
            content_id: self.content_id,
            content_length: self.content_length,
            content_type: self.content_type,
            name: self.name,
        })
    }
}

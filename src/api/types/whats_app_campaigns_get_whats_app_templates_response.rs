pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppTemplatesResponse {
    /// Number of whatsApp templates retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<GetWhatsAppTemplatesResponseTemplatesItem>>,
}

impl GetWhatsAppTemplatesResponse {
    pub fn builder() -> GetWhatsAppTemplatesResponseBuilder {
        <GetWhatsAppTemplatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppTemplatesResponseBuilder {
    count: Option<i64>,
    templates: Option<Vec<GetWhatsAppTemplatesResponseTemplatesItem>>,
}

impl GetWhatsAppTemplatesResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn templates(mut self, value: Vec<GetWhatsAppTemplatesResponseTemplatesItem>) -> Self {
        self.templates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppTemplatesResponse`].
    pub fn build(self) -> Result<GetWhatsAppTemplatesResponse, BuildError> {
        Ok(GetWhatsAppTemplatesResponse {
            count: self.count,
            templates: self.templates,
        })
    }
}

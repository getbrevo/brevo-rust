pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmtpTemplatesResponse {
    /// Count of transactional email templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<GetSmtpTemplateOverview>>,
}

impl GetSmtpTemplatesResponse {
    pub fn builder() -> GetSmtpTemplatesResponseBuilder {
        <GetSmtpTemplatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmtpTemplatesResponseBuilder {
    count: Option<i64>,
    templates: Option<Vec<GetSmtpTemplateOverview>>,
}

impl GetSmtpTemplatesResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn templates(mut self, value: Vec<GetSmtpTemplateOverview>) -> Self {
        self.templates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmtpTemplatesResponse`].
    pub fn build(self) -> Result<GetSmtpTemplatesResponse, BuildError> {
        Ok(GetSmtpTemplatesResponse {
            count: self.count,
            templates: self.templates,
        })
    }
}

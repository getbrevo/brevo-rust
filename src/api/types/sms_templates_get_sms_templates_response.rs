pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmsTemplatesResponse {
    /// Count of transactional sms templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<GetSmsTemplatesResponseTemplatesItem>>,
}

impl GetSmsTemplatesResponse {
    pub fn builder() -> GetSmsTemplatesResponseBuilder {
        <GetSmsTemplatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmsTemplatesResponseBuilder {
    count: Option<i64>,
    templates: Option<Vec<GetSmsTemplatesResponseTemplatesItem>>,
}

impl GetSmsTemplatesResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn templates(mut self, value: Vec<GetSmsTemplatesResponseTemplatesItem>) -> Self {
        self.templates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSmsTemplatesResponse`].
    pub fn build(self) -> Result<GetSmsTemplatesResponse, BuildError> {
        Ok(GetSmsTemplatesResponse {
            count: self.count,
            templates: self.templates,
        })
    }
}

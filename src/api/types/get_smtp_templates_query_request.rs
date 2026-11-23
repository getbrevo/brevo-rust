pub use crate::prelude::*;

/// Query parameters for getSmtpTemplates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSmtpTemplatesQueryRequest {
    /// Filter on the status of the template. Active = true, inactive = false
    #[serde(rename = "templateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_status: Option<bool>,
    /// Number of documents returned per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetSmtpTemplatesRequestSort>,
    /// Filter on the editor type used to create the template. Currently only `richTextEditor` is supported as a filter value.
    #[serde(rename = "editorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_type: Option<String>,
}

impl GetSmtpTemplatesQueryRequest {
    pub fn builder() -> GetSmtpTemplatesQueryRequestBuilder {
        <GetSmtpTemplatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSmtpTemplatesQueryRequestBuilder {
    template_status: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetSmtpTemplatesRequestSort>,
    editor_type: Option<String>,
}

impl GetSmtpTemplatesQueryRequestBuilder {
    pub fn template_status(mut self, value: bool) -> Self {
        self.template_status = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetSmtpTemplatesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn editor_type(mut self, value: impl Into<String>) -> Self {
        self.editor_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSmtpTemplatesQueryRequest`].
    pub fn build(self) -> Result<GetSmtpTemplatesQueryRequest, BuildError> {
        Ok(GetSmtpTemplatesQueryRequest {
            template_status: self.template_status,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            editor_type: self.editor_type,
        })
    }
}

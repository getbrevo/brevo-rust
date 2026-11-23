pub use crate::prelude::*;

/// Query parameters for getWhatsAppTemplates
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppTemplatesQueryRequest {
    /// **Mandatory if endDate is used**. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the templates created. **Prefer to pass your timezone in date-time format for accurate result**
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used**. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the templates created. **Prefer to pass your timezone in date-time format for accurate result**
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Index of the first document in the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort the results in the ascending/descending order of record modification. Default order is **descending** if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetWhatsAppTemplatesRequestSort>,
    /// source of the template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<GetWhatsAppTemplatesRequestSource>,
}

impl GetWhatsAppTemplatesQueryRequest {
    pub fn builder() -> GetWhatsAppTemplatesQueryRequestBuilder {
        <GetWhatsAppTemplatesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppTemplatesQueryRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetWhatsAppTemplatesRequestSort>,
    source: Option<GetWhatsAppTemplatesRequestSource>,
}

impl GetWhatsAppTemplatesQueryRequestBuilder {
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
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

    pub fn sort(mut self, value: GetWhatsAppTemplatesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn source(mut self, value: GetWhatsAppTemplatesRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppTemplatesQueryRequest`].
    pub fn build(self) -> Result<GetWhatsAppTemplatesQueryRequest, BuildError> {
        Ok(GetWhatsAppTemplatesQueryRequest {
            start_date: self.start_date,
            end_date: self.end_date,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            source: self.source,
        })
    }
}

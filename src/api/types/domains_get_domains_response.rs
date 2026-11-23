pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainsResponse {
    /// List of the domains available in your account
    #[serde(default)]
    pub domains: Vec<GetDomainsResponseDomainsItem>,
    /// Total number of domains
    #[serde(default)]
    pub count: i64,
    /// Current page number
    #[serde(default)]
    pub current_page: i64,
    /// Total number of pages
    #[serde(default)]
    pub total_pages: i64,
}

impl GetDomainsResponse {
    pub fn builder() -> GetDomainsResponseBuilder {
        <GetDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainsResponseBuilder {
    domains: Option<Vec<GetDomainsResponseDomainsItem>>,
    count: Option<i64>,
    current_page: Option<i64>,
    total_pages: Option<i64>,
}

impl GetDomainsResponseBuilder {
    pub fn domains(mut self, value: Vec<GetDomainsResponseDomainsItem>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn current_page(mut self, value: i64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDomainsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domains`](GetDomainsResponseBuilder::domains)
    /// - [`count`](GetDomainsResponseBuilder::count)
    /// - [`current_page`](GetDomainsResponseBuilder::current_page)
    /// - [`total_pages`](GetDomainsResponseBuilder::total_pages)
    pub fn build(self) -> Result<GetDomainsResponse, BuildError> {
        Ok(GetDomainsResponse {
            domains: self
                .domains
                .ok_or_else(|| BuildError::missing_field("domains"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            current_page: self
                .current_page
                .ok_or_else(|| BuildError::missing_field("current_page"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
        })
    }
}

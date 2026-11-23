pub use crate::prelude::*;

/// List of companies
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCompaniesResponse {
    /// List of companies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Company>>,
}

impl GetCompaniesResponse {
    pub fn builder() -> GetCompaniesResponseBuilder {
        <GetCompaniesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCompaniesResponseBuilder {
    items: Option<Vec<Company>>,
}

impl GetCompaniesResponseBuilder {
    pub fn items(mut self, value: Vec<Company>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCompaniesResponse`].
    pub fn build(self) -> Result<GetCompaniesResponse, BuildError> {
        Ok(GetCompaniesResponse { items: self.items })
    }
}

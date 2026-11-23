pub use crate::prelude::*;

/// List of Deals
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCrmDealsResponse {
    /// List of deals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Deal>>,
}

impl GetCrmDealsResponse {
    pub fn builder() -> GetCrmDealsResponseBuilder {
        <GetCrmDealsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrmDealsResponseBuilder {
    items: Option<Vec<Deal>>,
}

impl GetCrmDealsResponseBuilder {
    pub fn items(mut self, value: Vec<Deal>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCrmDealsResponse`].
    pub fn build(self) -> Result<GetCrmDealsResponse, BuildError> {
        Ok(GetCrmDealsResponse { items: self.items })
    }
}

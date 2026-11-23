pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetBalanceDefinitionListResponse {
    /// list of balance definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BalanceDefinition>>,
}

impl GetBalanceDefinitionListResponse {
    pub fn builder() -> GetBalanceDefinitionListResponseBuilder {
        <GetBalanceDefinitionListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBalanceDefinitionListResponseBuilder {
    items: Option<Vec<BalanceDefinition>>,
}

impl GetBalanceDefinitionListResponseBuilder {
    pub fn items(mut self, value: Vec<BalanceDefinition>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBalanceDefinitionListResponse`].
    pub fn build(self) -> Result<GetBalanceDefinitionListResponse, BuildError> {
        Ok(GetBalanceDefinitionListResponse { items: self.items })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLpListResponse {
    /// Loyalty Program list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LoyaltyProgram>>,
}

impl GetLpListResponse {
    pub fn builder() -> GetLpListResponseBuilder {
        <GetLpListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLpListResponseBuilder {
    items: Option<Vec<LoyaltyProgram>>,
}

impl GetLpListResponseBuilder {
    pub fn items(mut self, value: Vec<LoyaltyProgram>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLpListResponse`].
    pub fn build(self) -> Result<GetLpListResponse, BuildError> {
        Ok(GetLpListResponse { items: self.items })
    }
}

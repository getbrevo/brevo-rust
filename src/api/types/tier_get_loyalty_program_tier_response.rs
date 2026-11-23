pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyProgramTierResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Tier>>,
}

impl GetLoyaltyProgramTierResponse {
    pub fn builder() -> GetLoyaltyProgramTierResponseBuilder {
        <GetLoyaltyProgramTierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyProgramTierResponseBuilder {
    items: Option<Vec<Tier>>,
}

impl GetLoyaltyProgramTierResponseBuilder {
    pub fn items(mut self, value: Vec<Tier>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyProgramTierResponse`].
    pub fn build(self) -> Result<GetLoyaltyProgramTierResponse, BuildError> {
        Ok(GetLoyaltyProgramTierResponse { items: self.items })
    }
}

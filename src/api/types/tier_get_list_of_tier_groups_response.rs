pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetListOfTierGroupsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TierGroup>>,
}

impl GetListOfTierGroupsResponse {
    pub fn builder() -> GetListOfTierGroupsResponseBuilder {
        <GetListOfTierGroupsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetListOfTierGroupsResponseBuilder {
    items: Option<Vec<TierGroup>>,
}

impl GetListOfTierGroupsResponseBuilder {
    pub fn items(mut self, value: Vec<TierGroup>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetListOfTierGroupsResponse`].
    pub fn build(self) -> Result<GetListOfTierGroupsResponse, BuildError> {
        Ok(GetListOfTierGroupsResponse { items: self.items })
    }
}

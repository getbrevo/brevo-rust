pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PartiallyUpdateLoyaltyProgramRequest {
    /// Loyalty Program description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Loyalty Program meta data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Loyalty Program name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PartiallyUpdateLoyaltyProgramRequest {
    pub fn builder() -> PartiallyUpdateLoyaltyProgramRequestBuilder {
        <PartiallyUpdateLoyaltyProgramRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartiallyUpdateLoyaltyProgramRequestBuilder {
    description: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
}

impl PartiallyUpdateLoyaltyProgramRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartiallyUpdateLoyaltyProgramRequest`].
    pub fn build(self) -> Result<PartiallyUpdateLoyaltyProgramRequest, BuildError> {
        Ok(PartiallyUpdateLoyaltyProgramRequest {
            description: self.description,
            meta: self.meta,
            name: self.name,
        })
    }
}

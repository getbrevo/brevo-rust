pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateLoyaltyProgramRequest {
    /// Loyalty Program description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Loyalty Program meta data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Loyalty Program name
    #[serde(default)]
    pub name: String,
}

impl UpdateLoyaltyProgramRequest {
    pub fn builder() -> UpdateLoyaltyProgramRequestBuilder {
        <UpdateLoyaltyProgramRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLoyaltyProgramRequestBuilder {
    description: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
}

impl UpdateLoyaltyProgramRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdateLoyaltyProgramRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateLoyaltyProgramRequestBuilder::name)
    pub fn build(self) -> Result<UpdateLoyaltyProgramRequest, BuildError> {
        Ok(UpdateLoyaltyProgramRequest {
            description: self.description,
            meta: self.meta,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

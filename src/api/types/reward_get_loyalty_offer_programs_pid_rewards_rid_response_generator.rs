pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator {
    /// Timestamp when the reward was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Public facing description of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique identifier for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Generated pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Timestamp when the reward was created
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseGeneratorBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseGeneratorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseGeneratorBuilder {
    created_at: Option<String>,
    description: Option<String>,
    id: Option<String>,
    name: Option<String>,
    pattern: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseGeneratorBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidRewardsRidResponseGenerator {
            created_at: self.created_at,
            description: self.description,
            id: self.id,
            name: self.name,
            pattern: self.pattern,
            updated_at: self.updated_at,
        })
    }
}

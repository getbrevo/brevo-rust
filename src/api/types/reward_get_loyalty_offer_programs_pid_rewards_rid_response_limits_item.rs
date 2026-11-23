pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem {
    /// Timestamp when the reward limit was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Unit of time for the reward limit's availability (e.g., day/week/month/year).
    #[serde(rename = "durationUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<String>,
    /// Number of days/weeks/month/year for reward limit
    #[serde(rename = "durationValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i64>,
    /// Value of the limit
    #[serde(rename = "limitValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_value: Option<i64>,
    /// Unique identifier for the reward limit
    #[serde(rename = "rewardLimitId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_limit_id: Option<String>,
    /// Select true to calculate all redeems/attributions from the previous value of selected durationUnit to the current time
    #[serde(rename = "slidingSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sliding_schedule: Option<bool>,
    /// Type of reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Timestamp when the reward limit was created
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItemBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItemBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    duration_unit: Option<String>,
    duration_value: Option<i64>,
    limit_value: Option<i64>,
    reward_limit_id: Option<String>,
    sliding_schedule: Option<bool>,
    r#type: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItemBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: impl Into<String>) -> Self {
        self.duration_unit = Some(value.into());
        self
    }

    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn limit_value(mut self, value: i64) -> Self {
        self.limit_value = Some(value);
        self
    }

    pub fn reward_limit_id(mut self, value: impl Into<String>) -> Self {
        self.reward_limit_id = Some(value.into());
        self
    }

    pub fn sliding_schedule(mut self, value: bool) -> Self {
        self.sliding_schedule = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidRewardsRidResponseLimitsItem {
            created_at: self.created_at,
            duration_unit: self.duration_unit,
            duration_value: self.duration_value,
            limit_value: self.limit_value,
            reward_limit_id: self.reward_limit_id,
            sliding_schedule: self.sliding_schedule,
            r#type: self.r#type,
            updated_at: self.updated_at,
        })
    }
}

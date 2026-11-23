pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TierGroup {
    /// Timestamp when the tier group was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    #[serde(rename = "downgradeStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downgrade_strategy: Option<TierGroupDowngradeStrategy>,
    /// Tier group unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Associated loyalty program Id
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Tier group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Order of the tiers in the group in ascending order
    #[serde(rename = "tierOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_order: Option<Vec<String>>,
    /// Timestamp when the tier group was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    #[serde(rename = "upgradeStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_strategy: Option<TierGroupUpgradeStrategy>,
    /// Schedule configuration for tier upgrades.
    #[serde(rename = "upgradeSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_schedule: Option<TierGroupUpgradeSchedule>,
    /// Schedule configuration for tier downgrades.
    #[serde(rename = "downgradeSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downgrade_schedule: Option<TierGroupDowngradeSchedule>,
}

impl TierGroup {
    pub fn builder() -> TierGroupBuilder {
        <TierGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierGroupBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    downgrade_strategy: Option<TierGroupDowngradeStrategy>,
    id: Option<String>,
    loyalty_program_id: Option<String>,
    name: Option<String>,
    tier_order: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
    upgrade_strategy: Option<TierGroupUpgradeStrategy>,
    upgrade_schedule: Option<TierGroupUpgradeSchedule>,
    downgrade_schedule: Option<TierGroupDowngradeSchedule>,
}

impl TierGroupBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn downgrade_strategy(mut self, value: TierGroupDowngradeStrategy) -> Self {
        self.downgrade_strategy = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tier_order(mut self, value: Vec<String>) -> Self {
        self.tier_order = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn upgrade_strategy(mut self, value: TierGroupUpgradeStrategy) -> Self {
        self.upgrade_strategy = Some(value);
        self
    }

    pub fn upgrade_schedule(mut self, value: TierGroupUpgradeSchedule) -> Self {
        self.upgrade_schedule = Some(value);
        self
    }

    pub fn downgrade_schedule(mut self, value: TierGroupDowngradeSchedule) -> Self {
        self.downgrade_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TierGroup`].
    pub fn build(self) -> Result<TierGroup, BuildError> {
        Ok(TierGroup {
            created_at: self.created_at,
            downgrade_strategy: self.downgrade_strategy,
            id: self.id,
            loyalty_program_id: self.loyalty_program_id,
            name: self.name,
            tier_order: self.tier_order,
            updated_at: self.updated_at,
            upgrade_strategy: self.upgrade_strategy,
            upgrade_schedule: self.upgrade_schedule,
            downgrade_schedule: self.downgrade_schedule,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateTierGroupRequest {
    /// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    #[serde(rename = "downgradeStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downgrade_strategy: Option<CreateTierGroupRequestDowngradeStrategy>,
    /// Name of the tier group
    #[serde(default)]
    pub name: String,
    /// Additional metadata for the tier group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<CreateTierGroupRequestMeta>,
    /// Order of the tiers in the group in ascending order
    #[serde(rename = "tierOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_order: Option<Vec<String>>,
    /// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    #[serde(rename = "upgradeStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_strategy: Option<CreateTierGroupRequestUpgradeStrategy>,
    /// Schedule configuration for tier upgrades. Required when upgradeStrategy is set to a schedule-based strategy.
    #[serde(rename = "upgradeSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_schedule: Option<CreateTierGroupRequestUpgradeSchedule>,
    /// Schedule configuration for tier downgrades. Required when downgradeStrategy is set to a schedule-based strategy.
    #[serde(rename = "downgradeSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downgrade_schedule: Option<CreateTierGroupRequestDowngradeSchedule>,
}

impl CreateTierGroupRequest {
    pub fn builder() -> CreateTierGroupRequestBuilder {
        <CreateTierGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTierGroupRequestBuilder {
    downgrade_strategy: Option<CreateTierGroupRequestDowngradeStrategy>,
    name: Option<String>,
    meta: Option<CreateTierGroupRequestMeta>,
    tier_order: Option<Vec<String>>,
    upgrade_strategy: Option<CreateTierGroupRequestUpgradeStrategy>,
    upgrade_schedule: Option<CreateTierGroupRequestUpgradeSchedule>,
    downgrade_schedule: Option<CreateTierGroupRequestDowngradeSchedule>,
}

impl CreateTierGroupRequestBuilder {
    pub fn downgrade_strategy(mut self, value: CreateTierGroupRequestDowngradeStrategy) -> Self {
        self.downgrade_strategy = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn meta(mut self, value: CreateTierGroupRequestMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn tier_order(mut self, value: Vec<String>) -> Self {
        self.tier_order = Some(value);
        self
    }

    pub fn upgrade_strategy(mut self, value: CreateTierGroupRequestUpgradeStrategy) -> Self {
        self.upgrade_strategy = Some(value);
        self
    }

    pub fn upgrade_schedule(mut self, value: CreateTierGroupRequestUpgradeSchedule) -> Self {
        self.upgrade_schedule = Some(value);
        self
    }

    pub fn downgrade_schedule(mut self, value: CreateTierGroupRequestDowngradeSchedule) -> Self {
        self.downgrade_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTierGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateTierGroupRequestBuilder::name)
    pub fn build(self) -> Result<CreateTierGroupRequest, BuildError> {
        Ok(CreateTierGroupRequest {
            downgrade_strategy: self.downgrade_strategy,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            meta: self.meta,
            tier_order: self.tier_order,
            upgrade_strategy: self.upgrade_strategy,
            upgrade_schedule: self.upgrade_schedule,
            downgrade_schedule: self.downgrade_schedule,
        })
    }
}

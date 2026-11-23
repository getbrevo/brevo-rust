pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateTierGroupRequest {
    /// Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    #[serde(rename = "downgradeStrategy")]
    pub downgrade_strategy: UpdateTierGroupRequestDowngradeStrategy,
    /// Name of the tier group
    #[serde(default)]
    pub name: String,
    /// Additional metadata for the tier group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<UpdateTierGroupRequestMeta>,
    /// Order of the tiers in the group in ascending order
    #[serde(rename = "tierOrder")]
    #[serde(default)]
    pub tier_order: Vec<String>,
    /// Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    #[serde(rename = "upgradeStrategy")]
    pub upgrade_strategy: UpdateTierGroupRequestUpgradeStrategy,
}

impl UpdateTierGroupRequest {
    pub fn builder() -> UpdateTierGroupRequestBuilder {
        <UpdateTierGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTierGroupRequestBuilder {
    downgrade_strategy: Option<UpdateTierGroupRequestDowngradeStrategy>,
    name: Option<String>,
    meta: Option<UpdateTierGroupRequestMeta>,
    tier_order: Option<Vec<String>>,
    upgrade_strategy: Option<UpdateTierGroupRequestUpgradeStrategy>,
}

impl UpdateTierGroupRequestBuilder {
    pub fn downgrade_strategy(mut self, value: UpdateTierGroupRequestDowngradeStrategy) -> Self {
        self.downgrade_strategy = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn meta(mut self, value: UpdateTierGroupRequestMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn tier_order(mut self, value: Vec<String>) -> Self {
        self.tier_order = Some(value);
        self
    }

    pub fn upgrade_strategy(mut self, value: UpdateTierGroupRequestUpgradeStrategy) -> Self {
        self.upgrade_strategy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTierGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`downgrade_strategy`](UpdateTierGroupRequestBuilder::downgrade_strategy)
    /// - [`name`](UpdateTierGroupRequestBuilder::name)
    /// - [`tier_order`](UpdateTierGroupRequestBuilder::tier_order)
    /// - [`upgrade_strategy`](UpdateTierGroupRequestBuilder::upgrade_strategy)
    pub fn build(self) -> Result<UpdateTierGroupRequest, BuildError> {
        Ok(UpdateTierGroupRequest {
            downgrade_strategy: self
                .downgrade_strategy
                .ok_or_else(|| BuildError::missing_field("downgrade_strategy"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            meta: self.meta,
            tier_order: self
                .tier_order
                .ok_or_else(|| BuildError::missing_field("tier_order"))?,
            upgrade_strategy: self
                .upgrade_strategy
                .ok_or_else(|| BuildError::missing_field("upgrade_strategy"))?,
        })
    }
}

pub use crate::prelude::*;

/// Query parameters for revokeVouchers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeVouchersQueryRequest {
    /// Reward Attribution IDs (comma seperated)
    #[serde(rename = "attributedRewardIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributed_reward_ids: Option<String>,
}

impl RevokeVouchersQueryRequest {
    pub fn builder() -> RevokeVouchersQueryRequestBuilder {
        <RevokeVouchersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeVouchersQueryRequestBuilder {
    attributed_reward_ids: Option<String>,
}

impl RevokeVouchersQueryRequestBuilder {
    pub fn attributed_reward_ids(mut self, value: impl Into<String>) -> Self {
        self.attributed_reward_ids = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevokeVouchersQueryRequest`].
    pub fn build(self) -> Result<RevokeVouchersQueryRequest, BuildError> {
        Ok(RevokeVouchersQueryRequest {
            attributed_reward_ids: self.attributed_reward_ids,
        })
    }
}

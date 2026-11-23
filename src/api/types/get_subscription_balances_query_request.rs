pub use crate::prelude::*;

/// Query parameters for getSubscriptionBalances
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSubscriptionBalancesQueryRequest {
    /// Include balances tied to internal definitions.
    #[serde(rename = "includeInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_internal: Option<bool>,
}

impl GetSubscriptionBalancesQueryRequest {
    pub fn builder() -> GetSubscriptionBalancesQueryRequestBuilder {
        <GetSubscriptionBalancesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSubscriptionBalancesQueryRequestBuilder {
    include_internal: Option<bool>,
}

impl GetSubscriptionBalancesQueryRequestBuilder {
    pub fn include_internal(mut self, value: bool) -> Self {
        self.include_internal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSubscriptionBalancesQueryRequest`].
    pub fn build(self) -> Result<GetSubscriptionBalancesQueryRequest, BuildError> {
        Ok(GetSubscriptionBalancesQueryRequest {
            include_internal: self.include_internal,
        })
    }
}

pub use crate::prelude::*;

/// Query parameters for getParameterSubscriptionInfo
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetParameterSubscriptionInfoQueryRequest {
    /// The contact ID to filter by.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// A list of filter parameters for querying the subscription info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
    /// The loyalty subscription ID to filter by.
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Include balances tied to internal definitions.
    #[serde(rename = "includeInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_internal: Option<bool>,
}

impl GetParameterSubscriptionInfoQueryRequest {
    pub fn builder() -> GetParameterSubscriptionInfoQueryRequestBuilder {
        <GetParameterSubscriptionInfoQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoQueryRequestBuilder {
    contact_id: Option<String>,
    params: Option<String>,
    loyalty_subscription_id: Option<String>,
    include_internal: Option<bool>,
}

impl GetParameterSubscriptionInfoQueryRequestBuilder {
    pub fn contact_id(mut self, value: impl Into<String>) -> Self {
        self.contact_id = Some(value.into());
        self
    }

    pub fn params(mut self, value: impl Into<String>) -> Self {
        self.params = Some(value.into());
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn include_internal(mut self, value: bool) -> Self {
        self.include_internal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoQueryRequest`].
    pub fn build(self) -> Result<GetParameterSubscriptionInfoQueryRequest, BuildError> {
        Ok(GetParameterSubscriptionInfoQueryRequest {
            contact_id: self.contact_id,
            params: self.params,
            loyalty_subscription_id: self.loyalty_subscription_id,
            include_internal: self.include_internal,
        })
    }
}

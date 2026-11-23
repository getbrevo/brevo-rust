pub use crate::prelude::*;

/// Membership details of the subscription. Returned when the subscription could be resolved from the provided `contactId` or `loyaltySubscriptionId`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetParameterSubscriptionInfoResponseMembership {
    /// Timestamp when the subscription was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier of the loyalty subscription.
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Timestamp when the subscription was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl GetParameterSubscriptionInfoResponseMembership {
    pub fn builder() -> GetParameterSubscriptionInfoResponseMembershipBuilder {
        <GetParameterSubscriptionInfoResponseMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetParameterSubscriptionInfoResponseMembershipBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    loyalty_subscription_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl GetParameterSubscriptionInfoResponseMembershipBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetParameterSubscriptionInfoResponseMembership`].
    pub fn build(self) -> Result<GetParameterSubscriptionInfoResponseMembership, BuildError> {
        Ok(GetParameterSubscriptionInfoResponseMembership {
            created_at: self.created_at,
            loyalty_subscription_id: self.loyalty_subscription_id,
            updated_at: self.updated_at,
        })
    }
}

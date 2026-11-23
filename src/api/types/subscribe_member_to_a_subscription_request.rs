pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscribeMemberToASubscriptionRequest {
    /// Required if LoyaltySubscriptionId is not provided, must be greater than 0
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Required if ContactId is not provided, max length 64
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Required, each item must be greater than or equal to 1
    #[serde(rename = "memberContactIds")]
    #[serde(default)]
    pub member_contact_ids: Vec<i64>,
}

impl SubscribeMemberToASubscriptionRequest {
    pub fn builder() -> SubscribeMemberToASubscriptionRequestBuilder {
        <SubscribeMemberToASubscriptionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeMemberToASubscriptionRequestBuilder {
    contact_id: Option<i64>,
    loyalty_subscription_id: Option<String>,
    member_contact_ids: Option<Vec<i64>>,
}

impl SubscribeMemberToASubscriptionRequestBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn member_contact_ids(mut self, value: Vec<i64>) -> Self {
        self.member_contact_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeMemberToASubscriptionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`member_contact_ids`](SubscribeMemberToASubscriptionRequestBuilder::member_contact_ids)
    pub fn build(self) -> Result<SubscribeMemberToASubscriptionRequest, BuildError> {
        Ok(SubscribeMemberToASubscriptionRequest {
            contact_id: self.contact_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
            member_contact_ids: self
                .member_contact_ids
                .ok_or_else(|| BuildError::missing_field("member_contact_ids"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscribeToLoyaltyProgramRequest {
    /// Required contact ID; must be greater than 0.
    #[serde(rename = "contactId")]
    #[serde(default)]
    pub contact_id: i64,
    /// Optional subscription ID (max length 64).
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Optional creation date in ISO 8601 format (YYYY-MM-DDThh:mm:ss.ffffff+HH:MM). Must be in the past.
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub creation_date: Option<DateTime<FixedOffset>>,
    /// Optional metadata associated with the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

impl SubscribeToLoyaltyProgramRequest {
    pub fn builder() -> SubscribeToLoyaltyProgramRequestBuilder {
        <SubscribeToLoyaltyProgramRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscribeToLoyaltyProgramRequestBuilder {
    contact_id: Option<i64>,
    loyalty_subscription_id: Option<String>,
    creation_date: Option<DateTime<FixedOffset>>,
    meta: Option<HashMap<String, serde_json::Value>>,
}

impl SubscribeToLoyaltyProgramRequestBuilder {
    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn creation_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscribeToLoyaltyProgramRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_id`](SubscribeToLoyaltyProgramRequestBuilder::contact_id)
    pub fn build(self) -> Result<SubscribeToLoyaltyProgramRequest, BuildError> {
        Ok(SubscribeToLoyaltyProgramRequest {
            contact_id: self
                .contact_id
                .ok_or_else(|| BuildError::missing_field("contact_id"))?,
            loyalty_subscription_id: self.loyalty_subscription_id,
            creation_date: self.creation_date,
            meta: self.meta,
        })
    }
}

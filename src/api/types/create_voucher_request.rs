pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateVoucherRequest {
    /// Code generated to attribute reward to a contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Contact to attribute the reward
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Reward expiration date
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// One of contactId or loyaltySubscriptionId is required
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Offer meta information (key/value object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Reward id
    #[serde(rename = "rewardId")]
    #[serde(default)]
    pub reward_id: String,
    /// Value of the selected reward config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
    /// Date from which the voucher becomes valid. Accepts RFC 3339 or DD/MM/YYYY HH:MM AM/PM format. Converted to UTC using the organization's timezone.
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
}

impl CreateVoucherRequest {
    pub fn builder() -> CreateVoucherRequestBuilder {
        <CreateVoucherRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVoucherRequestBuilder {
    code: Option<String>,
    contact_id: Option<i64>,
    expiration_date: Option<String>,
    loyalty_subscription_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    reward_id: Option<String>,
    value: Option<f64>,
    valid_from: Option<String>,
}

impl CreateVoucherRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn valid_from(mut self, value: impl Into<String>) -> Self {
        self.valid_from = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateVoucherRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reward_id`](CreateVoucherRequestBuilder::reward_id)
    pub fn build(self) -> Result<CreateVoucherRequest, BuildError> {
        Ok(CreateVoucherRequest {
            code: self.code,
            contact_id: self.contact_id,
            expiration_date: self.expiration_date,
            loyalty_subscription_id: self.loyalty_subscription_id,
            meta: self.meta,
            reward_id: self
                .reward_id
                .ok_or_else(|| BuildError::missing_field("reward_id"))?,
            value: self.value,
            valid_from: self.valid_from,
        })
    }
}

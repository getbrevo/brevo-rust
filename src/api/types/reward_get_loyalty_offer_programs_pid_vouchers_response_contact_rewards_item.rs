pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem {
    /// Generated code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Timestamp when the reward limit was consumed
    #[serde(rename = "consumedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_at: Option<String>,
    /// Timestamp when the reward limit was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Reward expiration date
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Unique identifier for the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional data for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    /// Unit of the balance selected in the reward definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Timestamp when the reward limit was updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Value of the selected reward config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub value: Option<f64>,
    /// Date from which the voucher becomes valid
    #[serde(rename = "validFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub valid_from: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem {
    pub fn builder() -> GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItemBuilder {
        <GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItemBuilder {
    code: Option<String>,
    consumed_at: Option<String>,
    created_at: Option<String>,
    expiration_date: Option<String>,
    id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    reward_id: Option<String>,
    unit: Option<String>,
    updated_at: Option<String>,
    value: Option<f64>,
    valid_from: Option<DateTime<FixedOffset>>,
}

impl GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn consumed_at(mut self, value: impl Into<String>) -> Self {
        self.consumed_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn valid_from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.valid_from = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem, BuildError> {
        Ok(
            GetLoyaltyOfferProgramsPidVouchersResponseContactRewardsItem {
                code: self.code,
                consumed_at: self.consumed_at,
                created_at: self.created_at,
                expiration_date: self.expiration_date,
                id: self.id,
                meta: self.meta,
                reward_id: self.reward_id,
                unit: self.unit,
                updated_at: self.updated_at,
                value: self.value,
                valid_from: self.valid_from,
            },
        )
    }
}

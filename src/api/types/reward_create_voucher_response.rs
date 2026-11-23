pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateVoucherResponse {
    /// Generated code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Timestamp for when this reward was consumed
    #[serde(rename = "consumedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_at: Option<String>,
    /// Id of the contact that attributed the reward
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Timestamp for when this reward was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Expiration date of the reward
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiration_date: Option<DateTime<FixedOffset>>,
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Loyalty Program Id to which attributed reward belongs
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Additional data to define the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Unique identifier for the reward
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
    /// Timestamp for when this reward was updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
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

impl CreateVoucherResponse {
    pub fn builder() -> CreateVoucherResponseBuilder {
        <CreateVoucherResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVoucherResponseBuilder {
    code: Option<String>,
    consumed_at: Option<String>,
    contact_id: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    expiration_date: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    loyalty_program_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    reward_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    value: Option<f64>,
    valid_from: Option<DateTime<FixedOffset>>,
}

impl CreateVoucherResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn consumed_at(mut self, value: impl Into<String>) -> Self {
        self.consumed_at = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
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

    /// Consumes the builder and constructs a [`CreateVoucherResponse`].
    pub fn build(self) -> Result<CreateVoucherResponse, BuildError> {
        Ok(CreateVoucherResponse {
            code: self.code,
            consumed_at: self.consumed_at,
            contact_id: self.contact_id,
            created_at: self.created_at,
            expiration_date: self.expiration_date,
            id: self.id,
            loyalty_program_id: self.loyalty_program_id,
            meta: self.meta,
            reward_id: self.reward_id,
            updated_at: self.updated_at,
            value: self.value,
            valid_from: self.valid_from,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Redeem {
    /// Timestamp when the redemption was cancelled
    #[serde(rename = "cancelledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub cancelled_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the redemption was completed
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the contact
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Timestamp when the redemption was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the debit transaction
    #[serde(rename = "debitTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_transaction_id: Option<String>,
    /// Timestamp when the redemption expires
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the redemption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique identifier for the loyalty program
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Additional metadata associated with the redemption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Reason for rejection if the redemption was rejected
    #[serde(rename = "rejectReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// Timestamp when the redemption was rejected
    #[serde(rename = "rejectedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub rejected_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the reward attribution
    #[serde(rename = "rewardAttributionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_attribution_id: Option<String>,
    /// Current status of the redemption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Timestamp when the redemption was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl Redeem {
    pub fn builder() -> RedeemBuilder {
        <RedeemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedeemBuilder {
    cancelled_at: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
    contact_id: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    debit_transaction_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    loyalty_program_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    reject_reason: Option<String>,
    rejected_at: Option<DateTime<FixedOffset>>,
    reward_attribution_id: Option<String>,
    status: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl RedeemBuilder {
    pub fn cancelled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.cancelled_at = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
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

    pub fn debit_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.debit_transaction_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
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

    pub fn reject_reason(mut self, value: impl Into<String>) -> Self {
        self.reject_reason = Some(value.into());
        self
    }

    pub fn rejected_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.rejected_at = Some(value);
        self
    }

    pub fn reward_attribution_id(mut self, value: impl Into<String>) -> Self {
        self.reward_attribution_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Redeem`].
    pub fn build(self) -> Result<Redeem, BuildError> {
        Ok(Redeem {
            cancelled_at: self.cancelled_at,
            completed_at: self.completed_at,
            contact_id: self.contact_id,
            created_at: self.created_at,
            debit_transaction_id: self.debit_transaction_id,
            expires_at: self.expires_at,
            id: self.id,
            loyalty_program_id: self.loyalty_program_id,
            meta: self.meta,
            reject_reason: self.reject_reason,
            rejected_at: self.rejected_at,
            reward_attribution_id: self.reward_attribution_id,
            status: self.status,
            updated_at: self.updated_at,
        })
    }
}

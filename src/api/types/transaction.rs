pub use crate::prelude::*;

/// Represents a transaction involving a balance, including status and timestamps.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Transaction {
    /// Unique identifier (UUID) of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// The contact's total balance for this balance definition after the transaction was applied. Only returned when the transaction actually updated the balance (i.e. it was completed or auto-completed).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub balance: Option<f64>,
    /// The type of the transaction.
    #[serde(rename = "transactionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<TransactionTransactionType>,
    /// Optional metadata associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// The current status of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatus>,
    /// Unique identifier (UUID) of the associated loyalty program.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Unique identifier (UUID) of the associated balance definition.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Unique identifier of the contact associated with the transaction.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Optional timestamp indicating when the transaction event occurred (ISO 8601 format).
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub event_time: Option<DateTime<FixedOffset>>,
    /// Reason for rejection if the transaction was declined (nullable).
    #[serde(rename = "rejectReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// Timestamp when the transaction was rejected (nullable).
    #[serde(rename = "rejectedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub rejected_at: Option<DateTime<FixedOffset>>,
    /// Expiry date of the transaction (nullable).
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiration_date: Option<DateTime<FixedOffset>>,
    /// Timestamp when the transaction was completed (nullable).
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub completed_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the transaction was canceled (nullable).
    #[serde(rename = "cancelledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub cancelled_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the transaction was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the transaction was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl Transaction {
    pub fn builder() -> TransactionBuilder {
        <TransactionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionBuilder {
    id: Option<String>,
    amount: Option<f64>,
    balance: Option<f64>,
    transaction_type: Option<TransactionTransactionType>,
    meta: Option<HashMap<String, serde_json::Value>>,
    status: Option<TransactionStatus>,
    loyalty_program_id: Option<String>,
    balance_definition_id: Option<String>,
    contact_id: Option<i64>,
    event_time: Option<DateTime<FixedOffset>>,
    reject_reason: Option<String>,
    rejected_at: Option<DateTime<FixedOffset>>,
    expiration_date: Option<DateTime<FixedOffset>>,
    completed_at: Option<DateTime<FixedOffset>>,
    cancelled_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl TransactionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn transaction_type(mut self, value: TransactionTransactionType) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn status(mut self, value: TransactionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn event_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_time = Some(value);
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

    pub fn expiration_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn completed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn cancelled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.cancelled_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Transaction`].
    pub fn build(self) -> Result<Transaction, BuildError> {
        Ok(Transaction {
            id: self.id,
            amount: self.amount,
            balance: self.balance,
            transaction_type: self.transaction_type,
            meta: self.meta,
            status: self.status,
            loyalty_program_id: self.loyalty_program_id,
            balance_definition_id: self.balance_definition_id,
            contact_id: self.contact_id,
            event_time: self.event_time,
            reject_reason: self.reject_reason,
            rejected_at: self.rejected_at,
            expiration_date: self.expiration_date,
            completed_at: self.completed_at,
            cancelled_at: self.cancelled_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

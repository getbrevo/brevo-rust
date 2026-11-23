pub use crate::prelude::*;

/// Represents a record of a past transaction, including status and key timestamps.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem {
    /// The transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// The type of the transaction.
    #[serde(rename = "transactionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemTransactionType,
    >,
    /// Expiration date of the balance associated with this transaction.
    #[serde(rename = "balanceExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_expiration_date: Option<String>,
    /// Timestamp when the transaction was canceled, if applicable.
    #[serde(rename = "cancelledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
    /// Timestamp when the transaction was successfully completed.
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Timestamp when the transaction was initiated.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Unique identifier of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional metadata associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Reason for rejection, if the transaction was declined.
    #[serde(rename = "rejectReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// Timestamp when the transaction was rejected.
    #[serde(rename = "rejectedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_at: Option<String>,
    /// Current status of the transaction (e.g., pending, completed, rejected).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem {
    pub fn builder(
    ) -> GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemBuilder {
        <GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemBuilder {
    amount: Option<f64>,
    transaction_type: Option<
        GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemTransactionType,
    >,
    balance_expiration_date: Option<String>,
    cancelled_at: Option<String>,
    completed_at: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    reject_reason: Option<String>,
    rejected_at: Option<String>,
    status: Option<String>,
}

impl GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn transaction_type(
        mut self,
        value: GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItemTransactionType,
    ) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn balance_expiration_date(mut self, value: impl Into<String>) -> Self {
        self.balance_expiration_date = Some(value.into());
        self
    }

    pub fn cancelled_at(mut self, value: impl Into<String>) -> Self {
        self.cancelled_at = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn reject_reason(mut self, value: impl Into<String>) -> Self {
        self.reject_reason = Some(value.into());
        self
    }

    pub fn rejected_at(mut self, value: impl Into<String>) -> Self {
        self.rejected_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem`].
    pub fn build(
        self,
    ) -> Result<
        GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem,
        BuildError,
    > {
        Ok(
            GetLoyaltyBalanceProgramsPidTransactionHistoryResponseTransactionHistoryItem {
                amount: self.amount,
                transaction_type: self.transaction_type,
                balance_expiration_date: self.balance_expiration_date,
                cancelled_at: self.cancelled_at,
                completed_at: self.completed_at,
                created_at: self.created_at,
                id: self.id,
                meta: self.meta,
                reject_reason: self.reject_reason,
                rejected_at: self.rejected_at,
                status: self.status,
            },
        )
    }
}

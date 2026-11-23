pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BeginTransactionRequest {
    /// Transaction amount. A positive value creates a credit transaction and a negative value creates a debit transaction (unless transactionType is explicitly provided).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Explicit transaction type. If not provided, the type is inferred from the sign of the amount (positive = credit, negative = debit).
    #[serde(rename = "transactionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<BeginTransactionRequestTransactionType>,
    /// Unique identifier (UUID) of the associated balance definition.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(default)]
    pub balance_definition_id: String,
    /// Unique identifier of the contact involved in the transaction. Required unless `LoyaltySubscriptionId` is provided.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    /// Unique identifier for the loyalty subscription. Required unless `contactId` is provided.
    #[serde(rename = "LoyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
    /// Optional metadata associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Time-to-live for the transaction in seconds. Must be at least 10 seconds if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Timestamp specifying when the transaction event occurred (ISO 8601 / RFC 3339 format).
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub event_time: Option<DateTime<FixedOffset>>,
    /// Whether the transaction should be automatically completed.
    #[serde(rename = "autoComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_complete: Option<bool>,
    /// Expiry time for the balance in minutes. Must be greater than 0 if provided. Only applicable when autoComplete is true.
    #[serde(rename = "balanceExpiryInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_expiry_in_minutes: Option<i64>,
}

impl BeginTransactionRequest {
    pub fn builder() -> BeginTransactionRequestBuilder {
        <BeginTransactionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BeginTransactionRequestBuilder {
    amount: Option<f64>,
    transaction_type: Option<BeginTransactionRequestTransactionType>,
    balance_definition_id: Option<String>,
    contact_id: Option<i64>,
    loyalty_subscription_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    ttl: Option<i64>,
    event_time: Option<DateTime<FixedOffset>>,
    auto_complete: Option<bool>,
    balance_expiry_in_minutes: Option<i64>,
}

impl BeginTransactionRequestBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn transaction_type(mut self, value: BeginTransactionRequestTransactionType) -> Self {
        self.transaction_type = Some(value);
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

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }

    pub fn event_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_time = Some(value);
        self
    }

    pub fn auto_complete(mut self, value: bool) -> Self {
        self.auto_complete = Some(value);
        self
    }

    pub fn balance_expiry_in_minutes(mut self, value: i64) -> Self {
        self.balance_expiry_in_minutes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BeginTransactionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](BeginTransactionRequestBuilder::amount)
    /// - [`balance_definition_id`](BeginTransactionRequestBuilder::balance_definition_id)
    pub fn build(self) -> Result<BeginTransactionRequest, BuildError> {
        Ok(BeginTransactionRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            transaction_type: self.transaction_type,
            balance_definition_id: self
                .balance_definition_id
                .ok_or_else(|| BuildError::missing_field("balance_definition_id"))?,
            contact_id: self.contact_id,
            loyalty_subscription_id: self.loyalty_subscription_id,
            meta: self.meta,
            ttl: self.ttl,
            event_time: self.event_time,
            auto_complete: self.auto_complete,
            balance_expiry_in_minutes: self.balance_expiry_in_minutes,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBalanceOrderRequest {
    /// Order amount (must be non-zero).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Unique identifier (UUID) of the associated balance definition.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(default)]
    pub balance_definition_id: String,
    /// Unique identifier of the contact placing the order (must be ≥ 1).
    #[serde(rename = "contactId")]
    #[serde(default)]
    pub contact_id: i64,
    /// RFC3339 timestamp specifying when the order is due.
    #[serde(rename = "dueAt")]
    #[serde(default)]
    pub due_at: String,
    /// Optional RFC3339 timestamp defining order expiration.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Optional metadata associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Specifies the origin of the order.
    pub source: CreateBalanceOrderRequestSource,
}

impl CreateBalanceOrderRequest {
    pub fn builder() -> CreateBalanceOrderRequestBuilder {
        <CreateBalanceOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBalanceOrderRequestBuilder {
    amount: Option<f64>,
    balance_definition_id: Option<String>,
    contact_id: Option<i64>,
    due_at: Option<String>,
    expires_at: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    source: Option<CreateBalanceOrderRequestSource>,
}

impl CreateBalanceOrderRequestBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
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

    pub fn due_at(mut self, value: impl Into<String>) -> Self {
        self.due_at = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn source(mut self, value: CreateBalanceOrderRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBalanceOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateBalanceOrderRequestBuilder::amount)
    /// - [`balance_definition_id`](CreateBalanceOrderRequestBuilder::balance_definition_id)
    /// - [`contact_id`](CreateBalanceOrderRequestBuilder::contact_id)
    /// - [`due_at`](CreateBalanceOrderRequestBuilder::due_at)
    /// - [`source`](CreateBalanceOrderRequestBuilder::source)
    pub fn build(self) -> Result<CreateBalanceOrderRequest, BuildError> {
        Ok(CreateBalanceOrderRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            balance_definition_id: self
                .balance_definition_id
                .ok_or_else(|| BuildError::missing_field("balance_definition_id"))?,
            contact_id: self
                .contact_id
                .ok_or_else(|| BuildError::missing_field("contact_id"))?,
            due_at: self
                .due_at
                .ok_or_else(|| BuildError::missing_field("due_at"))?,
            expires_at: self.expires_at,
            meta: self.meta,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

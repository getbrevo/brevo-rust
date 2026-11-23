pub use crate::prelude::*;

/// Represents an order linked to a balance definition, including amount, due date, and transaction details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateBalanceOrderResponse {
    /// Order amount (must not be zero).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// Optional unique identifier (UUID) of the associated balance definition.
    #[serde(rename = "balanceDefinitionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_definition_id: Option<String>,
    /// Unique identifier of the contact placing the order (must be ≥ 1).
    #[serde(rename = "contactId")]
    #[serde(default)]
    pub contact_id: i64,
    /// RFC3339 timestamp indicating when the order was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// RFC3339 timestamp specifying when the order is due in the future.
    #[serde(rename = "dueAt")]
    #[serde(default)]
    pub due_at: String,
    /// Optional RFC3339 timestamp defining order expiration in the future.
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Unique identifier for the balance order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unique identifier of the loyalty program associated with the order.
    #[serde(rename = "loyaltyProgramId")]
    #[serde(default)]
    pub loyalty_program_id: String,
    /// Optional metadata associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Optional RFC3339 timestamp indicating when the order was processed.
    #[serde(rename = "processedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    /// Optional reference to the associated transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionid: Option<String>,
    /// RFC3339 timestamp indicating the last update to the order.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl CreateBalanceOrderResponse {
    pub fn builder() -> CreateBalanceOrderResponseBuilder {
        <CreateBalanceOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBalanceOrderResponseBuilder {
    amount: Option<f64>,
    balance_definition_id: Option<String>,
    contact_id: Option<i64>,
    created_at: Option<String>,
    due_at: Option<String>,
    expires_at: Option<String>,
    id: Option<String>,
    loyalty_program_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    processed_at: Option<String>,
    transactionid: Option<String>,
    updated_at: Option<String>,
}

impl CreateBalanceOrderResponseBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn processed_at(mut self, value: impl Into<String>) -> Self {
        self.processed_at = Some(value.into());
        self
    }

    pub fn transactionid(mut self, value: impl Into<String>) -> Self {
        self.transactionid = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBalanceOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateBalanceOrderResponseBuilder::amount)
    /// - [`contact_id`](CreateBalanceOrderResponseBuilder::contact_id)
    /// - [`created_at`](CreateBalanceOrderResponseBuilder::created_at)
    /// - [`due_at`](CreateBalanceOrderResponseBuilder::due_at)
    /// - [`loyalty_program_id`](CreateBalanceOrderResponseBuilder::loyalty_program_id)
    /// - [`updated_at`](CreateBalanceOrderResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreateBalanceOrderResponse, BuildError> {
        Ok(CreateBalanceOrderResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            balance_definition_id: self.balance_definition_id,
            contact_id: self
                .contact_id
                .ok_or_else(|| BuildError::missing_field("contact_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            due_at: self
                .due_at
                .ok_or_else(|| BuildError::missing_field("due_at"))?,
            expires_at: self.expires_at,
            id: self.id,
            loyalty_program_id: self
                .loyalty_program_id
                .ok_or_else(|| BuildError::missing_field("loyalty_program_id"))?,
            meta: self.meta,
            processed_at: self.processed_at,
            transactionid: self.transactionid,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

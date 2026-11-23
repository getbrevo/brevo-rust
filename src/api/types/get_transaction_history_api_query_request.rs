pub use crate::prelude::*;

/// Query parameters for getTransactionHistoryApi
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTransactionHistoryApiQueryRequest {
    /// Limit the number of records returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number to retrieve
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Field to sort by
    #[serde(rename = "sortField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestSort>,
    /// Contact ID
    #[serde(rename = "contactId")]
    #[serde(default)]
    pub contact_id: i64,
    /// Balance Definition ID
    #[serde(rename = "balanceDefinitionId")]
    #[serde(default)]
    pub balance_definition_id: String,
    /// Transaction status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestStatus>,
    /// Transaction type filter
    #[serde(rename = "transactionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestTransactionType>,
    /// Loyalty Subscription ID filter
    #[serde(rename = "loyaltySubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_subscription_id: Option<String>,
}

impl GetTransactionHistoryApiQueryRequest {
    pub fn builder() -> GetTransactionHistoryApiQueryRequestBuilder {
        <GetTransactionHistoryApiQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTransactionHistoryApiQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort_field: Option<String>,
    sort: Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestSort>,
    contact_id: Option<i64>,
    balance_definition_id: Option<String>,
    status: Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestStatus>,
    transaction_type: Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestTransactionType>,
    loyalty_subscription_id: Option<String>,
}

impl GetTransactionHistoryApiQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort_field(mut self, value: impl Into<String>) -> Self {
        self.sort_field = Some(value.into());
        self
    }

    pub fn sort(
        mut self,
        value: GetLoyaltyBalanceProgramsPidTransactionHistoryRequestSort,
    ) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: GetLoyaltyBalanceProgramsPidTransactionHistoryRequestStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction_type(
        mut self,
        value: GetLoyaltyBalanceProgramsPidTransactionHistoryRequestTransactionType,
    ) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn loyalty_subscription_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTransactionHistoryApiQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_id`](GetTransactionHistoryApiQueryRequestBuilder::contact_id)
    /// - [`balance_definition_id`](GetTransactionHistoryApiQueryRequestBuilder::balance_definition_id)
    pub fn build(self) -> Result<GetTransactionHistoryApiQueryRequest, BuildError> {
        Ok(GetTransactionHistoryApiQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort_field: self.sort_field,
            sort: self.sort,
            contact_id: self
                .contact_id
                .ok_or_else(|| BuildError::missing_field("contact_id"))?,
            balance_definition_id: self
                .balance_definition_id
                .ok_or_else(|| BuildError::missing_field("balance_definition_id"))?,
            status: self.status,
            transaction_type: self.transaction_type,
            loyalty_subscription_id: self.loyalty_subscription_id,
        })
    }
}

pub use crate::prelude::*;

/// Query parameters for getActiveBalancesApi
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetActiveBalancesApiQueryRequest {
    /// Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort Field
    #[serde(rename = "sortField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// Sort Order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetLoyaltyBalanceProgramsPidActiveBalanceRequestSort>,
    /// Contact ID
    #[serde(rename = "contactId")]
    #[serde(default)]
    pub contact_id: i64,
    /// Balance Definition ID
    #[serde(rename = "balanceDefinitionId")]
    #[serde(default)]
    pub balance_definition_id: String,
    /// Include balances tied to internal definitions.
    #[serde(rename = "includeInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_internal: Option<bool>,
}

impl GetActiveBalancesApiQueryRequest {
    pub fn builder() -> GetActiveBalancesApiQueryRequestBuilder {
        <GetActiveBalancesApiQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetActiveBalancesApiQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort_field: Option<String>,
    sort: Option<GetLoyaltyBalanceProgramsPidActiveBalanceRequestSort>,
    contact_id: Option<i64>,
    balance_definition_id: Option<String>,
    include_internal: Option<bool>,
}

impl GetActiveBalancesApiQueryRequestBuilder {
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

    pub fn sort(mut self, value: GetLoyaltyBalanceProgramsPidActiveBalanceRequestSort) -> Self {
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

    pub fn include_internal(mut self, value: bool) -> Self {
        self.include_internal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetActiveBalancesApiQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_id`](GetActiveBalancesApiQueryRequestBuilder::contact_id)
    /// - [`balance_definition_id`](GetActiveBalancesApiQueryRequestBuilder::balance_definition_id)
    pub fn build(self) -> Result<GetActiveBalancesApiQueryRequest, BuildError> {
        Ok(GetActiveBalancesApiQueryRequest {
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
            include_internal: self.include_internal,
        })
    }
}

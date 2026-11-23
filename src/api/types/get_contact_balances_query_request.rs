pub use crate::prelude::*;

/// Query parameters for getContactBalances
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactBalancesQueryRequest {
    /// Include balances tied to internal definitions.
    #[serde(rename = "includeInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_internal: Option<bool>,
    /// Limit the number of records returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Skip a number of records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetContactBalancesRequestSort>,
    /// Field to sort by
    #[serde(rename = "sortField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<GetContactBalancesRequestSortField>,
    /// Balance Definition ID (required)
    #[serde(rename = "balanceDefinitionId")]
    #[serde(default)]
    pub balance_definition_id: String,
}

impl GetContactBalancesQueryRequest {
    pub fn builder() -> GetContactBalancesQueryRequestBuilder {
        <GetContactBalancesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactBalancesQueryRequestBuilder {
    include_internal: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetContactBalancesRequestSort>,
    sort_field: Option<GetContactBalancesRequestSortField>,
    balance_definition_id: Option<String>,
}

impl GetContactBalancesQueryRequestBuilder {
    pub fn include_internal(mut self, value: bool) -> Self {
        self.include_internal = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetContactBalancesRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_field(mut self, value: GetContactBalancesRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn balance_definition_id(mut self, value: impl Into<String>) -> Self {
        self.balance_definition_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactBalancesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance_definition_id`](GetContactBalancesQueryRequestBuilder::balance_definition_id)
    pub fn build(self) -> Result<GetContactBalancesQueryRequest, BuildError> {
        Ok(GetContactBalancesQueryRequest {
            include_internal: self.include_internal,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            sort_field: self.sort_field,
            balance_definition_id: self
                .balance_definition_id
                .ok_or_else(|| BuildError::missing_field("balance_definition_id"))?,
        })
    }
}

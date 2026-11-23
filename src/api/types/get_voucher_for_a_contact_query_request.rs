pub use crate::prelude::*;

/// Query parameters for getVoucherForAContact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetVoucherForAContactQueryRequest {
    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Pagination offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetLoyaltyOfferProgramsPidVouchersRequestSort>,
    /// Sort field
    #[serde(rename = "sortField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<GetLoyaltyOfferProgramsPidVouchersRequestSortField>,
    /// Contact ID
    #[serde(rename = "contactId")]
    #[serde(default)]
    pub contact_id: i64,
    /// Metadata value for a Key filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_key_value: Option<String>,
    /// Reward ID
    #[serde(rename = "rewardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_id: Option<String>,
}

impl GetVoucherForAContactQueryRequest {
    pub fn builder() -> GetVoucherForAContactQueryRequestBuilder {
        <GetVoucherForAContactQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetVoucherForAContactQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<GetLoyaltyOfferProgramsPidVouchersRequestSort>,
    sort_field: Option<GetLoyaltyOfferProgramsPidVouchersRequestSortField>,
    contact_id: Option<i64>,
    metadata_key_value: Option<String>,
    reward_id: Option<String>,
}

impl GetVoucherForAContactQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: GetLoyaltyOfferProgramsPidVouchersRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_field(mut self, value: GetLoyaltyOfferProgramsPidVouchersRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn metadata_key_value(mut self, value: impl Into<String>) -> Self {
        self.metadata_key_value = Some(value.into());
        self
    }

    pub fn reward_id(mut self, value: impl Into<String>) -> Self {
        self.reward_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetVoucherForAContactQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_id`](GetVoucherForAContactQueryRequestBuilder::contact_id)
    pub fn build(self) -> Result<GetVoucherForAContactQueryRequest, BuildError> {
        Ok(GetVoucherForAContactQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            sort_field: self.sort_field,
            contact_id: self
                .contact_id
                .ok_or_else(|| BuildError::missing_field("contact_id"))?,
            metadata_key_value: self.metadata_key_value,
            reward_id: self.reward_id,
        })
    }
}

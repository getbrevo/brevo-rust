pub use crate::prelude::*;

/// Query parameters for getAllDeals
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllDealsQueryRequest {
    /// Filter by attributes. If you have a filter for the owner on your end, please send it as filters[attributes.deal_owner] and utilize the account email for the filtering.
    #[serde(rename = "filters[attributes.deal_name]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_attributes_deal_name: Option<String>,
    /// Filter by the deal owner. Pass the account email address of the deal owner.
    #[serde(rename = "filters[attributes.deal_owner]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_attributes_deal_owner: Option<String>,
    /// Filter by the deal stage. Pass the stage id, retrievable from GET /crm/pipeline/details/{pipelineID}.
    #[serde(rename = "filters[attributes.deal_stage]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_attributes_deal_stage: Option<String>,
    /// Filter by the pipeline. Pass the pipeline id, retrievable from GET /crm/pipeline/details/{pipelineID}.
    #[serde(rename = "filters[attributes.pipeline]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_attributes_pipeline: Option<String>,
    /// Filter by linked companies ids
    #[serde(rename = "filters[linkedCompaniesIds]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_linked_companies_ids: Option<String>,
    /// Filter by linked contacts ids
    #[serde(rename = "filters[linkedContactsIds]")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_linked_contacts_ids: Option<String>,
    /// Filter (urlencoded) the deals modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since: Option<String>,
    /// Filter (urlencoded) the deals created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    #[serde(rename = "createdSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_since: Option<String>,
    /// Index of the first document of the page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Number of documents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<GetCrmDealsRequestSort>,
    /// The field used to sort field names.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl GetAllDealsQueryRequest {
    pub fn builder() -> GetAllDealsQueryRequestBuilder {
        <GetAllDealsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllDealsQueryRequestBuilder {
    filters_attributes_deal_name: Option<String>,
    filters_attributes_deal_owner: Option<String>,
    filters_attributes_deal_stage: Option<String>,
    filters_attributes_pipeline: Option<String>,
    filters_linked_companies_ids: Option<String>,
    filters_linked_contacts_ids: Option<String>,
    modified_since: Option<String>,
    created_since: Option<String>,
    offset: Option<i64>,
    limit: Option<i64>,
    sort: Option<GetCrmDealsRequestSort>,
    sort_by: Option<String>,
}

impl GetAllDealsQueryRequestBuilder {
    pub fn filters_attributes_deal_name(mut self, value: impl Into<String>) -> Self {
        self.filters_attributes_deal_name = Some(value.into());
        self
    }

    pub fn filters_attributes_deal_owner(mut self, value: impl Into<String>) -> Self {
        self.filters_attributes_deal_owner = Some(value.into());
        self
    }

    pub fn filters_attributes_deal_stage(mut self, value: impl Into<String>) -> Self {
        self.filters_attributes_deal_stage = Some(value.into());
        self
    }

    pub fn filters_attributes_pipeline(mut self, value: impl Into<String>) -> Self {
        self.filters_attributes_pipeline = Some(value.into());
        self
    }

    pub fn filters_linked_companies_ids(mut self, value: impl Into<String>) -> Self {
        self.filters_linked_companies_ids = Some(value.into());
        self
    }

    pub fn filters_linked_contacts_ids(mut self, value: impl Into<String>) -> Self {
        self.filters_linked_contacts_ids = Some(value.into());
        self
    }

    pub fn modified_since(mut self, value: impl Into<String>) -> Self {
        self.modified_since = Some(value.into());
        self
    }

    pub fn created_since(mut self, value: impl Into<String>) -> Self {
        self.created_since = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn sort(mut self, value: GetCrmDealsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAllDealsQueryRequest`].
    pub fn build(self) -> Result<GetAllDealsQueryRequest, BuildError> {
        Ok(GetAllDealsQueryRequest {
            filters_attributes_deal_name: self.filters_attributes_deal_name,
            filters_attributes_deal_owner: self.filters_attributes_deal_owner,
            filters_attributes_deal_stage: self.filters_attributes_deal_stage,
            filters_attributes_pipeline: self.filters_attributes_pipeline,
            filters_linked_companies_ids: self.filters_linked_companies_ids,
            filters_linked_contacts_ids: self.filters_linked_contacts_ids,
            modified_since: self.modified_since,
            created_since: self.created_since,
            offset: self.offset,
            limit: self.limit,
            sort: self.sort,
            sort_by: self.sort_by,
        })
    }
}

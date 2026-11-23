pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostCrmDealsRequest {
    /// Attributes for deal creation To assign owner of a Deal you can send attributes.deal_owner and utilize the account email or ID. If you want to create a deal on a specific pipeline and stage you can use the following attributes `pipeline` and `deal_stage`. Pipeline and deal_stage are ids you can fetch using this endpoint `/crm/pipeline/details/{pipelineID}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Company ids to be linked with deal
    #[serde(rename = "linkedCompaniesIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_companies_ids: Option<Vec<String>>,
    /// Contact ids to be linked with deal
    #[serde(rename = "linkedContactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Name of deal
    #[serde(default)]
    pub name: String,
}

impl PostCrmDealsRequest {
    pub fn builder() -> PostCrmDealsRequestBuilder {
        <PostCrmDealsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmDealsRequestBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    linked_companies_ids: Option<Vec<String>>,
    linked_contacts_ids: Option<Vec<i64>>,
    name: Option<String>,
}

impl PostCrmDealsRequestBuilder {
    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn linked_companies_ids(mut self, value: Vec<String>) -> Self {
        self.linked_companies_ids = Some(value);
        self
    }

    pub fn linked_contacts_ids(mut self, value: Vec<i64>) -> Self {
        self.linked_contacts_ids = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCrmDealsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostCrmDealsRequestBuilder::name)
    pub fn build(self) -> Result<PostCrmDealsRequest, BuildError> {
        Ok(PostCrmDealsRequest {
            attributes: self.attributes,
            linked_companies_ids: self.linked_companies_ids,
            linked_contacts_ids: self.linked_contacts_ids,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

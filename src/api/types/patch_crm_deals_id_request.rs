pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchCrmDealsIdRequest {
    /// Attributes for deal update To assign owner of a Deal you can send attributes.deal_owner and utilize the account email or ID. If you wish to update the pipeline of a deal you need to provide the `pipeline` and the `deal_stage` Pipeline and deal_stage are ids you can fetch using this endpoint `/crm/pipeline/details/{pipelineID}`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Warning - Using PATCH on linkedCompaniesIds replaces the list of linked companies. Omitted IDs will be removed.
    #[serde(rename = "linkedCompaniesIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_companies_ids: Option<Vec<String>>,
    /// Warning - Using PATCH on linkedContactIds replaces the list of linked contacts. Omitted IDs will be removed.
    #[serde(rename = "linkedContactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Name of deal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchCrmDealsIdRequest {
    pub fn builder() -> PatchCrmDealsIdRequestBuilder {
        <PatchCrmDealsIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchCrmDealsIdRequestBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    linked_companies_ids: Option<Vec<String>>,
    linked_contacts_ids: Option<Vec<i64>>,
    name: Option<String>,
}

impl PatchCrmDealsIdRequestBuilder {
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

    /// Consumes the builder and constructs a [`PatchCrmDealsIdRequest`].
    pub fn build(self) -> Result<PatchCrmDealsIdRequest, BuildError> {
        Ok(PatchCrmDealsIdRequest {
            attributes: self.attributes,
            linked_companies_ids: self.linked_companies_ids,
            linked_contacts_ids: self.linked_contacts_ids,
            name: self.name,
        })
    }
}

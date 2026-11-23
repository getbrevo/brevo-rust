pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchCompaniesIdRequest {
    /// Attributes for company update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Country code if phone_number is passed in attributes.
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<i64>,
    /// Warning - Using PATCH on linkedContactIds replaces the list of linked contacts. Omitted IDs will be removed.
    #[serde(rename = "linkedContactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Warning - Using PATCH on linkedDealsIds replaces the list of linked deals. Omitted IDs will be removed.
    #[serde(rename = "linkedDealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_deals_ids: Option<Vec<String>>,
    /// Name of company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchCompaniesIdRequest {
    pub fn builder() -> PatchCompaniesIdRequestBuilder {
        <PatchCompaniesIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchCompaniesIdRequestBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    country_code: Option<i64>,
    linked_contacts_ids: Option<Vec<i64>>,
    linked_deals_ids: Option<Vec<String>>,
    name: Option<String>,
}

impl PatchCompaniesIdRequestBuilder {
    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn country_code(mut self, value: i64) -> Self {
        self.country_code = Some(value);
        self
    }

    pub fn linked_contacts_ids(mut self, value: Vec<i64>) -> Self {
        self.linked_contacts_ids = Some(value);
        self
    }

    pub fn linked_deals_ids(mut self, value: Vec<String>) -> Self {
        self.linked_deals_ids = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchCompaniesIdRequest`].
    pub fn build(self) -> Result<PatchCompaniesIdRequest, BuildError> {
        Ok(PatchCompaniesIdRequest {
            attributes: self.attributes,
            country_code: self.country_code,
            linked_contacts_ids: self.linked_contacts_ids,
            linked_deals_ids: self.linked_deals_ids,
            name: self.name,
        })
    }
}

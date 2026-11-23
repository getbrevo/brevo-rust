pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostCompaniesRequest {
    /// Attributes for company creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Country code if phone_number is passed in attributes.
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<i64>,
    /// Contact ids to be linked with company
    #[serde(rename = "linkedContactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Deal ids to be linked with company
    #[serde(rename = "linkedDealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_deals_ids: Option<Vec<String>>,
    /// Name of company
    #[serde(default)]
    pub name: String,
}

impl PostCompaniesRequest {
    pub fn builder() -> PostCompaniesRequestBuilder {
        <PostCompaniesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCompaniesRequestBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    country_code: Option<i64>,
    linked_contacts_ids: Option<Vec<i64>>,
    linked_deals_ids: Option<Vec<String>>,
    name: Option<String>,
}

impl PostCompaniesRequestBuilder {
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

    /// Consumes the builder and constructs a [`PostCompaniesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostCompaniesRequestBuilder::name)
    pub fn build(self) -> Result<PostCompaniesRequest, BuildError> {
        Ok(PostCompaniesRequest {
            attributes: self.attributes,
            country_code: self.country_code,
            linked_contacts_ids: self.linked_contacts_ids,
            linked_deals_ids: self.linked_deals_ids,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

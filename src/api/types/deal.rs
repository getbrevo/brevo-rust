pub use crate::prelude::*;

/// Deal Details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Deal {
    /// Unique deal id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Deal attributes with values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Contact ids for contacts linked to this deal
    #[serde(rename = "linkedContactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Companies ids for companies linked to this deal
    #[serde(rename = "linkedCompaniesIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_companies_ids: Option<Vec<String>>,
}

impl Deal {
    pub fn builder() -> DealBuilder {
        <DealBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DealBuilder {
    id: Option<String>,
    attributes: Option<HashMap<String, serde_json::Value>>,
    linked_contacts_ids: Option<Vec<i64>>,
    linked_companies_ids: Option<Vec<String>>,
}

impl DealBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn linked_contacts_ids(mut self, value: Vec<i64>) -> Self {
        self.linked_contacts_ids = Some(value);
        self
    }

    pub fn linked_companies_ids(mut self, value: Vec<String>) -> Self {
        self.linked_companies_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Deal`].
    pub fn build(self) -> Result<Deal, BuildError> {
        Ok(Deal {
            id: self.id,
            attributes: self.attributes,
            linked_contacts_ids: self.linked_contacts_ids,
            linked_companies_ids: self.linked_companies_ids,
        })
    }
}

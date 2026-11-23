pub use crate::prelude::*;

/// Company Details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Company {
    /// Company attributes with values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Unique company id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Contact ids for contacts linked to this company
    #[serde(rename = "linkedContactsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_contacts_ids: Option<Vec<i64>>,
    /// Deal ids for deals linked to this company
    #[serde(rename = "linkedDealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_deals_ids: Option<Vec<String>>,
}

impl Company {
    pub fn builder() -> CompanyBuilder {
        <CompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    linked_contacts_ids: Option<Vec<i64>>,
    linked_deals_ids: Option<Vec<String>>,
}

impl CompanyBuilder {
    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`Company`].
    pub fn build(self) -> Result<Company, BuildError> {
        Ok(Company {
            attributes: self.attributes,
            id: self.id,
            linked_contacts_ids: self.linked_contacts_ids,
            linked_deals_ids: self.linked_deals_ids,
        })
    }
}

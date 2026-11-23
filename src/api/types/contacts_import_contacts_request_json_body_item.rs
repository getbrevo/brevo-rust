pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportContactsRequestJsonBodyItem {
    /// List of attributes to be imported
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl ImportContactsRequestJsonBodyItem {
    pub fn builder() -> ImportContactsRequestJsonBodyItemBuilder {
        <ImportContactsRequestJsonBodyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportContactsRequestJsonBodyItemBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    email: Option<String>,
}

impl ImportContactsRequestJsonBodyItemBuilder {
    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportContactsRequestJsonBodyItem`].
    pub fn build(self) -> Result<ImportContactsRequestJsonBodyItem, BuildError> {
        Ok(ImportContactsRequestJsonBodyItem {
            attributes: self.attributes,
            email: self.email,
        })
    }
}

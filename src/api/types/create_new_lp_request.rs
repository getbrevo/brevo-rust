pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateNewLpRequest {
    /// Optional description of the loyalty program (max 256 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional unique document ID.
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// Optional metadata related to the loyalty program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Required name of the loyalty program (max 128 chars).
    #[serde(default)]
    pub name: String,
}

impl CreateNewLpRequest {
    pub fn builder() -> CreateNewLpRequestBuilder {
        <CreateNewLpRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateNewLpRequestBuilder {
    description: Option<String>,
    document_id: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
}

impl CreateNewLpRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateNewLpRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateNewLpRequestBuilder::name)
    pub fn build(self) -> Result<CreateNewLpRequest, BuildError> {
        Ok(CreateNewLpRequest {
            description: self.description,
            document_id: self.document_id,
            meta: self.meta,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

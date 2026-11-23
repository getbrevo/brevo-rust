pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportDealsCreationAndUpdationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes::option")]
    pub file: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<HashMap<String, serde_json::Value>>,
}
impl ImportDealsCreationAndUpdationRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        if let Some(ref file_data) = self.file {
            form = form.part(
                "file",
                reqwest::multipart::Part::bytes(file_data.clone())
                    .file_name("file")
                    .mime_str("application/octet-stream")
                    .unwrap(),
            );
        }

        if let Some(ref value) = self.mapping {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("mapping", json_str);
            }
        }

        form
    }
}

impl ImportDealsCreationAndUpdationRequest {
    pub fn builder() -> ImportDealsCreationAndUpdationRequestBuilder {
        <ImportDealsCreationAndUpdationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportDealsCreationAndUpdationRequestBuilder {
    file: Option<Vec<u8>>,
    mapping: Option<HashMap<String, serde_json::Value>>,
}

impl ImportDealsCreationAndUpdationRequestBuilder {
    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    pub fn mapping(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.mapping = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportDealsCreationAndUpdationRequest`].
    pub fn build(self) -> Result<ImportDealsCreationAndUpdationRequest, BuildError> {
        Ok(ImportDealsCreationAndUpdationRequest {
            file: self.file,
            mapping: self.mapping,
        })
    }
}

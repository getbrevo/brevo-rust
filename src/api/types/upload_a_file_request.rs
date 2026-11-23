pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadAFileRequest {
    #[serde(rename = "companyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    #[serde(rename = "dealId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deal_id: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::base64_bytes")]
    pub file: Vec<u8>,
}
impl UploadAFileRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        if let Some(ref value) = self.company_id {
            form = form.text("companyId", value.clone());
        }

        if let Some(ref value) = self.contact_id {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("contactId", json_str);
            }
        }

        if let Some(ref value) = self.deal_id {
            form = form.text("dealId", value.clone());
        }

        form
    }
}

impl UploadAFileRequest {
    pub fn builder() -> UploadAFileRequestBuilder {
        <UploadAFileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadAFileRequestBuilder {
    company_id: Option<String>,
    contact_id: Option<i64>,
    deal_id: Option<String>,
    file: Option<Vec<u8>>,
}

impl UploadAFileRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn deal_id(mut self, value: impl Into<String>) -> Self {
        self.deal_id = Some(value.into());
        self
    }

    pub fn file(mut self, value: Vec<u8>) -> Self {
        self.file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UploadAFileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file`](UploadAFileRequestBuilder::file)
    pub fn build(self) -> Result<UploadAFileRequest, BuildError> {
        Ok(UploadAFileRequest {
            company_id: self.company_id,
            contact_id: self.contact_id,
            deal_id: self.deal_id,
            file: self.file.ok_or_else(|| BuildError::missing_field("file"))?,
        })
    }
}

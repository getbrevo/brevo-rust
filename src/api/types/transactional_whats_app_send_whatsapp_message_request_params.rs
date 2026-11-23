pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SendWhatsappMessageRequestParams {
    /// List of phone numbers of the contacts
    #[serde(rename = "contactNumbers")]
    #[serde(default)]
    pub contact_numbers: Vec<String>,
    /// Pass the set of attributes to customize the template. For example, {"FNAME":"Joe", "LNAME":"Doe"}.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    /// WhatsApp Number with country code. Example, 85264318721
    #[serde(rename = "senderNumber")]
    #[serde(default)]
    pub sender_number: String,
    /// ID of the template to send
    #[serde(rename = "templateId")]
    #[serde(default)]
    pub template_id: i64,
}

impl SendWhatsappMessageRequestParams {
    pub fn builder() -> SendWhatsappMessageRequestParamsBuilder {
        <SendWhatsappMessageRequestParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendWhatsappMessageRequestParamsBuilder {
    contact_numbers: Option<Vec<String>>,
    params: Option<HashMap<String, serde_json::Value>>,
    sender_number: Option<String>,
    template_id: Option<i64>,
}

impl SendWhatsappMessageRequestParamsBuilder {
    pub fn contact_numbers(mut self, value: Vec<String>) -> Self {
        self.contact_numbers = Some(value);
        self
    }

    pub fn params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn sender_number(mut self, value: impl Into<String>) -> Self {
        self.sender_number = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendWhatsappMessageRequestParams`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contact_numbers`](SendWhatsappMessageRequestParamsBuilder::contact_numbers)
    /// - [`sender_number`](SendWhatsappMessageRequestParamsBuilder::sender_number)
    /// - [`template_id`](SendWhatsappMessageRequestParamsBuilder::template_id)
    pub fn build(self) -> Result<SendWhatsappMessageRequestParams, BuildError> {
        Ok(SendWhatsappMessageRequestParams {
            contact_numbers: self
                .contact_numbers
                .ok_or_else(|| BuildError::missing_field("contact_numbers"))?,
            params: self.params,
            sender_number: self
                .sender_number
                .ok_or_else(|| BuildError::missing_field("sender_number"))?,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
        })
    }
}

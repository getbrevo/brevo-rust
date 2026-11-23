pub use crate::prelude::*;

/// Query parameters for getContactInfo
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoQueryRequest {
    /// email_id for Email, phone_id for SMS attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
    #[serde(rename = "identifierType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_type: Option<GetContactInfoRequestIdentifierType>,
    /// **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be lower than equal to endDate. Must not be greater than the current date.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be greater than equal to startDate. Must not be greater than the current date.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl GetContactInfoQueryRequest {
    pub fn builder() -> GetContactInfoQueryRequestBuilder {
        <GetContactInfoQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoQueryRequestBuilder {
    identifier_type: Option<GetContactInfoRequestIdentifierType>,
    start_date: Option<String>,
    end_date: Option<String>,
}

impl GetContactInfoQueryRequestBuilder {
    pub fn identifier_type(mut self, value: GetContactInfoRequestIdentifierType) -> Self {
        self.identifier_type = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetContactInfoQueryRequest`].
    pub fn build(self) -> Result<GetContactInfoQueryRequest, BuildError> {
        Ok(GetContactInfoQueryRequest {
            identifier_type: self.identifier_type,
            start_date: self.start_date,
            end_date: self.end_date,
        })
    }
}

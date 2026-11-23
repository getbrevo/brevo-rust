pub use crate::prelude::*;

/// Query parameters for deleteContact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteContactQueryRequest {
    /// email_id for Email, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
    #[serde(rename = "identifierType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_type: Option<DeleteContactRequestIdentifierType>,
}

impl DeleteContactQueryRequest {
    pub fn builder() -> DeleteContactQueryRequestBuilder {
        <DeleteContactQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteContactQueryRequestBuilder {
    identifier_type: Option<DeleteContactRequestIdentifierType>,
}

impl DeleteContactQueryRequestBuilder {
    pub fn identifier_type(mut self, value: DeleteContactRequestIdentifierType) -> Self {
        self.identifier_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteContactQueryRequest`].
    pub fn build(self) -> Result<DeleteContactQueryRequest, BuildError> {
        Ok(DeleteContactQueryRequest {
            identifier_type: self.identifier_type,
        })
    }
}

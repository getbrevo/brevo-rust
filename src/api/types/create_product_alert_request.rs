pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductAlertRequest {
    #[serde(rename = "contactIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_identifiers: Option<CreateProductAlertRequestContactIdentifiers>,
}

impl CreateProductAlertRequest {
    pub fn builder() -> CreateProductAlertRequestBuilder {
        <CreateProductAlertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductAlertRequestBuilder {
    contact_identifiers: Option<CreateProductAlertRequestContactIdentifiers>,
}

impl CreateProductAlertRequestBuilder {
    pub fn contact_identifiers(
        mut self,
        value: CreateProductAlertRequestContactIdentifiers,
    ) -> Self {
        self.contact_identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateProductAlertRequest`].
    pub fn build(self) -> Result<CreateProductAlertRequest, BuildError> {
        Ok(CreateProductAlertRequest {
            contact_identifiers: self.contact_identifiers,
        })
    }
}

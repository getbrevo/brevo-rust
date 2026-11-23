pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductAlertRequestContactIdentifiers {
    /// Email address of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Contact ID in your system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// Sms of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<String>,
}

impl CreateProductAlertRequestContactIdentifiers {
    pub fn builder() -> CreateProductAlertRequestContactIdentifiersBuilder {
        <CreateProductAlertRequestContactIdentifiersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductAlertRequestContactIdentifiersBuilder {
    email: Option<String>,
    ext_id: Option<String>,
    sms: Option<String>,
}

impl CreateProductAlertRequestContactIdentifiersBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn sms(mut self, value: impl Into<String>) -> Self {
        self.sms = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProductAlertRequestContactIdentifiers`].
    pub fn build(self) -> Result<CreateProductAlertRequestContactIdentifiers, BuildError> {
        Ok(CreateProductAlertRequestContactIdentifiers {
            email: self.email,
            ext_id: self.ext_id,
            sms: self.sms,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePaymentRequestRequest {
    pub cart: Cart,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Configuration>,
    /// Brevo ID of the contact requested to pay.
    #[serde(rename = "contactId")]
    #[serde(default)]
    pub contact_id: i64,
    /// Description of payment request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
    /// Reference of the payment request, it will appear on the payment page.
    #[serde(default)]
    pub reference: String,
}

impl CreatePaymentRequestRequest {
    pub fn builder() -> CreatePaymentRequestRequestBuilder {
        <CreatePaymentRequestRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentRequestRequestBuilder {
    cart: Option<Cart>,
    configuration: Option<Configuration>,
    contact_id: Option<i64>,
    description: Option<String>,
    notification: Option<Notification>,
    reference: Option<String>,
}

impl CreatePaymentRequestRequestBuilder {
    pub fn cart(mut self, value: Cart) -> Self {
        self.cart = Some(value);
        self
    }

    pub fn configuration(mut self, value: Configuration) -> Self {
        self.configuration = Some(value);
        self
    }

    pub fn contact_id(mut self, value: i64) -> Self {
        self.contact_id = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn notification(mut self, value: Notification) -> Self {
        self.notification = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentRequestRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cart`](CreatePaymentRequestRequestBuilder::cart)
    /// - [`contact_id`](CreatePaymentRequestRequestBuilder::contact_id)
    /// - [`reference`](CreatePaymentRequestRequestBuilder::reference)
    pub fn build(self) -> Result<CreatePaymentRequestRequest, BuildError> {
        Ok(CreatePaymentRequestRequest {
            cart: self.cart.ok_or_else(|| BuildError::missing_field("cart"))?,
            configuration: self.configuration,
            contact_id: self
                .contact_id
                .ok_or_else(|| BuildError::missing_field("contact_id"))?,
            description: self.description,
            notification: self.notification,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}

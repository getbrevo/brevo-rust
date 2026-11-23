pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetPaymentRequestResponse {
    pub cart: Cart,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Configuration>,
    /// Brevo ID of the contact requested to pay.
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<i64>,
    pub notification: Notification,
    /// number of reminders sent.
    #[serde(rename = "numberOfRemindersSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_reminders_sent: Option<i64>,
    /// Reference of the payment request, it will appear on the payment page.
    #[serde(default)]
    pub reference: String,
    /// Status of the payment request.
    pub status: GetPaymentRequestResponseStatus,
}

impl GetPaymentRequestResponse {
    pub fn builder() -> GetPaymentRequestResponseBuilder {
        <GetPaymentRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPaymentRequestResponseBuilder {
    cart: Option<Cart>,
    configuration: Option<Configuration>,
    contact_id: Option<i64>,
    notification: Option<Notification>,
    number_of_reminders_sent: Option<i64>,
    reference: Option<String>,
    status: Option<GetPaymentRequestResponseStatus>,
}

impl GetPaymentRequestResponseBuilder {
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

    pub fn notification(mut self, value: Notification) -> Self {
        self.notification = Some(value);
        self
    }

    pub fn number_of_reminders_sent(mut self, value: i64) -> Self {
        self.number_of_reminders_sent = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetPaymentRequestResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPaymentRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cart`](GetPaymentRequestResponseBuilder::cart)
    /// - [`notification`](GetPaymentRequestResponseBuilder::notification)
    /// - [`reference`](GetPaymentRequestResponseBuilder::reference)
    /// - [`status`](GetPaymentRequestResponseBuilder::status)
    pub fn build(self) -> Result<GetPaymentRequestResponse, BuildError> {
        Ok(GetPaymentRequestResponse {
            cart: self.cart.ok_or_else(|| BuildError::missing_field("cart"))?,
            configuration: self.configuration,
            contact_id: self.contact_id,
            notification: self
                .notification
                .ok_or_else(|| BuildError::missing_field("notification"))?,
            number_of_reminders_sent: self.number_of_reminders_sent,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentRequestResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
    /// URL of the payment request created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreatePaymentRequestResponse {
    pub fn builder() -> CreatePaymentRequestResponseBuilder {
        <CreatePaymentRequestResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentRequestResponseBuilder {
    id: Option<i64>,
    url: Option<String>,
}

impl CreatePaymentRequestResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentRequestResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreatePaymentRequestResponseBuilder::id)
    pub fn build(self) -> Result<CreatePaymentRequestResponse, BuildError> {
        Ok(CreatePaymentRequestResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
        })
    }
}

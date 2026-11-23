pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WalletPassRedeemRequest {
    /// Amount spent during the visit or transaction. Useful for point-of-sale integrations and purchase tracking.
    #[serde(rename = "amountSpent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_spent: Option<f64>,
    /// Arbitrary key/value pairs merged into the scan event's properties, for use in automations and segmentation (e.g. register id, cashier, department). Reserved keys take precedence over the same keys supplied here: `passId` is always set by the endpoint, and `location`, `amountSpent` and `eventTag` are replaced by the matching top-level fields when those are sent.
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
    /// Free-form tag identifying the event or context of the scan (e.g. an event name a contact attended). Useful to segment or route automations.
    #[serde(rename = "eventTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tag: Option<String>,
    /// Location where the scan occurred. Can be a store name, address, or any identifier relevant to your use case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl WalletPassRedeemRequest {
    pub fn builder() -> WalletPassRedeemRequestBuilder {
        <WalletPassRedeemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletPassRedeemRequestBuilder {
    amount_spent: Option<f64>,
    custom_data: Option<HashMap<String, serde_json::Value>>,
    event_tag: Option<String>,
    location: Option<String>,
}

impl WalletPassRedeemRequestBuilder {
    pub fn amount_spent(mut self, value: f64) -> Self {
        self.amount_spent = Some(value);
        self
    }

    pub fn custom_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_data = Some(value);
        self
    }

    pub fn event_tag(mut self, value: impl Into<String>) -> Self {
        self.event_tag = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WalletPassRedeemRequest`].
    pub fn build(self) -> Result<WalletPassRedeemRequest, BuildError> {
        Ok(WalletPassRedeemRequest {
            amount_spent: self.amount_spent,
            custom_data: self.custom_data,
            event_tag: self.event_tag,
            location: self.location,
        })
    }
}

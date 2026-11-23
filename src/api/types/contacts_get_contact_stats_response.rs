pub use crate::prelude::*;

/// Campaign Statistics for the contact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetContactStatsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked: Option<Vec<GetContactStatsResponseClickedItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaints: Option<Vec<GetContactStatsResponseComplaintsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<Vec<GetContactStatsResponseDeliveredItem>>,
    #[serde(rename = "hardBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_bounces: Option<Vec<GetContactStatsResponseHardBouncesItem>>,
    #[serde(rename = "messagesSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_sent: Option<Vec<GetContactStatsResponseMessagesSentItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened: Option<Vec<GetContactStatsResponseOpenedItem>>,
    #[serde(rename = "softBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_bounces: Option<Vec<GetContactStatsResponseSoftBouncesItem>>,
    #[serde(rename = "transacAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transac_attributes: Option<Vec<GetContactStatsResponseTransacAttributesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscriptions: Option<GetContactStatsResponseUnsubscriptions>,
}

impl GetContactStatsResponse {
    pub fn builder() -> GetContactStatsResponseBuilder {
        <GetContactStatsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactStatsResponseBuilder {
    clicked: Option<Vec<GetContactStatsResponseClickedItem>>,
    complaints: Option<Vec<GetContactStatsResponseComplaintsItem>>,
    delivered: Option<Vec<GetContactStatsResponseDeliveredItem>>,
    hard_bounces: Option<Vec<GetContactStatsResponseHardBouncesItem>>,
    messages_sent: Option<Vec<GetContactStatsResponseMessagesSentItem>>,
    opened: Option<Vec<GetContactStatsResponseOpenedItem>>,
    soft_bounces: Option<Vec<GetContactStatsResponseSoftBouncesItem>>,
    transac_attributes: Option<Vec<GetContactStatsResponseTransacAttributesItem>>,
    unsubscriptions: Option<GetContactStatsResponseUnsubscriptions>,
}

impl GetContactStatsResponseBuilder {
    pub fn clicked(mut self, value: Vec<GetContactStatsResponseClickedItem>) -> Self {
        self.clicked = Some(value);
        self
    }

    pub fn complaints(mut self, value: Vec<GetContactStatsResponseComplaintsItem>) -> Self {
        self.complaints = Some(value);
        self
    }

    pub fn delivered(mut self, value: Vec<GetContactStatsResponseDeliveredItem>) -> Self {
        self.delivered = Some(value);
        self
    }

    pub fn hard_bounces(mut self, value: Vec<GetContactStatsResponseHardBouncesItem>) -> Self {
        self.hard_bounces = Some(value);
        self
    }

    pub fn messages_sent(mut self, value: Vec<GetContactStatsResponseMessagesSentItem>) -> Self {
        self.messages_sent = Some(value);
        self
    }

    pub fn opened(mut self, value: Vec<GetContactStatsResponseOpenedItem>) -> Self {
        self.opened = Some(value);
        self
    }

    pub fn soft_bounces(mut self, value: Vec<GetContactStatsResponseSoftBouncesItem>) -> Self {
        self.soft_bounces = Some(value);
        self
    }

    pub fn transac_attributes(
        mut self,
        value: Vec<GetContactStatsResponseTransacAttributesItem>,
    ) -> Self {
        self.transac_attributes = Some(value);
        self
    }

    pub fn unsubscriptions(mut self, value: GetContactStatsResponseUnsubscriptions) -> Self {
        self.unsubscriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactStatsResponse`].
    pub fn build(self) -> Result<GetContactStatsResponse, BuildError> {
        Ok(GetContactStatsResponse {
            clicked: self.clicked,
            complaints: self.complaints,
            delivered: self.delivered,
            hard_bounces: self.hard_bounces,
            messages_sent: self.messages_sent,
            opened: self.opened,
            soft_bounces: self.soft_bounces,
            transac_attributes: self.transac_attributes,
            unsubscriptions: self.unsubscriptions,
        })
    }
}

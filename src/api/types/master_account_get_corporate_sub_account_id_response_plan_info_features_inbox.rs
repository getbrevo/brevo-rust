pub use crate::prelude::*;

/// Inbox details / Not applicable on ENTv2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox {
    /// Quantity of inbox provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available inboxes for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoFeaturesInboxBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoFeaturesInboxBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesInboxBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesInboxBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox {
            quantity: self.quantity,
            remaining: self.remaining,
        })
    }
}

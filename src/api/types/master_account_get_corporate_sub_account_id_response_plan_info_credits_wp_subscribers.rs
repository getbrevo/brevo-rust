pub use crate::prelude::*;

/// Push credits remaining on the sub-account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers {
    /// Quantity of Push sending limits provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available Push sending limits for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribersBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribersBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribersBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers, BuildError> {
        Ok(
            GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers {
                quantity: self.quantity,
                remaining: self.remaining,
            },
        )
    }
}

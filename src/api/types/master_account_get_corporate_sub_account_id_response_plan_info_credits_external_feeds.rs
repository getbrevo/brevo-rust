pub use crate::prelude::*;

/// externalFeeds credits remaining on the sub-account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds {
    /// Quantity of externalFeeds messaging limits
    /// provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available externalFeeds messaging limits for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeedsBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeedsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeedsBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeedsBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds, BuildError> {
        Ok(
            GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds {
                quantity: self.quantity,
                remaining: self.remaining,
            },
        )
    }
}

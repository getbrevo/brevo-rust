pub use crate::prelude::*;

/// Landing page details / Not applicable on ENTv2
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage {
    /// Quantity of landing pages provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Available landing pages for use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPageBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPageBuilder {
    quantity: Option<i64>,
    remaining: Option<i64>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPageBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage`].
    pub fn build(
        self,
    ) -> Result<GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage, BuildError> {
        Ok(
            GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage {
                quantity: self.quantity,
                remaining: self.remaining,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PutCorporateSubAccountIdPlanRequest {
    /// Credit details to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<PutCorporateSubAccountIdPlanRequestCredits>,
    /// Features details to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PutCorporateSubAccountIdPlanRequestFeatures>,
}

impl PutCorporateSubAccountIdPlanRequest {
    pub fn builder() -> PutCorporateSubAccountIdPlanRequestBuilder {
        <PutCorporateSubAccountIdPlanRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateSubAccountIdPlanRequestBuilder {
    credits: Option<PutCorporateSubAccountIdPlanRequestCredits>,
    features: Option<PutCorporateSubAccountIdPlanRequestFeatures>,
}

impl PutCorporateSubAccountIdPlanRequestBuilder {
    pub fn credits(mut self, value: PutCorporateSubAccountIdPlanRequestCredits) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn features(mut self, value: PutCorporateSubAccountIdPlanRequestFeatures) -> Self {
        self.features = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateSubAccountIdPlanRequest`].
    pub fn build(self) -> Result<PutCorporateSubAccountIdPlanRequest, BuildError> {
        Ok(PutCorporateSubAccountIdPlanRequest {
            credits: self.credits,
            features: self.features,
        })
    }
}

pub use crate::prelude::*;

/// Sub-account plan details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfo {
    /// Credits quota and remaining credits on the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<GetCorporateSubAccountIdResponsePlanInfoCredits>,
    /// Features available on the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<GetCorporateSubAccountIdResponsePlanInfoFeatures>,
    /// type of the plan
    #[serde(rename = "planType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
}

impl GetCorporateSubAccountIdResponsePlanInfo {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoBuilder {
    credits: Option<GetCorporateSubAccountIdResponsePlanInfoCredits>,
    features: Option<GetCorporateSubAccountIdResponsePlanInfoFeatures>,
    plan_type: Option<String>,
}

impl GetCorporateSubAccountIdResponsePlanInfoBuilder {
    pub fn credits(mut self, value: GetCorporateSubAccountIdResponsePlanInfoCredits) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn features(mut self, value: GetCorporateSubAccountIdResponsePlanInfoFeatures) -> Self {
        self.features = Some(value);
        self
    }

    pub fn plan_type(mut self, value: impl Into<String>) -> Self {
        self.plan_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfo`].
    pub fn build(self) -> Result<GetCorporateSubAccountIdResponsePlanInfo, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfo {
            credits: self.credits,
            features: self.features,
            plan_type: self.plan_type,
        })
    }
}

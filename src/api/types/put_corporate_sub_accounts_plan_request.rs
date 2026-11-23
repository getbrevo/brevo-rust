pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PutCorporateSubAccountsPlanRequest {
    /// Credit details to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<PutCorporateSubAccountsPlanRequestCredits>,
    /// Features details to update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PutCorporateSubAccountsPlanRequestFeatures>,
    /// List of sub-account ids
    #[serde(rename = "subAccountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_ids: Option<Vec<i64>>,
}

impl PutCorporateSubAccountsPlanRequest {
    pub fn builder() -> PutCorporateSubAccountsPlanRequestBuilder {
        <PutCorporateSubAccountsPlanRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateSubAccountsPlanRequestBuilder {
    credits: Option<PutCorporateSubAccountsPlanRequestCredits>,
    features: Option<PutCorporateSubAccountsPlanRequestFeatures>,
    sub_account_ids: Option<Vec<i64>>,
}

impl PutCorporateSubAccountsPlanRequestBuilder {
    pub fn credits(mut self, value: PutCorporateSubAccountsPlanRequestCredits) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn features(mut self, value: PutCorporateSubAccountsPlanRequestFeatures) -> Self {
        self.features = Some(value);
        self
    }

    pub fn sub_account_ids(mut self, value: Vec<i64>) -> Self {
        self.sub_account_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateSubAccountsPlanRequest`].
    pub fn build(self) -> Result<PutCorporateSubAccountsPlanRequest, BuildError> {
        Ok(PutCorporateSubAccountsPlanRequest {
            credits: self.credits,
            features: self.features,
            sub_account_ids: self.sub_account_ids,
        })
    }
}

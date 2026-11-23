pub use crate::prelude::*;

/// Plan details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCorporateMasterAccountResponsePlanInfo {
    /// Plan currency
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// List of provided features in the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<GetCorporateMasterAccountResponsePlanInfoFeaturesItem>>,
    /// Timestamp of next billing date
    #[serde(rename = "nextBillingAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_at: Option<i64>,
    /// Plan period type
    #[serde(rename = "planPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_period: Option<GetCorporateMasterAccountResponsePlanInfoPlanPeriod>,
    /// Plan amount
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    /// Number of sub-accounts
    #[serde(rename = "subAccounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_accounts: Option<i64>,
}

impl GetCorporateMasterAccountResponsePlanInfo {
    pub fn builder() -> GetCorporateMasterAccountResponsePlanInfoBuilder {
        <GetCorporateMasterAccountResponsePlanInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateMasterAccountResponsePlanInfoBuilder {
    currency_code: Option<String>,
    features: Option<Vec<GetCorporateMasterAccountResponsePlanInfoFeaturesItem>>,
    next_billing_at: Option<i64>,
    plan_period: Option<GetCorporateMasterAccountResponsePlanInfoPlanPeriod>,
    price: Option<f64>,
    sub_accounts: Option<i64>,
}

impl GetCorporateMasterAccountResponsePlanInfoBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn features(
        mut self,
        value: Vec<GetCorporateMasterAccountResponsePlanInfoFeaturesItem>,
    ) -> Self {
        self.features = Some(value);
        self
    }

    pub fn next_billing_at(mut self, value: i64) -> Self {
        self.next_billing_at = Some(value);
        self
    }

    pub fn plan_period(
        mut self,
        value: GetCorporateMasterAccountResponsePlanInfoPlanPeriod,
    ) -> Self {
        self.plan_period = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn sub_accounts(mut self, value: i64) -> Self {
        self.sub_accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateMasterAccountResponsePlanInfo`].
    pub fn build(self) -> Result<GetCorporateMasterAccountResponsePlanInfo, BuildError> {
        Ok(GetCorporateMasterAccountResponsePlanInfo {
            currency_code: self.currency_code,
            features: self.features,
            next_billing_at: self.next_billing_at,
            plan_period: self.plan_period,
            price: self.price,
            sub_accounts: self.sub_accounts,
        })
    }
}

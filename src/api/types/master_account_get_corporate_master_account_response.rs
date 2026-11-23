pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCorporateMasterAccountResponse {
    /// Billing details of the master account organization
    #[serde(rename = "billingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_info: Option<GetCorporateMasterAccountResponseBillingInfo>,
    /// Company name of master account organization
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Currency code of the master account organization
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// Email id of master account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Unique identifier of the master account organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Plan details
    #[serde(rename = "planInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_info: Option<GetCorporateMasterAccountResponsePlanInfo>,
    /// Timezone of the master account organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl GetCorporateMasterAccountResponse {
    pub fn builder() -> GetCorporateMasterAccountResponseBuilder {
        <GetCorporateMasterAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateMasterAccountResponseBuilder {
    billing_info: Option<GetCorporateMasterAccountResponseBillingInfo>,
    company_name: Option<String>,
    currency_code: Option<String>,
    email: Option<String>,
    id: Option<i64>,
    plan_info: Option<GetCorporateMasterAccountResponsePlanInfo>,
    timezone: Option<String>,
}

impl GetCorporateMasterAccountResponseBuilder {
    pub fn billing_info(mut self, value: GetCorporateMasterAccountResponseBillingInfo) -> Self {
        self.billing_info = Some(value);
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn plan_info(mut self, value: GetCorporateMasterAccountResponsePlanInfo) -> Self {
        self.plan_info = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateMasterAccountResponse`].
    pub fn build(self) -> Result<GetCorporateMasterAccountResponse, BuildError> {
        Ok(GetCorporateMasterAccountResponse {
            billing_info: self.billing_info,
            company_name: self.company_name,
            currency_code: self.currency_code,
            email: self.email,
            id: self.id,
            plan_info: self.plan_info,
            timezone: self.timezone,
        })
    }
}

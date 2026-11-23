pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAccountResponse {
    /// Unique identifier for the organization
    #[serde(default)]
    pub organization_id: String,
    /// Unique identifier for the user
    #[serde(default)]
    pub user_id: i64,
    /// Indicates if the account has enterprise features
    #[serde(default)]
    pub enterprise: bool,
    /// Name of the company
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    /// Login Email
    #[serde(default)]
    pub email: String,
    /// First Name
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    /// Last Name
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    /// Address informations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<GetAccountResponseAddress>,
    #[serde(rename = "marketingAutomation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_automation: Option<GetAccountResponseMarketingAutomation>,
    /// Information about your plans and credits
    #[serde(default)]
    pub plan: Vec<GetAccountResponsePlanItem>,
    /// Detailed information about different plan categories
    #[serde(rename = "planVerticals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_verticals: Option<Vec<GetAccountResponsePlanVerticalsItem>>,
    /// Information about your transactional email account
    #[serde(default)]
    pub relay: GetAccountResponseRelay,
}

impl GetAccountResponse {
    pub fn builder() -> GetAccountResponseBuilder {
        <GetAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseBuilder {
    organization_id: Option<String>,
    user_id: Option<i64>,
    enterprise: Option<bool>,
    company_name: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<GetAccountResponseAddress>,
    marketing_automation: Option<GetAccountResponseMarketingAutomation>,
    plan: Option<Vec<GetAccountResponsePlanItem>>,
    plan_verticals: Option<Vec<GetAccountResponsePlanVerticalsItem>>,
    relay: Option<GetAccountResponseRelay>,
}

impl GetAccountResponseBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn enterprise(mut self, value: bool) -> Self {
        self.enterprise = Some(value);
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn address(mut self, value: GetAccountResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn marketing_automation(mut self, value: GetAccountResponseMarketingAutomation) -> Self {
        self.marketing_automation = Some(value);
        self
    }

    pub fn plan(mut self, value: Vec<GetAccountResponsePlanItem>) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn plan_verticals(mut self, value: Vec<GetAccountResponsePlanVerticalsItem>) -> Self {
        self.plan_verticals = Some(value);
        self
    }

    pub fn relay(mut self, value: GetAccountResponseRelay) -> Self {
        self.relay = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](GetAccountResponseBuilder::organization_id)
    /// - [`user_id`](GetAccountResponseBuilder::user_id)
    /// - [`enterprise`](GetAccountResponseBuilder::enterprise)
    /// - [`company_name`](GetAccountResponseBuilder::company_name)
    /// - [`email`](GetAccountResponseBuilder::email)
    /// - [`first_name`](GetAccountResponseBuilder::first_name)
    /// - [`last_name`](GetAccountResponseBuilder::last_name)
    /// - [`plan`](GetAccountResponseBuilder::plan)
    /// - [`relay`](GetAccountResponseBuilder::relay)
    pub fn build(self) -> Result<GetAccountResponse, BuildError> {
        Ok(GetAccountResponse {
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
            enterprise: self
                .enterprise
                .ok_or_else(|| BuildError::missing_field("enterprise"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            address: self.address,
            marketing_automation: self.marketing_automation,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            plan_verticals: self.plan_verticals,
            relay: self
                .relay
                .ok_or_else(|| BuildError::missing_field("relay"))?,
        })
    }
}

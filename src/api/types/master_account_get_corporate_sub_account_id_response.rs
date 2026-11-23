pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponse {
    /// Sub-account company name
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Email id of the sub-account organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// List of groups associated with the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GetCorporateSubAccountIdResponseGroupsItem>>,
    /// Name of the sub-account user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sub-account plan details
    #[serde(rename = "planInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_info: Option<GetCorporateSubAccountIdResponsePlanInfo>,
}

impl GetCorporateSubAccountIdResponse {
    pub fn builder() -> GetCorporateSubAccountIdResponseBuilder {
        <GetCorporateSubAccountIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponseBuilder {
    company_name: Option<String>,
    email: Option<String>,
    groups: Option<Vec<GetCorporateSubAccountIdResponseGroupsItem>>,
    name: Option<String>,
    plan_info: Option<GetCorporateSubAccountIdResponsePlanInfo>,
}

impl GetCorporateSubAccountIdResponseBuilder {
    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn groups(mut self, value: Vec<GetCorporateSubAccountIdResponseGroupsItem>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn plan_info(mut self, value: GetCorporateSubAccountIdResponsePlanInfo) -> Self {
        self.plan_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponse`].
    pub fn build(self) -> Result<GetCorporateSubAccountIdResponse, BuildError> {
        Ok(GetCorporateSubAccountIdResponse {
            company_name: self.company_name,
            email: self.email,
            groups: self.groups,
            name: self.name,
            plan_info: self.plan_info,
        })
    }
}

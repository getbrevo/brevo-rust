pub use crate::prelude::*;

/// Features available on the sub-account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeatures {
    /// Inbox details / Not applicable on ENTv2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox>,
    /// Landing page details / Not applicable on ENTv2
    #[serde(rename = "landingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage>,
    /// Sales and service users to use phone, sales and
    /// conversations
    #[serde(rename = "salesUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_users: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers>,
    /// Marketing users to manage the marketing channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeatures {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoFeaturesBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoFeaturesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoFeaturesBuilder {
    inbox: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox>,
    landing_page: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage>,
    sales_users: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers>,
    users: Option<GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers>,
}

impl GetCorporateSubAccountIdResponsePlanInfoFeaturesBuilder {
    pub fn inbox(mut self, value: GetCorporateSubAccountIdResponsePlanInfoFeaturesInbox) -> Self {
        self.inbox = Some(value);
        self
    }

    pub fn landing_page(
        mut self,
        value: GetCorporateSubAccountIdResponsePlanInfoFeaturesLandingPage,
    ) -> Self {
        self.landing_page = Some(value);
        self
    }

    pub fn sales_users(
        mut self,
        value: GetCorporateSubAccountIdResponsePlanInfoFeaturesSalesUsers,
    ) -> Self {
        self.sales_users = Some(value);
        self
    }

    pub fn users(mut self, value: GetCorporateSubAccountIdResponsePlanInfoFeaturesUsers) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoFeatures`].
    pub fn build(self) -> Result<GetCorporateSubAccountIdResponsePlanInfoFeatures, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoFeatures {
            inbox: self.inbox,
            landing_page: self.landing_page,
            sales_users: self.sales_users,
            users: self.users,
        })
    }
}

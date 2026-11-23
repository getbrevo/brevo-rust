pub use crate::prelude::*;

/// Features details to update
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateSubAccountIdPlanRequestFeatures {
    /// Number of inboxes / Not required on ENTv2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox: Option<i64>,
    /// Number of landing pages
    #[serde(rename = "landingPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page: Option<i64>,
    /// Number of sales and service users | only available in ENT-V2
    #[serde(rename = "salesUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_users: Option<i64>,
    /// Number of multi-users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<i64>,
}

impl PutCorporateSubAccountIdPlanRequestFeatures {
    pub fn builder() -> PutCorporateSubAccountIdPlanRequestFeaturesBuilder {
        <PutCorporateSubAccountIdPlanRequestFeaturesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateSubAccountIdPlanRequestFeaturesBuilder {
    inbox: Option<i64>,
    landing_page: Option<i64>,
    sales_users: Option<i64>,
    users: Option<i64>,
}

impl PutCorporateSubAccountIdPlanRequestFeaturesBuilder {
    pub fn inbox(mut self, value: i64) -> Self {
        self.inbox = Some(value);
        self
    }

    pub fn landing_page(mut self, value: i64) -> Self {
        self.landing_page = Some(value);
        self
    }

    pub fn sales_users(mut self, value: i64) -> Self {
        self.sales_users = Some(value);
        self
    }

    pub fn users(mut self, value: i64) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateSubAccountIdPlanRequestFeatures`].
    pub fn build(self) -> Result<PutCorporateSubAccountIdPlanRequestFeatures, BuildError> {
        Ok(PutCorporateSubAccountIdPlanRequestFeatures {
            inbox: self.inbox,
            landing_page: self.landing_page,
            sales_users: self.sales_users,
            users: self.users,
        })
    }
}

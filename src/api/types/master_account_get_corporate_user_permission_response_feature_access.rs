pub use crate::prelude::*;

/// Granular feature permissions given to the user.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateUserPermissionResponseFeatureAccess {
    /// Permission on analytics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<Vec<String>>,
    /// Permission on api keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_keys: Option<Vec<String>>,
    /// Permission on apps management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_management: Option<Vec<String>>,
    /// Permission on create sub-accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_sub_organizations: Option<Vec<String>>,
    /// Permission on manage sub-accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_sub_organizations: Option<Vec<String>>,
    /// Permission on my plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_plan: Option<Vec<String>>,
    /// Permission on groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_organization_groups: Option<Vec<String>>,
    /// Permission on user management
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_management: Option<Vec<String>>,
}

impl GetCorporateUserPermissionResponseFeatureAccess {
    pub fn builder() -> GetCorporateUserPermissionResponseFeatureAccessBuilder {
        <GetCorporateUserPermissionResponseFeatureAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateUserPermissionResponseFeatureAccessBuilder {
    analytics: Option<Vec<String>>,
    api_keys: Option<Vec<String>>,
    apps_management: Option<Vec<String>>,
    create_sub_organizations: Option<Vec<String>>,
    manage_sub_organizations: Option<Vec<String>>,
    my_plan: Option<Vec<String>>,
    sub_organization_groups: Option<Vec<String>>,
    user_management: Option<Vec<String>>,
}

impl GetCorporateUserPermissionResponseFeatureAccessBuilder {
    pub fn analytics(mut self, value: Vec<String>) -> Self {
        self.analytics = Some(value);
        self
    }

    pub fn api_keys(mut self, value: Vec<String>) -> Self {
        self.api_keys = Some(value);
        self
    }

    pub fn apps_management(mut self, value: Vec<String>) -> Self {
        self.apps_management = Some(value);
        self
    }

    pub fn create_sub_organizations(mut self, value: Vec<String>) -> Self {
        self.create_sub_organizations = Some(value);
        self
    }

    pub fn manage_sub_organizations(mut self, value: Vec<String>) -> Self {
        self.manage_sub_organizations = Some(value);
        self
    }

    pub fn my_plan(mut self, value: Vec<String>) -> Self {
        self.my_plan = Some(value);
        self
    }

    pub fn sub_organization_groups(mut self, value: Vec<String>) -> Self {
        self.sub_organization_groups = Some(value);
        self
    }

    pub fn user_management(mut self, value: Vec<String>) -> Self {
        self.user_management = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateUserPermissionResponseFeatureAccess`].
    pub fn build(self) -> Result<GetCorporateUserPermissionResponseFeatureAccess, BuildError> {
        Ok(GetCorporateUserPermissionResponseFeatureAccess {
            analytics: self.analytics,
            api_keys: self.api_keys,
            apps_management: self.apps_management,
            create_sub_organizations: self.create_sub_organizations,
            manage_sub_organizations: self.manage_sub_organizations,
            my_plan: self.my_plan,
            sub_organization_groups: self.sub_organization_groups,
            user_management: self.user_management,
        })
    }
}

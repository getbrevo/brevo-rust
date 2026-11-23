pub use crate::prelude::*;

/// Feature accessiblity given to the user. (Required only
/// if status is active)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateInvitedUsersListResponseUsersItemFeatureAccess {
    /// Analytics dashboard accessibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<Vec<String>>,
    /// Api keys accessiblity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_keys: Option<Vec<String>>,
    /// Apps management accessiblity | Not available in
    /// ENTv2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps_management: Option<Vec<String>>,
    /// Authorization to create sub-organization in the
    /// admin account. If the user creating the
    /// sub-organization, belongs to a group, the user must
    /// choose a group at the sub-organization creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_sub_organizations: Option<Vec<String>>,
    /// Authorization to manage and access sub-organizations
    /// in the admin account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_sub_organizations: Option<Vec<String>>,
    /// My plan accessiblity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_plan: Option<Vec<String>>,
    /// Group creation, modification or deletion
    /// accessibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_organization_groups: Option<Vec<String>>,
    /// User management accessiblity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_management: Option<Vec<String>>,
}

impl GetCorporateInvitedUsersListResponseUsersItemFeatureAccess {
    pub fn builder() -> GetCorporateInvitedUsersListResponseUsersItemFeatureAccessBuilder {
        <GetCorporateInvitedUsersListResponseUsersItemFeatureAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateInvitedUsersListResponseUsersItemFeatureAccessBuilder {
    analytics: Option<Vec<String>>,
    api_keys: Option<Vec<String>>,
    apps_management: Option<Vec<String>>,
    create_sub_organizations: Option<Vec<String>>,
    manage_sub_organizations: Option<Vec<String>>,
    my_plan: Option<Vec<String>>,
    sub_organization_groups: Option<Vec<String>>,
    user_management: Option<Vec<String>>,
}

impl GetCorporateInvitedUsersListResponseUsersItemFeatureAccessBuilder {
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

    /// Consumes the builder and constructs a [`GetCorporateInvitedUsersListResponseUsersItemFeatureAccess`].
    pub fn build(
        self,
    ) -> Result<GetCorporateInvitedUsersListResponseUsersItemFeatureAccess, BuildError> {
        Ok(GetCorporateInvitedUsersListResponseUsersItemFeatureAccess {
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

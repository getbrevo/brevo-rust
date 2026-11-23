pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactStatsResponseUnsubscriptions {
    /// Contact has been unsubscribed from the administrator
    #[serde(rename = "adminUnsubscription")]
    #[serde(default)]
    pub admin_unsubscription: Vec<GetContactStatsResponseUnsubscriptionsAdminUnsubscriptionItem>,
    /// Contact has unsubscribed via the unsubscription link in the email
    #[serde(rename = "userUnsubscription")]
    #[serde(default)]
    pub user_unsubscription: Vec<GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem>,
}

impl GetContactStatsResponseUnsubscriptions {
    pub fn builder() -> GetContactStatsResponseUnsubscriptionsBuilder {
        <GetContactStatsResponseUnsubscriptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactStatsResponseUnsubscriptionsBuilder {
    admin_unsubscription:
        Option<Vec<GetContactStatsResponseUnsubscriptionsAdminUnsubscriptionItem>>,
    user_unsubscription: Option<Vec<GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem>>,
}

impl GetContactStatsResponseUnsubscriptionsBuilder {
    pub fn admin_unsubscription(
        mut self,
        value: Vec<GetContactStatsResponseUnsubscriptionsAdminUnsubscriptionItem>,
    ) -> Self {
        self.admin_unsubscription = Some(value);
        self
    }

    pub fn user_unsubscription(
        mut self,
        value: Vec<GetContactStatsResponseUnsubscriptionsUserUnsubscriptionItem>,
    ) -> Self {
        self.user_unsubscription = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactStatsResponseUnsubscriptions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admin_unsubscription`](GetContactStatsResponseUnsubscriptionsBuilder::admin_unsubscription)
    /// - [`user_unsubscription`](GetContactStatsResponseUnsubscriptionsBuilder::user_unsubscription)
    pub fn build(self) -> Result<GetContactStatsResponseUnsubscriptions, BuildError> {
        Ok(GetContactStatsResponseUnsubscriptions {
            admin_unsubscription: self
                .admin_unsubscription
                .ok_or_else(|| BuildError::missing_field("admin_unsubscription"))?,
            user_unsubscription: self
                .user_unsubscription
                .ok_or_else(|| BuildError::missing_field("user_unsubscription"))?,
        })
    }
}

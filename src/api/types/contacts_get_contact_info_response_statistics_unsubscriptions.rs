pub use crate::prelude::*;

/// Listing of the unsubscription for the contact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsUnsubscriptions {
    /// Contact has been unsubscribed from the administrator
    #[serde(rename = "adminUnsubscription")]
    #[serde(default)]
    pub admin_unsubscription:
        Vec<GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem>,
    /// Contact unsubscribe via unsubscription link in a campaign
    #[serde(rename = "userUnsubscription")]
    #[serde(default)]
    pub user_unsubscription:
        Vec<GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem>,
}

impl GetContactInfoResponseStatisticsUnsubscriptions {
    pub fn builder() -> GetContactInfoResponseStatisticsUnsubscriptionsBuilder {
        <GetContactInfoResponseStatisticsUnsubscriptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsUnsubscriptionsBuilder {
    admin_unsubscription:
        Option<Vec<GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem>>,
    user_unsubscription:
        Option<Vec<GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem>>,
}

impl GetContactInfoResponseStatisticsUnsubscriptionsBuilder {
    pub fn admin_unsubscription(
        mut self,
        value: Vec<GetContactInfoResponseStatisticsUnsubscriptionsAdminUnsubscriptionItem>,
    ) -> Self {
        self.admin_unsubscription = Some(value);
        self
    }

    pub fn user_unsubscription(
        mut self,
        value: Vec<GetContactInfoResponseStatisticsUnsubscriptionsUserUnsubscriptionItem>,
    ) -> Self {
        self.user_unsubscription = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsUnsubscriptions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admin_unsubscription`](GetContactInfoResponseStatisticsUnsubscriptionsBuilder::admin_unsubscription)
    /// - [`user_unsubscription`](GetContactInfoResponseStatisticsUnsubscriptionsBuilder::user_unsubscription)
    pub fn build(self) -> Result<GetContactInfoResponseStatisticsUnsubscriptions, BuildError> {
        Ok(GetContactInfoResponseStatisticsUnsubscriptions {
            admin_unsubscription: self
                .admin_unsubscription
                .ok_or_else(|| BuildError::missing_field("admin_unsubscription"))?,
            user_unsubscription: self
                .user_unsubscription
                .ok_or_else(|| BuildError::missing_field("user_unsubscription"))?,
        })
    }
}

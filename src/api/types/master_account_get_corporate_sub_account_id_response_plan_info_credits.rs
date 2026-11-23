pub use crate::prelude::*;

/// Credits quota and remaining credits on the sub-account
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountIdResponsePlanInfoCredits {
    /// Email credits remaining on the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsEmails>,
    /// externalFeeds credits remaining on the sub-account
    #[serde(rename = "externalFeeds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_feeds: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds>,
    /// SMS credits remaining on the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsSms>,
    /// Whatsapp credits remaining on the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp>,
    /// Push credits remaining on the sub-account
    #[serde(rename = "wpSubscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wp_subscribers: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCredits {
    pub fn builder() -> GetCorporateSubAccountIdResponsePlanInfoCreditsBuilder {
        <GetCorporateSubAccountIdResponsePlanInfoCreditsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountIdResponsePlanInfoCreditsBuilder {
    emails: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsEmails>,
    external_feeds: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds>,
    sms: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsSms>,
    whatsapp: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp>,
    wp_subscribers: Option<GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers>,
}

impl GetCorporateSubAccountIdResponsePlanInfoCreditsBuilder {
    pub fn emails(mut self, value: GetCorporateSubAccountIdResponsePlanInfoCreditsEmails) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn external_feeds(
        mut self,
        value: GetCorporateSubAccountIdResponsePlanInfoCreditsExternalFeeds,
    ) -> Self {
        self.external_feeds = Some(value);
        self
    }

    pub fn sms(mut self, value: GetCorporateSubAccountIdResponsePlanInfoCreditsSms) -> Self {
        self.sms = Some(value);
        self
    }

    pub fn whatsapp(
        mut self,
        value: GetCorporateSubAccountIdResponsePlanInfoCreditsWhatsapp,
    ) -> Self {
        self.whatsapp = Some(value);
        self
    }

    pub fn wp_subscribers(
        mut self,
        value: GetCorporateSubAccountIdResponsePlanInfoCreditsWpSubscribers,
    ) -> Self {
        self.wp_subscribers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountIdResponsePlanInfoCredits`].
    pub fn build(self) -> Result<GetCorporateSubAccountIdResponsePlanInfoCredits, BuildError> {
        Ok(GetCorporateSubAccountIdResponsePlanInfoCredits {
            emails: self.emails,
            external_feeds: self.external_feeds,
            sms: self.sms,
            whatsapp: self.whatsapp,
            wp_subscribers: self.wp_subscribers,
        })
    }
}

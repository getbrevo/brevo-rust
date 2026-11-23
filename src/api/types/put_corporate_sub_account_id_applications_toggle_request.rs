pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateSubAccountIdApplicationsToggleRequest {
    /// Set this field to enable or disable Analytics on the
    /// sub-account. Requires the master account's plan to include
    /// Analytics; otherwise the call returns a 400 error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics: Option<bool>,
    /// Set this field to enable or disable Automation on the
    /// sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<bool>,
    /// Set this field to enable or disable Conversations on the
    /// sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<bool>,
    /// Set this field to enable or disable Sales CRM on the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crm: Option<bool>,
    /// Set this field to enable or disable Email Campaigns on the
    /// sub-account
    #[serde(rename = "email-campaigns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_campaigns: Option<bool>,
    /// Set this field to enable or disable Facebook ads on the
    /// sub-account
    #[serde(rename = "facebook-ads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_ads: Option<bool>,
    /// Set this field to enable or disable Inbox on the sub-account /
    /// Not applicable on ENTv2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox: Option<bool>,
    /// Set this field to enable or disable Landing pages on the
    /// sub-account
    #[serde(rename = "landing-pages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_pages: Option<bool>,
    /// Set this field to enable or disable Meetings on the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meetings: Option<bool>,
    /// Set this field to enable or disable SMS Marketing on the
    /// sub-account
    #[serde(rename = "sms-campaigns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_campaigns: Option<bool>,
    /// Set this field to enable or disable Transactional Email on the
    /// sub-account
    #[serde(rename = "transactional-emails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_emails: Option<bool>,
    /// Set this field to enable or disable Transactional SMS on the
    /// sub-account
    #[serde(rename = "transactional-sms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_sms: Option<bool>,
    /// Set this field to enable or disable Web Push on the sub-account
    #[serde(rename = "web-push")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_push: Option<bool>,
    /// Set this field to enable or disable Whatsapp campaigns on the
    /// sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp: Option<bool>,
}

impl PutCorporateSubAccountIdApplicationsToggleRequest {
    pub fn builder() -> PutCorporateSubAccountIdApplicationsToggleRequestBuilder {
        <PutCorporateSubAccountIdApplicationsToggleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateSubAccountIdApplicationsToggleRequestBuilder {
    analytics: Option<bool>,
    automation: Option<bool>,
    conversations: Option<bool>,
    crm: Option<bool>,
    email_campaigns: Option<bool>,
    facebook_ads: Option<bool>,
    inbox: Option<bool>,
    landing_pages: Option<bool>,
    meetings: Option<bool>,
    sms_campaigns: Option<bool>,
    transactional_emails: Option<bool>,
    transactional_sms: Option<bool>,
    web_push: Option<bool>,
    whatsapp: Option<bool>,
}

impl PutCorporateSubAccountIdApplicationsToggleRequestBuilder {
    pub fn analytics(mut self, value: bool) -> Self {
        self.analytics = Some(value);
        self
    }

    pub fn automation(mut self, value: bool) -> Self {
        self.automation = Some(value);
        self
    }

    pub fn conversations(mut self, value: bool) -> Self {
        self.conversations = Some(value);
        self
    }

    pub fn crm(mut self, value: bool) -> Self {
        self.crm = Some(value);
        self
    }

    pub fn email_campaigns(mut self, value: bool) -> Self {
        self.email_campaigns = Some(value);
        self
    }

    pub fn facebook_ads(mut self, value: bool) -> Self {
        self.facebook_ads = Some(value);
        self
    }

    pub fn inbox(mut self, value: bool) -> Self {
        self.inbox = Some(value);
        self
    }

    pub fn landing_pages(mut self, value: bool) -> Self {
        self.landing_pages = Some(value);
        self
    }

    pub fn meetings(mut self, value: bool) -> Self {
        self.meetings = Some(value);
        self
    }

    pub fn sms_campaigns(mut self, value: bool) -> Self {
        self.sms_campaigns = Some(value);
        self
    }

    pub fn transactional_emails(mut self, value: bool) -> Self {
        self.transactional_emails = Some(value);
        self
    }

    pub fn transactional_sms(mut self, value: bool) -> Self {
        self.transactional_sms = Some(value);
        self
    }

    pub fn web_push(mut self, value: bool) -> Self {
        self.web_push = Some(value);
        self
    }

    pub fn whatsapp(mut self, value: bool) -> Self {
        self.whatsapp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateSubAccountIdApplicationsToggleRequest`].
    pub fn build(self) -> Result<PutCorporateSubAccountIdApplicationsToggleRequest, BuildError> {
        Ok(PutCorporateSubAccountIdApplicationsToggleRequest {
            analytics: self.analytics,
            automation: self.automation,
            conversations: self.conversations,
            crm: self.crm,
            email_campaigns: self.email_campaigns,
            facebook_ads: self.facebook_ads,
            inbox: self.inbox,
            landing_pages: self.landing_pages,
            meetings: self.meetings,
            sms_campaigns: self.sms_campaigns,
            transactional_emails: self.transactional_emails,
            transactional_sms: self.transactional_sms,
            web_push: self.web_push,
            whatsapp: self.whatsapp,
        })
    }
}

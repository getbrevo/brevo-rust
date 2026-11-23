pub use crate::prelude::*;

/// Set the filter for the contacts to be exported.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestContactExportRequestCustomContactFilter {
    /// **Mandatory if neither actionForEmailCampaigns nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of the provided action, restricted to the contacts selected by the other filters below. * **allContacts** - Fetch all the selected contacts. * **subscribed & unsubscribed** - Fetch the subscribed / unsubscribed (blacklisted via any means) contacts among the selected contacts. * **unsubscribedPerList** - Fetch the contacts that are unsubscribed from the given listId only; this action requires listId.
    ///
    /// The selection must be narrowed by at least one of **listId**, **segmentId**, **modifiedSince** or **createdSince**. Any of them can be combined with the action, so for example `unsubscribed` together with `modifiedSince` exports the contacts that are currently blocklisted and were modified after the given date-time, with no list or segment involved.
    #[serde(rename = "actionForContacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_for_contacts:
        Option<RequestContactExportRequestCustomContactFilterActionForContacts>,
    /// **Mandatory if neither actionForContacts nor actionForSmsCampaigns is passed.** This will export the contacts on the basis of provided action applied on email campaigns. * **openers & nonOpeners** - emailCampaignId is mandatory. Fetch the list of readers / non-readers for a particular email campaign. * **clickers & nonClickers** - emailCampaignId is mandatory. Fetch the list of clickers / non-clickers for a particular email campaign. * **unsubscribed** - emailCampaignId is mandatory. Fetch the list of all unsubscribed (blacklisted via any means) contacts for a particular email campaign. * **hardBounces & softBounces** - emailCampaignId is optional. Fetch the list of hard bounces / soft bounces for a particular / all email campaign(s).
    #[serde(rename = "actionForEmailCampaigns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_for_email_campaigns:
        Option<RequestContactExportRequestCustomContactFilterActionForEmailCampaigns>,
    /// **Mandatory if neither actionForContacts nor actionForEmailCampaigns is passed.** This will export the contacts on the basis of provided action applied on sms campaigns. * **unsubscribed** - Fetch the list of all unsubscribed (blacklisted via any means) contacts for all / particular sms campaigns. * **hardBounces & softBounces** - Fetch the list of hard bounces / soft bounces for all / particular sms campaigns.
    #[serde(rename = "actionForSmsCampaigns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_for_sms_campaigns:
        Option<RequestContactExportRequestCustomContactFilterActionForSmsCampaigns>,
    /// Considered only if **actionForContacts** is passed, ignored otherwise. Exports only the contacts created after the given date-time. The value must be a full RFC 3339 date-time: a date-only value such as `2026-10-01` is rejected, and so is a value without a timezone, so pass either the `Z` suffix or an explicit `+HH:MM` offset.
    #[serde(rename = "createdSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_since: Option<DateTime<FixedOffset>>,
    /// Considered only if **actionForEmailCampaigns** is passed, ignored otherwise. **Mandatory if action is one of the following - openers, nonOpeners, clickers, nonClickers, unsubscribed.** The id of the email campaign for which the corresponding action shall be applied in the filter.
    #[serde(rename = "emailCampaignId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_campaign_id: Option<i64>,
    /// ID of the list. Considered only if **actionForContacts** is passed. At least one of listId, segmentId, modifiedSince or createdSince must be provided; listId and segmentId cannot be used together.
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i64>,
    /// Considered only if **actionForContacts** is passed, ignored otherwise. Exports only the contacts modified after the given date-time. The value must be a full RFC 3339 date-time: a date-only value such as `2026-10-01` is rejected, and so is a value without a timezone, so pass either the `Z` suffix or an explicit `+HH:MM` offset. Note that a contact's modification date changes on any update, not only on a subscription change.
    #[serde(rename = "modifiedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub modified_since: Option<DateTime<FixedOffset>>,
    /// ID of the segment. Considered only if **actionForContacts** is passed. At least one of listId, segmentId, modifiedSince or createdSince must be provided; listId and segmentId cannot be used together.
    #[serde(rename = "segmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<i64>,
    /// Considered only if **actionForSmsCampaigns** is passed, ignored otherwise. The id of sms campaign for which the corresponding action shall be applied in the filter.
    #[serde(rename = "smsCampaignId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_campaign_id: Option<i64>,
}

impl RequestContactExportRequestCustomContactFilter {
    pub fn builder() -> RequestContactExportRequestCustomContactFilterBuilder {
        <RequestContactExportRequestCustomContactFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestContactExportRequestCustomContactFilterBuilder {
    action_for_contacts: Option<RequestContactExportRequestCustomContactFilterActionForContacts>,
    action_for_email_campaigns:
        Option<RequestContactExportRequestCustomContactFilterActionForEmailCampaigns>,
    action_for_sms_campaigns:
        Option<RequestContactExportRequestCustomContactFilterActionForSmsCampaigns>,
    created_since: Option<DateTime<FixedOffset>>,
    email_campaign_id: Option<i64>,
    list_id: Option<i64>,
    modified_since: Option<DateTime<FixedOffset>>,
    segment_id: Option<i64>,
    sms_campaign_id: Option<i64>,
}

impl RequestContactExportRequestCustomContactFilterBuilder {
    pub fn action_for_contacts(
        mut self,
        value: RequestContactExportRequestCustomContactFilterActionForContacts,
    ) -> Self {
        self.action_for_contacts = Some(value);
        self
    }

    pub fn action_for_email_campaigns(
        mut self,
        value: RequestContactExportRequestCustomContactFilterActionForEmailCampaigns,
    ) -> Self {
        self.action_for_email_campaigns = Some(value);
        self
    }

    pub fn action_for_sms_campaigns(
        mut self,
        value: RequestContactExportRequestCustomContactFilterActionForSmsCampaigns,
    ) -> Self {
        self.action_for_sms_campaigns = Some(value);
        self
    }

    pub fn created_since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_since = Some(value);
        self
    }

    pub fn email_campaign_id(mut self, value: i64) -> Self {
        self.email_campaign_id = Some(value);
        self
    }

    pub fn list_id(mut self, value: i64) -> Self {
        self.list_id = Some(value);
        self
    }

    pub fn modified_since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.modified_since = Some(value);
        self
    }

    pub fn segment_id(mut self, value: i64) -> Self {
        self.segment_id = Some(value);
        self
    }

    pub fn sms_campaign_id(mut self, value: i64) -> Self {
        self.sms_campaign_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestContactExportRequestCustomContactFilter`].
    pub fn build(self) -> Result<RequestContactExportRequestCustomContactFilter, BuildError> {
        Ok(RequestContactExportRequestCustomContactFilter {
            action_for_contacts: self.action_for_contacts,
            action_for_email_campaigns: self.action_for_email_campaigns,
            action_for_sms_campaigns: self.action_for_sms_campaigns,
            created_since: self.created_since,
            email_campaign_id: self.email_campaign_id,
            list_id: self.list_id,
            modified_since: self.modified_since,
            segment_id: self.segment_id,
            sms_campaign_id: self.sms_campaign_id,
        })
    }
}

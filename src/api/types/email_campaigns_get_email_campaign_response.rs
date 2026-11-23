pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetEmailCampaignResponse {
    /// Url of the attachment file. Only available if the campaign has an attachment.
    #[serde(rename = "attachmentFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_file: Option<String>,
    /// Status of A/B Test for the campaign. abTesting = false means it is disabled, & abTesting = true means it is enabled.
    #[serde(rename = "abTesting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_testing: Option<bool>,
    /// ID of the campaign
    #[serde(default)]
    pub id: i64,
    /// Name of the campaign
    #[serde(default)]
    pub name: String,
    /// Preview text or preheader of the email campaign
    #[serde(rename = "previewText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// UTC date-time on which campaign is scheduled (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// It is true if you have chosen to send your campaign at best time, otherwise it is false
    #[serde(rename = "sendAtBestTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at_best_time: Option<bool>,
    /// The size of your ab-test groups. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "splitRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_rule: Option<i64>,
    /// Status of the campaign
    pub status: GetEmailCampaignResponseStatus,
    /// Subject of the campaign. Only available if `abTesting` flag of the campaign is `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Subject A of the ab-test campaign. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "subjectA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_a: Option<String>,
    /// Subject B of the ab-test campaign. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "subjectB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_b: Option<String>,
    /// Type of campaign
    pub r#type: GetEmailCampaignResponseType,
    /// Criteria for the winning version. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "winnerCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner_criteria: Option<String>,
    /// The duration of the test in hours at the end of which the winning version will be sent. Only available if `abTesting` flag of the campaign is `true`
    #[serde(rename = "winnerDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner_delay: Option<i64>,
    /// URL of the attachment file associated with the campaign. Empty string if no attachment is present.
    #[serde(rename = "attachmentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// Creation UTC date-time of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// Expiration date configuration for the email campaign, if set. Contains the duration and unit of the email expiry.
    #[serde(rename = "emailExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_expiration_date: Option<GetEmailCampaignResponseEmailExpirationDate>,
    /// Footer of the campaign
    #[serde(default)]
    pub footer: String,
    /// Header of the campaign
    #[serde(default)]
    pub header: String,
    /// HTML content of the campaign
    #[serde(rename = "htmlContent")]
    #[serde(default)]
    pub html_content: String,
    /// Status of inline image. inlineImageActivation = false means image can’t be embedded, & inlineImageActivation = true means image can be embedded, in the email.
    #[serde(rename = "inlineImageActivation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_image_activation: Option<bool>,
    /// Status of mirror links in campaign. mirrorActive = false means mirror links are deactivated, & mirrorActive = true means mirror links are activated, in the campaign
    #[serde(rename = "mirrorActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_active: Option<bool>,
    /// UTC date-time of last modification of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// FOR TRIGGER ONLY ! Type of trigger campaign.recurring = false means contact can receive the same Trigger campaign only once, & recurring = true means contact can receive the same Trigger campaign several times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<bool>,
    /// Email defined as the "Reply to" of the campaign
    #[serde(rename = "replyTo")]
    #[serde(default)]
    pub reply_to: String,
    /// Total number of non-delivered campaigns for a particular campaign id.
    #[serde(rename = "returnBounce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_bounce: Option<i64>,
    #[serde(default)]
    pub sender: GetEmailCampaignResponseSender,
    /// Sent UTC date-time of the campaign (YYYY-MM-DDTHH:mm:ss.SSSZ). Only available if 'status' of the campaign is 'sent'
    #[serde(rename = "sentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_date: Option<String>,
    /// Link to share the campaign on social media. For trigger campaigns, this returns a descriptive message instead of a URL. For classic campaigns that have not been sent, this also returns a descriptive message.
    #[serde(rename = "shareLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_link: Option<String>,
    /// Tag of the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// List of tags of the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Retrieved the status of test email sending. (true=Test email has been sent  false=Test email has not been sent)
    #[serde(rename = "testSent")]
    #[serde(default)]
    pub test_sent: bool,
    /// Customisation of the "to" field of the campaign
    #[serde(rename = "toField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_field: Option<String>,
    /// The utm_campaign value applied to the campaign's tracking links. Falls back to your account's global UTM settings when no custom value was set on the campaign, or the campaign name if neither is configured. Only returned when UTM tracking is enabled on the campaign.
    #[serde(rename = "utmCampaignValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_campaign_value: Option<String>,
    /// The utm_content value applied to the campaign's tracking links. Falls back to your account's global UTM settings when no custom value was set on the campaign. Only returned when UTM tracking is enabled on the campaign and a value is set at one of these levels.
    #[serde(rename = "utmContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
    /// Legacy numeric utm_id value applied to the campaign's tracking links. Present only when UTM tracking is enabled on the campaign and the resolved utm_id is numeric (typically the campaign's ID). For customer-supplied non-numeric values (for example `promo_042`), read `utmId` (string) instead. Kept for backward compatibility.
    #[serde(rename = "utmID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_id: Option<i64>,
    /// The utm_id value applied to the campaign's tracking links, returned verbatim as a string. Falls back to your account's global UTM settings when no custom value was set on the campaign. Only returned when UTM tracking is enabled on the campaign and a value is set at one of these levels. Preferred field for new consumers — covers both numeric IDs and customer-supplied non-numeric strings.
    #[serde(rename = "utmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_id_string: Option<String>,
    /// The utm_medium value applied to the campaign's tracking links. Falls back to your account's global UTM settings when no custom value was set on the campaign, or the default `email` if neither is configured. Case is preserved verbatim. Only returned when UTM tracking is enabled on the campaign.
    #[serde(rename = "utmMedium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    /// The utm_source value applied to the campaign's tracking links. Falls back to your account's global UTM settings when no custom value was set on the campaign, or the account default (`brevo` or `sendinblue`) if neither is configured. Case is preserved verbatim. Only returned when UTM tracking is enabled on the campaign.
    #[serde(rename = "utmSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    /// The utm_term value applied to the campaign's tracking links. Falls back to your account's global UTM settings when no custom value was set on the campaign. Only returned when UTM tracking is enabled on the campaign and a value is set at one of these levels.
    #[serde(rename = "utmTerm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
    #[serde(default)]
    pub recipients: GetCampaignRecipients,
    #[serde(default)]
    pub statistics: GetExtendedCampaignStats,
}

impl GetEmailCampaignResponse {
    pub fn builder() -> GetEmailCampaignResponseBuilder {
        <GetEmailCampaignResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailCampaignResponseBuilder {
    attachment_file: Option<String>,
    ab_testing: Option<bool>,
    id: Option<i64>,
    name: Option<String>,
    preview_text: Option<String>,
    scheduled_at: Option<String>,
    send_at_best_time: Option<bool>,
    split_rule: Option<i64>,
    status: Option<GetEmailCampaignResponseStatus>,
    subject: Option<String>,
    subject_a: Option<String>,
    subject_b: Option<String>,
    r#type: Option<GetEmailCampaignResponseType>,
    winner_criteria: Option<String>,
    winner_delay: Option<i64>,
    attachment_url: Option<String>,
    created_at: Option<String>,
    email_expiration_date: Option<GetEmailCampaignResponseEmailExpirationDate>,
    footer: Option<String>,
    header: Option<String>,
    html_content: Option<String>,
    inline_image_activation: Option<bool>,
    mirror_active: Option<bool>,
    modified_at: Option<String>,
    recurring: Option<bool>,
    reply_to: Option<String>,
    return_bounce: Option<i64>,
    sender: Option<GetEmailCampaignResponseSender>,
    sent_date: Option<String>,
    share_link: Option<String>,
    tag: Option<String>,
    tags: Option<Vec<String>>,
    test_sent: Option<bool>,
    to_field: Option<String>,
    utm_campaign_value: Option<String>,
    utm_content: Option<String>,
    utm_id: Option<i64>,
    utm_id_string: Option<String>,
    utm_medium: Option<String>,
    utm_source: Option<String>,
    utm_term: Option<String>,
    recipients: Option<GetCampaignRecipients>,
    statistics: Option<GetExtendedCampaignStats>,
}

impl GetEmailCampaignResponseBuilder {
    pub fn attachment_file(mut self, value: impl Into<String>) -> Self {
        self.attachment_file = Some(value.into());
        self
    }

    pub fn ab_testing(mut self, value: bool) -> Self {
        self.ab_testing = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn preview_text(mut self, value: impl Into<String>) -> Self {
        self.preview_text = Some(value.into());
        self
    }

    pub fn scheduled_at(mut self, value: impl Into<String>) -> Self {
        self.scheduled_at = Some(value.into());
        self
    }

    pub fn send_at_best_time(mut self, value: bool) -> Self {
        self.send_at_best_time = Some(value);
        self
    }

    pub fn split_rule(mut self, value: i64) -> Self {
        self.split_rule = Some(value);
        self
    }

    pub fn status(mut self, value: GetEmailCampaignResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn subject_a(mut self, value: impl Into<String>) -> Self {
        self.subject_a = Some(value.into());
        self
    }

    pub fn subject_b(mut self, value: impl Into<String>) -> Self {
        self.subject_b = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: GetEmailCampaignResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn winner_criteria(mut self, value: impl Into<String>) -> Self {
        self.winner_criteria = Some(value.into());
        self
    }

    pub fn winner_delay(mut self, value: i64) -> Self {
        self.winner_delay = Some(value);
        self
    }

    pub fn attachment_url(mut self, value: impl Into<String>) -> Self {
        self.attachment_url = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn email_expiration_date(
        mut self,
        value: GetEmailCampaignResponseEmailExpirationDate,
    ) -> Self {
        self.email_expiration_date = Some(value);
        self
    }

    pub fn footer(mut self, value: impl Into<String>) -> Self {
        self.footer = Some(value.into());
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.header = Some(value.into());
        self
    }

    pub fn html_content(mut self, value: impl Into<String>) -> Self {
        self.html_content = Some(value.into());
        self
    }

    pub fn inline_image_activation(mut self, value: bool) -> Self {
        self.inline_image_activation = Some(value);
        self
    }

    pub fn mirror_active(mut self, value: bool) -> Self {
        self.mirror_active = Some(value);
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
        self
    }

    pub fn recurring(mut self, value: bool) -> Self {
        self.recurring = Some(value);
        self
    }

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }

    pub fn return_bounce(mut self, value: i64) -> Self {
        self.return_bounce = Some(value);
        self
    }

    pub fn sender(mut self, value: GetEmailCampaignResponseSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn sent_date(mut self, value: impl Into<String>) -> Self {
        self.sent_date = Some(value.into());
        self
    }

    pub fn share_link(mut self, value: impl Into<String>) -> Self {
        self.share_link = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn test_sent(mut self, value: bool) -> Self {
        self.test_sent = Some(value);
        self
    }

    pub fn to_field(mut self, value: impl Into<String>) -> Self {
        self.to_field = Some(value.into());
        self
    }

    pub fn utm_campaign_value(mut self, value: impl Into<String>) -> Self {
        self.utm_campaign_value = Some(value.into());
        self
    }

    pub fn utm_content(mut self, value: impl Into<String>) -> Self {
        self.utm_content = Some(value.into());
        self
    }

    pub fn utm_id(mut self, value: i64) -> Self {
        self.utm_id = Some(value);
        self
    }

    pub fn utm_id_string(mut self, value: impl Into<String>) -> Self {
        self.utm_id_string = Some(value.into());
        self
    }

    pub fn utm_medium(mut self, value: impl Into<String>) -> Self {
        self.utm_medium = Some(value.into());
        self
    }

    pub fn utm_source(mut self, value: impl Into<String>) -> Self {
        self.utm_source = Some(value.into());
        self
    }

    pub fn utm_term(mut self, value: impl Into<String>) -> Self {
        self.utm_term = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: GetCampaignRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn statistics(mut self, value: GetExtendedCampaignStats) -> Self {
        self.statistics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEmailCampaignResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetEmailCampaignResponseBuilder::id)
    /// - [`name`](GetEmailCampaignResponseBuilder::name)
    /// - [`status`](GetEmailCampaignResponseBuilder::status)
    /// - [`r#type`](GetEmailCampaignResponseBuilder::r#type)
    /// - [`created_at`](GetEmailCampaignResponseBuilder::created_at)
    /// - [`footer`](GetEmailCampaignResponseBuilder::footer)
    /// - [`header`](GetEmailCampaignResponseBuilder::header)
    /// - [`html_content`](GetEmailCampaignResponseBuilder::html_content)
    /// - [`modified_at`](GetEmailCampaignResponseBuilder::modified_at)
    /// - [`reply_to`](GetEmailCampaignResponseBuilder::reply_to)
    /// - [`sender`](GetEmailCampaignResponseBuilder::sender)
    /// - [`test_sent`](GetEmailCampaignResponseBuilder::test_sent)
    /// - [`recipients`](GetEmailCampaignResponseBuilder::recipients)
    /// - [`statistics`](GetEmailCampaignResponseBuilder::statistics)
    pub fn build(self) -> Result<GetEmailCampaignResponse, BuildError> {
        Ok(GetEmailCampaignResponse {
            attachment_file: self.attachment_file,
            ab_testing: self.ab_testing,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            preview_text: self.preview_text,
            scheduled_at: self.scheduled_at,
            send_at_best_time: self.send_at_best_time,
            split_rule: self.split_rule,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subject: self.subject,
            subject_a: self.subject_a,
            subject_b: self.subject_b,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            winner_criteria: self.winner_criteria,
            winner_delay: self.winner_delay,
            attachment_url: self.attachment_url,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            email_expiration_date: self.email_expiration_date,
            footer: self
                .footer
                .ok_or_else(|| BuildError::missing_field("footer"))?,
            header: self
                .header
                .ok_or_else(|| BuildError::missing_field("header"))?,
            html_content: self
                .html_content
                .ok_or_else(|| BuildError::missing_field("html_content"))?,
            inline_image_activation: self.inline_image_activation,
            mirror_active: self.mirror_active,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            recurring: self.recurring,
            reply_to: self
                .reply_to
                .ok_or_else(|| BuildError::missing_field("reply_to"))?,
            return_bounce: self.return_bounce,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            sent_date: self.sent_date,
            share_link: self.share_link,
            tag: self.tag,
            tags: self.tags,
            test_sent: self
                .test_sent
                .ok_or_else(|| BuildError::missing_field("test_sent"))?,
            to_field: self.to_field,
            utm_campaign_value: self.utm_campaign_value,
            utm_content: self.utm_content,
            utm_id: self.utm_id,
            utm_id_string: self.utm_id_string,
            utm_medium: self.utm_medium,
            utm_source: self.utm_source,
            utm_term: self.utm_term,
            recipients: self
                .recipients
                .ok_or_else(|| BuildError::missing_field("recipients"))?,
            statistics: self
                .statistics
                .ok_or_else(|| BuildError::missing_field("statistics"))?,
        })
    }
}

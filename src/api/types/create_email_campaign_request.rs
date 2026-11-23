pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateEmailCampaignRequest {
    /// Status of A/B Test. abTesting = false means it is disabled & abTesting = true means it is enabled. **subjectA, subjectB, splitRule, winnerCriteria & winnerDelay** will be considered when abTesting is set to true. subjectA & subjectB are mandatory together & subject if passed is ignored. **Can be set to true only if sendAtBestTime is false**. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B
    #[serde(rename = "abTesting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_testing: Option<bool>,
    /// Absolute url of the attachment (no local file). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    #[serde(rename = "attachmentUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    /// To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy. Learn more about setting an email expiration date. For reference , ``https://help.brevo.com/hc/en-us/articles/4413566705298-Create-an-email-campaign``
    #[serde(rename = "emailExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_expiration_date: Option<CreateEmailCampaignRequestEmailExpirationDate>,
    /// Footer of the email campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    /// Header of the email campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// **Mandatory if htmlUrl and templateId are empty.** Body of the message (HTML). Must have more than 10 characters and be less than 1MB in size. Cannot be used together with `htmlUrl` or `templateId`.
    #[serde(rename = "htmlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_content: Option<String>,
    /// **Mandatory if htmlContent and templateId are empty.** URL to the message (HTML). Cannot be used together with `htmlContent` or `templateId`. For example: **https://html.domain.com**
    #[serde(rename = "htmlUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    /// **Mandatory if ipWarmupEnable is set to true**. Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%.
    #[serde(rename = "increaseRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increase_rate: Option<i64>,
    /// **Mandatory if ipWarmupEnable is set to true**. Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000.
    #[serde(rename = "initialQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_quota: Option<i64>,
    /// Use true to embedded the images in your email. Final size of the email should be less than **4MB**. Campaigns with embedded images can _not be sent to more than 5000 contacts_
    #[serde(rename = "inlineImageActivation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_image_activation: Option<bool>,
    /// **Available for dedicated ip clients**. Set this to true if you wish to warm up your ip.
    #[serde(rename = "ipWarmupEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_warmup_enable: Option<bool>,
    /// Use true to enable the mirror link
    #[serde(rename = "mirrorActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_active: Option<bool>,
    /// Name of the campaign
    #[serde(default)]
    pub name: String,
    /// Pass the set of attributes to customize the type classic campaign. For example: **{"FNAME":"Joe", "LNAME":"Doe"}**. Only available if **type** is **classic**. It's considered only if campaign is in _New Template Language format_. The New Template Language is dependent on the values of **subject, htmlContent/htmlUrl, sender.name & toField**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,
    /// Preview text or preheader of the email campaign
    #[serde(rename = "previewText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// Segment ids and List ids to include/exclude from campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<CreateEmailCampaignRequestRecipients>,
    /// Email on which the campaign recipients will be able to reply to
    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result**. If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part). For example: **2017-06-01T12:30:00+02:00**
    #[serde(rename = "scheduledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    /// Set this to true if you want to send your campaign at best time.
    #[serde(rename = "sendAtBestTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at_best_time: Option<bool>,
    /// Sender details including id or email and name (optional). Only one of either Sender’s email or Sender’s ID shall be passed in one request at a time. Passing both `email` and `id` will result in an error. For example: **{"name":"xyz", "email":"example@abc.com"}** or **{"name":"xyz", "id":123}**
    #[serde(default)]
    pub sender: CreateEmailCampaignRequestSender,
    /// Add the size of your test groups. **Mandatory if abTesting = true & 'recipients' is passed**. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else
    #[serde(rename = "splitRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_rule: Option<i64>,
    /// Subject of the campaign. **Mandatory if abTesting is false**. Ignored if abTesting is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Subject A of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_a: Option<String>,
    /// Subject B of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    #[serde(rename = "subjectB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_b: Option<String>,
    /// Tag of the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// **Mandatory if htmlContent and htmlUrl are empty.** Id of the transactional email template with status _active_. Used to copy only its content fetched from htmlContent/htmlUrl to an email campaign for RSS feature. Cannot be used together with `htmlContent` or `htmlUrl`.
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    #[serde(rename = "toField")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_field: Option<String>,
    /// Enter an unsubscription page id. The page id is a 24 digit alphanumeric id that can be found in the URL when editing the page. If not entered, then the default unsubscription page will be used.
    #[serde(rename = "unsubscriptionPageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscription_page_id: Option<String>,
    /// **Mandatory if templateId is used containing the {{ update_profile }} tag**. Enter an update profile form id. The form id is a 24 digit alphanumeric id that can be found in the URL when editing the form. If not entered, then the default update profile form will be used.
    #[serde(rename = "updateFormId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_form_id: Option<String>,
    /// Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed
    #[serde(rename = "utmCampaign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    /// Customize the utm_content value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    #[serde(rename = "utmContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
    /// Customize the utm_id value. Appears on outgoing tracking links alongside utm_campaign. When omitted or empty, the utm_id entry from the account's global utm_settings is used if enabled; otherwise no utm_id parameter is emitted.
    #[serde(rename = "utmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_id: Option<String>,
    /// Customize the utm_medium value. When omitted or empty, the utm_medium entry from the account's global utm_settings is used if set; otherwise the default `email` is used.
    #[serde(rename = "utmMedium")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    /// Customize the utm_source value. When omitted or empty, the utm_source entry from the account's global utm_settings is used if set; otherwise the account default (`brevo` or `sendinblue`) is used.
    #[serde(rename = "utmSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    /// Customize the utm_term value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    #[serde(rename = "utmTerm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
    /// Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
    #[serde(rename = "winnerCriteria")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner_criteria: Option<CreateEmailCampaignRequestWinnerCriteria>,
    /// Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerDelay` is ignored if passed
    #[serde(rename = "winnerDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner_delay: Option<i64>,
}

impl CreateEmailCampaignRequest {
    pub fn builder() -> CreateEmailCampaignRequestBuilder {
        <CreateEmailCampaignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEmailCampaignRequestBuilder {
    ab_testing: Option<bool>,
    attachment_url: Option<String>,
    email_expiration_date: Option<CreateEmailCampaignRequestEmailExpirationDate>,
    footer: Option<String>,
    header: Option<String>,
    html_content: Option<String>,
    html_url: Option<String>,
    increase_rate: Option<i64>,
    initial_quota: Option<i64>,
    inline_image_activation: Option<bool>,
    ip_warmup_enable: Option<bool>,
    mirror_active: Option<bool>,
    name: Option<String>,
    params: Option<HashMap<String, serde_json::Value>>,
    preview_text: Option<String>,
    recipients: Option<CreateEmailCampaignRequestRecipients>,
    reply_to: Option<String>,
    scheduled_at: Option<String>,
    send_at_best_time: Option<bool>,
    sender: Option<CreateEmailCampaignRequestSender>,
    split_rule: Option<i64>,
    subject: Option<String>,
    subject_a: Option<String>,
    subject_b: Option<String>,
    tag: Option<String>,
    template_id: Option<i64>,
    to_field: Option<String>,
    unsubscription_page_id: Option<String>,
    update_form_id: Option<String>,
    utm_campaign: Option<String>,
    utm_content: Option<String>,
    utm_id: Option<String>,
    utm_medium: Option<String>,
    utm_source: Option<String>,
    utm_term: Option<String>,
    winner_criteria: Option<CreateEmailCampaignRequestWinnerCriteria>,
    winner_delay: Option<i64>,
}

impl CreateEmailCampaignRequestBuilder {
    pub fn ab_testing(mut self, value: bool) -> Self {
        self.ab_testing = Some(value);
        self
    }

    pub fn attachment_url(mut self, value: impl Into<String>) -> Self {
        self.attachment_url = Some(value.into());
        self
    }

    pub fn email_expiration_date(
        mut self,
        value: CreateEmailCampaignRequestEmailExpirationDate,
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

    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.html_url = Some(value.into());
        self
    }

    pub fn increase_rate(mut self, value: i64) -> Self {
        self.increase_rate = Some(value);
        self
    }

    pub fn initial_quota(mut self, value: i64) -> Self {
        self.initial_quota = Some(value);
        self
    }

    pub fn inline_image_activation(mut self, value: bool) -> Self {
        self.inline_image_activation = Some(value);
        self
    }

    pub fn ip_warmup_enable(mut self, value: bool) -> Self {
        self.ip_warmup_enable = Some(value);
        self
    }

    pub fn mirror_active(mut self, value: bool) -> Self {
        self.mirror_active = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(value);
        self
    }

    pub fn preview_text(mut self, value: impl Into<String>) -> Self {
        self.preview_text = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: CreateEmailCampaignRequestRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
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

    pub fn sender(mut self, value: CreateEmailCampaignRequestSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn split_rule(mut self, value: i64) -> Self {
        self.split_rule = Some(value);
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

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn to_field(mut self, value: impl Into<String>) -> Self {
        self.to_field = Some(value.into());
        self
    }

    pub fn unsubscription_page_id(mut self, value: impl Into<String>) -> Self {
        self.unsubscription_page_id = Some(value.into());
        self
    }

    pub fn update_form_id(mut self, value: impl Into<String>) -> Self {
        self.update_form_id = Some(value.into());
        self
    }

    pub fn utm_campaign(mut self, value: impl Into<String>) -> Self {
        self.utm_campaign = Some(value.into());
        self
    }

    pub fn utm_content(mut self, value: impl Into<String>) -> Self {
        self.utm_content = Some(value.into());
        self
    }

    pub fn utm_id(mut self, value: impl Into<String>) -> Self {
        self.utm_id = Some(value.into());
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

    pub fn winner_criteria(mut self, value: CreateEmailCampaignRequestWinnerCriteria) -> Self {
        self.winner_criteria = Some(value);
        self
    }

    pub fn winner_delay(mut self, value: i64) -> Self {
        self.winner_delay = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEmailCampaignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateEmailCampaignRequestBuilder::name)
    /// - [`sender`](CreateEmailCampaignRequestBuilder::sender)
    pub fn build(self) -> Result<CreateEmailCampaignRequest, BuildError> {
        Ok(CreateEmailCampaignRequest {
            ab_testing: self.ab_testing,
            attachment_url: self.attachment_url,
            email_expiration_date: self.email_expiration_date,
            footer: self.footer,
            header: self.header,
            html_content: self.html_content,
            html_url: self.html_url,
            increase_rate: self.increase_rate,
            initial_quota: self.initial_quota,
            inline_image_activation: self.inline_image_activation,
            ip_warmup_enable: self.ip_warmup_enable,
            mirror_active: self.mirror_active,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            params: self.params,
            preview_text: self.preview_text,
            recipients: self.recipients,
            reply_to: self.reply_to,
            scheduled_at: self.scheduled_at,
            send_at_best_time: self.send_at_best_time,
            sender: self
                .sender
                .ok_or_else(|| BuildError::missing_field("sender"))?,
            split_rule: self.split_rule,
            subject: self.subject,
            subject_a: self.subject_a,
            subject_b: self.subject_b,
            tag: self.tag,
            template_id: self.template_id,
            to_field: self.to_field,
            unsubscription_page_id: self.unsubscription_page_id,
            update_form_id: self.update_form_id,
            utm_campaign: self.utm_campaign,
            utm_content: self.utm_content,
            utm_id: self.utm_id,
            utm_medium: self.utm_medium,
            utm_source: self.utm_source,
            utm_term: self.utm_term,
            winner_criteria: self.winner_criteria,
            winner_delay: self.winner_delay,
        })
    }
}

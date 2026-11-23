pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDoiContactRequest {
    /// Pass the set of attributes and their values. **These attributes must be present in your Brevo account**. For eg. **{'FNAME':'Elly', 'LNAME':'Roger', 'COUNTRIES': ['India','China']}**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, CreateDoiContactRequestAttributesValue>>,
    /// Consent of the DOI recipient for open (pixel) and click tracking in the double opt-in confirmation email, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(rename = "contactPixelTrackingConsent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// Email address where the confirmation email will be sent. This email address will be the identifier for all other contact attributes.
    #[serde(default)]
    pub email: String,
    /// Lists under user account where contact should not be added
    #[serde(rename = "excludeListIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_list_ids: Option<Vec<i64>>,
    /// Lists under user account where contact should be added
    #[serde(rename = "includeListIds")]
    #[serde(default)]
    pub include_list_ids: Vec<i64>,
    /// URL of the web page that user will be redirected to after clicking on the double opt in URL. When editing your DOI template you can reference this URL by using the tag **{{ params.DOIurl }}**.
    #[serde(rename = "redirectionUrl")]
    #[serde(default)]
    pub redirection_url: String,
    /// Id of the Double opt-in (DOI) template
    #[serde(rename = "templateId")]
    #[serde(default)]
    pub template_id: i64,
}

impl CreateDoiContactRequest {
    pub fn builder() -> CreateDoiContactRequestBuilder {
        <CreateDoiContactRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDoiContactRequestBuilder {
    attributes: Option<HashMap<String, CreateDoiContactRequestAttributesValue>>,
    contact_pixel_tracking_consent: Option<bool>,
    email: Option<String>,
    exclude_list_ids: Option<Vec<i64>>,
    include_list_ids: Option<Vec<i64>>,
    redirection_url: Option<String>,
    template_id: Option<i64>,
}

impl CreateDoiContactRequestBuilder {
    pub fn attributes(
        mut self,
        value: HashMap<String, CreateDoiContactRequestAttributesValue>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn contact_pixel_tracking_consent(mut self, value: bool) -> Self {
        self.contact_pixel_tracking_consent = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn exclude_list_ids(mut self, value: Vec<i64>) -> Self {
        self.exclude_list_ids = Some(value);
        self
    }

    pub fn include_list_ids(mut self, value: Vec<i64>) -> Self {
        self.include_list_ids = Some(value);
        self
    }

    pub fn redirection_url(mut self, value: impl Into<String>) -> Self {
        self.redirection_url = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDoiContactRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](CreateDoiContactRequestBuilder::email)
    /// - [`include_list_ids`](CreateDoiContactRequestBuilder::include_list_ids)
    /// - [`redirection_url`](CreateDoiContactRequestBuilder::redirection_url)
    /// - [`template_id`](CreateDoiContactRequestBuilder::template_id)
    pub fn build(self) -> Result<CreateDoiContactRequest, BuildError> {
        Ok(CreateDoiContactRequest {
            attributes: self.attributes,
            contact_pixel_tracking_consent: self.contact_pixel_tracking_consent,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            exclude_list_ids: self.exclude_list_ids,
            include_list_ids: self
                .include_list_ids
                .ok_or_else(|| BuildError::missing_field("include_list_ids"))?,
            redirection_url: self
                .redirection_url
                .ok_or_else(|| BuildError::missing_field("redirection_url"))?,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
        })
    }
}

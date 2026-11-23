pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendTransacEmailRequestToItem {
    /// Consent of the recipient for open (pixel) and click tracking, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    #[serde(rename = "contactPixelTrackingConsent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_pixel_tracking_consent: Option<bool>,
    /// Email address of the recipient
    #[serde(default)]
    pub email: String,
    /// Display name of the recipient. Maximum length is 70 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Pass `true` to send the recipient no open pixel, so no open is recorded for them, not even in aggregate statistics. Only available to Enterprise plans: for other accounts the field is accepted but ignored. Considered only if the recipient is not tracked: `contactPixelTrackingConsent` is `false`, or not passed and your account does not track unknown-consent contacts. Click tracking is unchanged. If `false` or not passed, the open pixel is added as usual. A value other than `true`/`false` is rejected.
    #[serde(rename = "removeTrackingPixel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tracking_pixel: Option<bool>,
}

impl SendTransacEmailRequestToItem {
    pub fn builder() -> SendTransacEmailRequestToItemBuilder {
        <SendTransacEmailRequestToItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendTransacEmailRequestToItemBuilder {
    contact_pixel_tracking_consent: Option<bool>,
    email: Option<String>,
    name: Option<String>,
    remove_tracking_pixel: Option<bool>,
}

impl SendTransacEmailRequestToItemBuilder {
    pub fn contact_pixel_tracking_consent(mut self, value: bool) -> Self {
        self.contact_pixel_tracking_consent = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn remove_tracking_pixel(mut self, value: bool) -> Self {
        self.remove_tracking_pixel = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendTransacEmailRequestToItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](SendTransacEmailRequestToItemBuilder::email)
    pub fn build(self) -> Result<SendTransacEmailRequestToItem, BuildError> {
        Ok(SendTransacEmailRequestToItem {
            contact_pixel_tracking_consent: self.contact_pixel_tracking_consent,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            remove_tracking_pixel: self.remove_tracking_pixel,
        })
    }
}

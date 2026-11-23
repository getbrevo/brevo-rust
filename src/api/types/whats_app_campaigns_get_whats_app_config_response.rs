pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWhatsAppConfigResponse {
    /// Verification status information of the Business account
    #[serde(rename = "businessStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_status: Option<String>,
    /// Status of the name associated with WhatsApp Phone number
    #[serde(rename = "phoneNumberNameStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_name_status: Option<GetWhatsAppConfigResponsePhoneNumberNameStatus>,
    /// Quality status of phone number associated with WhatsApp account. There are three quality ratings. example - **High (GREEN) , Medium (YELLOW) and Low(RED)**
    #[serde(rename = "phoneNumberQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_quality: Option<GetWhatsAppConfigResponsePhoneNumberQuality>,
    /// Sending limit of the WhatsApp account
    #[serde(rename = "sendingLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_limit: Option<String>,
    /// Id of the WhatsApp business account
    #[serde(rename = "whatsappBusinessAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_business_account_id: Option<String>,
    /// Status information related to WhatsApp Api account
    #[serde(rename = "whatsappBusinessAccountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_business_account_status:
        Option<GetWhatsAppConfigResponseWhatsappBusinessAccountStatus>,
}

impl GetWhatsAppConfigResponse {
    pub fn builder() -> GetWhatsAppConfigResponseBuilder {
        <GetWhatsAppConfigResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWhatsAppConfigResponseBuilder {
    business_status: Option<String>,
    phone_number_name_status: Option<GetWhatsAppConfigResponsePhoneNumberNameStatus>,
    phone_number_quality: Option<GetWhatsAppConfigResponsePhoneNumberQuality>,
    sending_limit: Option<String>,
    whatsapp_business_account_id: Option<String>,
    whatsapp_business_account_status:
        Option<GetWhatsAppConfigResponseWhatsappBusinessAccountStatus>,
}

impl GetWhatsAppConfigResponseBuilder {
    pub fn business_status(mut self, value: impl Into<String>) -> Self {
        self.business_status = Some(value.into());
        self
    }

    pub fn phone_number_name_status(
        mut self,
        value: GetWhatsAppConfigResponsePhoneNumberNameStatus,
    ) -> Self {
        self.phone_number_name_status = Some(value);
        self
    }

    pub fn phone_number_quality(
        mut self,
        value: GetWhatsAppConfigResponsePhoneNumberQuality,
    ) -> Self {
        self.phone_number_quality = Some(value);
        self
    }

    pub fn sending_limit(mut self, value: impl Into<String>) -> Self {
        self.sending_limit = Some(value.into());
        self
    }

    pub fn whatsapp_business_account_id(mut self, value: impl Into<String>) -> Self {
        self.whatsapp_business_account_id = Some(value.into());
        self
    }

    pub fn whatsapp_business_account_status(
        mut self,
        value: GetWhatsAppConfigResponseWhatsappBusinessAccountStatus,
    ) -> Self {
        self.whatsapp_business_account_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWhatsAppConfigResponse`].
    pub fn build(self) -> Result<GetWhatsAppConfigResponse, BuildError> {
        Ok(GetWhatsAppConfigResponse {
            business_status: self.business_status,
            phone_number_name_status: self.phone_number_name_status,
            phone_number_quality: self.phone_number_quality,
            sending_limit: self.sending_limit,
            whatsapp_business_account_id: self.whatsapp_business_account_id,
            whatsapp_business_account_status: self.whatsapp_business_account_status,
        })
    }
}

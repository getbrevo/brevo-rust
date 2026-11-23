pub use crate::prelude::*;

/// Credit details to update
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PutCorporateSubAccountsPlanRequestCredits {
    /// Number of email credits | Pass the value -1 for unlimited
    /// emails in ENTv2 only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<i64>,
    /// Number of externalFeeds credits, possible values are 0 or 1
    /// |available in ENTv2 only
    #[serde(rename = "externalFeeds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_feeds: Option<i64>,
    /// Number of SMS credits | Pass the value -1 for unlimited SMS
    /// in ENTv2 only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sms: Option<f64>,
    /// Number of whatsapp credits | Pass the value -1 for unlimited
    /// whatsapp in ENTv2 only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub whatsapp: Option<f64>,
    /// Number of Push credits, possible value is 0 and -1 |
    /// available in ENT-v2 only
    #[serde(rename = "wpSubscribers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wp_subscribers: Option<i64>,
}

impl PutCorporateSubAccountsPlanRequestCredits {
    pub fn builder() -> PutCorporateSubAccountsPlanRequestCreditsBuilder {
        <PutCorporateSubAccountsPlanRequestCreditsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateSubAccountsPlanRequestCreditsBuilder {
    email: Option<i64>,
    external_feeds: Option<i64>,
    sms: Option<f64>,
    whatsapp: Option<f64>,
    wp_subscribers: Option<i64>,
}

impl PutCorporateSubAccountsPlanRequestCreditsBuilder {
    pub fn email(mut self, value: i64) -> Self {
        self.email = Some(value);
        self
    }

    pub fn external_feeds(mut self, value: i64) -> Self {
        self.external_feeds = Some(value);
        self
    }

    pub fn sms(mut self, value: f64) -> Self {
        self.sms = Some(value);
        self
    }

    pub fn whatsapp(mut self, value: f64) -> Self {
        self.whatsapp = Some(value);
        self
    }

    pub fn wp_subscribers(mut self, value: i64) -> Self {
        self.wp_subscribers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateSubAccountsPlanRequestCredits`].
    pub fn build(self) -> Result<PutCorporateSubAccountsPlanRequestCredits, BuildError> {
        Ok(PutCorporateSubAccountsPlanRequestCredits {
            email: self.email,
            external_feeds: self.external_feeds,
            sms: self.sms,
            whatsapp: self.whatsapp,
            wp_subscribers: self.wp_subscribers,
        })
    }
}

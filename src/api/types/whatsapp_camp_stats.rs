pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WhatsappCampStats {
    #[serde(default)]
    pub delivered: i64,
    #[serde(rename = "notSent")]
    #[serde(default)]
    pub not_sent: i64,
    #[serde(default)]
    pub read: i64,
    #[serde(default)]
    pub sent: i64,
    #[serde(default)]
    pub unsubscribe: i64,
}

impl WhatsappCampStats {
    pub fn builder() -> WhatsappCampStatsBuilder {
        <WhatsappCampStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WhatsappCampStatsBuilder {
    delivered: Option<i64>,
    not_sent: Option<i64>,
    read: Option<i64>,
    sent: Option<i64>,
    unsubscribe: Option<i64>,
}

impl WhatsappCampStatsBuilder {
    pub fn delivered(mut self, value: i64) -> Self {
        self.delivered = Some(value);
        self
    }

    pub fn not_sent(mut self, value: i64) -> Self {
        self.not_sent = Some(value);
        self
    }

    pub fn read(mut self, value: i64) -> Self {
        self.read = Some(value);
        self
    }

    pub fn sent(mut self, value: i64) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn unsubscribe(mut self, value: i64) -> Self {
        self.unsubscribe = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WhatsappCampStats`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivered`](WhatsappCampStatsBuilder::delivered)
    /// - [`not_sent`](WhatsappCampStatsBuilder::not_sent)
    /// - [`read`](WhatsappCampStatsBuilder::read)
    /// - [`sent`](WhatsappCampStatsBuilder::sent)
    /// - [`unsubscribe`](WhatsappCampStatsBuilder::unsubscribe)
    pub fn build(self) -> Result<WhatsappCampStats, BuildError> {
        Ok(WhatsappCampStats {
            delivered: self
                .delivered
                .ok_or_else(|| BuildError::missing_field("delivered"))?,
            not_sent: self
                .not_sent
                .ok_or_else(|| BuildError::missing_field("not_sent"))?,
            read: self.read.ok_or_else(|| BuildError::missing_field("read"))?,
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
            unsubscribe: self
                .unsubscribe
                .ok_or_else(|| BuildError::missing_field("unsubscribe"))?,
        })
    }
}

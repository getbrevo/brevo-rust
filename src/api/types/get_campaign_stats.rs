pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetCampaignStats {
    /// Numbers of times your email has been opened automatically through Apple MPP.
    #[serde(rename = "appleMppOpens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_mpp_opens: Option<i64>,
    /// Number of total clicks for the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickers: Option<i64>,
    /// Number of complaints (Spam reports) for the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaints: Option<i64>,
    /// Number of deferred emails for the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred: Option<i64>,
    /// Number of delivered emails for the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// Rate of recipients without any privacy protection option enabled in their email client, applied to all delivered emails
    #[serde(rename = "estimatedViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_views: Option<i64>,
    /// Number of hard bounces for the campaign
    #[serde(rename = "hardBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_bounces: Option<i64>,
    /// List Id of email campaign (only in case of get email campaign(s)(not for global stats))
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i64>,
    /// Percentage of recipients who open the email out of your total number of recipients. Depending on your Campaign settings, they may include Apple MPP opens.
    #[serde(rename = "opensRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub opens_rate: Option<f64>,
    /// Total number of non-delivered campaigns for a particular campaign id.
    #[serde(rename = "returnBounce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_bounce: Option<i64>,
    /// Number of sent emails for the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<i64>,
    /// Number of softbounce for the campaign
    #[serde(rename = "softBounces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_bounces: Option<i64>,
    /// Recipients without any privacy protection option enabled in their email email client
    #[serde(rename = "trackableViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackable_views: Option<i64>,
    /// Rate of recipients without any privacy protection option enabled in their email client
    #[serde(rename = "trackableViewsRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub trackable_views_rate: Option<f64>,
    /// Number of unique clicks for the campaign
    #[serde(rename = "uniqueClicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i64>,
    /// Number of unique openings for the campaign
    #[serde(rename = "uniqueViews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_views: Option<i64>,
    /// Number of unsubscription for the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscriptions: Option<i64>,
    /// Number of openings for the campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed: Option<i64>,
}

impl GetCampaignStats {
    pub fn builder() -> GetCampaignStatsBuilder {
        <GetCampaignStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCampaignStatsBuilder {
    apple_mpp_opens: Option<i64>,
    clickers: Option<i64>,
    complaints: Option<i64>,
    deferred: Option<i64>,
    delivered: Option<i64>,
    estimated_views: Option<i64>,
    hard_bounces: Option<i64>,
    list_id: Option<i64>,
    opens_rate: Option<f64>,
    return_bounce: Option<i64>,
    sent: Option<i64>,
    soft_bounces: Option<i64>,
    trackable_views: Option<i64>,
    trackable_views_rate: Option<f64>,
    unique_clicks: Option<i64>,
    unique_views: Option<i64>,
    unsubscriptions: Option<i64>,
    viewed: Option<i64>,
}

impl GetCampaignStatsBuilder {
    pub fn apple_mpp_opens(mut self, value: i64) -> Self {
        self.apple_mpp_opens = Some(value);
        self
    }

    pub fn clickers(mut self, value: i64) -> Self {
        self.clickers = Some(value);
        self
    }

    pub fn complaints(mut self, value: i64) -> Self {
        self.complaints = Some(value);
        self
    }

    pub fn deferred(mut self, value: i64) -> Self {
        self.deferred = Some(value);
        self
    }

    pub fn delivered(mut self, value: i64) -> Self {
        self.delivered = Some(value);
        self
    }

    pub fn estimated_views(mut self, value: i64) -> Self {
        self.estimated_views = Some(value);
        self
    }

    pub fn hard_bounces(mut self, value: i64) -> Self {
        self.hard_bounces = Some(value);
        self
    }

    pub fn list_id(mut self, value: i64) -> Self {
        self.list_id = Some(value);
        self
    }

    pub fn opens_rate(mut self, value: f64) -> Self {
        self.opens_rate = Some(value);
        self
    }

    pub fn return_bounce(mut self, value: i64) -> Self {
        self.return_bounce = Some(value);
        self
    }

    pub fn sent(mut self, value: i64) -> Self {
        self.sent = Some(value);
        self
    }

    pub fn soft_bounces(mut self, value: i64) -> Self {
        self.soft_bounces = Some(value);
        self
    }

    pub fn trackable_views(mut self, value: i64) -> Self {
        self.trackable_views = Some(value);
        self
    }

    pub fn trackable_views_rate(mut self, value: f64) -> Self {
        self.trackable_views_rate = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: i64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    pub fn unique_views(mut self, value: i64) -> Self {
        self.unique_views = Some(value);
        self
    }

    pub fn unsubscriptions(mut self, value: i64) -> Self {
        self.unsubscriptions = Some(value);
        self
    }

    pub fn viewed(mut self, value: i64) -> Self {
        self.viewed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCampaignStats`].
    pub fn build(self) -> Result<GetCampaignStats, BuildError> {
        Ok(GetCampaignStats {
            apple_mpp_opens: self.apple_mpp_opens,
            clickers: self.clickers,
            complaints: self.complaints,
            deferred: self.deferred,
            delivered: self.delivered,
            estimated_views: self.estimated_views,
            hard_bounces: self.hard_bounces,
            list_id: self.list_id,
            opens_rate: self.opens_rate,
            return_bounce: self.return_bounce,
            sent: self.sent,
            soft_bounces: self.soft_bounces,
            trackable_views: self.trackable_views,
            trackable_views_rate: self.trackable_views_rate,
            unique_clicks: self.unique_clicks,
            unique_views: self.unique_views,
            unsubscriptions: self.unsubscriptions,
            viewed: self.viewed,
        })
    }
}

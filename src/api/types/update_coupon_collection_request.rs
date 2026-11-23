pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCouponCollectionRequest {
    /// A default coupon to be used in case there are no coupons left
    #[serde(rename = "defaultCoupon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_coupon: Option<String>,
    /// Specify an expiration date for the coupon collection in RFC3339 format. Use null to remove the expiration date.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiration_date: Option<DateTime<FixedOffset>>,
    /// Send a notification alert (email) when the remaining coupons count is equal or fall bellow this number. Use null to disable alerts.
    #[serde(rename = "remainingCouponsAlert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_coupons_alert: Option<i64>,
    /// Send a notification alert (email) when the remaining days until the expiration date are equal or fall bellow this number. Use null to disable alerts.
    #[serde(rename = "remainingDaysAlert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_days_alert: Option<i64>,
}

impl UpdateCouponCollectionRequest {
    pub fn builder() -> UpdateCouponCollectionRequestBuilder {
        <UpdateCouponCollectionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCouponCollectionRequestBuilder {
    default_coupon: Option<String>,
    expiration_date: Option<DateTime<FixedOffset>>,
    remaining_coupons_alert: Option<i64>,
    remaining_days_alert: Option<i64>,
}

impl UpdateCouponCollectionRequestBuilder {
    pub fn default_coupon(mut self, value: impl Into<String>) -> Self {
        self.default_coupon = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn remaining_coupons_alert(mut self, value: i64) -> Self {
        self.remaining_coupons_alert = Some(value);
        self
    }

    pub fn remaining_days_alert(mut self, value: i64) -> Self {
        self.remaining_days_alert = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCouponCollectionRequest`].
    pub fn build(self) -> Result<UpdateCouponCollectionRequest, BuildError> {
        Ok(UpdateCouponCollectionRequest {
            default_coupon: self.default_coupon,
            expiration_date: self.expiration_date,
            remaining_coupons_alert: self.remaining_coupons_alert,
            remaining_days_alert: self.remaining_days_alert,
        })
    }
}

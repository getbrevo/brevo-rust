pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCouponCollectionRequest {
    /// Default coupons collection name
    #[serde(rename = "defaultCoupon")]
    #[serde(default)]
    pub default_coupon: String,
    /// Specify an expiration date for the coupon collection in RFC3339 format. Use null to remove the expiration date.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiration_date: Option<DateTime<FixedOffset>>,
    /// Name of the coupons collection
    #[serde(default)]
    pub name: String,
    /// Send a notification alert (email) when the remaining coupons count is equal or fall bellow this number. Use null to disable alerts.
    #[serde(rename = "remainingCouponsAlert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_coupons_alert: Option<i64>,
    /// Send a notification alert (email) when the remaining days until the expiration date are equal or fall bellow this number. Use null to disable alerts.
    #[serde(rename = "remainingDaysAlert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_days_alert: Option<i64>,
}

impl CreateCouponCollectionRequest {
    pub fn builder() -> CreateCouponCollectionRequestBuilder {
        <CreateCouponCollectionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCouponCollectionRequestBuilder {
    default_coupon: Option<String>,
    expiration_date: Option<DateTime<FixedOffset>>,
    name: Option<String>,
    remaining_coupons_alert: Option<i64>,
    remaining_days_alert: Option<i64>,
}

impl CreateCouponCollectionRequestBuilder {
    pub fn default_coupon(mut self, value: impl Into<String>) -> Self {
        self.default_coupon = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateCouponCollectionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_coupon`](CreateCouponCollectionRequestBuilder::default_coupon)
    /// - [`name`](CreateCouponCollectionRequestBuilder::name)
    pub fn build(self) -> Result<CreateCouponCollectionRequest, BuildError> {
        Ok(CreateCouponCollectionRequest {
            default_coupon: self
                .default_coupon
                .ok_or_else(|| BuildError::missing_field("default_coupon"))?,
            expiration_date: self.expiration_date,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            remaining_coupons_alert: self.remaining_coupons_alert,
            remaining_days_alert: self.remaining_days_alert,
        })
    }
}

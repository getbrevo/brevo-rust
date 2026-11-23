pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCouponCollection {
    /// Datetime on which the collection was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The default coupon of the collection.
    #[serde(rename = "defaultCoupon")]
    #[serde(default)]
    pub default_coupon: String,
    /// Expiration date for the coupon collection in RFC3339 format.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub expiration_date: Option<DateTime<FixedOffset>>,
    /// The id of the collection.
    #[serde(default)]
    pub id: String,
    /// The name of the collection.
    #[serde(default)]
    pub name: String,
    /// Number of coupons that have not been sent yet.
    #[serde(rename = "remainingCoupons")]
    #[serde(default)]
    pub remaining_coupons: i64,
    /// If present, an email notification is going to be sent when the total number of available coupons falls below the defined threshold.
    #[serde(rename = "remainingCouponsAlert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_coupons_alert: Option<i64>,
    /// If present, an email notification is going to be sent the defined amount of days before to the expiration date.
    #[serde(rename = "remainingDaysAlert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_days_alert: Option<i64>,
    /// Total number of coupons in the collection.
    #[serde(rename = "totalCoupons")]
    #[serde(default)]
    pub total_coupons: i64,
}

impl GetCouponCollection {
    pub fn builder() -> GetCouponCollectionBuilder {
        <GetCouponCollectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCouponCollectionBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    default_coupon: Option<String>,
    expiration_date: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    name: Option<String>,
    remaining_coupons: Option<i64>,
    remaining_coupons_alert: Option<i64>,
    remaining_days_alert: Option<i64>,
    total_coupons: Option<i64>,
}

impl GetCouponCollectionBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn default_coupon(mut self, value: impl Into<String>) -> Self {
        self.default_coupon = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn remaining_coupons(mut self, value: i64) -> Self {
        self.remaining_coupons = Some(value);
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

    pub fn total_coupons(mut self, value: i64) -> Self {
        self.total_coupons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCouponCollection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetCouponCollectionBuilder::created_at)
    /// - [`default_coupon`](GetCouponCollectionBuilder::default_coupon)
    /// - [`id`](GetCouponCollectionBuilder::id)
    /// - [`name`](GetCouponCollectionBuilder::name)
    /// - [`remaining_coupons`](GetCouponCollectionBuilder::remaining_coupons)
    /// - [`total_coupons`](GetCouponCollectionBuilder::total_coupons)
    pub fn build(self) -> Result<GetCouponCollection, BuildError> {
        Ok(GetCouponCollection {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            default_coupon: self
                .default_coupon
                .ok_or_else(|| BuildError::missing_field("default_coupon"))?,
            expiration_date: self.expiration_date,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            remaining_coupons: self
                .remaining_coupons
                .ok_or_else(|| BuildError::missing_field("remaining_coupons"))?,
            remaining_coupons_alert: self.remaining_coupons_alert,
            remaining_days_alert: self.remaining_days_alert,
            total_coupons: self
                .total_coupons
                .ok_or_else(|| BuildError::missing_field("total_coupons"))?,
        })
    }
}

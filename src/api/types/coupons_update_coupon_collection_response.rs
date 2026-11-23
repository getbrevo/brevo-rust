pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCouponCollectionResponse {
    /// The default coupon of the collection
    #[serde(rename = "defaultCoupon")]
    #[serde(default)]
    pub default_coupon: String,
    /// The id of the collection
    #[serde(default)]
    pub id: String,
    /// The name of the collection
    #[serde(default)]
    pub name: String,
}

impl UpdateCouponCollectionResponse {
    pub fn builder() -> UpdateCouponCollectionResponseBuilder {
        <UpdateCouponCollectionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCouponCollectionResponseBuilder {
    default_coupon: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl UpdateCouponCollectionResponseBuilder {
    pub fn default_coupon(mut self, value: impl Into<String>) -> Self {
        self.default_coupon = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateCouponCollectionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_coupon`](UpdateCouponCollectionResponseBuilder::default_coupon)
    /// - [`id`](UpdateCouponCollectionResponseBuilder::id)
    /// - [`name`](UpdateCouponCollectionResponseBuilder::name)
    pub fn build(self) -> Result<UpdateCouponCollectionResponse, BuildError> {
        Ok(UpdateCouponCollectionResponse {
            default_coupon: self
                .default_coupon
                .ok_or_else(|| BuildError::missing_field("default_coupon"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

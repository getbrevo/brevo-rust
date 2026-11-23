pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCouponsRequest {
    /// The id of the coupon collection for which the coupons will be created
    #[serde(rename = "collectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(default)]
    pub coupons: Vec<String>,
}

impl CreateCouponsRequest {
    pub fn builder() -> CreateCouponsRequestBuilder {
        <CreateCouponsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCouponsRequestBuilder {
    collection_id: Option<String>,
    coupons: Option<Vec<String>>,
}

impl CreateCouponsRequestBuilder {
    pub fn collection_id(mut self, value: impl Into<String>) -> Self {
        self.collection_id = Some(value.into());
        self
    }

    pub fn coupons(mut self, value: Vec<String>) -> Self {
        self.coupons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCouponsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`collection_id`](CreateCouponsRequestBuilder::collection_id)
    /// - [`coupons`](CreateCouponsRequestBuilder::coupons)
    pub fn build(self) -> Result<CreateCouponsRequest, BuildError> {
        Ok(CreateCouponsRequest {
            collection_id: self
                .collection_id
                .ok_or_else(|| BuildError::missing_field("collection_id"))?,
            coupons: self
                .coupons
                .ok_or_else(|| BuildError::missing_field("coupons"))?,
        })
    }
}

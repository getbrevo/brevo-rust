pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem {
    /// Timestamp when the product was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// URL of the product image
    #[serde(rename = "imageRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ref: Option<String>,
    /// Unique identifier for the product
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem {
    pub fn builder() -> GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItemBuilder {
        <GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItemBuilder {
    created_at: Option<String>,
    image_ref: Option<String>,
    product_id: Option<String>,
    value: Option<String>,
}

impl GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItemBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn image_ref(mut self, value: impl Into<String>) -> Self {
        self.image_ref = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem`].
    pub fn build(
        self,
    ) -> Result<GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem, BuildError> {
        Ok(GetLoyaltyOfferProgramsPidRewardsRidResponseProductsItem {
            created_at: self.created_at,
            image_ref: self.image_ref,
            product_id: self.product_id,
            value: self.value,
        })
    }
}

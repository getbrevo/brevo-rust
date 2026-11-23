pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateUpdateBatchProductsRequestProductsItem {
    /// Brand of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Category ID-s of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the product deleted from the shop's database
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Description of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Product ID for which you requested the details
    #[serde(default)]
    pub id: String,
    /// Absolute URL to the cover image of the product
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// product deleted from the shop's database
    #[serde(rename = "isDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// Meta data of product such as description, vendor, producer, stock level. The total characters of cumulative metaInfo shall not exceed **20000 characters**.
    #[serde(rename = "metaInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_info:
        Option<HashMap<String, CreateUpdateBatchProductsRequestProductsItemMetaInfoValue>>,
    /// **Mandatory in case of creation**. Name of the product, as displayed in the shop
    #[serde(default)]
    pub name: String,
    /// Parent product id of the product
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Price of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    /// Alternative price of the product
    #[serde(rename = "alternativePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub alternative_price: Option<f64>,
    /// Product identifier from the shop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Current stock value of the product from the shop's database
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stock: Option<f64>,
    /// URL to the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreateUpdateBatchProductsRequestProductsItem {
    pub fn builder() -> CreateUpdateBatchProductsRequestProductsItemBuilder {
        <CreateUpdateBatchProductsRequestProductsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateUpdateBatchProductsRequestProductsItemBuilder {
    brand: Option<String>,
    categories: Option<Vec<String>>,
    deleted_at: Option<String>,
    description: Option<String>,
    id: Option<String>,
    image_url: Option<String>,
    is_deleted: Option<bool>,
    meta_info: Option<HashMap<String, CreateUpdateBatchProductsRequestProductsItemMetaInfoValue>>,
    name: Option<String>,
    parent_id: Option<String>,
    price: Option<f64>,
    alternative_price: Option<f64>,
    sku: Option<String>,
    stock: Option<f64>,
    url: Option<String>,
}

impl CreateUpdateBatchProductsRequestProductsItemBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: impl Into<String>) -> Self {
        self.deleted_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn is_deleted(mut self, value: bool) -> Self {
        self.is_deleted = Some(value);
        self
    }

    pub fn meta_info(
        mut self,
        value: HashMap<String, CreateUpdateBatchProductsRequestProductsItemMetaInfoValue>,
    ) -> Self {
        self.meta_info = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn alternative_price(mut self, value: f64) -> Self {
        self.alternative_price = Some(value);
        self
    }

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
        self
    }

    pub fn stock(mut self, value: f64) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateUpdateBatchProductsRequestProductsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateUpdateBatchProductsRequestProductsItemBuilder::id)
    /// - [`name`](CreateUpdateBatchProductsRequestProductsItemBuilder::name)
    pub fn build(self) -> Result<CreateUpdateBatchProductsRequestProductsItem, BuildError> {
        Ok(CreateUpdateBatchProductsRequestProductsItem {
            brand: self.brand,
            categories: self.categories,
            deleted_at: self.deleted_at,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_url: self.image_url,
            is_deleted: self.is_deleted,
            meta_info: self.meta_info,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_id: self.parent_id,
            price: self.price,
            alternative_price: self.alternative_price,
            sku: self.sku,
            stock: self.stock,
            url: self.url,
        })
    }
}

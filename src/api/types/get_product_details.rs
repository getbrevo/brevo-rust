pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetProductDetails {
    /// Brand of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Category ID-s of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Creation UTC date-time of the product (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
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
    /// Product deleted from the shop's database but not from Brevo
    #[serde(rename = "isDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// Meta data of product such as description, vendor, producer, stock level, etc.
    #[serde(rename = "metaInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_info: Option<HashMap<String, serde_json::Value>>,
    /// Last modification UTC date-time of the product (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    pub modified_at: String,
    /// Name of the product for which you requested the details
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
    /// S3 url of original image
    #[serde(rename = "s3Original")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3original: Option<String>,
    /// S3 thumbnail url of original image in 120x120 dimension for analytics section
    #[serde(rename = "s3ThumbAnalytics")]
    #[serde(default)]
    pub s3thumb_analytics: String,
    /// S3 thumbnail url of original image in 600x400 dimension for editor section
    #[serde(rename = "s3ThumbEditor")]
    #[serde(default)]
    pub s3thumb_editor: String,
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

impl GetProductDetails {
    pub fn builder() -> GetProductDetailsBuilder {
        <GetProductDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProductDetailsBuilder {
    brand: Option<String>,
    categories: Option<Vec<String>>,
    created_at: Option<String>,
    description: Option<String>,
    id: Option<String>,
    image_url: Option<String>,
    is_deleted: Option<bool>,
    meta_info: Option<HashMap<String, serde_json::Value>>,
    modified_at: Option<String>,
    name: Option<String>,
    parent_id: Option<String>,
    price: Option<f64>,
    alternative_price: Option<f64>,
    s3original: Option<String>,
    s3thumb_analytics: Option<String>,
    s3thumb_editor: Option<String>,
    sku: Option<String>,
    stock: Option<f64>,
    url: Option<String>,
}

impl GetProductDetailsBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn categories(mut self, value: Vec<String>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn meta_info(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta_info = Some(value);
        self
    }

    pub fn modified_at(mut self, value: impl Into<String>) -> Self {
        self.modified_at = Some(value.into());
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

    pub fn s3original(mut self, value: impl Into<String>) -> Self {
        self.s3original = Some(value.into());
        self
    }

    pub fn s3thumb_analytics(mut self, value: impl Into<String>) -> Self {
        self.s3thumb_analytics = Some(value.into());
        self
    }

    pub fn s3thumb_editor(mut self, value: impl Into<String>) -> Self {
        self.s3thumb_editor = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetProductDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetProductDetailsBuilder::created_at)
    /// - [`id`](GetProductDetailsBuilder::id)
    /// - [`modified_at`](GetProductDetailsBuilder::modified_at)
    /// - [`name`](GetProductDetailsBuilder::name)
    /// - [`s3thumb_analytics`](GetProductDetailsBuilder::s3thumb_analytics)
    /// - [`s3thumb_editor`](GetProductDetailsBuilder::s3thumb_editor)
    pub fn build(self) -> Result<GetProductDetails, BuildError> {
        Ok(GetProductDetails {
            brand: self.brand,
            categories: self.categories,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_url: self.image_url,
            is_deleted: self.is_deleted,
            meta_info: self.meta_info,
            modified_at: self
                .modified_at
                .ok_or_else(|| BuildError::missing_field("modified_at"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_id: self.parent_id,
            price: self.price,
            alternative_price: self.alternative_price,
            s3original: self.s3original,
            s3thumb_analytics: self
                .s3thumb_analytics
                .ok_or_else(|| BuildError::missing_field("s3thumb_analytics"))?,
            s3thumb_editor: self
                .s3thumb_editor
                .ok_or_else(|| BuildError::missing_field("s3thumb_editor"))?,
            sku: self.sku,
            stock: self.stock,
            url: self.url,
        })
    }
}

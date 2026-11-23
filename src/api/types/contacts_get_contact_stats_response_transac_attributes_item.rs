pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetContactStatsResponseTransacAttributesItem {
    /// Date of the order
    #[serde(rename = "orderDate")]
    #[serde(default)]
    pub order_date: NaiveDate,
    /// ID of the order
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: i64,
    /// Price of the order
    #[serde(rename = "orderPrice")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub order_price: f64,
}

impl GetContactStatsResponseTransacAttributesItem {
    pub fn builder() -> GetContactStatsResponseTransacAttributesItemBuilder {
        <GetContactStatsResponseTransacAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactStatsResponseTransacAttributesItemBuilder {
    order_date: Option<NaiveDate>,
    order_id: Option<i64>,
    order_price: Option<f64>,
}

impl GetContactStatsResponseTransacAttributesItemBuilder {
    pub fn order_date(mut self, value: NaiveDate) -> Self {
        self.order_date = Some(value);
        self
    }

    pub fn order_id(mut self, value: i64) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn order_price(mut self, value: f64) -> Self {
        self.order_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetContactStatsResponseTransacAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_date`](GetContactStatsResponseTransacAttributesItemBuilder::order_date)
    /// - [`order_id`](GetContactStatsResponseTransacAttributesItemBuilder::order_id)
    /// - [`order_price`](GetContactStatsResponseTransacAttributesItemBuilder::order_price)
    pub fn build(self) -> Result<GetContactStatsResponseTransacAttributesItem, BuildError> {
        Ok(GetContactStatsResponseTransacAttributesItem {
            order_date: self
                .order_date
                .ok_or_else(|| BuildError::missing_field("order_date"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            order_price: self
                .order_price
                .ok_or_else(|| BuildError::missing_field("order_price"))?,
        })
    }
}

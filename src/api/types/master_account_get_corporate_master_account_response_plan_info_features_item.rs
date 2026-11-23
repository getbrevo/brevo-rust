pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateMasterAccountResponsePlanInfoFeaturesItem {
    /// Name of the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Quantity provided in the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Quantity with overages provided in the plan (only
    /// applicable on ENTv2)
    #[serde(rename = "quantityWithOverages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_with_overages: Option<i64>,
    /// Quantity remaining in the plan
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
    /// Unit value of the feature
    #[serde(rename = "unitValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_value: Option<String>,
    /// Quantity consumed by master
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
    /// Quantity consumed by sub-organizations over the
    /// admin plan limit (only applicable on ENTv2)
    #[serde(rename = "usedOverages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_overages: Option<i64>,
}

impl GetCorporateMasterAccountResponsePlanInfoFeaturesItem {
    pub fn builder() -> GetCorporateMasterAccountResponsePlanInfoFeaturesItemBuilder {
        <GetCorporateMasterAccountResponsePlanInfoFeaturesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateMasterAccountResponsePlanInfoFeaturesItemBuilder {
    name: Option<String>,
    quantity: Option<i64>,
    quantity_with_overages: Option<i64>,
    remaining: Option<i64>,
    unit_value: Option<String>,
    used: Option<i64>,
    used_overages: Option<i64>,
}

impl GetCorporateMasterAccountResponsePlanInfoFeaturesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn quantity_with_overages(mut self, value: i64) -> Self {
        self.quantity_with_overages = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    pub fn unit_value(mut self, value: impl Into<String>) -> Self {
        self.unit_value = Some(value.into());
        self
    }

    pub fn used(mut self, value: i64) -> Self {
        self.used = Some(value);
        self
    }

    pub fn used_overages(mut self, value: i64) -> Self {
        self.used_overages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateMasterAccountResponsePlanInfoFeaturesItem`].
    pub fn build(
        self,
    ) -> Result<GetCorporateMasterAccountResponsePlanInfoFeaturesItem, BuildError> {
        Ok(GetCorporateMasterAccountResponsePlanInfoFeaturesItem {
            name: self.name,
            quantity: self.quantity,
            quantity_with_overages: self.quantity_with_overages,
            remaining: self.remaining,
            unit_value: self.unit_value,
            used: self.used,
            used_overages: self.used_overages,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAttributesResponseAttributesItem {
    /// Calculated value formula
    #[serde(rename = "calculatedValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_value: Option<String>,
    /// Category of the attribute
    pub category: GetAttributesResponseAttributesItemCategory,
    /// Parameter only available for "category" type attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<GetAttributesResponseAttributesItemEnumerationItem>>,
    /// Parameter only available for "multiple-choice" type attributes.
    #[serde(rename = "multiCategoryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_category_options: Option<Vec<String>>,
    /// Name of the attribute
    #[serde(default)]
    pub name: String,
    /// Type of the attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GetAttributesResponseAttributesItemType>,
}

impl GetAttributesResponseAttributesItem {
    pub fn builder() -> GetAttributesResponseAttributesItemBuilder {
        <GetAttributesResponseAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAttributesResponseAttributesItemBuilder {
    calculated_value: Option<String>,
    category: Option<GetAttributesResponseAttributesItemCategory>,
    enumeration: Option<Vec<GetAttributesResponseAttributesItemEnumerationItem>>,
    multi_category_options: Option<Vec<String>>,
    name: Option<String>,
    r#type: Option<GetAttributesResponseAttributesItemType>,
}

impl GetAttributesResponseAttributesItemBuilder {
    pub fn calculated_value(mut self, value: impl Into<String>) -> Self {
        self.calculated_value = Some(value.into());
        self
    }

    pub fn category(mut self, value: GetAttributesResponseAttributesItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn enumeration(
        mut self,
        value: Vec<GetAttributesResponseAttributesItemEnumerationItem>,
    ) -> Self {
        self.enumeration = Some(value);
        self
    }

    pub fn multi_category_options(mut self, value: Vec<String>) -> Self {
        self.multi_category_options = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: GetAttributesResponseAttributesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAttributesResponseAttributesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](GetAttributesResponseAttributesItemBuilder::category)
    /// - [`name`](GetAttributesResponseAttributesItemBuilder::name)
    pub fn build(self) -> Result<GetAttributesResponseAttributesItem, BuildError> {
        Ok(GetAttributesResponseAttributesItem {
            calculated_value: self.calculated_value,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            enumeration: self.enumeration,
            multi_category_options: self.multi_category_options,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type,
        })
    }
}

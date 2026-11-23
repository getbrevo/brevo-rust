pub use crate::prelude::*;

/// Company attribute details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCrmAttributesCompaniesResponseItem {
    /// Internal name of the attribute
    #[serde(rename = "internalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    /// Type of the attribute
    #[serde(rename = "attributeTypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type_name: Option<String>,
    /// Display label of the attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Options for single-select or multi-choice attributes
    #[serde(rename = "attributeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_options: Option<Vec<GetCrmAttributesCompaniesResponseItemAttributeOptionsItem>>,
    /// Whether this attribute is required
    #[serde(rename = "isRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// Whether this attribute value is read-only
    #[serde(rename = "isValueReadonly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_readonly: Option<bool>,
}

impl GetCrmAttributesCompaniesResponseItem {
    pub fn builder() -> GetCrmAttributesCompaniesResponseItemBuilder {
        <GetCrmAttributesCompaniesResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCrmAttributesCompaniesResponseItemBuilder {
    internal_name: Option<String>,
    attribute_type_name: Option<String>,
    label: Option<String>,
    attribute_options: Option<Vec<GetCrmAttributesCompaniesResponseItemAttributeOptionsItem>>,
    is_required: Option<bool>,
    is_value_readonly: Option<bool>,
}

impl GetCrmAttributesCompaniesResponseItemBuilder {
    pub fn internal_name(mut self, value: impl Into<String>) -> Self {
        self.internal_name = Some(value.into());
        self
    }

    pub fn attribute_type_name(mut self, value: impl Into<String>) -> Self {
        self.attribute_type_name = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn attribute_options(
        mut self,
        value: Vec<GetCrmAttributesCompaniesResponseItemAttributeOptionsItem>,
    ) -> Self {
        self.attribute_options = Some(value);
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn is_value_readonly(mut self, value: bool) -> Self {
        self.is_value_readonly = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCrmAttributesCompaniesResponseItem`].
    pub fn build(self) -> Result<GetCrmAttributesCompaniesResponseItem, BuildError> {
        Ok(GetCrmAttributesCompaniesResponseItem {
            internal_name: self.internal_name,
            attribute_type_name: self.attribute_type_name,
            label: self.label,
            attribute_options: self.attribute_options,
            is_required: self.is_required,
            is_value_readonly: self.is_value_readonly,
        })
    }
}

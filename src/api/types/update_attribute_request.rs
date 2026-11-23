pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAttributeRequest {
    /// List of the values and labels that the attribute can take. **Use only if the attribute's category is "category"** None of the category options can exceed max 200 characters. For example, **[{"value":1, "label":"male"}, {"value":2, "label":"female"}]**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<UpdateAttributeRequestEnumerationItem>>,
    /// Use this option to add multiple-choice attributes options only if the attribute's category is "normal". **This option is specifically designed for updating multiple-choice attributes. None of the multicategory options can exceed max 200 characters.**. For example: **["USA","INDIA"]**
    #[serde(rename = "multiCategoryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_category_options: Option<Vec<String>>,
    /// Value of the attribute to update. **Use only if the attribute's category is 'calculated' or 'global'**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateAttributeRequest {
    pub fn builder() -> UpdateAttributeRequestBuilder {
        <UpdateAttributeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAttributeRequestBuilder {
    enumeration: Option<Vec<UpdateAttributeRequestEnumerationItem>>,
    multi_category_options: Option<Vec<String>>,
    value: Option<String>,
}

impl UpdateAttributeRequestBuilder {
    pub fn enumeration(mut self, value: Vec<UpdateAttributeRequestEnumerationItem>) -> Self {
        self.enumeration = Some(value);
        self
    }

    pub fn multi_category_options(mut self, value: Vec<String>) -> Self {
        self.multi_category_options = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAttributeRequest`].
    pub fn build(self) -> Result<UpdateAttributeRequest, BuildError> {
        Ok(UpdateAttributeRequest {
            enumeration: self.enumeration,
            multi_category_options: self.multi_category_options,
            value: self.value,
        })
    }
}

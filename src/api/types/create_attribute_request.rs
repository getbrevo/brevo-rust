pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAttributeRequest {
    /// List of values and labels that the attribute can take. **Use only if the attribute's category is "category"**. None of the category options can exceed max 200 characters. For example: **[{"value":1, "label":"male"}, {"value":2, "label":"female"}]**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enumeration: Option<Vec<CreateAttributeRequestEnumerationItem>>,
    /// Type of the attribute. **Use only if the attribute's category is 'calculated' or 'global'**
    #[serde(rename = "isRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
    /// List of options you want to add for multiple-choice attribute. **Use only if the attribute's category is "normal" and attribute's type is "multiple-choice". None of the multicategory options can exceed max 200 characters.** For example: **["USA","INDIA"]**
    #[serde(rename = "multiCategoryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_category_options: Option<Vec<String>>,
    /// Type of the attribute. **Use only if the attribute's category is 'normal', 'category' or 'transactional'** Type **user and multiple-choice** is only available if the category is **normal** attribute Type **id** is only available if the category is **transactional** attribute Type **category** is only available if the category is **category** attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateAttributeRequestType>,
    /// Value of the attribute. **Use only if the attribute's category is 'calculated' or 'global'**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateAttributeRequest {
    pub fn builder() -> CreateAttributeRequestBuilder {
        <CreateAttributeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAttributeRequestBuilder {
    enumeration: Option<Vec<CreateAttributeRequestEnumerationItem>>,
    is_recurring: Option<bool>,
    multi_category_options: Option<Vec<String>>,
    r#type: Option<CreateAttributeRequestType>,
    value: Option<String>,
}

impl CreateAttributeRequestBuilder {
    pub fn enumeration(mut self, value: Vec<CreateAttributeRequestEnumerationItem>) -> Self {
        self.enumeration = Some(value);
        self
    }

    pub fn is_recurring(mut self, value: bool) -> Self {
        self.is_recurring = Some(value);
        self
    }

    pub fn multi_category_options(mut self, value: Vec<String>) -> Self {
        self.multi_category_options = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateAttributeRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAttributeRequest`].
    pub fn build(self) -> Result<CreateAttributeRequest, BuildError> {
        Ok(CreateAttributeRequest {
            enumeration: self.enumeration,
            is_recurring: self.is_recurring,
            multi_category_options: self.multi_category_options,
            r#type: self.r#type,
            value: self.value,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostCrmAttributesRequest {
    /// The type of attribute (must be one of the defined enums)
    #[serde(rename = "attributeType")]
    pub attribute_type: PostCrmAttributesRequestAttributeType,
    /// A description of the attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The label for the attribute (max 50 characters, cannot be empty)
    #[serde(default)]
    pub label: String,
    /// The type of object the attribute belongs to. Must be either `companies` or `deals`.
    #[serde(rename = "objectType")]
    pub object_type: PostCrmAttributesRequestObjectType,
    /// Options for multi-choice or single-select attributes
    #[serde(rename = "optionsLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_labels: Option<Vec<String>>,
}

impl PostCrmAttributesRequest {
    pub fn builder() -> PostCrmAttributesRequestBuilder {
        <PostCrmAttributesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCrmAttributesRequestBuilder {
    attribute_type: Option<PostCrmAttributesRequestAttributeType>,
    description: Option<String>,
    label: Option<String>,
    object_type: Option<PostCrmAttributesRequestObjectType>,
    options_labels: Option<Vec<String>>,
}

impl PostCrmAttributesRequestBuilder {
    pub fn attribute_type(mut self, value: PostCrmAttributesRequestAttributeType) -> Self {
        self.attribute_type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn object_type(mut self, value: PostCrmAttributesRequestObjectType) -> Self {
        self.object_type = Some(value);
        self
    }

    pub fn options_labels(mut self, value: Vec<String>) -> Self {
        self.options_labels = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCrmAttributesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attribute_type`](PostCrmAttributesRequestBuilder::attribute_type)
    /// - [`label`](PostCrmAttributesRequestBuilder::label)
    /// - [`object_type`](PostCrmAttributesRequestBuilder::object_type)
    pub fn build(self) -> Result<PostCrmAttributesRequest, BuildError> {
        Ok(PostCrmAttributesRequest {
            attribute_type: self
                .attribute_type
                .ok_or_else(|| BuildError::missing_field("attribute_type"))?,
            description: self.description,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            object_type: self
                .object_type
                .ok_or_else(|| BuildError::missing_field("object_type"))?,
            options_labels: self.options_labels,
        })
    }
}

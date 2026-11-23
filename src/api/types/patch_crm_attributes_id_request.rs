pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchCrmAttributesIdRequest {
    /// Attribute display label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Updated labels for selectable options
    #[serde(rename = "optionsLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_labels: Option<Vec<PatchCrmAttributesIdRequestOptionsLabelsItem>>,
    /// The type of object the attribute belongs to, it cannot be updated after creation
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<PatchCrmAttributesIdRequestObjectType>,
}

impl PatchCrmAttributesIdRequest {
    pub fn builder() -> PatchCrmAttributesIdRequestBuilder {
        <PatchCrmAttributesIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchCrmAttributesIdRequestBuilder {
    label: Option<String>,
    options_labels: Option<Vec<PatchCrmAttributesIdRequestOptionsLabelsItem>>,
    object_type: Option<PatchCrmAttributesIdRequestObjectType>,
}

impl PatchCrmAttributesIdRequestBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn options_labels(
        mut self,
        value: Vec<PatchCrmAttributesIdRequestOptionsLabelsItem>,
    ) -> Self {
        self.options_labels = Some(value);
        self
    }

    pub fn object_type(mut self, value: PatchCrmAttributesIdRequestObjectType) -> Self {
        self.object_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchCrmAttributesIdRequest`].
    pub fn build(self) -> Result<PatchCrmAttributesIdRequest, BuildError> {
        Ok(PatchCrmAttributesIdRequest {
            label: self.label,
            options_labels: self.options_labels,
            object_type: self.object_type,
        })
    }
}

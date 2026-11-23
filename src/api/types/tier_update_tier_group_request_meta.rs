pub use crate::prelude::*;

/// Additional metadata for the tier group.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateTierGroupRequestMeta {
    /// Indicates whether the tier group is internal.
    #[serde(rename = "isInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateTierGroupRequestMeta {
    pub fn builder() -> UpdateTierGroupRequestMetaBuilder {
        <UpdateTierGroupRequestMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTierGroupRequestMetaBuilder {
    is_internal: Option<bool>,
}

impl UpdateTierGroupRequestMetaBuilder {
    pub fn is_internal(mut self, value: bool) -> Self {
        self.is_internal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTierGroupRequestMeta`].
    pub fn build(self) -> Result<UpdateTierGroupRequestMeta, BuildError> {
        Ok(UpdateTierGroupRequestMeta {
            is_internal: self.is_internal,
            extra: Default::default(),
        })
    }
}

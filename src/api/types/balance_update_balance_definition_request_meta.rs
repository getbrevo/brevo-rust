pub use crate::prelude::*;

/// Optional metadata for the balance definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateBalanceDefinitionRequestMeta {
    /// Indicates whether the balance definition is internal.
    #[serde(rename = "isInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateBalanceDefinitionRequestMeta {
    pub fn builder() -> UpdateBalanceDefinitionRequestMetaBuilder {
        <UpdateBalanceDefinitionRequestMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBalanceDefinitionRequestMetaBuilder {
    is_internal: Option<bool>,
}

impl UpdateBalanceDefinitionRequestMetaBuilder {
    pub fn is_internal(mut self, value: bool) -> Self {
        self.is_internal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBalanceDefinitionRequestMeta`].
    pub fn build(self) -> Result<UpdateBalanceDefinitionRequestMeta, BuildError> {
        Ok(UpdateBalanceDefinitionRequestMeta {
            is_internal: self.is_internal,
            extra: Default::default(),
        })
    }
}

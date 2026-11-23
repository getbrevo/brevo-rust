pub use crate::prelude::*;

/// Additional metadata for the balance definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMeta {
    /// Indicates whether the balance definition is internal.
    #[serde(rename = "isInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMeta {
    pub fn builder() -> PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMetaBuilder {
        <PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMetaBuilder {
    is_internal: Option<bool>,
}

impl PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMetaBuilder {
    pub fn is_internal(mut self, value: bool) -> Self {
        self.is_internal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMeta`].
    pub fn build(
        self,
    ) -> Result<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMeta, BuildError> {
        Ok(PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMeta {
            is_internal: self.is_internal,
            extra: Default::default(),
        })
    }
}

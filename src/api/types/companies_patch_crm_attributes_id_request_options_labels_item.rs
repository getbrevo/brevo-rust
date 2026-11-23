pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchCrmAttributesIdRequestOptionsLabelsItem {
    /// Internal option identifier
    #[serde(default)]
    pub key: String,
    /// Display label for the option
    #[serde(default)]
    pub value: String,
}

impl PatchCrmAttributesIdRequestOptionsLabelsItem {
    pub fn builder() -> PatchCrmAttributesIdRequestOptionsLabelsItemBuilder {
        <PatchCrmAttributesIdRequestOptionsLabelsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchCrmAttributesIdRequestOptionsLabelsItemBuilder {
    key: Option<String>,
    value: Option<String>,
}

impl PatchCrmAttributesIdRequestOptionsLabelsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchCrmAttributesIdRequestOptionsLabelsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PatchCrmAttributesIdRequestOptionsLabelsItemBuilder::key)
    /// - [`value`](PatchCrmAttributesIdRequestOptionsLabelsItemBuilder::value)
    pub fn build(self) -> Result<PatchCrmAttributesIdRequestOptionsLabelsItem, BuildError> {
        Ok(PatchCrmAttributesIdRequestOptionsLabelsItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

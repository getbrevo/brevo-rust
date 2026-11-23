pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAttributeRequestEnumerationItem {
    /// Label of the value
    #[serde(default)]
    pub label: String,
    /// Id of the value
    #[serde(default)]
    pub value: i64,
}

impl UpdateAttributeRequestEnumerationItem {
    pub fn builder() -> UpdateAttributeRequestEnumerationItemBuilder {
        <UpdateAttributeRequestEnumerationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAttributeRequestEnumerationItemBuilder {
    label: Option<String>,
    value: Option<i64>,
}

impl UpdateAttributeRequestEnumerationItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAttributeRequestEnumerationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](UpdateAttributeRequestEnumerationItemBuilder::label)
    /// - [`value`](UpdateAttributeRequestEnumerationItemBuilder::value)
    pub fn build(self) -> Result<UpdateAttributeRequestEnumerationItem, BuildError> {
        Ok(UpdateAttributeRequestEnumerationItem {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

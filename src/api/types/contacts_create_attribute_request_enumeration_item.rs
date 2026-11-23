pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAttributeRequestEnumerationItem {
    /// Label of the value
    #[serde(default)]
    pub label: String,
    /// Id of the value
    #[serde(default)]
    pub value: i64,
}

impl CreateAttributeRequestEnumerationItem {
    pub fn builder() -> CreateAttributeRequestEnumerationItemBuilder {
        <CreateAttributeRequestEnumerationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAttributeRequestEnumerationItemBuilder {
    label: Option<String>,
    value: Option<i64>,
}

impl CreateAttributeRequestEnumerationItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAttributeRequestEnumerationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](CreateAttributeRequestEnumerationItemBuilder::label)
    /// - [`value`](CreateAttributeRequestEnumerationItemBuilder::value)
    pub fn build(self) -> Result<CreateAttributeRequestEnumerationItem, BuildError> {
        Ok(CreateAttributeRequestEnumerationItem {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

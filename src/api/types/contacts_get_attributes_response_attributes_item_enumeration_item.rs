pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAttributesResponseAttributesItemEnumerationItem {
    /// Label of the "category" type attribute
    #[serde(default)]
    pub label: String,
    /// Numeric ID of the "category" type attribute value. Set to 0 when the raw value cannot be converted to an integer (for example non-numeric values such as "en" or "fr"). Refer to `valueStr` for the original string representation.
    #[serde(default)]
    pub value: i64,
    /// String representation of the "category" type attribute value. Always contains the original value as stored. Use this field when the attribute value is non-numeric (e.g. "en", "fr") or when you need the exact string form alongside the numeric `value`.
    #[serde(rename = "valueStr")]
    #[serde(default)]
    pub value_str: String,
}

impl GetAttributesResponseAttributesItemEnumerationItem {
    pub fn builder() -> GetAttributesResponseAttributesItemEnumerationItemBuilder {
        <GetAttributesResponseAttributesItemEnumerationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAttributesResponseAttributesItemEnumerationItemBuilder {
    label: Option<String>,
    value: Option<i64>,
    value_str: Option<String>,
}

impl GetAttributesResponseAttributesItemEnumerationItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn value_str(mut self, value: impl Into<String>) -> Self {
        self.value_str = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAttributesResponseAttributesItemEnumerationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](GetAttributesResponseAttributesItemEnumerationItemBuilder::label)
    /// - [`value`](GetAttributesResponseAttributesItemEnumerationItemBuilder::value)
    /// - [`value_str`](GetAttributesResponseAttributesItemEnumerationItemBuilder::value_str)
    pub fn build(self) -> Result<GetAttributesResponseAttributesItemEnumerationItem, BuildError> {
        Ok(GetAttributesResponseAttributesItemEnumerationItem {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            value_str: self
                .value_str
                .ok_or_else(|| BuildError::missing_field("value_str"))?,
        })
    }
}

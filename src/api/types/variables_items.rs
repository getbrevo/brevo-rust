pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VariablesItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datatype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl VariablesItems {
    pub fn builder() -> VariablesItemsBuilder {
        <VariablesItemsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VariablesItemsBuilder {
    datatype: Option<String>,
    default: Option<String>,
    name: Option<String>,
}

impl VariablesItemsBuilder {
    pub fn datatype(mut self, value: impl Into<String>) -> Self {
        self.datatype = Some(value.into());
        self
    }

    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VariablesItems`].
    pub fn build(self) -> Result<VariablesItems, BuildError> {
        Ok(VariablesItems {
            datatype: self.datatype,
            default: self.default,
            name: self.name,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAllExternalFeedsResponseFeedsItemHeadersItem {
    /// Header name
    #[serde(default)]
    pub name: String,
    /// Header value
    #[serde(default)]
    pub value: String,
}

impl GetAllExternalFeedsResponseFeedsItemHeadersItem {
    pub fn builder() -> GetAllExternalFeedsResponseFeedsItemHeadersItemBuilder {
        <GetAllExternalFeedsResponseFeedsItemHeadersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAllExternalFeedsResponseFeedsItemHeadersItemBuilder {
    name: Option<String>,
    value: Option<String>,
}

impl GetAllExternalFeedsResponseFeedsItemHeadersItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAllExternalFeedsResponseFeedsItemHeadersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GetAllExternalFeedsResponseFeedsItemHeadersItemBuilder::name)
    /// - [`value`](GetAllExternalFeedsResponseFeedsItemHeadersItemBuilder::value)
    pub fn build(self) -> Result<GetAllExternalFeedsResponseFeedsItemHeadersItem, BuildError> {
        Ok(GetAllExternalFeedsResponseFeedsItemHeadersItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

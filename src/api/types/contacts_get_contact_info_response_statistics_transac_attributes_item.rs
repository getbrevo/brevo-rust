pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetContactInfoResponseStatisticsTransacAttributesItem {}

impl GetContactInfoResponseStatisticsTransacAttributesItem {
    pub fn builder() -> GetContactInfoResponseStatisticsTransacAttributesItemBuilder {
        <GetContactInfoResponseStatisticsTransacAttributesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetContactInfoResponseStatisticsTransacAttributesItemBuilder {}

impl GetContactInfoResponseStatisticsTransacAttributesItemBuilder {
    /// Consumes the builder and constructs a [`GetContactInfoResponseStatisticsTransacAttributesItem`].
    pub fn build(
        self,
    ) -> Result<GetContactInfoResponseStatisticsTransacAttributesItem, BuildError> {
        Ok(GetContactInfoResponseStatisticsTransacAttributesItem {})
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne {
    /// Identifiers attached with the associated object record.
    pub identifiers:
        Option<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOneIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne {
    pub fn builder() -> GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOneBuilder {
        <GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOneBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOneBuilder {
    identifiers: Option<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOneIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOneBuilder {
    pub fn identifiers(
        mut self,
        value: GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOneIdentifiers,
    ) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne`].
    pub fn build(
        self,
    ) -> Result<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne, BuildError> {
        Ok(
            GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemOne {
                identifiers: self.identifiers,
            },
        )
    }
}

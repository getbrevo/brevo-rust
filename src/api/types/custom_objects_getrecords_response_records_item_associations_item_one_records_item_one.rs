pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne {
    /// Identifiers attached with the associated object record. Only includes the internal ID.
    pub identifiers:
        Option<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOneIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne {
    pub fn builder() -> GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOneBuilder {
        <GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOneBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOneBuilder {
    identifiers: Option<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOneIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOneBuilder {
    pub fn identifiers(
        mut self,
        value: GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOneIdentifiers,
    ) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne`].
    pub fn build(
        self,
    ) -> Result<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne, BuildError> {
        Ok(
            GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemOne {
                identifiers: self.identifiers,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero {
    /// Identifiers attached with the associated object record. Only includes the internal ID.
    pub identifiers:
        Option<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZeroIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero {
    pub fn builder() -> GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZeroBuilder {
        <GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZeroBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZeroBuilder {
    identifiers: Option<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZeroIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZeroBuilder {
    pub fn identifiers(
        mut self,
        value: GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZeroIdentifiers,
    ) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero`].
    pub fn build(
        self,
    ) -> Result<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero, BuildError> {
        Ok(
            GetrecordsResponseRecordsItemAssociationsItemOneRecordsItemZero {
                identifiers: self.identifiers,
            },
        )
    }
}

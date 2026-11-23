pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero {
    /// Identifiers attached with the associated object record.
    pub identifiers:
        Option<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZeroIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero {
    pub fn builder() -> GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZeroBuilder {
        <GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZeroBuilder {
    identifiers:
        Option<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZeroIdentifiers>,
}

impl GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZeroBuilder {
    pub fn identifiers(
        mut self,
        value: GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZeroIdentifiers,
    ) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero`].
    pub fn build(
        self,
    ) -> Result<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero, BuildError> {
        Ok(
            GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItemZero {
                identifiers: self.identifiers,
            },
        )
    }
}

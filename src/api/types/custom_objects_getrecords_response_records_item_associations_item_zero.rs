pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetrecordsResponseRecordsItemAssociationsItemZero {
    /// Type of the associated object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItem>>,
}

impl GetrecordsResponseRecordsItemAssociationsItemZero {
    pub fn builder() -> GetrecordsResponseRecordsItemAssociationsItemZeroBuilder {
        <GetrecordsResponseRecordsItemAssociationsItemZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseRecordsItemAssociationsItemZeroBuilder {
    object_type: Option<String>,
    records: Option<Vec<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItem>>,
}

impl GetrecordsResponseRecordsItemAssociationsItemZeroBuilder {
    pub fn object_type(mut self, value: impl Into<String>) -> Self {
        self.object_type = Some(value.into());
        self
    }

    pub fn records(
        mut self,
        value: Vec<GetrecordsResponseRecordsItemAssociationsItemZeroRecordsItem>,
    ) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponseRecordsItemAssociationsItemZero`].
    pub fn build(self) -> Result<GetrecordsResponseRecordsItemAssociationsItemZero, BuildError> {
        Ok(GetrecordsResponseRecordsItemAssociationsItemZero {
            object_type: self.object_type,
            records: self.records,
        })
    }
}

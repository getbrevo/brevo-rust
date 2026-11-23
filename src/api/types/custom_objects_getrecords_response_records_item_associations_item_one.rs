pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetrecordsResponseRecordsItemAssociationsItemOne {
    /// Type of the associated object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItem>>,
}

impl GetrecordsResponseRecordsItemAssociationsItemOne {
    pub fn builder() -> GetrecordsResponseRecordsItemAssociationsItemOneBuilder {
        <GetrecordsResponseRecordsItemAssociationsItemOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseRecordsItemAssociationsItemOneBuilder {
    object_type: Option<String>,
    records: Option<Vec<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItem>>,
}

impl GetrecordsResponseRecordsItemAssociationsItemOneBuilder {
    pub fn object_type(mut self, value: impl Into<String>) -> Self {
        self.object_type = Some(value.into());
        self
    }

    pub fn records(
        mut self,
        value: Vec<GetrecordsResponseRecordsItemAssociationsItemOneRecordsItem>,
    ) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponseRecordsItemAssociationsItemOne`].
    pub fn build(self) -> Result<GetrecordsResponseRecordsItemAssociationsItemOne, BuildError> {
        Ok(GetrecordsResponseRecordsItemAssociationsItemOne {
            object_type: self.object_type,
            records: self.records,
        })
    }
}

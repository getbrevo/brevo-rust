pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetrecordsResponse {
    /// Total number of object records for an object type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<GetrecordsResponseRecordsItem>>,
}

impl GetrecordsResponse {
    pub fn builder() -> GetrecordsResponseBuilder {
        <GetrecordsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetrecordsResponseBuilder {
    count: Option<i64>,
    records: Option<Vec<GetrecordsResponseRecordsItem>>,
}

impl GetrecordsResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<GetrecordsResponseRecordsItem>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetrecordsResponse`].
    pub fn build(self) -> Result<GetrecordsResponse, BuildError> {
        Ok(GetrecordsResponse {
            count: self.count,
            records: self.records,
        })
    }
}

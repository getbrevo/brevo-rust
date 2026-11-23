pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCodeCountResponse {
    /// Number of codes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl GetCodeCountResponse {
    pub fn builder() -> GetCodeCountResponseBuilder {
        <GetCodeCountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCodeCountResponseBuilder {
    count: Option<i64>,
}

impl GetCodeCountResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCodeCountResponse`].
    pub fn build(self) -> Result<GetCodeCountResponse, BuildError> {
        Ok(GetCodeCountResponse { count: self.count })
    }
}

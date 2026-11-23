pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountActivityResponse {
    /// Get user activity logs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<GetAccountActivityResponseLogsItem>>,
}

impl GetAccountActivityResponse {
    pub fn builder() -> GetAccountActivityResponseBuilder {
        <GetAccountActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountActivityResponseBuilder {
    logs: Option<Vec<GetAccountActivityResponseLogsItem>>,
}

impl GetAccountActivityResponseBuilder {
    pub fn logs(mut self, value: Vec<GetAccountActivityResponseLogsItem>) -> Self {
        self.logs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAccountActivityResponse`].
    pub fn build(self) -> Result<GetAccountActivityResponse, BuildError> {
        Ok(GetAccountActivityResponse { logs: self.logs })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProcessesResponse {
    /// Total number of processes available on your account
    #[serde(default)]
    pub count: i64,
    /// List of background processes on your account
    #[serde(default)]
    pub processes: Vec<GetProcessesResponseProcessesItem>,
}

impl GetProcessesResponse {
    pub fn builder() -> GetProcessesResponseBuilder {
        <GetProcessesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProcessesResponseBuilder {
    count: Option<i64>,
    processes: Option<Vec<GetProcessesResponseProcessesItem>>,
}

impl GetProcessesResponseBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn processes(mut self, value: Vec<GetProcessesResponseProcessesItem>) -> Self {
        self.processes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProcessesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](GetProcessesResponseBuilder::count)
    /// - [`processes`](GetProcessesResponseBuilder::processes)
    pub fn build(self) -> Result<GetProcessesResponse, BuildError> {
        Ok(GetProcessesResponse {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            processes: self
                .processes
                .ok_or_else(|| BuildError::missing_field("processes"))?,
        })
    }
}

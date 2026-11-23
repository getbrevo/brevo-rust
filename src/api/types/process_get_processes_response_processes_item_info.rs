pub use crate::prelude::*;

/// Additional process information, only returned for completed IMPORTUSER processes. Contains URLs to CSV files with details about problematic records.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProcessesResponseProcessesItemInfo {
    /// Import process details with URLs to CSV reports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import: Option<GetProcessesResponseProcessesItemInfoImport>,
}

impl GetProcessesResponseProcessesItemInfo {
    pub fn builder() -> GetProcessesResponseProcessesItemInfoBuilder {
        <GetProcessesResponseProcessesItemInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProcessesResponseProcessesItemInfoBuilder {
    import: Option<GetProcessesResponseProcessesItemInfoImport>,
}

impl GetProcessesResponseProcessesItemInfoBuilder {
    pub fn import(mut self, value: GetProcessesResponseProcessesItemInfoImport) -> Self {
        self.import = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProcessesResponseProcessesItemInfo`].
    pub fn build(self) -> Result<GetProcessesResponseProcessesItemInfo, BuildError> {
        Ok(GetProcessesResponseProcessesItemInfo {
            import: self.import,
        })
    }
}

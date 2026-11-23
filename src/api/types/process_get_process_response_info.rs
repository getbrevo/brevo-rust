pub use crate::prelude::*;

/// Additional process information, only returned for completed IMPORTUSER processes. Contains URLs to CSV files with details about problematic records.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProcessResponseInfo {
    /// Import process details with URLs to CSV reports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import: Option<GetProcessResponseInfoImport>,
}

impl GetProcessResponseInfo {
    pub fn builder() -> GetProcessResponseInfoBuilder {
        <GetProcessResponseInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProcessResponseInfoBuilder {
    import: Option<GetProcessResponseInfoImport>,
}

impl GetProcessResponseInfoBuilder {
    pub fn import(mut self, value: GetProcessResponseInfoImport) -> Self {
        self.import = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProcessResponseInfo`].
    pub fn build(self) -> Result<GetProcessResponseInfo, BuildError> {
        Ok(GetProcessResponseInfo {
            import: self.import,
        })
    }
}

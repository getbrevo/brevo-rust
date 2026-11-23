pub use crate::prelude::*;

/// Percentage of a particular event for both versions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AbTestVersionStats {
    /// percentage of an event for version A
    #[serde(rename = "Version A")]
    #[serde(default)]
    pub version_a: String,
    /// percentage of an event for version B
    #[serde(rename = "Version B")]
    #[serde(default)]
    pub version_b: String,
}

impl AbTestVersionStats {
    pub fn builder() -> AbTestVersionStatsBuilder {
        <AbTestVersionStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AbTestVersionStatsBuilder {
    version_a: Option<String>,
    version_b: Option<String>,
}

impl AbTestVersionStatsBuilder {
    pub fn version_a(mut self, value: impl Into<String>) -> Self {
        self.version_a = Some(value.into());
        self
    }

    pub fn version_b(mut self, value: impl Into<String>) -> Self {
        self.version_b = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AbTestVersionStats`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version_a`](AbTestVersionStatsBuilder::version_a)
    /// - [`version_b`](AbTestVersionStatsBuilder::version_b)
    pub fn build(self) -> Result<AbTestVersionStats, BuildError> {
        Ok(AbTestVersionStats {
            version_a: self
                .version_a
                .ok_or_else(|| BuildError::missing_field("version_a"))?,
            version_b: self
                .version_b
                .ok_or_else(|| BuildError::missing_field("version_b"))?,
        })
    }
}

pub use crate::prelude::*;

/// Stage details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineStagesItem {
    /// Stage id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Stage name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PipelineStagesItem {
    pub fn builder() -> PipelineStagesItemBuilder {
        <PipelineStagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineStagesItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl PipelineStagesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineStagesItem`].
    pub fn build(self) -> Result<PipelineStagesItem, BuildError> {
        Ok(PipelineStagesItem {
            id: self.id,
            name: self.name,
        })
    }
}

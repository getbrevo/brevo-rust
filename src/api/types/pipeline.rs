pub use crate::prelude::*;

/// Pipeline details with stages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Pipeline {
    /// Pipeline id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,
    /// Pipeline Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// List of pipeline stages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<PipelineStagesItem>>,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        <PipelineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineBuilder {
    pipeline: Option<String>,
    pipeline_name: Option<String>,
    stages: Option<Vec<PipelineStagesItem>>,
}

impl PipelineBuilder {
    pub fn pipeline(mut self, value: impl Into<String>) -> Self {
        self.pipeline = Some(value.into());
        self
    }

    pub fn pipeline_name(mut self, value: impl Into<String>) -> Self {
        self.pipeline_name = Some(value.into());
        self
    }

    pub fn stages(mut self, value: Vec<PipelineStagesItem>) -> Self {
        self.stages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Pipeline`].
    pub fn build(self) -> Result<Pipeline, BuildError> {
        Ok(Pipeline {
            pipeline: self.pipeline,
            pipeline_name: self.pipeline_name,
            stages: self.stages,
        })
    }
}

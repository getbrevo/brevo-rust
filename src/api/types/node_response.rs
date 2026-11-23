pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NodeResponse {
    /// array[object]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Box<NodeResponse>>>,
    /// Array values for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<Box<NodeResponse>>>,
    /// Boolean values for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    /// Description for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Float values for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub float: Option<f64>,
    /// int64
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    /// Operator selected for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// Boolean values for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

impl NodeResponse {
    pub fn builder() -> NodeResponseBuilder {
        <NodeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NodeResponseBuilder {
    args: Option<Vec<Box<NodeResponse>>>,
    array: Option<Vec<Box<NodeResponse>>>,
    boolean: Option<bool>,
    description: Option<String>,
    float: Option<f64>,
    int: Option<i64>,
    op: Option<String>,
    string: Option<String>,
}

impl NodeResponseBuilder {
    pub fn args(mut self, value: Vec<Box<NodeResponse>>) -> Self {
        self.args = Some(value);
        self
    }

    pub fn array(mut self, value: Vec<Box<NodeResponse>>) -> Self {
        self.array = Some(value);
        self
    }

    pub fn boolean(mut self, value: bool) -> Self {
        self.boolean = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn float(mut self, value: f64) -> Self {
        self.float = Some(value);
        self
    }

    pub fn int(mut self, value: i64) -> Self {
        self.int = Some(value);
        self
    }

    pub fn op(mut self, value: impl Into<String>) -> Self {
        self.op = Some(value.into());
        self
    }

    pub fn string(mut self, value: impl Into<String>) -> Self {
        self.string = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NodeResponse`].
    pub fn build(self) -> Result<NodeResponse, BuildError> {
        Ok(NodeResponse {
            args: self.args,
            array: self.array,
            boolean: self.boolean,
            description: self.description,
            float: self.float,
            int: self.int,
            op: self.op,
            string: self.string,
        })
    }
}

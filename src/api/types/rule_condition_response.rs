pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RuleConditionResponse {
    /// Metric to indicate AND between rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Box<RuleConditionResponse>>>,
    /// Condition of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lhs: Option<ValueResponse>,
    /// Selected operator for the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// Metric to indicate OR between rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Box<RuleConditionResponse>>>,
    /// Action of the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rhs: Option<ValueResponse>,
}

impl RuleConditionResponse {
    pub fn builder() -> RuleConditionResponseBuilder {
        <RuleConditionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RuleConditionResponseBuilder {
    and: Option<Vec<Box<RuleConditionResponse>>>,
    lhs: Option<ValueResponse>,
    op: Option<String>,
    or: Option<Vec<Box<RuleConditionResponse>>>,
    rhs: Option<ValueResponse>,
}

impl RuleConditionResponseBuilder {
    pub fn and(mut self, value: Vec<Box<RuleConditionResponse>>) -> Self {
        self.and = Some(value);
        self
    }

    pub fn lhs(mut self, value: ValueResponse) -> Self {
        self.lhs = Some(value);
        self
    }

    pub fn op(mut self, value: impl Into<String>) -> Self {
        self.op = Some(value.into());
        self
    }

    pub fn or(mut self, value: Vec<Box<RuleConditionResponse>>) -> Self {
        self.or = Some(value);
        self
    }

    pub fn rhs(mut self, value: ValueResponse) -> Self {
        self.rhs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RuleConditionResponse`].
    pub fn build(self) -> Result<RuleConditionResponse, BuildError> {
        Ok(RuleConditionResponse {
            and: self.and,
            lhs: self.lhs,
            op: self.op,
            or: self.or,
            rhs: self.rhs,
        })
    }
}

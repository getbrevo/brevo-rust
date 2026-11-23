pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ValueResponse {
    /// Array values to define the rule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<Box<ValueResponse>>>,
    /// Boolean values for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    /// string
    #[serde(rename = "contactProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_property: Option<String>,
    /// Selected date for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Selected event property for rule definition
    #[serde(rename = "eventProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_property: Option<String>,
    /// Created expression for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<NodeResponse>,
    /// Boolean values for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub number: Option<f64>,
    /// String values for rule definition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
}

impl ValueResponse {
    pub fn builder() -> ValueResponseBuilder {
        <ValueResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValueResponseBuilder {
    array: Option<Vec<Box<ValueResponse>>>,
    boolean: Option<bool>,
    contact_property: Option<String>,
    date: Option<String>,
    event_property: Option<String>,
    expression: Option<NodeResponse>,
    number: Option<f64>,
    string: Option<String>,
}

impl ValueResponseBuilder {
    pub fn array(mut self, value: Vec<Box<ValueResponse>>) -> Self {
        self.array = Some(value);
        self
    }

    pub fn boolean(mut self, value: bool) -> Self {
        self.boolean = Some(value);
        self
    }

    pub fn contact_property(mut self, value: impl Into<String>) -> Self {
        self.contact_property = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn event_property(mut self, value: impl Into<String>) -> Self {
        self.event_property = Some(value.into());
        self
    }

    pub fn expression(mut self, value: NodeResponse) -> Self {
        self.expression = Some(value);
        self
    }

    pub fn number(mut self, value: f64) -> Self {
        self.number = Some(value);
        self
    }

    pub fn string(mut self, value: impl Into<String>) -> Self {
        self.string = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValueResponse`].
    pub fn build(self) -> Result<ValueResponse, BuildError> {
        Ok(ValueResponse {
            array: self.array,
            boolean: self.boolean,
            contact_property: self.contact_property,
            date: self.date,
            event_property: self.event_property,
            expression: self.expression,
            number: self.number,
            string: self.string,
        })
    }
}

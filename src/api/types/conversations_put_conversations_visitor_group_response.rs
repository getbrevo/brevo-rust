pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PutConversationsVisitorGroupResponse {
    /// group ID the visitor was assigned to
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<serde_json::Value>,
    /// visitor ID
    #[serde(rename = "visitorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<serde_json::Value>,
}

impl PutConversationsVisitorGroupResponse {
    pub fn builder() -> PutConversationsVisitorGroupResponseBuilder {
        <PutConversationsVisitorGroupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutConversationsVisitorGroupResponseBuilder {
    group_id: Option<serde_json::Value>,
    visitor_id: Option<serde_json::Value>,
}

impl PutConversationsVisitorGroupResponseBuilder {
    pub fn group_id(mut self, value: serde_json::Value) -> Self {
        self.group_id = Some(value);
        self
    }

    pub fn visitor_id(mut self, value: serde_json::Value) -> Self {
        self.visitor_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutConversationsVisitorGroupResponse`].
    pub fn build(self) -> Result<PutConversationsVisitorGroupResponse, BuildError> {
        Ok(PutConversationsVisitorGroupResponse {
            group_id: self.group_id,
            visitor_id: self.visitor_id,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PutConversationsVisitorGroupRequest {
    #[serde(rename = "groupId")]
    pub group_id: serde_json::Value,
}

impl PutConversationsVisitorGroupRequest {
    pub fn builder() -> PutConversationsVisitorGroupRequestBuilder {
        <PutConversationsVisitorGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutConversationsVisitorGroupRequestBuilder {
    group_id: Option<serde_json::Value>,
}

impl PutConversationsVisitorGroupRequestBuilder {
    pub fn group_id(mut self, value: serde_json::Value) -> Self {
        self.group_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutConversationsVisitorGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PutConversationsVisitorGroupRequestBuilder::group_id)
    pub fn build(self) -> Result<PutConversationsVisitorGroupRequest, BuildError> {
        Ok(PutConversationsVisitorGroupRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}

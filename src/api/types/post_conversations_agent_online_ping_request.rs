pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostConversationsAgentOnlinePingRequest {
    /// Agent's email address. When sending online pings from a standalone system, it's hard to maintain a 1-to-1 relationship between the users of both systems. In this case, an agent can be specified by their email address. If there's no agent with the specified email address in your Brevo organization, a dummy agent will be created automatically.
    #[serde(rename = "agentEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_email: Option<String>,
    /// Agent ID. It can be found on the agent's page or received <a href="https://developers.brevo.com/docs/conversations-webhooks">from a webhook</a>. Alternatively, you can use `agentEmail` + `agentName` + `receivedFrom` instead (all 3 fields required).
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Agent's name.
    #[serde(rename = "agentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// Mark your messages to distinguish messages created by you from the others.
    #[serde(rename = "receivedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_from: Option<String>,
}

impl PostConversationsAgentOnlinePingRequest {
    pub fn builder() -> PostConversationsAgentOnlinePingRequestBuilder {
        <PostConversationsAgentOnlinePingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostConversationsAgentOnlinePingRequestBuilder {
    agent_email: Option<String>,
    agent_id: Option<String>,
    agent_name: Option<String>,
    received_from: Option<String>,
}

impl PostConversationsAgentOnlinePingRequestBuilder {
    pub fn agent_email(mut self, value: impl Into<String>) -> Self {
        self.agent_email = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn received_from(mut self, value: impl Into<String>) -> Self {
        self.received_from = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostConversationsAgentOnlinePingRequest`].
    pub fn build(self) -> Result<PostConversationsAgentOnlinePingRequest, BuildError> {
        Ok(PostConversationsAgentOnlinePingRequest {
            agent_email: self.agent_email,
            agent_id: self.agent_id,
            agent_name: self.agent_name,
            received_from: self.received_from,
        })
    }
}

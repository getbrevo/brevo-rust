use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ConversationsClient {
    pub http_client: HttpClient,
}

impl ConversationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Sets the agent's status to online for 2-3 minutes. We recommend pinging this endpoint every minute for as long as the agent has to be considered online. You must provide either `agentId` alone, or all three of `agentEmail` + `agentName` + `receivedFrom`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .sets_agents_status_to_online_for23minutes(
    ///             &PostConversationsAgentOnlinePingRequest {
    ///                 agent_id: Some("d9nKoegKSjmCtyK78".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn sets_agents_status_to_online_for23minutes(
        &self,
        request: &PostConversationsAgentOnlinePingRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "conversations/agentOnlinePing",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send a message as an agent to an existing visitor's conversation. You must provide either `agentId` alone, or all three of `agentEmail` + `agentName` + `receivedFrom` to identify the agent.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .send_a_message_as_an_agent(
    ///             &PostConversationsMessagesRequest {
    ///                 agent_id: Some("d9nKoegKSjmCtyK78".to_string()),
    ///                 text: "Hello! How can I help you?".to_string(),
    ///                 visitor_id: "kZMvWhf8npAu3H6qd57w2Hv6nh6rnxvg".to_string(),
    ///                 agent_email: None,
    ///                 agent_name: None,
    ///                 received_from: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_a_message_as_an_agent(
        &self,
        request: &PostConversationsMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "conversations/messages",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a single message by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .get_a_message(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_a_message(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("conversations/messages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the text of a message sent by an agent. Only messages of type `agent` can be edited. The `text` and `html` fields of the message will be updated.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .update_a_message_sent_by_an_agent(
    ///             &"id".to_string(),
    ///             &PutConversationsMessagesIdRequest {
    ///                 text: "Good morning! How can I help you?".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_a_message_sent_by_an_agent(
        &self,
        id: &str,
        request: &PutConversationsMessagesIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("conversations/messages/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a message sent by an agent. Only messages of type `agent` can be deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .delete_a_message_sent_by_an_agent(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_a_message_sent_by_an_agent(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("conversations/messages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Send an automated (pushed) message to a visitor on behalf of an agent. Example use cases: order status updates, announcing new features in your web app, etc.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client.conversations.send_an_automated_message_to_a_visitor(&PostConversationsPushedMessagesRequest {
    ///         group_id: Some("PjRBMhWGen6aRHjif".to_string()),
    ///         text: "Your order has shipped! Here's your tracking number: 9114 5847 3325 9667 4328 88".to_string(),
    ///         visitor_id: "kZMvWhf8npAu3H6qd57w2Hv6nh6rnxvg".to_string(),
    ///         agent_id: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn send_an_automated_message_to_a_visitor(
        &self,
        request: &PostConversationsPushedMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "conversations/pushedMessages",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a single automated (pushed) message by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message sent previously
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .get_an_automated_message(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_an_automated_message(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("conversations/pushedMessages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the text of an automated (pushed) message. The `text` and `html` fields of the message will be updated.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client.conversations.update_an_automated_message(&"id".to_string(), &PutConversationsPushedMessagesIdRequest {
    ///         text: "Your order has shipped! Here's your tracking number: 9114 5847 4668 7775 9233 54".to_string()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn update_an_automated_message(
        &self,
        id: &str,
        request: &PutConversationsPushedMessagesIdRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsMessage, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("conversations/pushedMessages/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete an automated (pushed) message by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the message
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .delete_an_automated_message(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_an_automated_message(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("conversations/pushedMessages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Assigns a visitor to a specific agent group or removes them from their current group.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use brevo_rust::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = BrevoClient::new(config).expect("Failed to build client");
    ///     client
    ///         .conversations
    ///         .set_visitor_group_assignment(
    ///             &PutConversationsVisitorGroupRequest {
    ///                 group_id: serde_json::json!("PjRBMhWGen6aRHjif"),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_visitor_group_assignment(
        &self,
        request: &PutConversationsVisitorGroupRequest,
        options: Option<RequestOptions>,
    ) -> Result<PutConversationsVisitorGroupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "conversations/visitorGroup",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}

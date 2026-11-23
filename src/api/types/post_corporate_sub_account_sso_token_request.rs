pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateSubAccountSsoTokenRequest {
    /// User email of sub-account organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Id of the sub-account organization
    #[serde(default)]
    pub id: i64,
    /// **Set target after login success** * **automation** - Redirect
    /// to Automation after login * **email_campaign** - Redirect to
    /// Email Campaign after login * **contacts** - Redirect to Contacts
    /// after login * **landing_pages** - Redirect to Landing Pages
    /// after login * **email_transactional** - Redirect to Email
    /// Transactional after login * **senders** - Redirect to Senders
    /// after login * **sms_campaign** - Redirect to Sms Campaign after
    /// login * **sms_transactional** - Redirect to Sms Transactional
    /// after login
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<PostCorporateSubAccountSsoTokenRequestTarget>,
    /// Set the full target URL after login success. The user will land
    /// directly on this target URL after login
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PostCorporateSubAccountSsoTokenRequest {
    pub fn builder() -> PostCorporateSubAccountSsoTokenRequestBuilder {
        <PostCorporateSubAccountSsoTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateSubAccountSsoTokenRequestBuilder {
    email: Option<String>,
    id: Option<i64>,
    target: Option<PostCorporateSubAccountSsoTokenRequestTarget>,
    url: Option<String>,
}

impl PostCorporateSubAccountSsoTokenRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn target(mut self, value: PostCorporateSubAccountSsoTokenRequestTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateSubAccountSsoTokenRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostCorporateSubAccountSsoTokenRequestBuilder::id)
    pub fn build(self) -> Result<PostCorporateSubAccountSsoTokenRequest, BuildError> {
        Ok(PostCorporateSubAccountSsoTokenRequest {
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            target: self.target,
            url: self.url,
        })
    }
}

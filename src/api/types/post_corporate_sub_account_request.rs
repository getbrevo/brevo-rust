pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostCorporateSubAccountRequest {
    /// Set the name of the sub-account company
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    /// Email address for the organization
    #[serde(default)]
    pub email: String,
    /// Set the group(s) for the sub-account
    #[serde(rename = "groupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Set the language of the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<PostCorporateSubAccountRequestLanguage>,
    /// Set the timezone of the sub-account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl PostCorporateSubAccountRequest {
    pub fn builder() -> PostCorporateSubAccountRequestBuilder {
        <PostCorporateSubAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCorporateSubAccountRequestBuilder {
    company_name: Option<String>,
    email: Option<String>,
    group_ids: Option<Vec<String>>,
    language: Option<PostCorporateSubAccountRequestLanguage>,
    timezone: Option<String>,
}

impl PostCorporateSubAccountRequestBuilder {
    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn group_ids(mut self, value: Vec<String>) -> Self {
        self.group_ids = Some(value);
        self
    }

    pub fn language(mut self, value: PostCorporateSubAccountRequestLanguage) -> Self {
        self.language = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostCorporateSubAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_name`](PostCorporateSubAccountRequestBuilder::company_name)
    /// - [`email`](PostCorporateSubAccountRequestBuilder::email)
    pub fn build(self) -> Result<PostCorporateSubAccountRequest, BuildError> {
        Ok(PostCorporateSubAccountRequest {
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            group_ids: self.group_ids,
            language: self.language,
            timezone: self.timezone,
        })
    }
}

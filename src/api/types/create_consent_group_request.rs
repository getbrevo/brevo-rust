pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateConsentGroupRequest {
    /// Unique name for the consent group (max 255 characters)
    #[serde(default)]
    pub name: String,
    /// Optional description (max 500 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Controls how contacts are added to the group. `manual` — contacts are added explicitly via the API. `automatic` — contacts are added automatically at signup.
    #[serde(rename = "signupMode")]
    pub signup_mode: CreateConsentGroupRequestSignupMode,
    /// Optional list of contact list IDs. Contacts from these lists will be copied once into this consent group at creation time.
    #[serde(rename = "listIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_ids: Option<Vec<i64>>,
}

impl CreateConsentGroupRequest {
    pub fn builder() -> CreateConsentGroupRequestBuilder {
        <CreateConsentGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConsentGroupRequestBuilder {
    name: Option<String>,
    description: Option<String>,
    signup_mode: Option<CreateConsentGroupRequestSignupMode>,
    list_ids: Option<Vec<i64>>,
}

impl CreateConsentGroupRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn signup_mode(mut self, value: CreateConsentGroupRequestSignupMode) -> Self {
        self.signup_mode = Some(value);
        self
    }

    pub fn list_ids(mut self, value: Vec<i64>) -> Self {
        self.list_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateConsentGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateConsentGroupRequestBuilder::name)
    /// - [`signup_mode`](CreateConsentGroupRequestBuilder::signup_mode)
    pub fn build(self) -> Result<CreateConsentGroupRequest, BuildError> {
        Ok(CreateConsentGroupRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            signup_mode: self
                .signup_mode
                .ok_or_else(|| BuildError::missing_field("signup_mode"))?,
            list_ids: self.list_ids,
        })
    }
}

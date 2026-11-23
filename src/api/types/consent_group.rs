pub use crate::prelude::*;

/// A consent group representing a category of contact opt-in/opt-out preferences.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConsentGroup {
    /// Unique identifier for the consent group
    #[serde(default)]
    pub id: i64,
    /// Display name of the consent group
    #[serde(default)]
    pub name: String,
    /// Optional description of the consent group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Controls how contacts are added to the group. `manual` — contacts are added explicitly via the API. `automatic` — contacts are added automatically at signup.
    #[serde(rename = "signupMode")]
    pub signup_mode: ConsentGroupSignupMode,
    /// Number of contacts currently in this consent group
    #[serde(rename = "contactsCount")]
    #[serde(default)]
    pub contacts_count: i64,
    /// Whether this is a system-created default consent group
    #[serde(rename = "isDefault")]
    #[serde(default)]
    pub is_default: bool,
    /// UTC date-time when the consent group was created (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// UTC date-time when the consent group was last updated (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ConsentGroup {
    pub fn builder() -> ConsentGroupBuilder {
        <ConsentGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsentGroupBuilder {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    signup_mode: Option<ConsentGroupSignupMode>,
    contacts_count: Option<i64>,
    is_default: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ConsentGroupBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn signup_mode(mut self, value: ConsentGroupSignupMode) -> Self {
        self.signup_mode = Some(value);
        self
    }

    pub fn contacts_count(mut self, value: i64) -> Self {
        self.contacts_count = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsentGroup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConsentGroupBuilder::id)
    /// - [`name`](ConsentGroupBuilder::name)
    /// - [`signup_mode`](ConsentGroupBuilder::signup_mode)
    /// - [`contacts_count`](ConsentGroupBuilder::contacts_count)
    /// - [`is_default`](ConsentGroupBuilder::is_default)
    /// - [`created_at`](ConsentGroupBuilder::created_at)
    /// - [`updated_at`](ConsentGroupBuilder::updated_at)
    pub fn build(self) -> Result<ConsentGroup, BuildError> {
        Ok(ConsentGroup {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            signup_mode: self
                .signup_mode
                .ok_or_else(|| BuildError::missing_field("signup_mode"))?,
            contacts_count: self
                .contacts_count
                .ok_or_else(|| BuildError::missing_field("contacts_count"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

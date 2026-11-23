pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LoyaltyProgram {
    /// Loyalty Program unique identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Loyalty Program name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Loyalty Program description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Loyalty Program metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Current state of the loyalty program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<LoyaltyProgramState>,
    /// Loyalty Program subscription pool ID.
    #[serde(rename = "subscriptionPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pool_id: Option<String>,
    /// Loyalty Program subscription generator ID.
    #[serde(rename = "subscriptionGeneratorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_generator_id: Option<String>,
    /// Subscription code pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Number of available subscription codes.
    #[serde(rename = "codeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_count: Option<i64>,
    /// Associated document identifier.
    #[serde(rename = "documentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// Contact attribute name used for birthday tracking.
    #[serde(rename = "birthdayAttribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday_attribute: Option<String>,
    /// Timestamp when the loyalty program was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the loyalty program was last modified.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl LoyaltyProgram {
    pub fn builder() -> LoyaltyProgramBuilder {
        <LoyaltyProgramBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoyaltyProgramBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    meta: Option<HashMap<String, serde_json::Value>>,
    state: Option<LoyaltyProgramState>,
    subscription_pool_id: Option<String>,
    subscription_generator_id: Option<String>,
    pattern: Option<String>,
    code_count: Option<i64>,
    document_id: Option<String>,
    birthday_attribute: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl LoyaltyProgramBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn state(mut self, value: LoyaltyProgramState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn subscription_pool_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_pool_id = Some(value.into());
        self
    }

    pub fn subscription_generator_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_generator_id = Some(value.into());
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    pub fn code_count(mut self, value: i64) -> Self {
        self.code_count = Some(value);
        self
    }

    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn birthday_attribute(mut self, value: impl Into<String>) -> Self {
        self.birthday_attribute = Some(value.into());
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

    /// Consumes the builder and constructs a [`LoyaltyProgram`].
    pub fn build(self) -> Result<LoyaltyProgram, BuildError> {
        Ok(LoyaltyProgram {
            id: self.id,
            name: self.name,
            description: self.description,
            meta: self.meta,
            state: self.state,
            subscription_pool_id: self.subscription_pool_id,
            subscription_generator_id: self.subscription_generator_id,
            pattern: self.pattern,
            code_count: self.code_count,
            document_id: self.document_id,
            birthday_attribute: self.birthday_attribute,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

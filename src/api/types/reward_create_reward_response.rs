pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRewardResponse {
    /// Unique identifier for the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Loyalty program to which the reward belongs
    #[serde(rename = "loyaltyProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty_program_id: Option<String>,
    /// Name of the reward
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public description of the reward
    #[serde(rename = "publicDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_description: Option<String>,
    /// Public image URL of the reward
    #[serde(rename = "publicImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_image: Option<String>,
    /// Public name of the reward
    #[serde(rename = "publicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,
    /// Timestamp when the reward was created
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Timestamp when the reward was last updated
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl CreateRewardResponse {
    pub fn builder() -> CreateRewardResponseBuilder {
        <CreateRewardResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRewardResponseBuilder {
    id: Option<String>,
    loyalty_program_id: Option<String>,
    name: Option<String>,
    public_description: Option<String>,
    public_image: Option<String>,
    public_name: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl CreateRewardResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn loyalty_program_id(mut self, value: impl Into<String>) -> Self {
        self.loyalty_program_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn public_description(mut self, value: impl Into<String>) -> Self {
        self.public_description = Some(value.into());
        self
    }

    pub fn public_image(mut self, value: impl Into<String>) -> Self {
        self.public_image = Some(value.into());
        self
    }

    pub fn public_name(mut self, value: impl Into<String>) -> Self {
        self.public_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateRewardResponse`].
    pub fn build(self) -> Result<CreateRewardResponse, BuildError> {
        Ok(CreateRewardResponse {
            id: self.id,
            loyalty_program_id: self.loyalty_program_id,
            name: self.name,
            public_description: self.public_description,
            public_image: self.public_image,
            public_name: self.public_name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        })
    }
}

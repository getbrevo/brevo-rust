pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRewardRequest {
    /// Internal name of the reward
    #[serde(default)]
    pub name: String,
    /// Public facing description of the reward
    #[serde(rename = "publicDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_description: Option<String>,
    /// URL of the public image for the reward
    #[serde(rename = "publicImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_image: Option<String>,
    /// Public facing name of the reward
    #[serde(rename = "publicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,
}

impl CreateRewardRequest {
    pub fn builder() -> CreateRewardRequestBuilder {
        <CreateRewardRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRewardRequestBuilder {
    name: Option<String>,
    public_description: Option<String>,
    public_image: Option<String>,
    public_name: Option<String>,
}

impl CreateRewardRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreateRewardRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateRewardRequestBuilder::name)
    pub fn build(self) -> Result<CreateRewardRequest, BuildError> {
        Ok(CreateRewardRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            public_description: self.public_description,
            public_image: self.public_image,
            public_name: self.public_name,
        })
    }
}

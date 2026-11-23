pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEmailCampaignResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateEmailCampaignResponse {
    pub fn builder() -> CreateEmailCampaignResponseBuilder {
        <CreateEmailCampaignResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEmailCampaignResponseBuilder {
    id: Option<i64>,
}

impl CreateEmailCampaignResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEmailCampaignResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateEmailCampaignResponseBuilder::id)
    pub fn build(self) -> Result<CreateEmailCampaignResponse, BuildError> {
        Ok(CreateEmailCampaignResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

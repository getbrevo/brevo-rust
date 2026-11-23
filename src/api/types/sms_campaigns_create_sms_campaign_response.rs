pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSmsCampaignResponse {
    /// ID of the object created
    #[serde(default)]
    pub id: i64,
}

impl CreateSmsCampaignResponse {
    pub fn builder() -> CreateSmsCampaignResponseBuilder {
        <CreateSmsCampaignResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSmsCampaignResponseBuilder {
    id: Option<i64>,
}

impl CreateSmsCampaignResponseBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSmsCampaignResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSmsCampaignResponseBuilder::id)
    pub fn build(self) -> Result<CreateSmsCampaignResponse, BuildError> {
        Ok(CreateSmsCampaignResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

pub use crate::prelude::*;

/// Status of the campaign
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCampaignStatus {
    /// Note:- **replicateTemplate** status will be available **only for template type campaigns.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateCampaignStatusStatus>,
}

impl UpdateCampaignStatus {
    pub fn builder() -> UpdateCampaignStatusBuilder {
        <UpdateCampaignStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCampaignStatusBuilder {
    status: Option<UpdateCampaignStatusStatus>,
}

impl UpdateCampaignStatusBuilder {
    pub fn status(mut self, value: UpdateCampaignStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCampaignStatus`].
    pub fn build(self) -> Result<UpdateCampaignStatus, BuildError> {
        Ok(UpdateCampaignStatus {
            status: self.status,
        })
    }
}

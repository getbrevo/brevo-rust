pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAbTestCampaignResultResponseClickedLinks {
    #[serde(rename = "Version A")]
    #[serde(default)]
    pub version_a: AbTestVersionClicks,
    #[serde(rename = "Version B")]
    #[serde(default)]
    pub version_b: AbTestVersionClicks,
}

impl GetAbTestCampaignResultResponseClickedLinks {
    pub fn builder() -> GetAbTestCampaignResultResponseClickedLinksBuilder {
        <GetAbTestCampaignResultResponseClickedLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAbTestCampaignResultResponseClickedLinksBuilder {
    version_a: Option<AbTestVersionClicks>,
    version_b: Option<AbTestVersionClicks>,
}

impl GetAbTestCampaignResultResponseClickedLinksBuilder {
    pub fn version_a(mut self, value: AbTestVersionClicks) -> Self {
        self.version_a = Some(value);
        self
    }

    pub fn version_b(mut self, value: AbTestVersionClicks) -> Self {
        self.version_b = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAbTestCampaignResultResponseClickedLinks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version_a`](GetAbTestCampaignResultResponseClickedLinksBuilder::version_a)
    /// - [`version_b`](GetAbTestCampaignResultResponseClickedLinksBuilder::version_b)
    pub fn build(self) -> Result<GetAbTestCampaignResultResponseClickedLinks, BuildError> {
        Ok(GetAbTestCampaignResultResponseClickedLinks {
            version_a: self
                .version_a
                .ok_or_else(|| BuildError::missing_field("version_a"))?,
            version_b: self
                .version_b
                .ok_or_else(|| BuildError::missing_field("version_b"))?,
        })
    }
}

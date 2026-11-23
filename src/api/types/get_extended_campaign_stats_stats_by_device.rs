pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetExtendedCampaignStatsStatsByDevice {
    /// Statistics of the campaign on the basis of desktop devices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<HashMap<String, GetDeviceBrowserStats>>,
    /// Statistics of the campaign on the basis of mobile devices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<HashMap<String, GetDeviceBrowserStats>>,
    /// Statistics of the campaign on the basis of tablet devices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablet: Option<HashMap<String, GetDeviceBrowserStats>>,
    /// Statistics of the campaign on the basis of unknown devices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<HashMap<String, GetDeviceBrowserStats>>,
}

impl GetExtendedCampaignStatsStatsByDevice {
    pub fn builder() -> GetExtendedCampaignStatsStatsByDeviceBuilder {
        <GetExtendedCampaignStatsStatsByDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetExtendedCampaignStatsStatsByDeviceBuilder {
    desktop: Option<HashMap<String, GetDeviceBrowserStats>>,
    mobile: Option<HashMap<String, GetDeviceBrowserStats>>,
    tablet: Option<HashMap<String, GetDeviceBrowserStats>>,
    unknown: Option<HashMap<String, GetDeviceBrowserStats>>,
}

impl GetExtendedCampaignStatsStatsByDeviceBuilder {
    pub fn desktop(mut self, value: HashMap<String, GetDeviceBrowserStats>) -> Self {
        self.desktop = Some(value);
        self
    }

    pub fn mobile(mut self, value: HashMap<String, GetDeviceBrowserStats>) -> Self {
        self.mobile = Some(value);
        self
    }

    pub fn tablet(mut self, value: HashMap<String, GetDeviceBrowserStats>) -> Self {
        self.tablet = Some(value);
        self
    }

    pub fn unknown(mut self, value: HashMap<String, GetDeviceBrowserStats>) -> Self {
        self.unknown = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetExtendedCampaignStatsStatsByDevice`].
    pub fn build(self) -> Result<GetExtendedCampaignStatsStatsByDevice, BuildError> {
        Ok(GetExtendedCampaignStatsStatsByDevice {
            desktop: self.desktop,
            mobile: self.mobile,
            tablet: self.tablet,
            unknown: self.unknown,
        })
    }
}

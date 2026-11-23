pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchCompaniesLinkUnlinkIdRequest {
    /// Contact ids for contacts to be linked with company
    #[serde(rename = "linkContactIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_contact_ids: Option<Vec<i64>>,
    /// Deal ids for deals to be linked with company
    #[serde(rename = "linkDealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_deals_ids: Option<Vec<String>>,
    /// Contact ids for contacts to be unlinked from company
    #[serde(rename = "unlinkContactIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlink_contact_ids: Option<Vec<i64>>,
    /// Deal ids for deals to be unlinked from company
    #[serde(rename = "unlinkDealsIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlink_deals_ids: Option<Vec<String>>,
}

impl PatchCompaniesLinkUnlinkIdRequest {
    pub fn builder() -> PatchCompaniesLinkUnlinkIdRequestBuilder {
        <PatchCompaniesLinkUnlinkIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchCompaniesLinkUnlinkIdRequestBuilder {
    link_contact_ids: Option<Vec<i64>>,
    link_deals_ids: Option<Vec<String>>,
    unlink_contact_ids: Option<Vec<i64>>,
    unlink_deals_ids: Option<Vec<String>>,
}

impl PatchCompaniesLinkUnlinkIdRequestBuilder {
    pub fn link_contact_ids(mut self, value: Vec<i64>) -> Self {
        self.link_contact_ids = Some(value);
        self
    }

    pub fn link_deals_ids(mut self, value: Vec<String>) -> Self {
        self.link_deals_ids = Some(value);
        self
    }

    pub fn unlink_contact_ids(mut self, value: Vec<i64>) -> Self {
        self.unlink_contact_ids = Some(value);
        self
    }

    pub fn unlink_deals_ids(mut self, value: Vec<String>) -> Self {
        self.unlink_deals_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchCompaniesLinkUnlinkIdRequest`].
    pub fn build(self) -> Result<PatchCompaniesLinkUnlinkIdRequest, BuildError> {
        Ok(PatchCompaniesLinkUnlinkIdRequest {
            link_contact_ids: self.link_contact_ids,
            link_deals_ids: self.link_deals_ids,
            unlink_contact_ids: self.unlink_contact_ids,
            unlink_deals_ids: self.unlink_deals_ids,
        })
    }
}

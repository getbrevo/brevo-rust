pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchCrmDealsLinkUnlinkIdRequest {
    /// Company ids to be linked with deal
    #[serde(rename = "linkCompanyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_company_ids: Option<Vec<String>>,
    /// Contact ids for contacts to be linked with deal
    #[serde(rename = "linkContactIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_contact_ids: Option<Vec<i64>>,
    /// Company ids to be unlinked from deal
    #[serde(rename = "unlinkCompanyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlink_company_ids: Option<Vec<String>>,
    /// Contact ids for contacts to be unlinked from deal
    #[serde(rename = "unlinkContactIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlink_contact_ids: Option<Vec<i64>>,
}

impl PatchCrmDealsLinkUnlinkIdRequest {
    pub fn builder() -> PatchCrmDealsLinkUnlinkIdRequestBuilder {
        <PatchCrmDealsLinkUnlinkIdRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchCrmDealsLinkUnlinkIdRequestBuilder {
    link_company_ids: Option<Vec<String>>,
    link_contact_ids: Option<Vec<i64>>,
    unlink_company_ids: Option<Vec<String>>,
    unlink_contact_ids: Option<Vec<i64>>,
}

impl PatchCrmDealsLinkUnlinkIdRequestBuilder {
    pub fn link_company_ids(mut self, value: Vec<String>) -> Self {
        self.link_company_ids = Some(value);
        self
    }

    pub fn link_contact_ids(mut self, value: Vec<i64>) -> Self {
        self.link_contact_ids = Some(value);
        self
    }

    pub fn unlink_company_ids(mut self, value: Vec<String>) -> Self {
        self.unlink_company_ids = Some(value);
        self
    }

    pub fn unlink_contact_ids(mut self, value: Vec<i64>) -> Self {
        self.unlink_contact_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchCrmDealsLinkUnlinkIdRequest`].
    pub fn build(self) -> Result<PatchCrmDealsLinkUnlinkIdRequest, BuildError> {
        Ok(PatchCrmDealsLinkUnlinkIdRequest {
            link_company_ids: self.link_company_ids,
            link_contact_ids: self.link_contact_ids,
            unlink_company_ids: self.unlink_company_ids,
            unlink_contact_ids: self.unlink_contact_ids,
        })
    }
}

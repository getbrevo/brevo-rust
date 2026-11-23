pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateUserEmailPermissionsRequest {
    /// All access to the features
    #[serde(default)]
    pub all_features_access: bool,
    #[serde(default)]
    pub privileges: Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItem>,
}

impl PutCorporateUserEmailPermissionsRequest {
    pub fn builder() -> PutCorporateUserEmailPermissionsRequestBuilder {
        <PutCorporateUserEmailPermissionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateUserEmailPermissionsRequestBuilder {
    all_features_access: Option<bool>,
    privileges: Option<Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItem>>,
}

impl PutCorporateUserEmailPermissionsRequestBuilder {
    pub fn all_features_access(mut self, value: bool) -> Self {
        self.all_features_access = Some(value);
        self
    }

    pub fn privileges(
        mut self,
        value: Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItem>,
    ) -> Self {
        self.privileges = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateUserEmailPermissionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`all_features_access`](PutCorporateUserEmailPermissionsRequestBuilder::all_features_access)
    /// - [`privileges`](PutCorporateUserEmailPermissionsRequestBuilder::privileges)
    pub fn build(self) -> Result<PutCorporateUserEmailPermissionsRequest, BuildError> {
        Ok(PutCorporateUserEmailPermissionsRequest {
            all_features_access: self
                .all_features_access
                .ok_or_else(|| BuildError::missing_field("all_features_access"))?,
            privileges: self
                .privileges
                .ok_or_else(|| BuildError::missing_field("privileges"))?,
        })
    }
}

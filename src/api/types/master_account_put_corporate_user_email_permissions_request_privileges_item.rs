pub use crate::prelude::*;

/// Permission on features
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    /// feature name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature>,
    /// Permission for the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions:
        Option<Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem>>,
}

impl PutCorporateUserEmailPermissionsRequestPrivilegesItem {
    pub fn builder() -> PutCorporateUserEmailPermissionsRequestPrivilegesItemBuilder {
        <PutCorporateUserEmailPermissionsRequestPrivilegesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutCorporateUserEmailPermissionsRequestPrivilegesItemBuilder {
    feature: Option<PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature>,
    permissions: Option<Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem>>,
}

impl PutCorporateUserEmailPermissionsRequestPrivilegesItemBuilder {
    pub fn feature(
        mut self,
        value: PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature,
    ) -> Self {
        self.feature = Some(value);
        self
    }

    pub fn permissions(
        mut self,
        value: Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem>,
    ) -> Self {
        self.permissions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutCorporateUserEmailPermissionsRequestPrivilegesItem`].
    pub fn build(
        self,
    ) -> Result<PutCorporateUserEmailPermissionsRequestPrivilegesItem, BuildError> {
        Ok(PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: self.feature,
            permissions: self.permissions,
        })
    }
}

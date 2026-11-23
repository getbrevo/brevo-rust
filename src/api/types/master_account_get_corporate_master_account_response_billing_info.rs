pub use crate::prelude::*;

/// Billing details of the master account organization
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateMasterAccountResponseBillingInfo {
    /// Billing address of master account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<GetCorporateMasterAccountResponseBillingInfoAddress>,
    /// Company name of master account
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Billing email id of master account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Billing name of master account holder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<GetCorporateMasterAccountResponseBillingInfoName>,
}

impl GetCorporateMasterAccountResponseBillingInfo {
    pub fn builder() -> GetCorporateMasterAccountResponseBillingInfoBuilder {
        <GetCorporateMasterAccountResponseBillingInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateMasterAccountResponseBillingInfoBuilder {
    address: Option<GetCorporateMasterAccountResponseBillingInfoAddress>,
    company_name: Option<String>,
    email: Option<String>,
    name: Option<GetCorporateMasterAccountResponseBillingInfoName>,
}

impl GetCorporateMasterAccountResponseBillingInfoBuilder {
    pub fn address(mut self, value: GetCorporateMasterAccountResponseBillingInfoAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: GetCorporateMasterAccountResponseBillingInfoName) -> Self {
        self.name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateMasterAccountResponseBillingInfo`].
    pub fn build(self) -> Result<GetCorporateMasterAccountResponseBillingInfo, BuildError> {
        Ok(GetCorporateMasterAccountResponseBillingInfo {
            address: self.address,
            company_name: self.company_name,
            email: self.email,
            name: self.name,
        })
    }
}

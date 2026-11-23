pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateGroupIdResponseSubAccountsItem {
    /// Name of the sub-account organzation
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Creation date of the sub-account organzation
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Id of the sub-account organzation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl GetCorporateGroupIdResponseSubAccountsItem {
    pub fn builder() -> GetCorporateGroupIdResponseSubAccountsItemBuilder {
        <GetCorporateGroupIdResponseSubAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateGroupIdResponseSubAccountsItemBuilder {
    company_name: Option<String>,
    created_at: Option<String>,
    id: Option<i64>,
}

impl GetCorporateGroupIdResponseSubAccountsItemBuilder {
    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateGroupIdResponseSubAccountsItem`].
    pub fn build(self) -> Result<GetCorporateGroupIdResponseSubAccountsItem, BuildError> {
        Ok(GetCorporateGroupIdResponseSubAccountsItem {
            company_name: self.company_name,
            created_at: self.created_at,
            id: self.id,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateSubAccountResponseSubAccountsItem {
    /// Whether the sub-account is active or not
    #[serde(default)]
    pub active: bool,
    /// Name of the sub-account company
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    /// Timestamp when the sub-account was created
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: i64,
    /// Group details
    #[serde(default)]
    pub groups: Vec<GetCorporateSubAccountResponseSubAccountsItemGroupsItem>,
    /// id of the sub-account
    #[serde(default)]
    pub id: i64,
}

impl GetCorporateSubAccountResponseSubAccountsItem {
    pub fn builder() -> GetCorporateSubAccountResponseSubAccountsItemBuilder {
        <GetCorporateSubAccountResponseSubAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateSubAccountResponseSubAccountsItemBuilder {
    active: Option<bool>,
    company_name: Option<String>,
    created_at: Option<i64>,
    groups: Option<Vec<GetCorporateSubAccountResponseSubAccountsItemGroupsItem>>,
    id: Option<i64>,
}

impl GetCorporateSubAccountResponseSubAccountsItemBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn groups(
        mut self,
        value: Vec<GetCorporateSubAccountResponseSubAccountsItemGroupsItem>,
    ) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateSubAccountResponseSubAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](GetCorporateSubAccountResponseSubAccountsItemBuilder::active)
    /// - [`company_name`](GetCorporateSubAccountResponseSubAccountsItemBuilder::company_name)
    /// - [`created_at`](GetCorporateSubAccountResponseSubAccountsItemBuilder::created_at)
    /// - [`groups`](GetCorporateSubAccountResponseSubAccountsItemBuilder::groups)
    /// - [`id`](GetCorporateSubAccountResponseSubAccountsItemBuilder::id)
    pub fn build(self) -> Result<GetCorporateSubAccountResponseSubAccountsItem, BuildError> {
        Ok(GetCorporateSubAccountResponseSubAccountsItem {
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

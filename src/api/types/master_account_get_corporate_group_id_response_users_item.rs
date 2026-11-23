pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCorporateGroupIdResponseUsersItem {
    /// Email address of the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// First name of the user
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the user
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl GetCorporateGroupIdResponseUsersItem {
    pub fn builder() -> GetCorporateGroupIdResponseUsersItemBuilder {
        <GetCorporateGroupIdResponseUsersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCorporateGroupIdResponseUsersItemBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl GetCorporateGroupIdResponseUsersItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCorporateGroupIdResponseUsersItem`].
    pub fn build(self) -> Result<GetCorporateGroupIdResponseUsersItem, BuildError> {
        Ok(GetCorporateGroupIdResponseUsersItem {
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
        })
    }
}

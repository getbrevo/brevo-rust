pub use crate::prelude::*;

/// Information about who created the domain
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDomainsResponseDomainsItemCreator {
    /// Creator's user ID
    #[serde(default)]
    pub id: String,
    /// Creator's email address
    #[serde(default)]
    pub email: String,
    /// Domain creation date
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub creation_date: DateTime<FixedOffset>,
}

impl GetDomainsResponseDomainsItemCreator {
    pub fn builder() -> GetDomainsResponseDomainsItemCreatorBuilder {
        <GetDomainsResponseDomainsItemCreatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDomainsResponseDomainsItemCreatorBuilder {
    id: Option<String>,
    email: Option<String>,
    creation_date: Option<DateTime<FixedOffset>>,
}

impl GetDomainsResponseDomainsItemCreatorBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn creation_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.creation_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDomainsResponseDomainsItemCreator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetDomainsResponseDomainsItemCreatorBuilder::id)
    /// - [`email`](GetDomainsResponseDomainsItemCreatorBuilder::email)
    /// - [`creation_date`](GetDomainsResponseDomainsItemCreatorBuilder::creation_date)
    pub fn build(self) -> Result<GetDomainsResponseDomainsItemCreator, BuildError> {
        Ok(GetDomainsResponseDomainsItemCreator {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            creation_date: self
                .creation_date
                .ok_or_else(|| BuildError::missing_field("creation_date"))?,
        })
    }
}

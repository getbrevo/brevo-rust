pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSendersResponseSendersItem {
    /// Status of sender (true=activated, false=deactivated)
    #[serde(default)]
    pub active: bool,
    /// From Email associated to the sender
    #[serde(default)]
    pub email: String,
    /// Id of the sender
    #[serde(default)]
    pub id: i64,
    /// List of dedicated IP(s) available in the account. This
    /// data is displayed only for dedicated IPs. For standard accounts,
    /// this will be an empty array.
    #[serde(default)]
    pub ips: Vec<GetSendersResponseSendersItemIpsItem>,
    /// From Name associated to the sender
    #[serde(default)]
    pub name: String,
}

impl GetSendersResponseSendersItem {
    pub fn builder() -> GetSendersResponseSendersItemBuilder {
        <GetSendersResponseSendersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSendersResponseSendersItemBuilder {
    active: Option<bool>,
    email: Option<String>,
    id: Option<i64>,
    ips: Option<Vec<GetSendersResponseSendersItemIpsItem>>,
    name: Option<String>,
}

impl GetSendersResponseSendersItemBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn ips(mut self, value: Vec<GetSendersResponseSendersItemIpsItem>) -> Self {
        self.ips = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSendersResponseSendersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](GetSendersResponseSendersItemBuilder::active)
    /// - [`email`](GetSendersResponseSendersItemBuilder::email)
    /// - [`id`](GetSendersResponseSendersItemBuilder::id)
    /// - [`ips`](GetSendersResponseSendersItemBuilder::ips)
    /// - [`name`](GetSendersResponseSendersItemBuilder::name)
    pub fn build(self) -> Result<GetSendersResponseSendersItem, BuildError> {
        Ok(GetSendersResponseSendersItem {
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            ips: self.ips.ok_or_else(|| BuildError::missing_field("ips"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

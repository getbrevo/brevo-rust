pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSenderRequestIpsItem {
    /// Domain of the IP
    #[serde(default)]
    pub domain: String,
    /// Dedicated IP available in your account
    #[serde(default)]
    pub ip: String,
    /// Weight to apply to the IP. Sum of all IP weights must be
    /// 100. Should be passed for either ALL or NONE of the IPs.
    /// If it's not passed, the sending will be equally balanced
    /// on all IPs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

impl UpdateSenderRequestIpsItem {
    pub fn builder() -> UpdateSenderRequestIpsItemBuilder {
        <UpdateSenderRequestIpsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSenderRequestIpsItemBuilder {
    domain: Option<String>,
    ip: Option<String>,
    weight: Option<i64>,
}

impl UpdateSenderRequestIpsItemBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn ip(mut self, value: impl Into<String>) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSenderRequestIpsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](UpdateSenderRequestIpsItemBuilder::domain)
    /// - [`ip`](UpdateSenderRequestIpsItemBuilder::ip)
    pub fn build(self) -> Result<UpdateSenderRequestIpsItem, BuildError> {
        Ok(UpdateSenderRequestIpsItem {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
            weight: self.weight,
        })
    }
}

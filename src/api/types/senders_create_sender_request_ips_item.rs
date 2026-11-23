pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSenderRequestIpsItem {
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

impl CreateSenderRequestIpsItem {
    pub fn builder() -> CreateSenderRequestIpsItemBuilder {
        <CreateSenderRequestIpsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSenderRequestIpsItemBuilder {
    domain: Option<String>,
    ip: Option<String>,
    weight: Option<i64>,
}

impl CreateSenderRequestIpsItemBuilder {
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

    /// Consumes the builder and constructs a [`CreateSenderRequestIpsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](CreateSenderRequestIpsItemBuilder::domain)
    /// - [`ip`](CreateSenderRequestIpsItemBuilder::ip)
    pub fn build(self) -> Result<CreateSenderRequestIpsItem, BuildError> {
        Ok(CreateSenderRequestIpsItem {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            ip: self.ip.ok_or_else(|| BuildError::missing_field("ip"))?,
            weight: self.weight,
        })
    }
}

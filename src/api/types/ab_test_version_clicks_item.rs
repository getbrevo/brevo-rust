pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AbTestVersionClicksItem {
    /// Percentage of clicks of link with respect to total clicks
    #[serde(rename = "clickRate")]
    #[serde(default)]
    pub click_rate: String,
    /// Number of times a link is clicked
    #[serde(rename = "clicksCount")]
    #[serde(default)]
    pub clicks_count: i64,
    /// URL of the link
    #[serde(default)]
    pub link: String,
}

impl AbTestVersionClicksItem {
    pub fn builder() -> AbTestVersionClicksItemBuilder {
        <AbTestVersionClicksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AbTestVersionClicksItemBuilder {
    click_rate: Option<String>,
    clicks_count: Option<i64>,
    link: Option<String>,
}

impl AbTestVersionClicksItemBuilder {
    pub fn click_rate(mut self, value: impl Into<String>) -> Self {
        self.click_rate = Some(value.into());
        self
    }

    pub fn clicks_count(mut self, value: i64) -> Self {
        self.clicks_count = Some(value);
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AbTestVersionClicksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`click_rate`](AbTestVersionClicksItemBuilder::click_rate)
    /// - [`clicks_count`](AbTestVersionClicksItemBuilder::clicks_count)
    /// - [`link`](AbTestVersionClicksItemBuilder::link)
    pub fn build(self) -> Result<AbTestVersionClicksItem, BuildError> {
        Ok(AbTestVersionClicksItem {
            click_rate: self
                .click_rate
                .ok_or_else(|| BuildError::missing_field("click_rate"))?,
            clicks_count: self
                .clicks_count
                .ok_or_else(|| BuildError::missing_field("clicks_count"))?,
            link: self.link.ok_or_else(|| BuildError::missing_field("link"))?,
        })
    }
}

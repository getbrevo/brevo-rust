pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSegmentsResponseSegmentsItem {
    /// Name of the Segment Category
    #[serde(rename = "categoryName")]
    #[serde(default)]
    pub category_name: String,
    /// ID of the segment
    #[serde(default)]
    pub id: i64,
    /// Name of the Segment
    #[serde(rename = "segmentName")]
    #[serde(default)]
    pub segment_name: String,
    /// Last update UTC date-time of the segment (YYYY-MM-DDTHH:mm:ss.SSSZ)
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl GetSegmentsResponseSegmentsItem {
    pub fn builder() -> GetSegmentsResponseSegmentsItemBuilder {
        <GetSegmentsResponseSegmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSegmentsResponseSegmentsItemBuilder {
    category_name: Option<String>,
    id: Option<i64>,
    segment_name: Option<String>,
    updated_at: Option<String>,
}

impl GetSegmentsResponseSegmentsItemBuilder {
    pub fn category_name(mut self, value: impl Into<String>) -> Self {
        self.category_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn segment_name(mut self, value: impl Into<String>) -> Self {
        self.segment_name = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetSegmentsResponseSegmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category_name`](GetSegmentsResponseSegmentsItemBuilder::category_name)
    /// - [`id`](GetSegmentsResponseSegmentsItemBuilder::id)
    /// - [`segment_name`](GetSegmentsResponseSegmentsItemBuilder::segment_name)
    pub fn build(self) -> Result<GetSegmentsResponseSegmentsItem, BuildError> {
        Ok(GetSegmentsResponseSegmentsItem {
            category_name: self
                .category_name
                .ok_or_else(|| BuildError::missing_field("category_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            segment_name: self
                .segment_name
                .ok_or_else(|| BuildError::missing_field("segment_name"))?,
            updated_at: self.updated_at,
        })
    }
}

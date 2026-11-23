pub use crate::prelude::*;

/// Query parameters for getAssociatedRecords
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAssociatedRecordsQueryRequest {
    /// Internal Brevo ID of the source record. Must be a positive integer. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// External ID of the source record in your system. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_id: Option<String>,
    /// Email address of the source contact. Only accepted when `object_type` is `contact`. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number of the source contact, including the country code. It may be given with or without a leading `+`; percent-encode the `+` as `%2B`, because a literal `+` in a query string is read as a space. Only accepted when `object_type` is `contact`. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<String>,
    /// Restricts the response to the given associated object types. Repeat the parameter to request several types, for example `?type=contact&type=garage`. Maximum 5 types per call. Associations of every type are returned when omitted.
    #[serde(default)]
    pub r#type: Vec<Option<String>>,
    /// Number of records to skip before the first record of the page. Defaults to 0. Increase by 20 to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl GetAssociatedRecordsQueryRequest {
    pub fn builder() -> GetAssociatedRecordsQueryRequestBuilder {
        <GetAssociatedRecordsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAssociatedRecordsQueryRequestBuilder {
    id: Option<i64>,
    ext_id: Option<String>,
    email: Option<String>,
    sms: Option<String>,
    r#type: Option<Vec<Option<String>>>,
    offset: Option<i64>,
}

impl GetAssociatedRecordsQueryRequestBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn ext_id(mut self, value: impl Into<String>) -> Self {
        self.ext_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn sms(mut self, value: impl Into<String>) -> Self {
        self.sms = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: Vec<Option<String>>) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAssociatedRecordsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](GetAssociatedRecordsQueryRequestBuilder::r#type)
    pub fn build(self) -> Result<GetAssociatedRecordsQueryRequest, BuildError> {
        Ok(GetAssociatedRecordsQueryRequest {
            id: self.id,
            ext_id: self.ext_id,
            email: self.email,
            sms: self.sms,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            offset: self.offset,
        })
    }
}

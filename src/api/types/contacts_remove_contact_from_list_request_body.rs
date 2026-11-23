pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RemoveContactFromListRequestBody {
    RemoveContactFromListRequestBodyEmails(RemoveContactFromListRequestBodyEmails),

    RemoveContactFromListRequestBodyIds(RemoveContactFromListRequestBodyIds),

    RemoveContactFromListRequestBodyAll(RemoveContactFromListRequestBodyAll),

    RemoveContactFromListRequestBodyExtIds(RemoveContactFromListRequestBodyExtIds),
}

impl RemoveContactFromListRequestBody {
    pub fn is_remove_contact_from_list_request_body_emails(&self) -> bool {
        matches!(self, Self::RemoveContactFromListRequestBodyEmails(_))
    }

    pub fn is_remove_contact_from_list_request_body_ids(&self) -> bool {
        matches!(self, Self::RemoveContactFromListRequestBodyIds(_))
    }

    pub fn is_remove_contact_from_list_request_body_all(&self) -> bool {
        matches!(self, Self::RemoveContactFromListRequestBodyAll(_))
    }

    pub fn is_remove_contact_from_list_request_body_ext_ids(&self) -> bool {
        matches!(self, Self::RemoveContactFromListRequestBodyExtIds(_))
    }

    pub fn as_remove_contact_from_list_request_body_emails(
        &self,
    ) -> Option<&RemoveContactFromListRequestBodyEmails> {
        match self {
            Self::RemoveContactFromListRequestBodyEmails(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_remove_contact_from_list_request_body_emails(
        self,
    ) -> Option<RemoveContactFromListRequestBodyEmails> {
        match self {
            Self::RemoveContactFromListRequestBodyEmails(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_remove_contact_from_list_request_body_ids(
        &self,
    ) -> Option<&RemoveContactFromListRequestBodyIds> {
        match self {
            Self::RemoveContactFromListRequestBodyIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_remove_contact_from_list_request_body_ids(
        self,
    ) -> Option<RemoveContactFromListRequestBodyIds> {
        match self {
            Self::RemoveContactFromListRequestBodyIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_remove_contact_from_list_request_body_all(
        &self,
    ) -> Option<&RemoveContactFromListRequestBodyAll> {
        match self {
            Self::RemoveContactFromListRequestBodyAll(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_remove_contact_from_list_request_body_all(
        self,
    ) -> Option<RemoveContactFromListRequestBodyAll> {
        match self {
            Self::RemoveContactFromListRequestBodyAll(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_remove_contact_from_list_request_body_ext_ids(
        &self,
    ) -> Option<&RemoveContactFromListRequestBodyExtIds> {
        match self {
            Self::RemoveContactFromListRequestBodyExtIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_remove_contact_from_list_request_body_ext_ids(
        self,
    ) -> Option<RemoveContactFromListRequestBodyExtIds> {
        match self {
            Self::RemoveContactFromListRequestBodyExtIds(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for RemoveContactFromListRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RemoveContactFromListRequestBodyEmails(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::RemoveContactFromListRequestBodyIds(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::RemoveContactFromListRequestBodyAll(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::RemoveContactFromListRequestBodyExtIds(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

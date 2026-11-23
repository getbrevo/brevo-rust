pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AddContactToListRequestBody {
    AddContactToListRequestBodyEmails(AddContactToListRequestBodyEmails),

    AddContactToListRequestBodyIds(AddContactToListRequestBodyIds),

    AddContactToListRequestBodyExtIds(AddContactToListRequestBodyExtIds),
}

impl AddContactToListRequestBody {
    pub fn is_add_contact_to_list_request_body_emails(&self) -> bool {
        matches!(self, Self::AddContactToListRequestBodyEmails(_))
    }

    pub fn is_add_contact_to_list_request_body_ids(&self) -> bool {
        matches!(self, Self::AddContactToListRequestBodyIds(_))
    }

    pub fn is_add_contact_to_list_request_body_ext_ids(&self) -> bool {
        matches!(self, Self::AddContactToListRequestBodyExtIds(_))
    }

    pub fn as_add_contact_to_list_request_body_emails(
        &self,
    ) -> Option<&AddContactToListRequestBodyEmails> {
        match self {
            Self::AddContactToListRequestBodyEmails(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_add_contact_to_list_request_body_emails(
        self,
    ) -> Option<AddContactToListRequestBodyEmails> {
        match self {
            Self::AddContactToListRequestBodyEmails(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_add_contact_to_list_request_body_ids(
        &self,
    ) -> Option<&AddContactToListRequestBodyIds> {
        match self {
            Self::AddContactToListRequestBodyIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_add_contact_to_list_request_body_ids(
        self,
    ) -> Option<AddContactToListRequestBodyIds> {
        match self {
            Self::AddContactToListRequestBodyIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_add_contact_to_list_request_body_ext_ids(
        &self,
    ) -> Option<&AddContactToListRequestBodyExtIds> {
        match self {
            Self::AddContactToListRequestBodyExtIds(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_add_contact_to_list_request_body_ext_ids(
        self,
    ) -> Option<AddContactToListRequestBodyExtIds> {
        match self {
            Self::AddContactToListRequestBodyExtIds(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AddContactToListRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddContactToListRequestBodyEmails(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AddContactToListRequestBodyIds(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AddContactToListRequestBodyExtIds(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

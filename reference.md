# Reference
## Account
<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">get_account</a>() -> Result&lt;GetAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves details of your Brevo account.

**Use this to:**
- Get account information (email, name, company, address)
- Check plan details (type, credits, expiration)
- Get relay information (for transactional emails)
- Check Marketing Automation status
- Access organization and user identifiers

**Key information returned:**
- Complete account details (organization ID, user ID, company information)
- Address and contact information
- Plan configurations and credit allocations across different verticals
- Marketing Automation settings and tracker key (when enabled)
- SMTP relay configuration for transactional emails
- Enterprise features availability status

**Important considerations:**
- Provides comprehensive account overview for billing and configuration management
- Essential for understanding current plan limitations and feature availability
- Marketing Automation key is only returned when Marketing Automation is enabled on the account
- Plan verticals show detailed breakdown across Marketing, Chat, and CRM categories (only returned when plan verticals are available)
- Relay configuration crucial for transactional email setup and deliverability
- Enterprise status determines access to advanced features and sub-account management
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.account.get_account(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account.<a href="/src/api/resources/account/client.rs">get_account_activity</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, email: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetAccountActivityResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves user activity logs from your organization for security monitoring and audit compliance.

Use this to:
- Monitor user login activities and access patterns
- Track account modifications and configuration changes
- Generate security audit reports and compliance documentation
- Investigate suspicious activities and unauthorized access
- Monitor team member actions and account usage

Key information returned:
- Complete user activity details and timestamps
- User identification (email, IP address, browser)
- Action types and activity descriptions
- Security-relevant events and access logs
- Historical activity data for audit trails

Note: Requires Enterprise plan for access to organization activity logs.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .account
        .get_account_activity(
            &GetAccountActivityQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_date:** `Option<String>` 

Mandatory if endDate is used. Enter start date in UTC date (YYYY-MM-DD)
format to filter the activity in your account. Maximum time period that
can be selected is one month. Additionally, you can retrieve activity
logs from the past 12 months from the date of your search.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` 

Mandatory if startDate is used. Enter end date in UTC date (YYYY-MM-DD)
format to filter the activity in your account. Maximum time period that
can be selected is one month.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Enter the user's email address to filter their activity in the account.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## MasterAccount
<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">create_a_new_group_of_sub_accounts</a>(request: PostCorporateGroupRequest) -> Result&lt;PostCorporateGroupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows to create a group of sub-accounts
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .create_a_new_group_of_sub_accounts(
            &PostCorporateGroupRequest {
                group_name: "My group".to_string(),
                sub_account_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_name:** `String` — The name of the group of sub-accounts
    
</dd>
</dl>

<dl>
<dd>

**sub_account_ids:** `Option<Vec<String>>` — Pass the list of sub-account Ids to be included in the group
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">delete_sub_account_from_group</a>(group_id: String, request: PutCorporateGroupUnlinkGroupIdSubAccountsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows you to remove a sub-organization from a group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .delete_sub_account_from_group(
            &"groupId".to_string(),
            &PutCorporateGroupUnlinkGroupIdSubAccountsRequest {
                sub_account_ids: vec![423432, 234323, 87678],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `String` — Group id
    
</dd>
</dl>

<dl>
<dd>

**sub_account_ids:** `Vec<String>` — List of sub-account ids
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">get_a_group_details</a>(id: String) -> Result&lt;GetCorporateGroupIdResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows you to retrieve a specific group’s information such as
the list of sub-organizations and the user associated with the group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .get_a_group_details(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the group of sub-organization
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">update_a_group_of_sub_accounts</a>(id: String, request: PutCorporateGroupIdRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows to update a group of sub-accounts
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .update_a_group_of_sub_accounts(
            &"id".to_string(),
            &PutCorporateGroupIdRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the group
    
</dd>
</dl>

<dl>
<dd>

**group_name:** `Option<String>` — The name of the group of sub-accounts
    
</dd>
</dl>

<dl>
<dd>

**sub_account_ids:** `Option<Vec<String>>` — Pass the list of sub-account Ids to be included in the group
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">delete_a_group</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows you to delete a group of sub-organizations. When a
group is deleted, the sub-organizations are no longer part of this group.
The users associated with the group are no longer associated with the group
once deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .delete_a_group(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the group
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">get_sub_account_groups</a>() -> Result&lt;Vec&lt;GetSubAccountGroupsResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows you to list all groups created on your Admin account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.master_account.get_sub_account_groups(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">get_corporate_invited_users_list</a>(type_: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;GetCorporateInvitedUsersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows you to list all Admin users of your Admin account. You
can filter users by type (active or pending) and paginate results using
offset and limit.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .get_corporate_invited_users_list(
            &GetCorporateInvitedUsersListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<String>` 

User type (active | pending). This is required if offset is provided for
limited result.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` 

Page number for the result set. This is optional, default value will be
the 1st page.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 

Number of users to be displayed on each page. This is optional, the
default limit is 20, but max allowed limit is 100.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">list_of_all_i_ps</a>() -> Result&lt;Vec&lt;GetCorporateIpResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows you to retrieve the list of active IPs on your Admin
account
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.master_account.list_of_all_i_ps(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">get_the_details_of_requested_master_account</a>() -> Result&lt;GetCorporateMasterAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will provide the details of the master account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .get_the_details_of_requested_master_account(None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">generate_sso_token_to_access_admin_account</a>(request: PostCorporateSsoTokenRequest) -> Result&lt;GetSsoToken, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint generates an SSO token to authenticate and access the admin
account using the endpoint
https://account-app.brevo.com/account/login/corporate/sso/[token], where
[token] will be replaced by the actual token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .generate_sso_token_to_access_admin_account(
            &PostCorporateSsoTokenRequest {
                email: "vipin+ent-user@brevo.com".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — User email of admin account
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">get_the_list_of_all_the_sub_accounts_of_the_master_account</a>(offset: Option&lt;i64&gt;, limit: Option&lt;i64&gt;) -> Result&lt;GetCorporateSubAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will provide the list all the sub-accounts of the master
account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .get_the_list_of_all_the_sub_accounts_of_the_master_account(
            &GetTheListOfAllTheSubAccountsOfTheMasterAccountQueryRequest {
                offset: 1,
                limit: 1,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**offset:** `i64` — Index of the first sub-account in the page
    
</dd>
</dl>

<dl>
<dd>

**limit:** `i64` — Number of sub-accounts to be displayed on each page
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">create_a_new_sub_account_under_a_master_account</a>(request: PostCorporateSubAccountRequest) -> Result&lt;PostCorporateSubAccountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will create a new sub-account under a master account
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .create_a_new_sub_account_under_a_master_account(
            &PostCorporateSubAccountRequest {
                company_name: "Test Sub-account".to_string(),
                email: "test-sub@example.com".to_string(),
                group_ids: Some(vec![
                    "5f8f8c3b5f56a02d4433b3a7".to_string(),
                    "5f8f8c3b5f56a02d4433b3a8".to_string(),
                ]),
                language: Some(PostCorporateSubAccountRequestLanguage::Fr),
                timezone: Some("Europe/Paris".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_name:** `String` — Set the name of the sub-account company
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` — Email address for the organization
    
</dd>
</dl>

<dl>
<dd>

**group_ids:** `Option<Vec<String>>` — Set the group(s) for the sub-account
    
</dd>
</dl>

<dl>
<dd>

**language:** `Option<PostCorporateSubAccountRequestLanguage>` — Set the language of the sub-account
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — Set the timezone of the sub-account
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">associate_an_ip_to_sub_accounts</a>(request: PostCorporateSubAccountIpAssociateRequest) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows to associate an IP to sub-accounts
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .associate_an_ip_to_sub_accounts(
            &PostCorporateSubAccountIpAssociateRequest {
                ids: vec![234322, 325553, 893432],
                ip: "103.11.32.88".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ids:** `Vec<String>` 

Pass the list of sub-account Ids to be associated with the IP
address
    
</dd>
</dl>

<dl>
<dd>

**ip:** `String` — IP address
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">dissociate_an_ip_to_sub_accounts</a>(request: PutCorporateSubAccountIpDissociateRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows to dissociate an IP from sub-accounts
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .dissociate_an_ip_to_sub_accounts(
            &PutCorporateSubAccountIpDissociateRequest {
                ids: vec![234322, 325553, 893432],
                ip: "103.11.32.88".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ids:** `Vec<String>` 

Pass the list of sub-account Ids to be dissociated from the IP
address
    
</dd>
</dl>

<dl>
<dd>

**ip:** `String` — IP address
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">create_an_api_key_for_a_sub_account</a>(request: PostCorporateSubAccountKeyRequest) -> Result&lt;PostCorporateSubAccountKeyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will generate an API v3 key for a sub-account
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .create_an_api_key_for_a_sub_account(
            &PostCorporateSubAccountKeyRequest {
                id: 3232323,
                name: "My Api Key".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the sub-account organization
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the API key
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">generate_sso_token_to_access_sub_account</a>(request: PostCorporateSubAccountSsoTokenRequest) -> Result&lt;GetSsoToken, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint generates an sso token to authenticate and access a
sub-account of the master using the account endpoint
https://account-app.brevo.com/account/login/sub-account/sso/[token], where
[token] will be replaced by the actual token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .generate_sso_token_to_access_sub_account(
            &PostCorporateSubAccountSsoTokenRequest {
                id: 3232323,
                email: None,
                target: None,
                url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `Option<String>` — User email of sub-account organization
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — Id of the sub-account organization
    
</dd>
</dl>

<dl>
<dd>

**target:** `Option<PostCorporateSubAccountSsoTokenRequestTarget>` 

**Set target after login success** * **automation** - Redirect
to Automation after login * **email_campaign** - Redirect to
Email Campaign after login * **contacts** - Redirect to Contacts
after login * **landing_pages** - Redirect to Landing Pages
after login * **email_transactional** - Redirect to Email
Transactional after login * **senders** - Redirect to Senders
after login * **sms_campaign** - Redirect to Sms Campaign after
login * **sms_transactional** - Redirect to Sms Transactional
after login
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` 

Set the full target URL after login success. The user will land
directly on this target URL after login
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">get_sub_account_details</a>(id: String) -> Result&lt;GetCorporateSubAccountIdResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will provide the details for the specified sub-account company
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .get_sub_account_details(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the sub-account organization
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">delete_a_sub_account</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently deletes a sub-account from the corporate master account. Once deleted, all data associated with the sub-account organization is removed and cannot be recovered, so ensure the sub-account is no longer needed before proceeding.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .delete_a_sub_account(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the sub-account organization to be deleted
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">enable_disable_sub_account_application_s</a>(id: String, request: PutCorporateSubAccountIdApplicationsToggleRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

API endpoint for the Corporate owner to enable/disable applications on the
sub-account
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .enable_disable_sub_account_application_s(
            1000000,
            &PutCorporateSubAccountIdApplicationsToggleRequest {
                landing_pages: Some(true),
                meetings: Some(true),
                sms_campaigns: Some(false),
                web_push: Some(false),
                whatsapp: Some(true),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the sub-account organization (mandatory)
    
</dd>
</dl>

<dl>
<dd>

**analytics:** `Option<bool>` 

Set this field to enable or disable Analytics on the
sub-account. Requires the master account's plan to include
Analytics; otherwise the call returns a 400 error.
    
</dd>
</dl>

<dl>
<dd>

**automation:** `Option<bool>` 

Set this field to enable or disable Automation on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**conversations:** `Option<bool>` 

Set this field to enable or disable Conversations on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**crm:** `Option<bool>` — Set this field to enable or disable Sales CRM on the sub-account
    
</dd>
</dl>

<dl>
<dd>

**email_campaigns:** `Option<bool>` 

Set this field to enable or disable Email Campaigns on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**facebook_ads:** `Option<bool>` 

Set this field to enable or disable Facebook ads on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**inbox:** `Option<bool>` 

Set this field to enable or disable Inbox on the sub-account /
Not applicable on ENTv2
    
</dd>
</dl>

<dl>
<dd>

**landing_pages:** `Option<bool>` 

Set this field to enable or disable Landing pages on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**meetings:** `Option<bool>` — Set this field to enable or disable Meetings on the sub-account
    
</dd>
</dl>

<dl>
<dd>

**sms_campaigns:** `Option<bool>` 

Set this field to enable or disable SMS Marketing on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**transactional_emails:** `Option<bool>` 

Set this field to enable or disable Transactional Email on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**transactional_sms:** `Option<bool>` 

Set this field to enable or disable Transactional SMS on the
sub-account
    
</dd>
</dl>

<dl>
<dd>

**web_push:** `Option<bool>` — Set this field to enable or disable Web Push on the sub-account
    
</dd>
</dl>

<dl>
<dd>

**whatsapp:** `Option<bool>` 

Set this field to enable or disable Whatsapp campaigns on the
sub-account
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">update_sub_account_plan</a>(id: String, request: PutCorporateSubAccountIdPlanRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will update the sub-account plan. On the Corporate solution
new version v2, you can set an unlimited number of credits in your
sub-organization. Please pass the value “-1" to set the consumable in
unlimited mode.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .update_sub_account_plan(
            1000000,
            &PutCorporateSubAccountIdPlanRequest {
                credits: Some(PutCorporateSubAccountIdPlanRequestCredits {
                    email: Some(5000),
                    external_feeds: Some(1),
                    sms: Some(2000.0),
                    whatsapp: Some(100.0),
                    wp_subscribers: Some(-1),
                    ..Default::default()
                }),
                features: Some(PutCorporateSubAccountIdPlanRequestFeatures {
                    inbox: Some(10),
                    landing_page: Some(20),
                    sales_users: Some(6),
                    users: Some(15),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the sub-account organization
    
</dd>
</dl>

<dl>
<dd>

**credits:** `Option<PutCorporateSubAccountIdPlanRequestCredits>` — Credit details to update
    
</dd>
</dl>

<dl>
<dd>

**features:** `Option<PutCorporateSubAccountIdPlanRequestFeatures>` — Features details to update
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">update_sub_accounts_plan</a>(request: PutCorporateSubAccountsPlanRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will update multiple sub-accounts plan. On the Corporate
solution new version v2, you can set an unlimited number of credits in your
sub-organization. Please pass the value “-1" to set the consumable in
unlimited mode.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .update_sub_accounts_plan(
            &PutCorporateSubAccountsPlanRequest {
                credits: Some(PutCorporateSubAccountsPlanRequestCredits {
                    email: Some(5000),
                    external_feeds: Some(1),
                    sms: Some(2000.0),
                    whatsapp: Some(100.0),
                    wp_subscribers: Some(-1),
                    ..Default::default()
                }),
                features: Some(PutCorporateSubAccountsPlanRequestFeatures {
                    landing_page: Some(20),
                    sales_users: Some(6),
                    users: Some(15),
                    ..Default::default()
                }),
                sub_account_ids: Some(vec![4534345, 987893, 876785]),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**credits:** `Option<PutCorporateSubAccountsPlanRequestCredits>` — Credit details to update
    
</dd>
</dl>

<dl>
<dd>

**features:** `Option<PutCorporateSubAccountsPlanRequestFeatures>` — Features details to update
    
</dd>
</dl>

<dl>
<dd>

**sub_account_ids:** `Option<Vec<String>>` — List of sub-account ids
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">invite_admin_user</a>(request: InviteAdminUserRequest) -> Result&lt;InviteAdminUserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

`This endpoint allows you to invite a member to manage the Admin account
Features and their respective permissions are as below: - `my_plan`:
  - "all"
- `api`:
  - "none"
- `user_management`:
  - "all"
- `app_management` | Not available in ENTv2:
  - "all"
- `sub_organization_groups`
  - "create"
  - "edit_delete"
- `create_sub_organizations`
  - "all"
- `manage_sub_organizations`
  - "all"
- `analytics`
  - "download_data"
  - "create_alerts"
  - "my_looks"
  - "explore_create"
- `security`
  - "all"
**Note**: - If `all_features_access: false` then only privileges are
required otherwise if `true` then it's assumed that all permissions will be
there for the invited admin user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .invite_admin_user(
            &InviteAdminUserRequest {
                all_features_access: true,
                email: "inviteuser@example.com".to_string(),
                privileges: vec![InviteAdminUserRequestPrivilegesItem {
                    ..Default::default()
                }],
                group_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**all_features_access:** `bool` — All access to the features
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` — Email address for the organization
    
</dd>
</dl>

<dl>
<dd>

**group_ids:** `Option<Vec<String>>` — Ids of Group
    
</dd>
</dl>

<dl>
<dd>

**privileges:** `Vec<InviteAdminUserRequestPrivilegesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">resend_cancel_admin_user_invitation</a>(action: PutCorporateUserInvitationActionEmailRequestAction, email: String) -> Result&lt;PutCorporateUserInvitationActionEmailResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will allow the user to:
- Resend an admin user invitation
- Cancel an admin user invitation
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .resend_cancel_admin_user_invitation(
            &PutCorporateUserInvitationActionEmailRequestAction::Resend,
            &"email".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**action:** `PutCorporateUserInvitationActionEmailRequestAction` — Action to be performed (cancel / resend)
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` — Email address of the recipient
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">revoke_an_admin_user</a>(email: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint allows to revoke/remove an invited member of your Admin
account
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .revoke_an_admin_user(&"email".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email of the invited user
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">get_corporate_user_permission</a>(email: String) -> Result&lt;GetCorporateUserPermissionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will provide the list of admin user permissions
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .master_account
        .get_corporate_user_permission(&"email".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email of the invited user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.master_account.<a href="/src/api/resources/master_account/client.rs">change_admin_user_permissions</a>(email: String, request: PutCorporateUserEmailPermissionsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will allow you to change the permissions of Admin users of
your Admin account
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.master_account.change_admin_user_permissions(&"email".to_string(), &PutCorporateUserEmailPermissionsRequest {
        all_features_access: false,
        privileges: vec![PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::UserManagement),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::Api),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::MyPlan),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::None]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::AppsManagement),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::CreateSubOrganizations),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::SubOrganizationGroups),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::Create, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::EditDelete]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::ManageSubOrganizations),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::All]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::Security),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::None]),
            ..Default::default()
        }, PutCorporateUserEmailPermissionsRequestPrivilegesItem {
            feature: Some(PutCorporateUserEmailPermissionsRequestPrivilegesItemFeature::Analytics),
            permissions: Some(vec![PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::CreateAlerts, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::DownloadData, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::MyLooks, PutCorporateUserEmailPermissionsRequestPrivilegesItemPermissionsItem::ExploreCreate]),
            ..Default::default()
        }]
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email address of Admin user
    
</dd>
</dl>

<dl>
<dd>

**all_features_access:** `bool` — All access to the features
    
</dd>
</dl>

<dl>
<dd>

**privileges:** `Vec<PutCorporateUserEmailPermissionsRequestPrivilegesItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## User
<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">get_invited_users_list</a>() -> Result&lt;GetInvitedUsersListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the list of all users associated with your organization, including both active and pending invited users. Each user entry includes their email address, owner status, current invitation status, and feature access levels for marketing, CRM, and conversations.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.user.get_invited_users_list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">put_revoke_user_permission</a>(email: String) -> Result&lt;PutRevokeUserPermissionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Revokes all permissions for an invited user in the organization, effectively removing their access to the platform. If the user''s plan change generated credit notes, they are returned in the response for billing reconciliation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .user
        .put_revoke_user_permission(&"email".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email of the invited user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">inviteuser</a>(request: Inviteuser) -> Result&lt;InviteuserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

`Feature` - A Feature represents a specific functionality like Email
campaign, Deals, Calls, Automations, etc. on Brevo. While inviting a user,
determine which feature you want to manage access to. You must specify the
feature accurately to avoid errors. `Permission` - A Permission defines the
level of access or control a user has over a specific feature. While
inviting user, decide on the permission level required for the selected
feature. Make sure the chosen permission is related to the selected feature.
Features and their respective permissions are as below: - `email_campaigns`:
  - "create_edit_delete"
  - "send_schedule_suspend"
- `sms_campaigns`:
  - "create_edit_delete"
  - "send_schedule_suspend"
- `contacts`:
  - "view"
  - "create_edit_delete"
  - "import"
  - "export"
  - "list_and_attributes"
  - "forms"
- `templates`:
  - "create_edit_delete"
  - "activate_deactivate"
- `workflows`:
  - "create_edit_delete"
  - "activate_deactivate_pause"
  - "settings"
- `landing_pages`:
  - "all"
- `transactional_emails`:
  - "settings"
  - "logs"
- `smtp_api`:
  - "smtp"
  - "api_keys"
  - "authorized_ips"
- `user_management`:
  - "all"
- `sales_platform`:
  - "create_edit_deals"
  - "delete_deals"
  - "manage_others_deals_tasks"
  - "reports"
  - "settings"
- `phone`:
  - "all"
- `conversations`:
  - "access"
  - "assign"
  - "configure"
- `senders_domains_dedicated_ips`:
  - "senders_management"
  - "domains_management"
  - "dedicated_ips_management"
- `push_notifications`:
  - "view"
  - "create_edit_delete"
  - "send"
  - "settings"
- `companies`:
  - "manage_owned_companies"
  - "manage_other_companies"
  - "settings"
**Note**: - If `all_features_access: false` then only privileges are
required otherwise if `true` then it's assumed that all permissions will be
there for the invited user. - The availability of feature and its permission
depends on your current plan. Please select the features and permissions
accordingly.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .user
        .inviteuser(
            &Inviteuser {
                all_features_access: true,
                email: "inviteuser@example.com".to_string(),
                privileges: vec![InviteuserPrivilegesItem {
                    ..Default::default()
                }],
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">putresendcancelinvitation</a>(action: PutresendcancelinvitationRequestAction, email: String) -> Result&lt;PutresendcancelinvitationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resends or cancels a pending invitation for a user in the organization, depending on the action path parameter. Use `resend` to send a new invitation email to the user, or `cancel` to revoke the pending invitation entirely and remove the user''s pending access.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .user
        .putresendcancelinvitation(
            &PutresendcancelinvitationRequestAction::Resend,
            &"email".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**action:** `PutresendcancelinvitationRequestAction` — action
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` — Email of the invited user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">edit_user_permission</a>(request: Inviteuser) -> Result&lt;EditUserPermissionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

`Feature` - A Feature represents a specific functionality like Email
campaign, Deals, Calls, Automations, etc. on Brevo. While inviting a user,
determine which feature you want to manage access to. You must specify the
feature accurately to avoid errors. `Permission` - A Permission defines the
level of access or control a user has over a specific feature. While
inviting user, decide on the permission level required for the selected
feature. Make sure the chosen permission is related to the selected feature.
Features and their respective permissions are as below: - `email_campaigns`:
  - "create_edit_delete"
  - "send_schedule_suspend"
- `sms_campaigns`:
  - "create_edit_delete"
  - "send_schedule_suspend"
- `contacts`:
  - "view"
  - "create_edit_delete"
  - "import"
  - "export"
  - "list_and_attributes"
  - "forms"
- `templates`:
  - "create_edit_delete"
  - "activate_deactivate"
- `workflows`:
  - "create_edit_delete"
  - "activate_deactivate_pause"
  - "settings"
- `landing_pages`:
  - "all"
- `transactional_emails`:
  - "settings"
  - "logs"
- `smtp_api`:
  - "smtp"
  - "api_keys"
  - "authorized_ips"
- `user_management`:
  - "all"
- `sales_platform`:
  - "create_edit_deals"
  - "delete_deals"
  - "manage_others_deals_tasks"
  - "reports"
  - "settings"
- `phone`:
  - "all"
- `conversations`:
  - "access"
  - "assign"
  - "configure"
- `senders_domains_dedicated_ips`:
  - "senders_management"
  - "domains_management"
  - "dedicated_ips_management"
- `push_notifications`:
  - "view"
  - "create_edit_delete"
  - "send"
  - "settings"
- `companies`:
  - "manage_owned_companies"
  - "manage_other_companies"
  - "settings"
**Note**: - The privileges array remains the same as in the send invitation;
the user simply needs to provide the permissions that need to be updated. -
The availability of feature and its permission depends on your current plan.
Please select the features and permissions accordingly.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .user
        .edit_user_permission(
            &Inviteuser {
                all_features_access: true,
                email: "inviteuser@example.com".to_string(),
                privileges: vec![InviteuserPrivilegesItem {
                    ..Default::default()
                }],
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">get_user_permission</a>(email: String) -> Result&lt;GetUserPermissionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the granular feature-level permissions assigned to a specific user in the organization, identified by their email address. The response includes the user''s current status (active or pending) and a detailed list of privileges specifying which features and permission levels are granted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .user
        .get_user_permission(&"email".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — Email of the invited user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Process
<details><summary><code>client.process.<a href="/src/api/resources/process/client.rs">get_processes</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetProcessesRequestSort&gt;&gt;) -> Result&lt;GetProcessesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of background processes from your Brevo account with filtering and pagination.

**Use this to:**
- Monitor background process activity and status
- Track long-running operations and tasks
- Find process IDs for detailed status checking
- Review process history and performance
- Identify failed or stuck processes for troubleshooting

**Key information returned:**
- Process details (ID, name, status)
- Export download URLs for completed export processes
- Import details with CSV report URLs for completed import processes
- Total count of processes for pagination

**Important considerations:**
- Background processes handle long-running operations like imports and exports
- Process status indicates current state (queued, processing, completed)
- Export processes provide download URLs when completed
- Import processes provide CSV report URLs with details about problematic records
- Use pagination for accounts with many historical processes
- Sort options available for creation order (ascending or descending)
- Default limit is 10 results per page, maximum is 50
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .process
        .get_processes(
            &GetProcessesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number limitation for the result returned
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Beginning point in the list to retrieve from.
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetProcessesRequestSort>` 

Sort the results in the ascending/descending order of record creation.
Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.process.<a href="/src/api/resources/process/client.rs">get_process</a>(process_id: String) -> Result&lt;GetProcessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves detailed information about a specific background process.

**Use this to:**
- Get detailed status of a specific process
- Monitor process progress and completion
- Download results from completed export processes
- Check error details for failed processes
- Track process execution times

**Key information returned:**
- Complete process details (ID, name, status)
- Download URLs for completed export processes
- Import details with CSV report URLs for completed import processes

**Important considerations:**
- Process ID must exist in your account and not be deleted
- Completed export processes include download URLs
- Completed import processes include CSV report URLs with details about problematic records
- Different process types return different result structures
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.process.get_process(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**process_id:** `String` — Id of the process
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Senders
<details><summary><code>client.senders.<a href="/src/api/resources/senders/client.rs">get_senders</a>(ip: Option&lt;Option&lt;String&gt;&gt;, domain: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetSendersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of all email senders from your Brevo account with optional filtering.

**Use this to:**
- Get all available senders for email campaign setup
- Find sender details including ID, name, and email address
- Filter senders by IP address for dedicated IP users
- Filter senders by domain for domain-specific configurations
- Monitor sender configuration and status

**Key information returned:**
- Sender details (ID, name, email address)
- Sender status and verification information
- Associated IP addresses and domains (for dedicated IP accounts)
- Sender configuration settings

**Important considerations:**
- Standard accounts show empty IP arrays, dedicated IP accounts show IP assignments
- Filtering by IP only available for accounts with dedicated IPs
- Domain filtering helps organize senders by business units or brands
- Sender status indicates if sender is active and ready for campaign use
- Email verification required before sender can be used in campaigns
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .senders
        .get_senders(
            &GetSendersQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ip:** `Option<String>` — Filter your senders for a specific ip. **Available for dedicated IP usage only**
    
</dd>
</dl>

<dl>
<dd>

**domain:** `Option<String>` — Filter your senders for a specific domain
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.senders.<a href="/src/api/resources/senders/client.rs">create_sender</a>(request: CreateSenderRequest) -> Result&lt;CreateSenderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new email sender in your Brevo account.

**Use this to:**
- Add new senders for email campaigns
- Configure sender identity (name and email)
- Associate dedicated IPs with the sender (for dedicated IP accounts)
- Set up domain-based sender configurations

**Key information returned:**
- Created sender ID
- DKIM and SPF configuration status
- Success confirmation

**Important considerations:**
- Verification email sent to specified sender address
- DKIM and SPF configuration affects deliverability
- Dedicated IP accounts require IP association during creation
- IP weights must sum to 100 when specified
- Sender must be verified before use in campaigns
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .senders
        .create_sender(
            &CreateSenderRequest {
                email: "support@example.com".to_string(),
                name: "Support Team".to_string(),
                ips: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` 

From email to use for the sender. A verification email will be
sent to this address.
    
</dd>
</dl>

<dl>
<dd>

**ips:** `Option<Vec<CreateSenderRequestIpsItem>>` 

**Mandatory in case of dedicated IP**. IPs to associate to the
sender. Not required for standard accounts.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — From Name to use for the sender
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.senders.<a href="/src/api/resources/senders/client.rs">get_ips</a>() -> Result&lt;GetIpsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves all dedicated IPs associated with your Brevo account.

Use this to:
- List all your dedicated IPs
- Check the status of your dedicated IPs (active/inactive)
- Find IP addresses and associated domains for configuration purposes
- Monitor your IP reputation and deliverability
- Verify available IPs for sender configuration

Key information returned:
- IP ID and address
- Associated domain
- Active status
- IP configuration details
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.senders.get_ips(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.senders.<a href="/src/api/resources/senders/client.rs">update_sender</a>(sender_id: String, request: UpdateSenderRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing email sender's configuration.

Use this to:
- Modify sender display name or email address
- Update dedicated IP associations
- Change sender configuration settings
- Correct sender information

Key information returned:
- Success confirmation
- Updated sender details
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .senders
        .update_sender(
            1000000,
            &UpdateSenderRequest {
                name: Some("New Support Team".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sender_id:** `String` — Id of the sender
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — From Email to update the sender
    
</dd>
</dl>

<dl>
<dd>

**ips:** `Option<Vec<UpdateSenderRequestIpsItem>>` 

**Only in case of dedicated IP**. IPs to associate to the
sender. If passed, will replace all the existing IPs. Not required for standard accounts.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — From Name to update the sender
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.senders.<a href="/src/api/resources/senders/client.rs">delete_sender</a>(sender_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an email sender from your Brevo account.

Use this to:
- Remove senders that are no longer needed
- Clean up sender configurations
- Remove duplicate or test senders

Key information returned:
- Success confirmation message
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.senders.delete_sender(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sender_id:** `String` — Id of the sender
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.senders.<a href="/src/api/resources/senders/client.rs">get_ips_from_sender</a>(sender_id: String) -> Result&lt;GetIpsFromSenderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the dedicated IPs associated with a specific sender.

Use this to:
- Check IP configuration for a sender
- Verify dedicated IP associations
- Get IP details for troubleshooting
- Monitor sender IP configuration

Key information returned:
- List of associated dedicated IPs
- IP addresses and domain configurations
- IP status and settings
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.senders.get_ips_from_sender(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sender_id:** `String` — Id of the sender
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.senders.<a href="/src/api/resources/senders/client.rs">validate_sender_by_otp</a>(sender_id: String, request: ValidateSenderByOtpRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates a sender using the OTP (One-Time Password) received via email.

Use this to:
- Complete sender verification process
- Activate a newly created sender
- Verify ownership of the sender email address
- Enable the sender for use in email campaigns

Key information returned:
- Success confirmation of sender verification
- Sender activation status
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .senders
        .validate_sender_by_otp(1000000, &ValidateSenderByOtpRequest { otp: 123456 }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sender_id:** `String` — Id of the sender
    
</dd>
</dl>

<dl>
<dd>

**otp:** `i64` — 6 digit OTP received on email
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Domains
<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">get_domains</a>() -> Result&lt;GetDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves all domains associated with the account.

Use this to:
- List all domains
- Verify domain existence
- Check domain authentication and verification status
- Monitor domain configuration and provider information
- Review domain creation history and ownership

Key information returned:
- Domain details (ID, name, authentication status)
- Verification and authentication states
- Associated IP addresses and DNS providers
- Creator information and creation timestamps
- Pagination information for large domain lists
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.domains.get_domains(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">create_domain</a>(request: CreateDomainRequest) -> Result&lt;CreateDomainResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new domain in Brevo.

Use this to:
- Add new domains for sending emails
- Set up domain authentication for better deliverability
- Configure DNS records for email authentication
- Establish domain-based sender identities

Key information returned:
- Created domain ID and configuration
- Required DNS records for authentication
- Domain provider detection results
- Setup instructions and next steps
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .domains
        .create_domain(
            &CreateDomainRequest {
                name: "mycompany.com".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — Domain name to be added
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">get_domain_configuration</a>(domain_name: String) -> Result&lt;GetDomainConfigurationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves configuration of a specific domain, to know if the domain is valid or not.

Use this to:
- Check domain configuration
- Validate a domain configuration
- Monitor DNS record status
- Troubleshoot authentication issues

Key information returned:
- Domain verification and authentication status
- DNS records configuration and validation status
- Detailed authentication requirements
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .domains
        .get_domain_configuration(&"domainName".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_name:** `String` — Domain name
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">delete_domain</a>(domain_name: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a domain from Brevo.

Use this to:
- Remove existing domains
- Clean up unused domain configurations
- Remove test domains

Key information returned:
- Success confirmation message
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .domains
        .delete_domain(&"domainName".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_name:** `String` — Domain name
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">authenticate_domain</a>(domain_name: String) -> Result&lt;AuthenticateDomainResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Authenticates a specific domain.

Use this to:
- Authenticate a domain
- Verify DNS record configuration
- Complete domain setup for sending
- Enable domain for email authentication

Key information returned:
- Authentication success confirmation
- Domain readiness status for email sending
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .domains
        .authenticate_domain(&"domainName".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_name:** `String` — Domain name
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">get_webhooks</a>(type_: Option&lt;Option&lt;GetWebhooksRequestType&gt;&gt;, sort: Option&lt;Option&lt;GetWebhooksRequestSort&gt;&gt;) -> Result&lt;GetWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves all webhooks from your Brevo account with filtering and sorting options.

Use this to:
- Monitor webhook configurations and event handling
- List webhooks by type (transactional, marketing, inbound)
- Review webhook endpoints and authentication
- Track webhook creation and modification history
- Audit webhook event subscriptions

Key information returned:
- Complete webhook details and configuration
- Event types and channel subscriptions
- Authentication and security settings
- Webhook URLs and custom headers
- Creation and modification timestamps
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .get_webhooks(
            &GetWebhooksQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<GetWebhooksRequestType>` — Filter on webhook type
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetWebhooksRequestSort>` — Sort the results in the ascending/descending order of webhook creation
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">create_webhook</a>(request: CreateWebhookRequest) -> Result&lt;CreateWebhookResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new webhook to receive real-time notifications for specified events.

Use this to:
- Set up event notifications for transactional or marketing emails
- Configure webhook endpoints for campaign tracking
- Enable real-time monitoring of email delivery status
- Subscribe to contact list changes and updates
- Implement custom event handling and automation

Key information returned:
- Created webhook ID and configuration
- Success confirmation and setup details
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .create_webhook(
            &CreateWebhookRequest {
                url: "http://requestb.in/173lyyx1".to_string(),
                auth: None,
                batched: None,
                channel: None,
                description: None,
                domain: None,
                events: None,
                headers: None,
                r#type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**auth:** `Option<CreateWebhookRequestAuth>` — Add authentication on webhook url
    
</dd>
</dl>

<dl>
<dd>

**batched:** `Option<bool>` — To send batched webhooks
    
</dd>
</dl>

<dl>
<dd>

**channel:** `Option<CreateWebhookRequestChannel>` — Channel of the webhook
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Description of the webhook
    
</dd>
</dl>

<dl>
<dd>

**domain:** `Option<String>` 

Inbound domain of webhook, required in case of event type
`inbound`
    
</dd>
</dl>

<dl>
<dd>

**events:** `Option<Vec<CreateWebhookRequestEventsItem>>` 

Events triggering the webhook. Required for transactional and
marketing types, optional for inbound type (defaults to
`inboundEmailProcessed`). Possible values for **Transactional**
type webhook: `sent` OR `request`, `delivered`, `hardBounce`,
`softBounce`, `blocked`, `spam`, `invalid`, `deferred`, `click`,
`opened`, `uniqueOpened` and `unsubscribed`. Possible values for
**Marketing** type webhook: `spam`, `opened`, `click`,
`hardBounce`, `softBounce`, `unsubscribed`, `listAddition`,
`delivered`, `contactUpdated` & `contactDeleted`. Possible values
for **Inbound** type webhook: `inboundEmailProcessed`.
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<Vec<CreateWebhookRequestHeadersItem>>` — Custom headers to be send with webhooks
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<CreateWebhookRequestType>` — Type of the webhook
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — URL of the webhook
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">export_webhooks_history</a>(request: ExportWebhooksHistoryRequest) -> Result&lt;ExportWebhooksHistoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>
This is an enterprise feature. Contact us to activate it for your account.
</Note>

Submits a request to export webhook event history as a CSV file. The download link is sent to the `notifyURL` you provide in the request body.

Use this endpoint to:
- Export webhook event history filtered by date range, event type, or email address
- Generate reports for compliance, auditing, or performance analysis
- Track delivery patterns and webhook reliability over time
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .export_webhooks_history(
            &ExportWebhooksHistoryRequest {
                event: ExportWebhooksHistoryRequestEvent::InvalidParameter,
                notify_url: "https://brevo.com".to_string(),
                r#type: ExportWebhooksHistoryRequestType::Transactional,
                days: None,
                email: None,
                end_date: None,
                message_id: None,
                sort: None,
                start_date: None,
                webhook_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**days:** `Option<i64>` 

Number of days in the past including today (positive
integer). _Not compatible with 'startDate' and 'endDate'_
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Filter the history for a specific email
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` 

Mandatory if startDate is used. Ending date of the report
(YYYY-MM-DD). Must be greater than equal to startDate
    
</dd>
</dl>

<dl>
<dd>

**event:** `ExportWebhooksHistoryRequestEvent` — Filter the history for a specific event type
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `Option<String>` 

Filter the history for a specific message id. Applicable
only for transactional webhooks.
    
</dd>
</dl>

<dl>
<dd>

**notify_url:** `String` — Webhook URL to receive CSV file link
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<String>` — Sorting order of records (asc or desc)
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` 

Mandatory if endDate is used. Starting date of the history
(YYYY-MM-DD). Must be lower than equal to endDate
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ExportWebhooksHistoryRequestType` — Filter the history based on webhook type
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `Option<i64>` — Filter the history for a specific webhook id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">get_webhook</a>(webhook_id: String) -> Result&lt;GetWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves detailed information about a specific webhook configuration.

Use this to:
- Get complete webhook configuration and settings
- Check webhook event subscriptions and triggers
- Review authentication and security settings
- Verify webhook URL and custom headers
- Access webhook creation and modification history

Key information returned:
- Complete webhook details and configuration
- Event types and channel subscriptions
- Authentication credentials and methods
- Custom headers and request settings
- Webhook status and activity information
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.webhooks.get_webhook(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `String` — Id of the webhook
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">update_webhook</a>(webhook_id: String, request: UpdateWebhookRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing webhook configuration and event subscriptions.

Use this to:
- Modify webhook event subscriptions and triggers
- Update webhook URL and endpoint configuration
- Change authentication settings and credentials
- Adjust custom headers and request parameters
- Enable or disable specific webhook events

Key information returned:
- Success confirmation of webhook updates
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .update_webhook(
            1000000,
            &UpdateWebhookRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `String` — Id of the webhook
    
</dd>
</dl>

<dl>
<dd>

**auth:** `Option<UpdateWebhookRequestAuth>` — Add authentication on webhook url
    
</dd>
</dl>

<dl>
<dd>

**batched:** `Option<bool>` — To send batched webhooks
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Description of the webhook
    
</dd>
</dl>

<dl>
<dd>

**domain:** `Option<String>` — Inbound domain of webhook, used in case of event type `inbound`
    
</dd>
</dl>

<dl>
<dd>

**events:** `Option<Vec<UpdateWebhookRequestEventsItem>>` 

- Events triggering the webhook. Possible values for
**Transactional** type webhook: #### `sent` OR `request`,
`delivered`, `hardBounce`, `softBounce`, `blocked`, `spam`,
`invalid`, `deferred`, `click`, `opened`, `uniqueOpened` and
`unsubscribed` - Possible values for **Marketing** type webhook:
#### `spam`, `opened`, `click`, `hardBounce`, `softBounce`,
`unsubscribed`, `listAddition` & `delivered` - Possible values
for **Inbound** type webhook: #### `inboundEmailProcessed`
#### `reply`
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<Vec<UpdateWebhookRequestHeadersItem>>` — Custom headers to be send with webhooks
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — URL of the webhook
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">delete_webhook</a>(webhook_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently deletes a webhook and stops all event notifications.

Use this to:
- Remove unused or obsolete webhook configurations
- Clean up webhook endpoints and subscriptions
- Stop event notifications to specific URLs
- Maintain organized webhook management

Key information returned:
- Success confirmation of webhook deletion
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.webhooks.delete_webhook(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `String` — Id of the webhook
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ExternalFeeds
<details><summary><code>client.external_feeds.<a href="/src/api/resources/external_feeds/client.rs">get_all_external_feeds</a>(search: Option&lt;Option&lt;String&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetAllExternalFeedsRequestSort&gt;&gt;, auth_type: Option&lt;Option&lt;GetAllExternalFeedsRequestAuthType&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetAllExternalFeedsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves all external feeds from your Brevo account with filtering and pagination.

**Use this to:**
- Get an overview of all external data feeds
- Find feeds by name using search functionality
- Filter feeds by creation date range
- Browse feeds by authentication type
- Monitor feed library organization and usage

**Key information returned:**
- Feed details (UUID, name, URL, authentication type)
- Feed configuration and settings
- Creation and modification timestamps
- Feed status and error information
- Authentication and header configurations

**Important considerations:**
- External feeds enable dynamic content in email campaigns
- Feeds must be accessible from Brevo servers
- Authentication credentials are securely stored
- Feed performance affects campaign delivery
- Use pagination for accounts with many feeds
- Date range filtering limited to 30 days maximum
- Search functionality works on feed name matching
- Internal feeds are system-managed and cannot be modified
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .external_feeds
        .get_all_external_feeds(
            &GetAllExternalFeedsQueryRequest {
                search: Some("product".to_string()),
                start_date: Some(NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap()),
                end_date: Some(NaiveDate::parse_from_str("2024-01-31", "%Y-%m-%d").unwrap()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**search:** `Option<String>` — Can be used to filter records by search keyword on feed name
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` 

Mandatory if `endDate` is used. Starting date (YYYY-MM-DD) from which
you want to fetch the list. Can be maximum 30 days older than current
date.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` 

Mandatory if `startDate` is used. Ending date (YYYY-MM-DD) till which
you want to fetch the list. Maximum time period that can be selected is
one month.
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetAllExternalFeedsRequestSort>` 

Sort the results in the ascending/descending order of record creation.
Default order is **descending** if `sort` is not passed.
    
</dd>
</dl>

<dl>
<dd>

**auth_type:** `Option<GetAllExternalFeedsRequestAuthType>` — Filter the records by `authType` of the feed.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document on the page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.external_feeds.<a href="/src/api/resources/external_feeds/client.rs">create_external_feed</a>(request: CreateExternalFeedRequest) -> Result&lt;CreateExternalFeedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new external feed for dynamic content in email campaigns.

**Use this to:**
- Set up external data sources for dynamic content
- Configure authentication for protected feeds
- Enable real-time content updates in campaigns
- Establish connections to product catalogs, blogs, or APIs

**Key information returned:**
- Created feed UUID for reference in campaigns
- Success confirmation

**Important considerations:**
- Feed URL must be accessible from Brevo infrastructure
- Authentication credentials are securely encrypted
- Test feed accessibility before campaign use
- Consider feed response time for campaign performance
- Monitor feed reliability and uptime
- Use caching for frequently accessed feeds
- Maximum 5 retry attempts allowed for failed requests
- Custom headers support for API integration requirements
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .external_feeds
        .create_external_feed(
            &CreateExternalFeedRequest {
                name: "Public API Feed".to_string(),
                url: "https://jsonplaceholder.typicode.com/posts".to_string(),
                auth_type: Some(CreateExternalFeedRequestAuthType::NoAuth),
                max_retries: Some(3),
                cache: Some(true),
                username: None,
                password: None,
                token: None,
                headers: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — Name of the feed
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — URL of the external data source
    
</dd>
</dl>

<dl>
<dd>

**auth_type:** `Option<CreateExternalFeedRequestAuthType>` — Authentication type for accessing the feed
    
</dd>
</dl>

<dl>
<dd>

**username:** `Option<String>` — Username for basic authentication (required if authType is 'basic')
    
</dd>
</dl>

<dl>
<dd>

**password:** `Option<String>` — Password for basic authentication (required if authType is 'basic')
    
</dd>
</dl>

<dl>
<dd>

**token:** `Option<String>` — Token for token-based authentication (required if authType is 'token')
    
</dd>
</dl>

<dl>
<dd>

**max_retries:** `Option<i64>` — Maximum number of retry attempts for failed requests
    
</dd>
</dl>

<dl>
<dd>

**cache:** `Option<bool>` — Whether to cache the feed response
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<Vec<CreateExternalFeedRequestHeadersItem>>` — Custom HTTP headers for the feed request
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.external_feeds.<a href="/src/api/resources/external_feeds/client.rs">get_external_feed_by_uuid</a>(uuid: String) -> Result&lt;GetExternalFeedByUuidResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves details of a specific external feed by its UUID.

**Use this to:**
- Get complete configuration of an external feed
- Check feed authentication settings
- Review feed personalization options
- Verify feed URL and parameters
- Monitor feed modification history

**Key information returned:**
- Complete feed configuration and settings
- Authentication credentials and headers
- Personalization and fallback settings
- Creation and modification timestamps
- Cache and retry configurations

**Important considerations:**
- UUID must exist in your account
- Provides complete feed information for troubleshooting
- Essential before making modifications
- Shows current feed health status
- Useful for debugging feed issues
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .external_feeds
        .get_external_feed_by_uuid(&"b1c2d3e4-f5a6-47b8-89c0-d1e2f3a4b5c6".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**uuid:** `String` — UUID of the feed to fetch
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.external_feeds.<a href="/src/api/resources/external_feeds/client.rs">update_external_feed</a>(uuid: String, request: UpdateExternalFeedRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates configuration of an existing external feed.

**Use this to:**
- Update feed URLs when data sources change
- Modify authentication credentials
- Change cache and retry settings
- Update custom headers
- Rename feeds for better organization

**Key information returned:**
- Success confirmation message

**Important considerations:**
- Only provided fields will be updated
- Feed UUID must exist in your account
- Authentication changes require verification
- URL changes should be tested before campaign use
- Monitor campaigns using this feed after updates
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .external_feeds
        .update_external_feed(
            &"b1c2d3e4-f5a6-47b8-89c0-d1e2f3a4b5c6".to_string(),
            &UpdateExternalFeedRequest {
                name: Some("Updated Product Catalog".to_string()),
                url: Some("https://api.newstore.com/products/v2".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**uuid:** `String` — UUID of the feed to update
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of the feed
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — URL of the external data source
    
</dd>
</dl>

<dl>
<dd>

**auth_type:** `Option<UpdateExternalFeedRequestAuthType>` — Authentication type for accessing the feed
    
</dd>
</dl>

<dl>
<dd>

**username:** `Option<String>` — Username for basic authentication
    
</dd>
</dl>

<dl>
<dd>

**password:** `Option<String>` — Password for basic authentication
    
</dd>
</dl>

<dl>
<dd>

**token:** `Option<String>` — Token for token-based authentication
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<Vec<UpdateExternalFeedRequestHeadersItem>>` — Custom HTTP headers for the feed request
    
</dd>
</dl>

<dl>
<dd>

**max_retries:** `Option<i64>` — Maximum number of retry attempts for failed requests
    
</dd>
</dl>

<dl>
<dd>

**cache:** `Option<bool>` — Whether to cache the feed response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.external_feeds.<a href="/src/api/resources/external_feeds/client.rs">delete_external_feed</a>(uuid: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an external feed from your Brevo account.

**Use this to:**
- Remove external feeds that are no longer needed
- Clean up unused data sources
- Remove test or outdated feeds
- Maintain organized feed library

**Key information returned:**
- Success confirmation message

**Important considerations:**
- This action is PERMANENT and cannot be undone
- Feed configuration and history will be lost
- Check if feed is used in active campaigns before deletion
- Remove feed references from email templates
- Consider deactivating instead of deleting if unsure
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .external_feeds
        .delete_external_feed(&"b1c2d3e4-f5a6-47b8-89c0-d1e2f3a4b5c6".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**uuid:** `String` — UUID of the feed to delete
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CustomObjects
<details><summary><code>client.custom_objects.<a href="/src/api/resources/custom_objects/client.rs">upsertrecords</a>(object_type: String, request: UpsertrecordsRequest) -> Result&lt;UpsertrecordsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note title="Enterprise access only">Custom objects are only available to Enterprise plans.
This feature is in beta. These are subject to change.</Note>
Performs bulk create or update (upsert) operations for object records in a single asynchronous request. This endpoint is optimized for high-volume data imports and synchronization scenarios.

**How Upsert Works:**
- **Create**: Omit `identifiers`, or provide only `ext_id` (if it doesn't already exist). A new record is created with a Brevo-generated `id`.
- **Update**: Provide `id` (Brevo internal ID) or an `ext_id` that already exists. The matching record is updated with the new attribute values.
- **Important:** `id` is for **updates only**. Providing an `id` that does not belong to an existing record will fail during async processing (the HTTP response will still be 202, but the record will be rejected in the background). To create a new record with a stable external reference, use `ext_id` instead.

**Request Structure:**
Each object record in the `records` array can include:
- `identifiers`: Either `id` (internal Brevo ID) or `ext_id` (your external system ID) — required for updates. **Note:** use `id` (singular), not `ids`.
- `attributes`: Key-value pairs where each key is the attribute **key** (e.g., `company_name`), not the attribute label (e.g., "Company Name").
- `associations`: Controls linking and unlinking of associated records (optional). Each entry specifies:
    - `object_type`: The type of the associated object
    - `action`: `link` (default) to create the association, or `unlink` to remove it
    - `records`: The associated records to link or unlink (each identified by `ext_id` or `id`)
    - **Unlink is idempotent** — unlinking a non-existing association is a no-op (no error returned)
    - `link` and `unlink` actions can be submitted for the same `object_type` in a single record entry
    - Both associated records must already exist before a link can be created

> **Common mistake:** Passing the attribute **label** (the display name you see in the UI) instead of the attribute **key** will cause the attribute to be silently ignored and the record may not be created as expected.

**Asynchronous Processing:**
- Returns immediately with a `processId` (HTTP 202 Accepted)
- Use the processId to track status via the Get process API

**API and Schema Limitations:**
- Max 1000 object records per request
- Max request body size: 1 MB
- Max 500 attributes per object record (matches the schema limit of 500 attributes per object)
- Unknown attribute keys are silently ignored (no error, no attribute creation)
- Max 10 association records per associated object-type in each record of the request. If you need more, send multiple requests.

**Important Behaviors:**
- The object schema must be created before upserting records
- Unknown attribute keys are silently ignored (no error, no creation)
- Both associated object records must already exist before creating a link association
- Unlink operations are idempotent: attempting to unlink a non-existing association returns success
- `link` and `unlink` actions can be submitted for the same `object_type` in a single record entry
- Contact objects cannot be created via this endpoint
- For `category` and `multiple_category` attributes, pass the option **key** as the value (not the option label or option ID).
- The `id` identifier (internal Brevo ID) can only be used for **updating** existing records. To create new records, either omit identifiers (Brevo auto-generates an ID) or provide an `ext_id`.

**Errors:**
- Make sure both object records exist before associating them, else the API will return an error.
- This route does not create objects. The object where the object records are upserted by this API must be created already else the API will return an error "invalid object type".
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.custom_objects.upsertrecords(&"vehicle".to_string(), &UpsertrecordsRequest {
        records: vec![UpsertrecordsRequestRecordsItem {
            associations: Some(vec![UpsertrecordsRequestRecordsItemAssociationsItem {
                object_type: "garage".to_string(),
                action: Some(UpsertrecordsRequestRecordsItemAssociationsItemAction::Link),
                records: vec![UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem {
                    identifiers: Some(UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemIdentifiers {
                        id: Some(435435),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }, UpsertrecordsRequestRecordsItemAssociationsItem {
                object_type: "garage".to_string(),
                action: Some(UpsertrecordsRequestRecordsItemAssociationsItemAction::Unlink),
                records: vec![UpsertrecordsRequestRecordsItemAssociationsItemRecordsItem {
                    identifiers: Some(UpsertrecordsRequestRecordsItemAssociationsItemRecordsItemIdentifiers {
                        ext_id: Some("old-garage-001".to_string()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }]),
            attributes: Some(HashMap::from([("make".to_string(), serde_json::json!("Toyota")), ("model".to_string(), serde_json::json!("Camry")), ("year".to_string(), serde_json::json!(2020)), ("engine_type".to_string(), serde_json::json!("hybrid"))])),
            identifiers: Some(UpsertrecordsRequestRecordsItemIdentifiers {
                ext_id: Some("VIN123".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }]
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_type:** `String` — Object type for the records to upsert. Must be a previously created custom object type. Only lowercase alphanumeric characters and underscores are allowed (max 32 characters).
    
</dd>
</dl>

<dl>
<dd>

**records:** `Vec<UpsertrecordsRequestRecordsItem>` — List of object records to be upsert. Each record can have attributes, identifiers, and associations.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.custom_objects.<a href="/src/api/resources/custom_objects/client.rs">getrecords</a>(object_type: String, limit: Option&lt;String&gt;, page_num: Option&lt;String&gt;, sort: Option&lt;Option&lt;GetrecordsRequestSort&gt;&gt;, association: Option&lt;Option&lt;GetrecordsRequestAssociation&gt;&gt;) -> Result&lt;GetrecordsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note title="Enterprise access only">Custom objects are only available to Enterprise plans.
This feature is in beta. These are subject to change.</Note>
This API retrieves a list of object records along with their associated records and provides the total count of records for the specified object. **Note**: Contact as object type is not supported in this endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .custom_objects
        .getrecords(
            &"vehicle".to_string(),
            &GetrecordsQueryRequest {
                limit: 1000000,
                page_num: 1000000,
                sort: None,
                association: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_type:** `String` — Object type for the records to retrieve. Must be a previously created custom object type. Contact as object type is not supported in this endpoint.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `String` — Number of records returned per page
    
</dd>
</dl>

<dl>
<dd>

**page_num:** `String` — Page number for pagination. It's used to fetch the object records on a provided page number. Must be a valid positive integer.
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetrecordsRequestSort>` — Sort order, must be 'asc' or 'desc'. Default to 'desc' if not provided.
    
</dd>
</dl>

<dl>
<dd>

**association:** `Option<GetrecordsRequestAssociation>` — Whether to include associations, must be 'true' or 'false'. Default to 'false' if not provided.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.custom_objects.<a href="/src/api/resources/custom_objects/client.rs">batch_delete_object_records</a>(object_type: String, request: BatchDeleteObjectRecordsRequest) -> Result&lt;BatchDeleteObjectRecordsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to delete multiple object records of the same object-type in one request.
The request is accepted and processed asynchronously. You can track the status of the deletion process using the returned **processId**.
**Limitations:** - Each request can contain up to **1000** object record identifiers - Either `ids` or `ext_ids` must be provided, but **not both** in the same request - Deletion of Brevo standard object records is not supported via this endpoint - If more records must be deleted, send multiple batch requests
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.custom_objects.batch_delete_object_records(&"vehicle".to_string(), &BatchDeleteObjectRecordsRequest {
        identifiers: Some(BatchDeleteObjectRecordsRequestIdentifiers::BatchDeleteObjectRecordsRequestIdentifiersIds(BatchDeleteObjectRecordsRequestIdentifiersIds {
            ids: Default::default(),
            ..Default::default()
        })),
        ..Default::default()
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_type:** `String` — Object type for the records to delete
    
</dd>
</dl>

<dl>
<dd>

**identifiers:** `Option<BatchDeleteObjectRecordsRequestIdentifiers>` — Either `ids` or `ext_ids` must be provided, but not both in the same request.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.custom_objects.<a href="/src/api/resources/custom_objects/client.rs">get_associated_records</a>(object_type: String, id: Option&lt;Option&lt;String&gt;&gt;, ext_id: Option&lt;Option&lt;String&gt;&gt;, email: Option&lt;Option&lt;String&gt;&gt;, sms: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetAssociatedRecordsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note title="Enterprise access only">Custom objects are only available to Enterprise plans.
This feature is in beta. These are subject to change.</Note>
Returns the records associated with a single source record. Associations of every type are returned together in one paginated list, ordered by association creation time with the most recently created association first.

**Identifying the source record**
Provide exactly one of `id`, `ext_id`, `email` or `sms`. Passing none of them, or more than one, returns `400`. `email` and `sms` are only accepted when `object_type` is `contact`; using either with any other object type returns `400`.

**Object types**
Use the object type exactly as it is defined in your account, for example `vehicle` for a custom object of that name, or `contact` for contacts. An object type that does not exist in your account returns `400`.

**Filtering by associated object type**
Use `type` to restrict the response to one or more associated object types, for example `?type=contact&type=garage`. Up to 5 types can be requested per call; more returns `400`. When `type` is omitted, associations of every type are returned.

**Pagination**
Results are returned 20 per page. The page size is fixed and cannot be changed. Increase `offset` by 20 to walk through the pages until `has_more` is `false`. An `offset` beyond the last record returns an empty `items` array with `has_more` set to `false`.

**Working with contacts**
- `contact` is supported both as the source `object_type` and as an associated object type.
- An `id`, `ext_id`, `email` or `sms` that matches no contact returns `404`.
- If several contacts share the same `ext_id`, `email` or `sms`, identify the contact by `id` to be sure of which one is used.
- Contacts returned in `items` carry all of the contact's attributes, with attribute keys in lowercase — `email`, `first_name`, `last_name`, `sms`, `ext_id`, and any other contact attribute lowercased.
- For contacts, `ext_id`, `created_at` and `updated_at` are not returned on `object`. A contact's external ID is available as `attributes.ext_id` when it is set.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .custom_objects
        .get_associated_records(
            &"vehicle".to_string(),
            &GetAssociatedRecordsQueryRequest {
                id: Some(16789),
                ext_id: Some("507f1f77bc".to_string()),
                email: Some("jane.doe@example.com".to_string()),
                sms: Some("33612345678".to_string()),
                offset: Some(0),
                r#type: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_type:** `String` — Object type of the source record, exactly as defined in your account. Accepts any object type defined in the account, for example a custom object type or `contact`.
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — Internal Brevo ID of the source record. Must be a positive integer. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    
</dd>
</dl>

<dl>
<dd>

**ext_id:** `Option<String>` — External ID of the source record in your system. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Email address of the source contact. Only accepted when `object_type` is `contact`. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    
</dd>
</dl>

<dl>
<dd>

**sms:** `Option<String>` — Phone number of the source contact, including the country code. It may be given with or without a leading `+`; percent-encode the `+` as `%2B`, because a literal `+` in a query string is read as a space. Only accepted when `object_type` is `contact`. Provide exactly one of `id`, `ext_id`, `email` or `sms`.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — Restricts the response to the given associated object types. Repeat the parameter to request several types, for example `?type=contact&type=garage`. Maximum 5 types per call. Associations of every type are returned when omitted.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Number of records to skip before the first record of the page. Defaults to 0. Increase by 20 to fetch the next page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Contacts
<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_contacts</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, modified_since: Option&lt;Option&lt;String&gt;&gt;, created_since: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetContactsRequestSort&gt;&gt;, segment_id: Option&lt;Option&lt;String&gt;&gt;, filter: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetContacts, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all contacts from your Brevo account with support for pagination, filtering, and sorting. Results default to 50 contacts per page (maximum 1000) sorted in descending order of creation, and can be filtered by modification date, creation date, contact IDs (up to 20), list IDs, segment ID, or contact attributes using the equals operator. Note that either listIds or segmentId can be passed but not both simultaneously.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_contacts(
            &GetContactsQueryRequest {
                limit: None,
                offset: None,
                modified_since: None,
                created_since: None,
                sort: None,
                ids: vec![],
                segment_id: None,
                list_ids: vec![],
                filter: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**modified_since:** `Option<String>` — Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**created_since:** `Option<String>` — Filter (urlencoded) the contacts created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetContactsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**ids:** `Option<String>` — Filter by a list of contact IDs. You can pass a **maximum of 20 IDs**. All elements must be integers.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `Option<String>` — Id of the segment. **Either listIds or segmentId can be passed.** Must be a positive integer (minimum value of 1).
    
</dd>
</dl>

<dl>
<dd>

**list_ids:** `Option<String>` — Ids of the list. **Either listIds or segmentId can be passed.**
    
</dd>
</dl>

<dl>
<dd>

**filter:** `Option<String>` — Filter the contacts on the basis of attributes. **Allowed operator: equals. For multiple-choice options, the filter will apply an AND condition between the options. For category attributes, the filter will work with both id and value. (e.g. filter=equals(FIRSTNAME,"Antoine"), filter=equals(B1, true), filter=equals(DOB, "1989-11-23"), filter=equals(GENDER, "1"), filter=equals(GENDER, "MALE"), filter=equals(COUNTRY,"USA, INDIA")**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">create_contact</a>(request: CreateContactRequest) -> Result&lt;Option&lt;CreateContactResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>Follow this format when passing a "SMS" phone number as an attribute.
Accepted Number Formats 91xxxxxxxxxx +91xxxxxxxxxx 0091xxxxxxxxxx</Note>
Creates new contacts on Brevo. Contacts can be created by passing either - <br><br> 1. email address of the contact (email_id),  <br> 2. phone number of the contact (to be passed as "SMS" field in "attributes" along with proper country code), For example- {"SMS":"+91xxxxxxxxxx"} or {"SMS":"0091xxxxxxxxxx"} <br> 3. ext_id <br>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .create_contact(
            &CreateContactRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, CreateContactRequestAttributesValue>>` — Pass the set of attributes and their values. The attribute's parameter should be passed in capital letter while creating a contact. Values that don't match the attribute type (e.g. text or string in a date attribute) will be ignored. **These attributes must be present in your Brevo account**. For eg: **{"FNAME":"Elly", "LNAME":"Roger", "COUNTRIES": ["India","China"]}**
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Email address of the user. **Mandatory if "ext_id"  & "SMS" field is not passed.**
    
</dd>
</dl>

<dl>
<dd>

**email_blacklisted:** `Option<bool>` — Set this field to blacklist the contact for emails (emailBlacklisted = true)
    
</dd>
</dl>

<dl>
<dd>

**ext_id:** `Option<String>` — Pass your own Id to create a contact.
    
</dd>
</dl>

<dl>
<dd>

**list_ids:** `Option<Vec<String>>` — Ids of the lists to add the contact to
    
</dd>
</dl>

<dl>
<dd>

**sms_blacklisted:** `Option<bool>` — Set this field to blacklist the contact for SMS (smsBlacklisted = true)
    
</dd>
</dl>

<dl>
<dd>

**smtp_blacklist_sender:** `Option<Vec<String>>` — transactional email forbidden sender for contact. Use only for email Contact ( only available if updateEnabled = true )
    
</dd>
</dl>

<dl>
<dd>

**update_enabled:** `Option<bool>` — Facilitate to update the existing contact in the same request (updateEnabled = true)
    
</dd>
</dl>

<dl>
<dd>

**force_merge:** `Option<bool>` — When true, if the contact being created shares an identifier (email, SMS, ext_id, whatsapp, landline) with an existing contact, the two contacts are force-merged. The contact with the most recent `last_modified` timestamp is retained; the other is deleted. When false (default), a 4xx error is returned on identifier conflict.
    
</dd>
</dl>

<dl>
<dd>

**get_id:** `Option<bool>` — When true, the response returns the `id` of the surviving contact after merge.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_attributes</a>() -> Result&lt;GetAttributesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all contact attributes defined in your Brevo account, grouped by category (normal, transactional, category, calculated, global). Each attribute includes its name, type, and category, along with enumeration values for category-type attributes and options for multiple-choice-type attributes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.contacts.get_attributes(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">create_attribute</a>(attribute_category: CreateAttributeRequestAttributeCategory, attribute_name: String, request: CreateAttributeRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new contact attribute under the specified category and name. The required body properties depend on the category: use "type" for normal, transactional, or category attributes; use "value" for calculated or global attributes; use "enumeration" for category attributes; and use "multiCategoryOptions" for normal multiple-choice attributes. None of the category or multicategory option values can exceed 200 characters.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .create_attribute(
            &CreateAttributeRequestAttributeCategory::Normal,
            &"attributeName".to_string(),
            &CreateAttributeRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attribute_category:** `CreateAttributeRequestAttributeCategory` — Category of the attribute
    
</dd>
</dl>

<dl>
<dd>

**attribute_name:** `String` — Name of the attribute
    
</dd>
</dl>

<dl>
<dd>

**enumeration:** `Option<Vec<CreateAttributeRequestEnumerationItem>>` — List of values and labels that the attribute can take. **Use only if the attribute's category is "category"**. None of the category options can exceed max 200 characters. For example: **[{"value":1, "label":"male"}, {"value":2, "label":"female"}]**
    
</dd>
</dl>

<dl>
<dd>

**is_recurring:** `Option<bool>` — Type of the attribute. **Use only if the attribute's category is 'calculated' or 'global'**
    
</dd>
</dl>

<dl>
<dd>

**multi_category_options:** `Option<Vec<String>>` — List of options you want to add for multiple-choice attribute. **Use only if the attribute's category is "normal" and attribute's type is "multiple-choice". None of the multicategory options can exceed max 200 characters.** For example: **["USA","INDIA"]**
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<CreateAttributeRequestType>` — Type of the attribute. **Use only if the attribute's category is 'normal', 'category' or 'transactional'** Type **user and multiple-choice** is only available if the category is **normal** attribute Type **id** is only available if the category is **transactional** attribute Type **category** is only available if the category is **category** attribute
    
</dd>
</dl>

<dl>
<dd>

**value:** `Option<String>` — Value of the attribute. **Use only if the attribute's category is 'calculated' or 'global'**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">update_attribute</a>(attribute_category: UpdateAttributeRequestAttributeCategory, attribute_name: String, request: UpdateAttributeRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing contact attribute identified by its category and name. For category-type attributes, you can update the enumeration values; for calculated or global attributes, update the computed value formula; and for normal multiple-choice attributes, update the multicategory options. None of the category or multicategory option values can exceed 200 characters.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .update_attribute(
            &UpdateAttributeRequestAttributeCategory::Category,
            &"attributeName".to_string(),
            &UpdateAttributeRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attribute_category:** `UpdateAttributeRequestAttributeCategory` — Category of the attribute
    
</dd>
</dl>

<dl>
<dd>

**attribute_name:** `String` — Name of the existing attribute
    
</dd>
</dl>

<dl>
<dd>

**enumeration:** `Option<Vec<UpdateAttributeRequestEnumerationItem>>` — List of the values and labels that the attribute can take. **Use only if the attribute's category is "category"** None of the category options can exceed max 200 characters. For example, **[{"value":1, "label":"male"}, {"value":2, "label":"female"}]**
    
</dd>
</dl>

<dl>
<dd>

**multi_category_options:** `Option<Vec<String>>` — Use this option to add multiple-choice attributes options only if the attribute's category is "normal". **This option is specifically designed for updating multiple-choice attributes. None of the multicategory options can exceed max 200 characters.**. For example: **["USA","INDIA"]**
    
</dd>
</dl>

<dl>
<dd>

**value:** `Option<String>` — Value of the attribute to update. **Use only if the attribute's category is 'calculated' or 'global'**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">delete_attribute</a>(attribute_category: DeleteAttributeRequestAttributeCategory, attribute_name: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete an existing contact attribute by its category and name. The attribute must exist in the specified category (normal, transactional, category, calculated, or global), otherwise a 404 error is returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .delete_attribute(
            &DeleteAttributeRequestAttributeCategory::Normal,
            &"attributeName".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attribute_category:** `DeleteAttributeRequestAttributeCategory` — Category of the attribute
    
</dd>
</dl>

<dl>
<dd>

**attribute_name:** `String` — Name of the existing attribute
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">delete_multi_attribute_options</a>(attribute_type: String, multiple_choice_attribute: String, multiple_choice_attribute_option: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific option from an existing multiple-choice contact attribute. The attribute type must be "multiple-choice", and both the attribute name and the option to delete must already exist in your account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .delete_multi_attribute_options(
            &"multiple-choice".to_string(),
            &"multipleChoiceAttribute".to_string(),
            &"multipleChoiceAttributeOption".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attribute_type:** `String` — Type of the attribute
    
</dd>
</dl>

<dl>
<dd>

**multiple_choice_attribute:** `String` — Name of the existing multiple-choice attribute
    
</dd>
</dl>

<dl>
<dd>

**multiple_choice_attribute_option:** `String` — Name of the existing multiple-choice attribute option that you want to delete
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">update_batch_contacts</a>(request: UpdateBatchContactsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update multiple contacts in a single API call by passing an array of contact objects. Each contact in the array must be identified by one of: email, id, or sms (only one identifier per contact). You can update attributes, blacklist status, list memberships, ext_id, and transactional email forbidden senders for each contact in the batch.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .update_batch_contacts(
            &UpdateBatchContactsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**contacts:** `Option<Vec<UpdateBatchContactsRequestContactsItem>>` — List of contacts to be updated
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">create_doi_contact</a>(request: CreateDoiContactRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note title="How to use attributes param?">attributes param in this endpoint is an object containing key-value pairs where values can be either a string, integer, array, or boolean. You can create key-value pairs with these four datatypes. When a value is an array, it should be an array of strings.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .create_doi_contact(
            &CreateDoiContactRequest {
                email: "elly@example.com".to_string(),
                include_list_ids: vec![36],
                redirection_url: "http://requestb.in/173lyyx1".to_string(),
                template_id: 2,
                attributes: None,
                contact_pixel_tracking_consent: None,
                exclude_list_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, CreateDoiContactRequestAttributesValue>>` — Pass the set of attributes and their values. **These attributes must be present in your Brevo account**. For eg. **{'FNAME':'Elly', 'LNAME':'Roger', 'COUNTRIES': ['India','China']}**
    
</dd>
</dl>

<dl>
<dd>

**contact_pixel_tracking_consent:** `Option<bool>` — Consent of the DOI recipient for open (pixel) and click tracking in the double opt-in confirmation email, resolved by the sender at send time. Considered only if the per-contact pixel tracking consent feature is enabled for your account. Pass `true` if this recipient has consented to open and click tracking, in which case the open pixel and tracked links identify the recipient. Pass `false` to anonymise the open and click events (counted in aggregate statistics only). If it is not passed, the recipient is treated as unknown consent status and the email is still sent (the open and click are anonymised unless your account tracks unknown-consent contacts). A value other than `true`/`false` is rejected. Ignored when the feature is not enabled for your account.
    
</dd>
</dl>

<dl>
<dd>

**email:** `String` — Email address where the confirmation email will be sent. This email address will be the identifier for all other contact attributes.
    
</dd>
</dl>

<dl>
<dd>

**exclude_list_ids:** `Option<Vec<String>>` — Lists under user account where contact should not be added
    
</dd>
</dl>

<dl>
<dd>

**include_list_ids:** `Vec<String>` — Lists under user account where contact should be added
    
</dd>
</dl>

<dl>
<dd>

**redirection_url:** `String` — URL of the web page that user will be redirected to after clicking on the double opt in URL. When editing your DOI template you can reference this URL by using the tag **{{ params.DOIurl }}**.
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `String` — Id of the Double opt-in (DOI) template
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">request_contact_export</a>(request: RequestContactExportRequest) -> Result&lt;RequestContactExportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

It returns the background process ID which on completion calls the notify URL that you have set in the input. File will be available in csv.

When no contact matches the filter, no export process is created: the endpoint answers `200` with an empty object instead of `202` with a processId. A recurring export should treat that as "nothing to do" rather than as a failure, and must not assume a processId is always present.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .request_contact_export(
            &RequestContactExportRequest {
                custom_contact_filter: RequestContactExportRequestCustomContactFilter {
                    ..Default::default()
                },
                disable_notification: None,
                export_attributes: None,
                export_mandatory_attributes: None,
                export_metadata: None,
                export_date_in_utc: None,
                export_subscription_status: None,
                notify_url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**custom_contact_filter:** `RequestContactExportRequestCustomContactFilter` — Set the filter for the contacts to be exported.
    
</dd>
</dl>

<dl>
<dd>

**disable_notification:** `Option<bool>` — To avoid generating the email notification upon contact export, pass **true**
    
</dd>
</dl>

<dl>
<dd>

**export_attributes:** `Option<Vec<String>>` — List of all the attributes that you want to export. **These attributes must be present in your contact database. It is required if exportMandatoryAttributes is set false. ** For example: **['fname', 'lname', 'email']**
    
</dd>
</dl>

<dl>
<dd>

**export_mandatory_attributes:** `Option<bool>` — To export mandatory attributes like EMAIL, ADDED_TIME, MODIFIED_TIME
    
</dd>
</dl>

<dl>
<dd>

**export_metadata:** `Option<Vec<String>>` — Export metadata of contacts such as _listIds, ADDED_TIME, MODIFIED_TIME.
    
</dd>
</dl>

<dl>
<dd>

**export_date_in_utc:** `Option<bool>` — Specifies whether the date fields createdAt, modifiedAt in the exported data should be returned in UTC format.
    
</dd>
</dl>

<dl>
<dd>

**export_subscription_status:** `Option<Vec<String>>` — Export subscription status of contacts for email & sms marketting. Pass email_marketing to obtain the marketing email subscription status & sms_marketing to retrieve the marketing SMS status of the contact.
    
</dd>
</dl>

<dl>
<dd>

**notify_url:** `Option<String>` — Webhook that will be called once the export process is finished. For reference, https://help.brevo.com/hc/en-us/articles/360007666479
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_folders</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetFoldersRequestSort&gt;&gt;) -> Result&lt;GetFoldersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>Ongoing changes for this endpoint
We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
These are non breaking changes.
The default value for the attributes will be 0.
The uniqueSubscribers field is deprecated</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_folders(
            &GetFoldersQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetFoldersRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">create_folder</a>(request: CreateUpdateFolder) -> Result&lt;CreateFolderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new folder to organize your contact lists. Folders serve as containers for grouping related lists together. The folder name is required and must be provided in the request body.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .create_folder(
            &CreateUpdateFolder {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_folder</a>(folder_id: String) -> Result&lt;GetFolder, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>Ongoing changes for this endpoint.
We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
These are non breaking changes. The default value for the attributes will be 0.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.contacts.get_folder(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — id of the folder
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">update_folder</a>(folder_id: String, request: CreateUpdateFolder) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the name of an existing folder identified by its ID. The new folder name must be provided in the request body. Returns a 404 error if the folder ID does not exist.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .update_folder(
            1000000,
            &CreateUpdateFolder {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — Id of the folder
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">delete_folder</a>(folder_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a folder identified by its ID. Deleting a folder will also delete all the contact lists contained within it. This action cannot be undone.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.contacts.delete_folder(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — Id of the folder
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_folder_lists</a>(folder_id: String, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetFolderListsRequestSort&gt;&gt;) -> Result&lt;GetFolderListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>Ongoing changes for this endpoint.
We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
These are non breaking changes. The default value for the attributes will be 0.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_folder_lists(
            1000000,
            &GetFolderListsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — Id of the folder
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetFolderListsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">import_contacts</a>(request: ImportContactsRequest) -> Result&lt;ImportContactsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

It returns the background process ID which on completion calls the notify URL that you have set in the input. **Note**: - Any contact attribute that doesn't exist in your account will be ignored at import end.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .import_contacts(
            &ImportContactsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**disable_notification:** `Option<bool>` — To disable email notification
    
</dd>
</dl>

<dl>
<dd>

**email_blacklist:** `Option<bool>` — To blacklist all the contacts for email
    
</dd>
</dl>

<dl>
<dd>

**empty_contacts_attributes:** `Option<bool>` — To facilitate the choice to erase any attribute of the existing contacts with empty value. emptyContactsAttributes = true means the empty fields in your import will erase any attribute that currently contain data in Brevo, & emptyContactsAttributes = false means the empty fields will not affect your existing data ( **only available if `updateExistingContacts` set to true **)
    
</dd>
</dl>

<dl>
<dd>

**file_body:** `Option<String>` — **Mandatory if fileUrl and jsonBody is not defined.** CSV content to be imported. Use semicolon to separate multiple attributes. **Maximum allowed file body size is 10MB** . However we recommend a safe limit of around 8 MB to avoid the issues caused due to increase of file body size while parsing. Please use fileUrl instead to import bigger files.
    
</dd>
</dl>

<dl>
<dd>

**file_url:** `Option<String>` — **Mandatory if fileBody and jsonBody is not defined.** URL of the file to be imported (**no local file**). Possible file formats: #### .txt, .csv, .json
    
</dd>
</dl>

<dl>
<dd>

**json_body:** `Option<Vec<ImportContactsRequestJsonBodyItem>>` — **Mandatory if fileUrl and fileBody is not defined.** JSON content to be imported. **Maximum allowed json body size is 10MB** . However we recommend a safe limit of around 8 MB to avoid the issues caused due to increase of json body size while parsing. Please use fileUrl instead to import bigger files.
    
</dd>
</dl>

<dl>
<dd>

**list_ids:** `Option<Vec<String>>` — **Mandatory if newList is not defined.** Ids of the lists in which the contacts shall be imported. For example, **[2, 4, 7]**.
    
</dd>
</dl>

<dl>
<dd>

**consent_group_ids:** `Option<Vec<String>>` — **Optional.** Ids of the consent groups to which all imported contacts will be added. Requires consent groups to be enabled for the organisation. For example, **[1, 3]**.
    
</dd>
</dl>

<dl>
<dd>

**new_list:** `Option<ImportContactsRequestNewList>` — To create a new list and import the contacts into it, pass the listName and an optional folderId.
    
</dd>
</dl>

<dl>
<dd>

**notify_url:** `Option<String>` — URL that will be called once the import process is finished. For reference, https://help.brevo.com/hc/en-us/articles/360007666479
    
</dd>
</dl>

<dl>
<dd>

**sms_blacklist:** `Option<bool>` — To blacklist all the contacts for sms
    
</dd>
</dl>

<dl>
<dd>

**update_existing_contacts:** `Option<bool>` — To facilitate the choice to update the existing contacts
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_lists</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetListsRequestSort&gt;&gt;) -> Result&lt;GetListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>Ongoing changes for this endpoint.
We're dropping support for the response attributes totalSubscribers and totalBlacklisted.
These are non breaking changes. The default value for the attributes will be 0.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_lists(
            &GetListsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetListsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">create_list</a>(request: CreateListRequest) -> Result&lt;CreateListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new contact list inside a specified folder. Both the list name and the parent folder ID are required. The newly created list will be empty and ready to receive contacts via the add contacts endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .create_list(
            &CreateListRequest {
                folder_id: 2,
                name: "Magento Customer - ES".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — Id of the parent folder in which this list is to be created
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the list
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_list</a>(list_id: String, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the details of a specific contact list by its ID, including its name, folder ID, creation date, subscriber counts, and campaign statistics. You can optionally filter campaign statistics by providing startDate and endDate parameters (both must be used together in YYYY-MM-DD format).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_list(
            1000000,
            &GetListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — Id of the list
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used**. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to aggregate the sent email campaigns for a specific list id. **Prefer to pass your timezone in date-time format for accurate result**
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used**. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to aggregate the sent email campaigns for a specific list id. **Prefer to pass your timezone in date-time format for accurate result**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">update_list</a>(list_id: String, request: UpdateListRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing contact list identified by its ID. You can update the list name, move it to a different folder by providing a new folderId, or both. Only one of the two parameters (name, folderId) needs to be provided per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .update_list(
            1000000,
            &UpdateListRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — Id of the list
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<String>` — Id of the folder in which the list is to be moved. Either of the two parameters (name, folderId) can be updated at a time.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of the list. Either of the two parameters (name, folderId) can be updated at a time.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">delete_list</a>(list_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a contact list identified by its ID. The contacts in the list are not deleted; they are only removed from this list. Returns a 404 error if the list ID does not exist.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.contacts.delete_list(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — Id of the list
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_contacts_from_list</a>(list_id: String, modified_since: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetContactsFromListRequestSort&gt;&gt;) -> Result&lt;GetContacts, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all contacts belonging to a specific list, identified by its list ID. Results are paginated with a default of 50 contacts per page (maximum 500) and sorted in descending order of creation. You can optionally filter contacts by their modification date using the modifiedSince parameter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_contacts_from_list(
            1000000,
            &GetContactsFromListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — Id of the list
    
</dd>
</dl>

<dl>
<dd>

**modified_since:** `Option<String>` — Filter (urlencoded) the contacts modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetContactsFromListRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">add_contact_to_list</a>(list_id: String, request: AddContactToListRequestBody) -> Result&lt;PostContactInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .add_contact_to_list(
            1000000,
            &AddContactToListRequestBody::AddContactToListRequestBodyEmails(
                AddContactToListRequestBodyEmails {
                    emails: Some(vec![
                        "jeff32@example.com".to_string(),
                        "jim56@example.com".to_string(),
                    ]),
                    ..Default::default()
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — Id of the list
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">remove_contact_from_list</a>(list_id: String, request: RemoveContactFromListRequestBody) -> Result&lt;PostContactInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .remove_contact_from_list(
            1000000,
            &RemoveContactFromListRequestBody::RemoveContactFromListRequestBodyEmails(
                RemoveContactFromListRequestBodyEmails {
                    ..Default::default()
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — Id of the list
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_segments</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetSegmentsRequestSort&gt;&gt;) -> Result&lt;GetSegmentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve all contact segments defined in your Brevo account with support for pagination and sorting. Results default to 10 segments per page (maximum 50) sorted in descending order of creation. Each segment includes its ID, name, category name, and last update timestamp.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_segments(
            &GetSegmentsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetSegmentsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_contact_info</a>(identifier: GetContactInfoRequestIdentifier, identifier_type: Option&lt;Option&lt;GetContactInfoRequestIdentifierType&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetContactInfoResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>Follow this format when passing a "SMS" phone number as an attribute.
Accepted Number Formats 91xxxxxxxxxx +91xxxxxxxxxx 0091xxxxxxxxxx</Note>
There are 2 ways to get a contact <br><br> Option 1- https://api.brevo.com/v3/contacts/{identifier} <br><br> Option 2- https://api.brevo.com/v3/contacts/{identifier}?identifierType={} <br> <br> Option 1 only works if identifierType is email_id (for EMAIL), phone_id (for SMS) or contact_id (for ID of the contact),where you can directly pass the value of EMAIL, SMS and ID of the contact.   <br><br> Option 2 works for all identifierType, use email_id for EMAIL attribute, phone_id for SMS attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute <br><br>Along with the contact details, this endpoint will show the statistics of contact for the recent 90 days by default. To fetch the earlier statistics, please use Get contact campaign stats ``https://developers.brevo.com/reference/contacts-7#getcontactstats`` endpoint with the appropriate date ranges.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_contact_info(
            &GetContactInfoRequestIdentifier::String("identifier".to_string()),
            &GetContactInfoQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `GetContactInfoRequestIdentifier` — Email (urlencoded) OR ID of the contact OR its SMS attribute value OR EXT_ID attribute (urlencoded)
    
</dd>
</dl>

<dl>
<dd>

**identifier_type:** `Option<GetContactInfoRequestIdentifierType>` — email_id for Email, phone_id for SMS attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be lower than equal to endDate. Must not be greater than the current date.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be greater than equal to startDate. Must not be greater than the current date.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">update_contact</a>(identifier: UpdateContactRequestIdentifier, request: UpdateContactRequest, identifier_type: Option&lt;Option&lt;UpdateContactRequestIdentifierType&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>Follow this format when passing a "SMS" phone number as an attribute.
Accepted Number Formats 91xxxxxxxxxx +91xxxxxxxxxx 0091xxxxxxxxxx <br><br> If a blocklisted contact's email address is updated, it is going to remove that blocklisting from the contact and they will be resubscribed.</Note>
There are 2 ways to update a contact <br><br> Option 1- https://api.brevo.com/v3/contacts/{identifier} <br><br> Option 2- https://api.brevo.com/v3/contacts/{identifier}?identifierType={} <br> <br> Option 1 only works if identifierType is email_id (for EMAIL) or contact_id (for ID of the contact),where you can directly pass the value of EMAIL and ID of the contact.   <br><br> Option 2 works for all identifierType, use email_id for EMAIL attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE attribute
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .update_contact(
            &UpdateContactRequestIdentifier::String("identifier".to_string()),
            &UpdateContactRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `UpdateContactRequestIdentifier` — Email (urlencoded) OR ID of the contact OR EXT_ID attribute (urlencoded) OR its SMS attribute value OR its WHATSAPP attribute value OR its LANDLINE_NUMBER attribute value
    
</dd>
</dl>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, UpdateContactRequestAttributesValue>>` — Pass the set of attributes to be updated. **These attributes must be present in your account**. To update existing email address of a contact with the new one please pass EMAIL in attributes. For example, **{ "EMAIL":"newemail@domain.com", "FNAME":"Ellie", "LNAME":"Roger", "COUNTRIES":["India","China"]}**. The attribute's parameter should be passed in capital letter while updating a contact. Values that don't match the attribute type (e.g. text or string in a date attribute) will be ignored .Keep in mind transactional attributes can be updated the same way as normal attributes. Mobile Number in **SMS** field should be passed with proper country code. For example: **{"SMS":"+91xxxxxxxxxx"} or {"SMS":"0091xxxxxxxxxx"}**
    
</dd>
</dl>

<dl>
<dd>

**email_blacklisted:** `Option<bool>` — Set/unset this field to blacklist/allow the contact for emails (emailBlacklisted = true)
    
</dd>
</dl>

<dl>
<dd>

**ext_id:** `Option<String>` — Pass your own Id to update ext_id of a contact.
    
</dd>
</dl>

<dl>
<dd>

**list_ids:** `Option<Vec<String>>` — Ids of the lists to add the contact to
    
</dd>
</dl>

<dl>
<dd>

**sms_blacklisted:** `Option<bool>` — Set/unset this field to blacklist/allow the contact for SMS (smsBlacklisted = true)
    
</dd>
</dl>

<dl>
<dd>

**smtp_blacklist_sender:** `Option<Vec<String>>` — transactional email forbidden sender for contact. Use only for email Contact
    
</dd>
</dl>

<dl>
<dd>

**unlink_list_ids:** `Option<Vec<String>>` — Ids of the lists to remove the contact from
    
</dd>
</dl>

<dl>
<dd>

**force_merge:** `Option<bool>` — When true, if the contact being updated shares an identifier (email, SMS, ext_id, whatsapp, landline) with an existing contact, the two contacts are force-merged. The contact with the most recent `last_modified` timestamp is retained; the other is deleted. When false (default), a 4xx error is returned on identifier conflict.
    
</dd>
</dl>

<dl>
<dd>

**identifier_type:** `Option<UpdateContactRequestIdentifierType>` — email_id for Email, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">delete_contact</a>(identifier: DeleteContactRequestIdentifier, identifier_type: Option&lt;Option&lt;DeleteContactRequestIdentifierType&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

There are 2 ways to delete a contact <br><br> Option 1- https://api.brevo.com/v3/contacts/{identifier} <br><br> Option 2- https://api.brevo.com/v3/contacts/{identifier}?identifierType={} <br> <br> Option 1 only works if identifierType is email_id (for EMAIL) or contact_id (for ID of the contact),where you can directly pass the value of EMAIL and ID of the contact.   <br><br> Option 2 works for all identifierType, use email_id for EMAIL attribute, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .delete_contact(
            &DeleteContactRequestIdentifier::String("identifier".to_string()),
            &DeleteContactQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `DeleteContactRequestIdentifier` — Email (urlencoded) OR ID of the contact OR EXT_ID attribute (urlencoded)
    
</dd>
</dl>

<dl>
<dd>

**identifier_type:** `Option<DeleteContactRequestIdentifierType>` — email_id for Email, contact_id for ID of the contact, ext_id for EXT_ID attribute, phone_id for SMS attribute, whatsapp_id for WHATSAPP attribute, landline_number_id for LANDLINE_NUMBER attribute
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.contacts.<a href="/src/api/resources/contacts/client.rs">get_contact_stats</a>(identifier: GetContactStatsRequestIdentifier, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetContactStatsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve email campaign statistics for a specific contact identified by email address or numeric ID. Statistics include messages sent, opens, clicks, hard/soft bounces, deliveries, unsubscriptions, complaints, and transactional attributes. By default, data covers the last 90 days; use startDate and endDate parameters (YYYY-MM-DD) to specify a custom range with a maximum span of 90 days.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .contacts
        .get_contact_stats(
            &GetContactStatsRequestIdentifier::String("identifier".to_string()),
            &GetContactStatsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `GetContactStatsRequestIdentifier` — Email (urlencoded) OR ID of the contact
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be lower than equal to endDate. Must not be greater than the current date.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the statistic events specific to campaigns. Must be greater than equal to startDate. Must not be greater than the current date. Maximum difference between startDate and endDate should not be greater than 90 days.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ConsentGroups
<details><summary><code>client.consent_groups.<a href="/src/api/resources/consent_groups/client.rs">get_consent_groups</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, id: Option&lt;Option&lt;String&gt;&gt;, name: Option&lt;Option&lt;String&gt;&gt;, signup_mode: Option&lt;Option&lt;GetConsentGroupsRequestSignupMode&gt;&gt;) -> Result&lt;ConsentGroupsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of consent groups for the account.

<Note>This endpoint is only available when the Consent Groups feature is enabled for your account. Returns `403` if the feature is not activated.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .consent_groups
        .get_consent_groups(
            &GetConsentGroupsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Maximum number of results to return (default 10, max 50)
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Number of results to skip (default 0)
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — Filter by consent group ID
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Filter by name (case-insensitive partial match)
    
</dd>
</dl>

<dl>
<dd>

**signup_mode:** `Option<GetConsentGroupsRequestSignupMode>` — Filter by signup mode
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consent_groups.<a href="/src/api/resources/consent_groups/client.rs">create_consent_group</a>(request: CreateConsentGroupRequest) -> Result&lt;ConsentGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new consent group for the account.

<Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .consent_groups
        .create_consent_group(
            &CreateConsentGroupRequest {
                name: "Newsletter EU".to_string(),
                signup_mode: CreateConsentGroupRequestSignupMode::Manual,
                description: None,
                list_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — Unique name for the consent group (max 255 characters)
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Optional description (max 500 characters)
    
</dd>
</dl>

<dl>
<dd>

**signup_mode:** `CreateConsentGroupRequestSignupMode` — Controls how contacts are added to the group. `manual` — contacts are added explicitly via the API. `automatic` — contacts are added automatically at signup.
    
</dd>
</dl>

<dl>
<dd>

**list_ids:** `Option<Vec<String>>` — Optional list of contact list IDs. Contacts from these lists will be copied once into this consent group at creation time.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consent_groups.<a href="/src/api/resources/consent_groups/client.rs">get_consent_group</a>(id: String) -> Result&lt;ConsentGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single consent group by ID for the account.

<Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.consent_groups.get_consent_group(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the consent group
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consent_groups.<a href="/src/api/resources/consent_groups/client.rs">update_consent_group</a>(id: String, request: UpdateConsentGroupRequest) -> Result&lt;ConsentGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates name, description, or signupMode of a consent group. At least one field must be provided.

<Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .consent_groups
        .update_consent_group(
            1000000,
            &UpdateConsentGroupRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the consent group to update
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — New name for the consent group (max 255 characters)
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — New description (max 500 characters)
    
</dd>
</dl>

<dl>
<dd>

**signup_mode:** `Option<UpdateConsentGroupRequestSignupMode>` — New signup mode
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.consent_groups.<a href="/src/api/resources/consent_groups/client.rs">delete_consent_group</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a consent group by ID and removes it from all associated contacts.

<Note>This endpoint is only available when the Consent Groups feature is enabled for your account.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .consent_groups
        .delete_consent_group(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the consent group to delete
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Conversations
<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">sets_agents_status_to_online_for23minutes</a>(request: PostConversationsAgentOnlinePingRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sets the agent's status to online for 2-3 minutes. We recommend pinging this endpoint every minute for as long as the agent has to be considered online. You must provide either `agentId` alone, or all three of `agentEmail` + `agentName` + `receivedFrom`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .sets_agents_status_to_online_for23minutes(
            &PostConversationsAgentOnlinePingRequest {
                agent_id: Some("d9nKoegKSjmCtyK78".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent_email:** `Option<String>` — Agent's email address. When sending online pings from a standalone system, it's hard to maintain a 1-to-1 relationship between the users of both systems. In this case, an agent can be specified by their email address. If there's no agent with the specified email address in your Brevo organization, a dummy agent will be created automatically.
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `Option<String>` — Agent ID. It can be found on the agent's page or received <a href="https://developers.brevo.com/docs/conversations-webhooks">from a webhook</a>. Alternatively, you can use `agentEmail` + `agentName` + `receivedFrom` instead (all 3 fields required).
    
</dd>
</dl>

<dl>
<dd>

**agent_name:** `Option<String>` — Agent's name.
    
</dd>
</dl>

<dl>
<dd>

**received_from:** `Option<String>` — Mark your messages to distinguish messages created by you from the others.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">send_a_message_as_an_agent</a>(request: PostConversationsMessagesRequest) -> Result&lt;ConversationsMessage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a message as an agent to an existing visitor's conversation. You must provide either `agentId` alone, or all three of `agentEmail` + `agentName` + `receivedFrom` to identify the agent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .send_a_message_as_an_agent(
            &PostConversationsMessagesRequest {
                agent_id: Some("d9nKoegKSjmCtyK78".to_string()),
                text: "Hello! How can I help you?".to_string(),
                visitor_id: "kZMvWhf8npAu3H6qd57w2Hv6nh6rnxvg".to_string(),
                agent_email: None,
                agent_name: None,
                received_from: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent_email:** `Option<String>` — Agent's email address. When sending messages from a standalone system, it's hard to maintain a 1-to-1 relationship between the users of both systems. In this case, an agent can be specified by their email address.
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `Option<String>` — Agent ID. It can be found on the agent's page or received <a href="https://developers.brevo.com/docs/conversations-webhooks">from a webhook</a>. Alternatively, you can use `agentEmail` + `agentName` + `receivedFrom` instead (all 3 fields required).
    
</dd>
</dl>

<dl>
<dd>

**agent_name:** `Option<String>` — Agent's name.
    
</dd>
</dl>

<dl>
<dd>

**received_from:** `Option<String>` — Mark your messages to distinguish messages created by you from the others.
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` — Message text.
    
</dd>
</dl>

<dl>
<dd>

**visitor_id:** `String` — Visitor's ID received <a href="https://developers.brevo.com/docs/conversations-webhooks">from a webhook</a> or generated by you to <a href="https://developers.brevo.com/docs/customize-the-widget#identifying-existing-users">bind an existing user account to Conversations</a>.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">get_a_message</a>(id: String) -> Result&lt;ConversationsMessage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a single message by its ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .get_a_message(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the message
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">update_a_message_sent_by_an_agent</a>(id: String, request: PutConversationsMessagesIdRequest) -> Result&lt;ConversationsMessage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the text of a message sent by an agent. Only messages of type `agent` can be edited. The `text` and `html` fields of the message will be updated.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .update_a_message_sent_by_an_agent(
            &"id".to_string(),
            &PutConversationsMessagesIdRequest {
                text: "Good morning! How can I help you?".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the message
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` — The new message text.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">delete_a_message_sent_by_an_agent</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a message sent by an agent. Only messages of type `agent` can be deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .delete_a_message_sent_by_an_agent(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the message
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">send_an_automated_message_to_a_visitor</a>(request: PostConversationsPushedMessagesRequest) -> Result&lt;ConversationsMessage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send an automated (pushed) message to a visitor on behalf of an agent. Example use cases: order status updates, announcing new features in your web app, etc.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.conversations.send_an_automated_message_to_a_visitor(&PostConversationsPushedMessagesRequest {
        group_id: Some("PjRBMhWGen6aRHjif".to_string()),
        text: "Your order has shipped! Here's your tracking number: 9114 5847 3325 9667 4328 88".to_string(),
        visitor_id: "kZMvWhf8npAu3H6qd57w2Hv6nh6rnxvg".to_string(),
        agent_id: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent_id:** `Option<String>` — Agent ID. It can be found on the agent's page or received <a href="https://developers.brevo.com/docs/conversations-webhooks">from a webhook</a>.
    
</dd>
</dl>

<dl>
<dd>

**group_id:** `Option<String>` — Group ID. It can be found on the group's page.
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` — Message text.
    
</dd>
</dl>

<dl>
<dd>

**visitor_id:** `String` — Visitor's ID received <a href="https://developers.brevo.com/docs/conversations-webhooks">from a webhook</a> or generated by you to <a href="https://developers.brevo.com/docs/customize-the-widget#identifying-existing-users">bind an existing user account to Conversations</a>.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">get_an_automated_message</a>(id: String) -> Result&lt;ConversationsMessage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a single automated (pushed) message by its ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .get_an_automated_message(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the message sent previously
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">update_an_automated_message</a>(id: String, request: PutConversationsPushedMessagesIdRequest) -> Result&lt;ConversationsMessage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the text of an automated (pushed) message. The `text` and `html` fields of the message will be updated.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.conversations.update_an_automated_message(&"id".to_string(), &PutConversationsPushedMessagesIdRequest {
        text: "Your order has shipped! Here's your tracking number: 9114 5847 4668 7775 9233 54".to_string()
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the message
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` — The new message text.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">delete_an_automated_message</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an automated (pushed) message by its ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .delete_an_automated_message(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the message
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">set_visitor_group_assignment</a>(request: PutConversationsVisitorGroupRequest) -> Result&lt;PutConversationsVisitorGroupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Assigns a visitor to a specific agent group or removes them from their current group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .conversations
        .set_visitor_group_assignment(
            &PutConversationsVisitorGroupRequest {
                group_id: serde_json::json!("PjRBMhWGen6aRHjif"),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**group_id:** `serde_json::Value` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Ecommerce
<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_categories</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetCategoriesRequestSort&gt;&gt;, name: Option&lt;Option&lt;String&gt;&gt;, modified_since: Option&lt;Option&lt;String&gt;&gt;, created_since: Option&lt;Option&lt;String&gt;&gt;, is_deleted: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetCategoriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of all ecommerce categories stored in your Brevo account. Results are sorted by creation date in descending order by default, and can be filtered by category IDs, name, modification date, creation date, or deletion status. The response includes a `count` field with the total number of matching categories, and pagination defaults to 50 categories per page (maximum 100).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_categories(
            &GetCategoriesQueryRequest {
                limit: None,
                offset: None,
                sort: None,
                ids: vec![],
                name: None,
                modified_since: None,
                created_since: None,
                is_deleted: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetCategoriesRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**ids:** `Option<String>` — Filter by category ids
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Filter by category name
    
</dd>
</dl>

<dl>
<dd>

**modified_since:** `Option<String>` — Filter (urlencoded) the categories modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**created_since:** `Option<String>` — Filter (urlencoded) the categories created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**is_deleted:** `Option<String>` — Filter categories by their deletion status. If `false` is passed, only categories that are not deleted will be returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_update_category</a>(request: CreateUpdateCategoryRequest) -> Result&lt;Option&lt;CreateUpdateCategoryResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new ecommerce category or update an existing one, identified by the mandatory `id` field. When `updateEnabled` is set to `false` (the default), the endpoint performs an insert and returns `201`; if the category ID already exists, a `400` error is returned. When `updateEnabled` is `true`, the endpoint performs an upsert, returning `201` for a new category or `204` when an existing category is updated. The `name` field is mandatory for creation but optional for updates.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_update_category(
            &CreateUpdateCategoryRequest {
                id: "CAT123".to_string(),
                deleted_at: None,
                is_deleted: None,
                name: None,
                update_enabled: None,
                url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**deleted_at:** `Option<String>` — UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the category deleted from the shop's database
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — Unique Category ID as saved in the shop
    
</dd>
</dl>

<dl>
<dd>

**is_deleted:** `Option<bool>` — category deleted from the shop's database
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — **Mandatory in case of creation**. Name of the Category, as displayed in the shop
    
</dd>
</dl>

<dl>
<dd>

**update_enabled:** `Option<bool>` — Facilitate to update the existing category in the same request (updateEnabled = true)
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — URL to the category
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_update_batch_category</a>(request: CreateUpdateBatchCategoryRequest) -> Result&lt;CreateUpdateBatchCategoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create or update multiple ecommerce categories in a single request. The `categories` array accepts up to 100 category objects, each requiring a unique `id`. When `updateEnabled` is `false` (the default), all categories are inserted as new; if any ID already exists, a `400` error is returned. When `updateEnabled` is `true`, existing categories are updated and new ones are created via upsert. Duplicate IDs within the same request payload are rejected. The response returns the count of created and updated categories.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_update_batch_category(
            &CreateUpdateBatchCategoryRequest {
                categories: vec![CreateUpdateBatchCategoryRequestCategoriesItem {
                    id: "CAT123".to_string(),
                    ..Default::default()
                }],
                update_enabled: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**categories:** `Vec<CreateUpdateBatchCategoryRequestCategoriesItem>` — array of categories objects
    
</dd>
</dl>

<dl>
<dd>

**update_enabled:** `Option<bool>` — Facilitate to update the existing categories in the same request (updateEnabled = true)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_category_info</a>(id: String) -> Result&lt;GetCategoryDetails, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the full details of a single ecommerce category by its unique ID. The response includes the category name, URL, creation and modification timestamps, and deletion status. Returns a `404` error if no category matches the provided ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_category_info(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Category ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">activate_the_e_commerce_app</a>() -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Getting access to Brevo eCommerce.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.ecommerce.activate_the_e_commerce_app(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_attribution_metrics_for_one_or_more_brevo_campaigns_or_workflows</a>(period_from: Option&lt;Option&lt;String&gt;&gt;, period_to: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetEcommerceAttributionMetricsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve aggregated ecommerce attribution metrics for one or more Brevo email campaigns, SMS campaigns, or automation workflows. You can optionally filter by a date range using `periodFrom` and `periodTo` in RFC3339 format. The response includes per-source metrics (orders count, revenue, and average basket) as well as aggregated totals across all requested sources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_attribution_metrics_for_one_or_more_brevo_campaigns_or_workflows(
            &GetAttributionMetricsForOneOrMoreBrevoCampaignsOrWorkflowsQueryRequest {
                period_from: Some(DateTime::parse_from_rfc3339("2022-01-02T00:00:00Z").unwrap()),
                period_to: Some(DateTime::parse_from_rfc3339("2022-01-03T00:00:00Z").unwrap()),
                email_campaign_id_array: vec![Some("sale".to_string())],
                sms_campaign_id_array: vec![],
                automation_workflow_email_id_array: vec![],
                automation_workflow_sms_id_array: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**period_from:** `Option<String>` — When getting metrics for a specific period, define the starting datetime in RFC3339 format
    
</dd>
</dl>

<dl>
<dd>

**period_to:** `Option<String>` — When getting metrics for a specific period, define the end datetime in RFC3339 format
    
</dd>
</dl>

<dl>
<dd>

**email_campaign_id_array:** `Option<String>` — The email campaign ID(s) to get metrics for
    
</dd>
</dl>

<dl>
<dd>

**sms_campaign_id_array:** `Option<String>` — The SMS campaign ID(s) to get metrics for
    
</dd>
</dl>

<dl>
<dd>

**automation_workflow_email_id_array:** `Option<String>` — The automation workflow ID(s) to get email attribution metrics for
    
</dd>
</dl>

<dl>
<dd>

**automation_workflow_sms_id_array:** `Option<String>` — The automation workflow ID(s) to get SMS attribution metrics for
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_detailed_attribution_metrics_for_a_single_brevo_campaign_or_workflow</a>(conversion_source: GetEcommerceAttributionMetricsConversionSourceConversionSourceIdRequestConversionSource, conversion_source_id: String) -> Result&lt;GetEcommerceAttributionMetricsConversionSourceConversionSourceIdResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed attribution metrics for a single Brevo campaign or automation workflow, identified by its conversion source type and ID. The response includes orders count, revenue, average basket value, and the number of new customers attributed to that specific campaign or workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.ecommerce.get_detailed_attribution_metrics_for_a_single_brevo_campaign_or_workflow(&GetEcommerceAttributionMetricsConversionSourceConversionSourceIdRequestConversionSource::EmailCampaign, &"sale".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**conversion_source:** `GetEcommerceAttributionMetricsConversionSourceConversionSourceIdRequestConversionSource` — The Brevo campaign type or workflow type for which data will be retrieved
    
</dd>
</dl>

<dl>
<dd>

**conversion_source_id:** `String` — The Brevo campaign or automation workflow id for which data will be retrieved
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_attributed_product_sales_for_a_single_brevo_campaign_or_workflow</a>(conversion_source: GetEcommerceAttributionProductsConversionSourceConversionSourceIdRequestConversionSource, conversion_source_id: String) -> Result&lt;GetEcommerceAttributionProductsConversionSourceConversionSourceIdResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the list of products whose sales have been attributed to a specific Brevo campaign or automation workflow. Each product entry includes its ID, name, SKU, image URL, product URL, price, revenue, and orders count. The conversion source type must be one of `email_campaign`, `sms_campaign`, `automation_workflow_email`, or `automation_workflow_sms`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.ecommerce.get_attributed_product_sales_for_a_single_brevo_campaign_or_workflow(&GetEcommerceAttributionProductsConversionSourceConversionSourceIdRequestConversionSource::EmailCampaign, &"sale".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**conversion_source:** `GetEcommerceAttributionProductsConversionSourceConversionSourceIdRequestConversionSource` — The Brevo campaign or automation workflow type for which data will be retrieved
    
</dd>
</dl>

<dl>
<dd>

**conversion_source_id:** `String` — The Brevo campaign or automation workflow id for which data will be retrieved
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_the_iso4217compliant_display_currency_code_for_your_brevo_account</a>() -> Result&lt;GetEcommerceConfigDisplayCurrencyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the ISO 4217 display currency code currently configured for your Brevo ecommerce account. This currency is used to display monetary values across the ecommerce dashboard and reports. Returns a `403` error if ecommerce is not activated on the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_the_iso4217compliant_display_currency_code_for_your_brevo_account(None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">set_config_display_currency</a>(request: SetConfigDisplayCurrencyRequest) -> Result&lt;SetConfigDisplayCurrencyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Set or update the ISO 4217 display currency code for your Brevo ecommerce account. This currency determines how monetary values are displayed in the ecommerce dashboard and reports. The provided currency code must be a valid ISO 4217 code; invalid codes result in a `422` error. Returns a `403` error if ecommerce is not activated on the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .set_config_display_currency(
            &SetConfigDisplayCurrencyRequest {
                code: "EUR".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` — ISO 4217 compliant display currency code
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_orders</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetOrdersRequestSort&gt;&gt;, secondary_sort_by_modified_at: Option&lt;Option&lt;bool&gt;&gt;, modified_since: Option&lt;Option&lt;String&gt;&gt;, created_since: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the orders stored in your account. Each update sent for an order (for example a status change) is returned as a separate entry, so several entries can share the same `id`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_orders(
            &GetOrdersQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetOrdersRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**secondary_sort_by_modified_at:** `Option<bool>` — When `true`, orders with the same `createdAt` are then ordered by their `updatedAt`, in the same direction as `sort`. Use it to get a stable order when paging through the results with `limit` and `offset`. Default is `false`.
    
</dd>
</dl>

<dl>
<dd>

**modified_since:** `Option<String>` — Filter (urlencoded) the orders modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**created_since:** `Option<String>` — Filter (urlencoded) the orders created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_order</a>(request: Order) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an order or records a change to an existing order, identified by `id`. Each call is stored as a new entry for the order rather than replacing the previous one, so [Get order details](/reference/get-orders) can return several entries with the same `id` (one per status change or other update). Orders sent to this endpoint are processed as live data and can trigger automation workflows. To import past orders without triggering workflows, use [Create orders in batch](/reference/create-batch-order) with `historical: true`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_order(
            &Order {
                amount: 308.42,
                created_at: "2021-07-29T20:59:23.383Z".to_string(),
                id: "14".to_string(),
                products: vec![OrderProductsItem {
                    price: 99.99,
                    product_id: "P1".to_string(),
                    ..Default::default()
                }],
                status: "completed".to_string(),
                updated_at: "2021-07-30T10:59:23.383Z".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_batch_order</a>(request: CreateBatchOrderRequest) -> Result&lt;CreateBatchOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates multiple orders in one request instead of one order at a time. Each order is stored as a new entry rather than replacing a previous order with the same `id`, so [Get order details](/reference/get-orders) can return several entries with the same `id`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_batch_order(
            &CreateBatchOrderRequest {
                orders: vec![Order {
                    amount: 308.42,
                    created_at: "2021-07-29T20:59:23.383Z".to_string(),
                    id: "14".to_string(),
                    products: vec![OrderProductsItem {
                        price: 99.99,
                        product_id: "P1".to_string(),
                        ..Default::default()
                    }],
                    status: "completed".to_string(),
                    updated_at: "2021-07-30T10:59:23.383Z".to_string(),
                    ..Default::default()
                }],
                historical: None,
                notify_url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**historical:** `Option<bool>` — Defines whether you want your orders to be considered as live data or as historical data (import of past data, synchronising data). True: orders will not trigger any automation workflows. False: orders will trigger workflows as usual.
    
</dd>
</dl>

<dl>
<dd>

**notify_url:** `Option<String>` — Webhook URL to receive the status of the batch request
    
</dd>
</dl>

<dl>
<dd>

**orders:** `Vec<Order>` — array of order objects
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_products</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetProductsRequestSort&gt;&gt;, search: Option&lt;Option&lt;String&gt;&gt;, name: Option&lt;Option&lt;String&gt;&gt;, price_lte: Option&lt;Option&lt;f64&gt;&gt;, price_gte: Option&lt;Option&lt;f64&gt;&gt;, price_lt: Option&lt;Option&lt;f64&gt;&gt;, price_gt: Option&lt;Option&lt;f64&gt;&gt;, price_eq: Option&lt;Option&lt;f64&gt;&gt;, price_ne: Option&lt;Option&lt;f64&gt;&gt;, alternative_price_lte: Option&lt;Option&lt;f64&gt;&gt;, alternative_price_gte: Option&lt;Option&lt;f64&gt;&gt;, alternative_price_lt: Option&lt;Option&lt;f64&gt;&gt;, alternative_price_gt: Option&lt;Option&lt;f64&gt;&gt;, alternative_price_eq: Option&lt;Option&lt;f64&gt;&gt;, alternative_price_ne: Option&lt;Option&lt;f64&gt;&gt;, modified_since: Option&lt;Option&lt;String&gt;&gt;, created_since: Option&lt;Option&lt;String&gt;&gt;, sort_by_field: Option&lt;Option&lt;GetProductsRequestSortByField&gt;&gt;, is_deleted: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetProductsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of all ecommerce products stored in your Brevo account. Results are sorted by creation date in descending order by default, and can be filtered by product IDs, name (minimum 3 characters), price range, category IDs, modification date, creation date, or deletion status. Use the `search` parameter to query across SKU, name, and ID simultaneously — results are prioritized as exact SKU match > SKU prefix match > name match > ID match. Pagination defaults to 50 products per page (maximum 1000), and the response includes a `count` field with the total number of matching products.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_products(
            &GetProductsQueryRequest {
                limit: None,
                offset: None,
                sort: None,
                ids: vec![],
                search: None,
                name: None,
                price_lte: None,
                price_gte: None,
                price_lt: None,
                price_gt: None,
                price_eq: None,
                price_ne: None,
                alternative_price_lte: None,
                alternative_price_gte: None,
                alternative_price_lt: None,
                alternative_price_gt: None,
                alternative_price_eq: None,
                alternative_price_ne: None,
                categories: vec![],
                modified_since: None,
                created_since: None,
                sort_by_field: None,
                is_deleted: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetProductsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**ids:** `Option<String>` — Filter by product ids
    
</dd>
</dl>

<dl>
<dd>

**search:** `Option<String>` — Search products simultaneously across SKU, name, and ID fields. Results are returned in the following priority order: **exact SKU match** > **SKU prefix match** > **name match** > **ID match**. For example, `?search=123` on products with `{sku: "123"}` and `{sku: "123456"}` returns the exact SKU match first.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Filter by product name, minimum 3 characters should be present for search.
    
</dd>
</dl>

<dl>
<dd>

**price_lte:** `Option<f64>` — Price filter for products less than and equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**price_gte:** `Option<f64>` — Price filter for products greater than and equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**price_lt:** `Option<f64>` — Price filter for products less than particular amount
    
</dd>
</dl>

<dl>
<dd>

**price_gt:** `Option<f64>` — Price filter for products greater than particular amount
    
</dd>
</dl>

<dl>
<dd>

**price_eq:** `Option<f64>` — Price filter for products equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**price_ne:** `Option<f64>` — Price filter for products not equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**alternative_price_lte:** `Option<f64>` — Alternative price filter for products less than and equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**alternative_price_gte:** `Option<f64>` — Alternative price filter for products greater than and equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**alternative_price_lt:** `Option<f64>` — Alternative price filter for products less than particular amount
    
</dd>
</dl>

<dl>
<dd>

**alternative_price_gt:** `Option<f64>` — Alternative price filter for products greater than particular amount
    
</dd>
</dl>

<dl>
<dd>

**alternative_price_eq:** `Option<f64>` — Alternative price filter for products equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**alternative_price_ne:** `Option<f64>` — Alternative price filter for products not equals to particular amount
    
</dd>
</dl>

<dl>
<dd>

**categories:** `Option<String>` — Filter by categories ids
    
</dd>
</dl>

<dl>
<dd>

**modified_since:** `Option<String>` — Filter (urlencoded) the products modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**created_since:** `Option<String>` — Filter (urlencoded) the products created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**sort_by_field:** `Option<GetProductsRequestSortByField>` — Sort the results by a specific field. Default sort field is `created_at` when not passed.
    
</dd>
</dl>

<dl>
<dd>

**is_deleted:** `Option<String>` — Filter products by their deletion status. If `false` is passed, only products that are not deleted will be returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_update_product</a>(request: CreateUpdateProductRequest) -> Result&lt;Option&lt;CreateUpdateProductResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new ecommerce product or update an existing one, identified by the mandatory `id` field. When `updateEnabled` is `false` (the default), the endpoint inserts a new product and returns `201`; if the product ID already exists, a `400` error is returned. When `updateEnabled` is `true`, the endpoint performs an upsert, returning `201` for a new product or `204` for an update. The `name` field is mandatory for creation but optional for updates. Product images are downloaded, validated (max 5 MB, formats: jpeg, jpg, png, bmp, gif, webp), and re-hosted on S3. The `metaInfo` object supports up to 20 keys with a cumulative size limit of approximately 1000 KB.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_update_product(
            &CreateUpdateProductRequest {
                id: "P11".to_string(),
                name: "Iphone 11".to_string(),
                brand: None,
                categories: None,
                deleted_at: None,
                description: None,
                image_url: None,
                is_deleted: None,
                meta_info: None,
                parent_id: None,
                price: None,
                alternative_price: None,
                sku: None,
                stock: None,
                update_enabled: None,
                url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**brand:** `Option<String>` — Brand of the product
    
</dd>
</dl>

<dl>
<dd>

**categories:** `Option<Vec<String>>` — Category ID-s of the product
    
</dd>
</dl>

<dl>
<dd>

**deleted_at:** `Option<String>` — UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of the product deleted from the shop's database
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Description of the product
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — Product ID for which you requested the details
    
</dd>
</dl>

<dl>
<dd>

**image_url:** `Option<String>` — Absolute URL to the cover image of the product
    
</dd>
</dl>

<dl>
<dd>

**is_deleted:** `Option<bool>` — product deleted from the shop's database
    
</dd>
</dl>

<dl>
<dd>

**meta_info:** `Option<std::collections::HashMap<String, CreateUpdateProductRequestMetaInfoValue>>` — Meta data of product such as description, vendor, producer, stock level. The total characters of cumulative metaInfo shall not exceed **20000 characters**.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — **Mandatory in case of creation**. Name of the product, as displayed in the shop
    
</dd>
</dl>

<dl>
<dd>

**parent_id:** `Option<String>` — Parent product id of the product
    
</dd>
</dl>

<dl>
<dd>

**price:** `Option<String>` — Price of the product
    
</dd>
</dl>

<dl>
<dd>

**alternative_price:** `Option<String>` — Alternative price of the product
    
</dd>
</dl>

<dl>
<dd>

**sku:** `Option<String>` — Product identifier from the shop
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<f64>` — Current stock value of the product from the shop's database
    
</dd>
</dl>

<dl>
<dd>

**update_enabled:** `Option<bool>` — Facilitate to update the existing product in the same request (updateEnabled = true)
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — URL to the product
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_update_batch_products</a>(request: CreateUpdateBatchProductsRequest) -> Result&lt;CreateUpdateBatchProductsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create or update multiple ecommerce products in a single request. The `products` array accepts up to 100 product objects for creation (or up to 1000 when `updateEnabled` is `true` and the account has an increased limit). Each product requires a unique `id` and `name` (name is mandatory for creation only). When `updateEnabled` is `false`, all products are inserted as new; if any ID already exists, a `400` error is returned. When `updateEnabled` is `true`, existing products are updated and new ones are created via upsert. Duplicate IDs within the same request payload are rejected. The response returns the count of created and updated products.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_update_batch_products(
            &CreateUpdateBatchProductsRequest {
                products: vec![CreateUpdateBatchProductsRequestProductsItem {
                    id: "P11".to_string(),
                    name: "Iphone 11".to_string(),
                    ..Default::default()
                }],
                update_enabled: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**products:** `Vec<CreateUpdateBatchProductsRequestProductsItem>` — array of products objects
    
</dd>
</dl>

<dl>
<dd>

**update_enabled:** `Option<bool>` — Facilitate to update the existing products in the same request (updateEnabled = true)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_product_info</a>(id: String) -> Result&lt;GetProductDetails, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the full details of a single ecommerce product by its unique ID. The response includes the product name, price, SKU, URL, image URLs (original and thumbnails), categories, stock level, meta information, creation and modification timestamps, and deletion status. Returns a `404` error if no product matches the provided ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_product_info(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Product ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_product_alert</a>(id: String, type_: String, request: CreateProductAlertRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Register a contact to receive an alert for a specific product event, such as `back_in_stock`. At least one contact identifier (`ext_id`, `email`, or `sms`) must be provided; when multiple are given, priority is `ext_id` > `email` > `sms`. Returns a `404` error if the product ID does not exist, and a `403` error if product alerts are not enabled for the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_product_alert(
            &"id".to_string(),
            &"back_in_stock".to_string(),
            &CreateProductAlertRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Product ID
    
</dd>
</dl>

<dl>
<dd>

**type_:** `String` — Alert type
    
</dd>
</dl>

<dl>
<dd>

**contact_identifiers:** `Option<CreateProductAlertRequestContactIdentifiers>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Coupons
<details><summary><code>client.coupons.<a href="/src/api/resources/coupons/client.rs">get_coupon_collections</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetCouponCollectionsRequestSort&gt;&gt;, sort_by: Option&lt;Option&lt;GetCouponCollectionsRequestSortBy&gt;&gt;) -> Result&lt;GetCouponCollection, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of all coupon collections in your Brevo account. Results can be sorted by creation date, remaining coupons count, or expiration date, in ascending or descending order. Pagination defaults to 50 collections per page (maximum 100).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .coupons
        .get_coupon_collections(
            &GetCouponCollectionsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document on the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetCouponCollectionsRequestSort>` — Sort the results by creation time in ascending/descending order
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<GetCouponCollectionsRequestSortBy>` — The field used to sort coupon collections
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.coupons.<a href="/src/api/resources/coupons/client.rs">create_coupon_collection</a>(request: CreateCouponCollectionRequest) -> Result&lt;CreateCouponCollectionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new coupon collection with a name and a default coupon value. You can optionally set an expiration date in RFC3339 format and configure alert thresholds to receive email notifications when remaining coupons or remaining days before expiration fall below a specified number. The collection ID is auto-generated as a UUID and returned in the response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .coupons
        .create_coupon_collection(
            &CreateCouponCollectionRequest {
                default_coupon: "Winter".to_string(),
                name: "10%OFF".to_string(),
                expiration_date: None,
                remaining_coupons_alert: None,
                remaining_days_alert: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**default_coupon:** `String` — Default coupons collection name
    
</dd>
</dl>

<dl>
<dd>

**expiration_date:** `Option<String>` — Specify an expiration date for the coupon collection in RFC3339 format. Use null to remove the expiration date.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the coupons collection
    
</dd>
</dl>

<dl>
<dd>

**remaining_coupons_alert:** `Option<i64>` — Send a notification alert (email) when the remaining coupons count is equal or fall bellow this number. Use null to disable alerts.
    
</dd>
</dl>

<dl>
<dd>

**remaining_days_alert:** `Option<i64>` — Send a notification alert (email) when the remaining days until the expiration date are equal or fall bellow this number. Use null to disable alerts.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.coupons.<a href="/src/api/resources/coupons/client.rs">get_coupon_collection</a>(id: String) -> Result&lt;GetCouponCollection, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the details of a single coupon collection by its UUID. The response includes the collection name, default coupon value, total and remaining coupon counts, and creation timestamp. Returns a `404` error if no collection matches the provided ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .coupons
        .get_coupon_collection(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the collection to return
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.coupons.<a href="/src/api/resources/coupons/client.rs">update_coupon_collection</a>(id: String, request: UpdateCouponCollectionRequest) -> Result&lt;UpdateCouponCollectionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing coupon collection by its UUID. You can modify the default coupon value, set or remove the expiration date (pass `null` to remove), and configure or disable alert thresholds for remaining coupons or remaining days. Only the fields included in the request body are updated; omitted fields remain unchanged.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .coupons
        .update_coupon_collection(
            &"id".to_string(),
            &UpdateCouponCollectionRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the collection to update
    
</dd>
</dl>

<dl>
<dd>

**default_coupon:** `Option<String>` — A default coupon to be used in case there are no coupons left
    
</dd>
</dl>

<dl>
<dd>

**expiration_date:** `Option<String>` — Specify an expiration date for the coupon collection in RFC3339 format. Use null to remove the expiration date.
    
</dd>
</dl>

<dl>
<dd>

**remaining_coupons_alert:** `Option<i64>` — Send a notification alert (email) when the remaining coupons count is equal or fall bellow this number. Use null to disable alerts.
    
</dd>
</dl>

<dl>
<dd>

**remaining_days_alert:** `Option<i64>` — Send a notification alert (email) when the remaining days until the expiration date are equal or fall bellow this number. Use null to disable alerts.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.coupons.<a href="/src/api/resources/coupons/client.rs">create_coupons</a>(request: CreateCouponsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add coupons to an existing coupon collection. The `coupons` array must contain between 1 and 10,000 unique coupon code strings, all associated with the specified `collectionId`. Coupon creation is processed asynchronously and a `204` status is returned immediately upon acceptance. Returns a `404` error if the specified coupon collection does not exist.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .coupons
        .create_coupons(
            &CreateCouponsRequest {
                collection_id: "23befbae-1505-47a8-bd27-e30ef739f32c".to_string(),
                coupons: vec!["Uf12AF".to_string()],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**collection_id:** `String` — The id of the coupon collection for which the coupons will be created
    
</dd>
</dl>

<dl>
<dd>

**coupons:** `Vec<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Payments
<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">create_payment_request</a>(request: CreatePaymentRequestRequest) -> Result&lt;CreatePaymentRequestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new payment request for a Brevo contact. The request requires a reference (displayed on the payment page), a contact ID, and a cart with currency and amount in cents. You can optionally configure a custom success redirect URL and enable email notifications with reminders. Returns the payment request ID and its public payment URL. A `403` error is returned if Brevo Payments is not activated or the account is not validated.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .payments
        .create_payment_request(
            &CreatePaymentRequestRequest {
                cart: Cart {
                    currency: "EUR".to_string(),
                    specific_amount: 1200,
                },
                contact_id: 43,
                reference: "Invoice #INV0001".to_string(),
                configuration: None,
                description: None,
                notification: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cart:** `Cart` 
    
</dd>
</dl>

<dl>
<dd>

**configuration:** `Option<Configuration>` 
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `String` — Brevo ID of the contact requested to pay.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Description of payment request.
    
</dd>
</dl>

<dl>
<dd>

**notification:** `Option<Notification>` 
    
</dd>
</dl>

<dl>
<dd>

**reference:** `String` — Reference of the payment request, it will appear on the payment page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">get_payment_request</a>(id: String) -> Result&lt;GetPaymentRequestResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the details of a specific payment request by its ID. The response includes the reference, status (created, sent, reminderSent, or paid), cart details, notification configuration, contact ID, and the number of reminders sent. Returns a `404` error if no payment request matches the provided ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .payments
        .get_payment_request(&"050db7b0-9bb7-4c1e-9c68-5a8dace8c1dc".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Id of the payment Request
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">delete_payment_request</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a payment request by its UUID. Once deleted, the payment request can no longer be accessed or paid. Returns a `404` error if no payment request matches the provided ID, and a `403` error if Brevo Payments is not activated or the account is not validated.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .payments
        .delete_payment_request(&"9ae7d68a-565c-4695-9381-d8fb3e3a14cc".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the payment request.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Event
<details><summary><code>client.event.<a href="/src/api/resources/event/client.rs">get_events</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;GetEventsList, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>
This endpoint currently only supports custom events.
</Note>

Retrieve a list of events filtered by various criteria.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .event
        .get_events(
            &GetEventsQueryRequest {
                contact_id: vec![],
                event_name: vec![],
                object_type: vec![],
                start_date: None,
                end_date: None,
                limit: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**contact_id:** `Option<String>` — Filter by contact ID (repeatable)
    
</dd>
</dl>

<dl>
<dd>

**event_name:** `Option<String>` — Filter by event name (repeatable)
    
</dd>
</dl>

<dl>
<dd>

**object_type:** `Option<String>` — Filter by object type (repeatable)
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Mandatory if endDate is used. Start of date range (YYYY-MM-DD or RFC3339). Defaults to 6 months ago when omitted alongside endDate. Must be ≤ endDate.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — Mandatory if startDate is used. End of date range (YYYY-MM-DD or RFC3339). Must be ≥ startDate.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Max events to return. Default 100, min 1, max 10000.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Events to skip for pagination. Default 0, min 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.event.<a href="/src/api/resources/event/client.rs">create_event</a>(request: CreateEventRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create an event to track a contact's interaction. Events created with this endpoint are stored as custom events, including events named `cart_updated`, `cart_deleted` or `order_completed`, so they don't trigger ecommerce automations such as abandoned cart. To send ecommerce cart events, activate the ecommerce app and use the Brevo tracker or the [trackEvent endpoint](/docs/track-custom-events-rest).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .event
        .create_event(
            &CreateEventRequest {
                event_name: "video_played".to_string(),
                identifiers: CreateEventRequestIdentifiers {
                    ..Default::default()
                },
                contact_properties: None,
                event_date: None,
                event_properties: None,
                object: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**contact_properties:** `Option<std::collections::HashMap<String, CreateEventRequestContactPropertiesValue>>` — Properties defining the state of the contact associated to this event. Useful to update contact attributes defined in your contacts database while passing the event. For example: **"FIRSTNAME": "Jane" , "AGE": 37**
    
</dd>
</dl>

<dl>
<dd>

**event_date:** `Option<String>` — ISO 8601 timestamp of when the event occurred (e.g. "2024-01-24T17:39:57+01:00"). If no value is passed, the timestamp of the event creation is used.
    
</dd>
</dl>

<dl>
<dd>

**event_name:** `String` — The name of the event that occurred. This is how you will find your event in Brevo. Limited to 255 characters; only alphanumeric characters, hyphens (-), and underscores (_) are allowed.
    
</dd>
</dl>

<dl>
<dd>

**event_properties:** `Option<std::collections::HashMap<String, CreateEventRequestEventPropertiesValue>>` — Properties of the event. Top level properties and nested properties can be used to better segment contacts and personalise workflow conditions. The following field types are supported: string, number, boolean (true/false), date (Timestamp e.g. "2024-01-24T17:39:57+01:00"). Keys are limited to 255 characters, alphanumerical characters and - _ only. Size is limited to 50KB.
    
</dd>
</dl>

<dl>
<dd>

**identifiers:** `CreateEventRequestIdentifiers` — Identifies the contact associated with the event. At least one identifier is required.
    
</dd>
</dl>

<dl>
<dd>

**object:** `Option<CreateEventRequestObject>` — Identifiers of the object record associated with this event. Ignored if the object type or identifier for this record does not exist on the account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.event.<a href="/src/api/resources/event/client.rs">create_batch_events</a>(request: CreateBatchEventsRequest) -> Result&lt;BatchAcceptedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create multiple events to track contacts' interactions in a single request. Events created with this endpoint are stored as custom events, including events named `cart_updated`, `cart_deleted` or `order_completed`, so they don't trigger ecommerce automations such as abandoned cart. To send ecommerce cart events, activate the ecommerce app and use the Brevo tracker or the [trackEvent endpoint](/docs/track-custom-events-rest).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .event
        .create_batch_events(
            &CreateBatchEventsRequest {
                events: vec![CreateBatchEventsRequestEventsItem {
                    event_name: "order_created".to_string(),
                    identifiers: CreateBatchEventsRequestEventsItemIdentifiers {
                        ..Default::default()
                    },
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**events:** `Vec<CreateBatchEventsRequestEventsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## InboundParsing
<details><summary><code>client.inbound_parsing.<a href="/src/api/resources/inbound_parsing/client.rs">get_inbound_email_events</a>(sender: Option&lt;Option&lt;String&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetInboundEmailEventsRequestSort&gt;&gt;) -> Result&lt;GetInboundEmailEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will show the list of all the events for the received emails. When no date range is provided, the last 30 days of events are returned by default.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .inbound_parsing
        .get_inbound_email_events(
            &GetInboundEmailEventsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sender:** `Option<String>` — Email address of the sender.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Mandatory if endDate is used. Starting date (YYYY-MM-DD or YYYY-MM-DDTHH:mm:ss.SSSZ) from which you want to fetch the list. Maximum time period that can be selected is 30 days. Must not be in the future.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — Mandatory if startDate is used. Ending date (YYYY-MM-DD or YYYY-MM-DDTHH:mm:ss.SSSZ) till which you want to fetch the list. Maximum time period that can be selected is 30 days. Must not be in the future.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document on the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetInboundEmailEventsRequestSort>` — Sort the results in the ascending/descending order of record creation
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inbound_parsing.<a href="/src/api/resources/inbound_parsing/client.rs">get_inbound_email_events_by_uuid</a>(uuid: String) -> Result&lt;GetInboundEmailEventsByUuidResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will show the list of all events history for one particular received email.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .inbound_parsing
        .get_inbound_email_events_by_uuid(&"uuid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**uuid:** `String` — UUID to fetch events specific to received email
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inbound_parsing.<a href="/src/api/resources/inbound_parsing/client.rs">get_inbound_email_attachment</a>(download_token: String) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will retrieve inbound attachment with download token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .inbound_parsing
        .get_inbound_email_attachment(&"downloadToken".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**download_token:** `String` — Token to fetch a particular attachment
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Balance
<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">get_active_balances_api</a>(pid: String, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetLoyaltyBalanceProgramsPidActiveBalanceRequestSort&gt;&gt;, contact_id: Option&lt;String&gt;, balance_definition_id: Option&lt;String&gt;, include_internal: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;GetLoyaltyBalanceProgramsPidActiveBalanceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns Active Balances
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .get_active_balances_api(
            &"pid".to_string(),
            &GetActiveBalancesApiQueryRequest {
                contact_id: 1000000,
                balance_definition_id: "balanceDefinitionId".to_string(),
                limit: None,
                offset: None,
                sort_field: None,
                sort: None,
                include_internal: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Limit
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Offset
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<String>` — Sort Field
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetLoyaltyBalanceProgramsPidActiveBalanceRequestSort>` — Sort Order
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `String` — Contact ID
    
</dd>
</dl>

<dl>
<dd>

**balance_definition_id:** `String` — Balance Definition ID
    
</dd>
</dl>

<dl>
<dd>

**include_internal:** `Option<bool>` — Include balances tied to internal definitions.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">get_balance_definition_list</a>(pid: String, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;GetBalanceDefinitionListRequestSortField&gt;&gt;, sort: Option&lt;Option&lt;GetBalanceDefinitionListRequestSort&gt;&gt;, version: Option&lt;Option&lt;GetBalanceDefinitionListRequestVersion&gt;&gt;) -> Result&lt;Option&lt;GetBalanceDefinitionListResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns balance definition page
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .get_balance_definition_list(
            &"pid".to_string(),
            &GetBalanceDefinitionListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Limit the number of records returned
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Offset to paginate records
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<GetBalanceDefinitionListRequestSortField>` — Field to sort by
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetBalanceDefinitionListRequestSort>` — Sort direction
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetBalanceDefinitionListRequestVersion>` — Version
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">create_balance_definition</a>(pid: String, request: PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequest) -> Result&lt;BalanceDefinition, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates balance definition and returns information
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .create_balance_definition(
            &"pid".to_string(),
            &PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequest {
                name: "name".to_string(),
                unit: PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestUnit::Points,
                balance_availability_duration_modifier: None,
                balance_availability_duration_unit: None,
                balance_availability_duration_value: None,
                balance_expiration_date: None,
                balance_option_amount_overtaking_strategy: None,
                balance_option_credit_rounding: None,
                balance_option_debit_rounding: None,
                description: None,
                image_ref: None,
                max_amount: None,
                max_credit_amount_limit: None,
                max_debit_amount_limit: None,
                meta: None,
                min_amount: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**balance_availability_duration_modifier:** `Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationModifier>` — Defines when the balance expires within the selected duration.
    
</dd>
</dl>

<dl>
<dd>

**balance_availability_duration_unit:** `Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceAvailabilityDurationUnit>` — Unit of time for balance validity.
    
</dd>
</dl>

<dl>
<dd>

**balance_availability_duration_value:** `Option<i64>` — Number of time units before the balance expires.
    
</dd>
</dl>

<dl>
<dd>

**balance_expiration_date:** `Option<String>` — Fixed expiration date (`dd/mm` format) as an alternative to duration-based expiry.
    
</dd>
</dl>

<dl>
<dd>

**balance_option_amount_overtaking_strategy:** `Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionAmountOvertakingStrategy>` — Defines whether partial credit is allowed when reaching max balance.
    
</dd>
</dl>

<dl>
<dd>

**balance_option_credit_rounding:** `Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionCreditRounding>` — Defines rounding strategy for credit transactions.
    
</dd>
</dl>

<dl>
<dd>

**balance_option_debit_rounding:** `Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestBalanceOptionDebitRounding>` — Defines rounding strategy for debit transactions.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Short description of the balance definition.
    
</dd>
</dl>

<dl>
<dd>

**image_ref:** `Option<String>` — URL of an optional image reference.
    
</dd>
</dl>

<dl>
<dd>

**max_amount:** `Option<f64>` — Maximum allowable balance amount.
    
</dd>
</dl>

<dl>
<dd>

**max_credit_amount_limit:** `Option<f64>` — Maximum credit allowed per operation.
    
</dd>
</dl>

<dl>
<dd>

**max_debit_amount_limit:** `Option<f64>` — Maximum debit allowed per operation.
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestMeta>` — Additional metadata for the balance definition.
    
</dd>
</dl>

<dl>
<dd>

**min_amount:** `Option<f64>` — Minimum allowable balance amount.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the balance definition.
    
</dd>
</dl>

<dl>
<dd>

**unit:** `PostLoyaltyBalanceProgramsPidBalanceDefinitionsRequestUnit` — Unit of balance measurement.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">get_balance_definition</a>(pid: String, bdid: String, version: Option&lt;Option&lt;GetBalanceDefinitionRequestVersion&gt;&gt;) -> Result&lt;BalanceDefinition, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns balance definition
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .get_balance_definition(
            &"pid".to_string(),
            &"bdid".to_string(),
            &GetBalanceDefinitionQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**bdid:** `String` — Balance Definition Id
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetBalanceDefinitionRequestVersion>` — Version
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">update_balance_definition</a>(pid: String, bdid: String, request: UpdateBalanceDefinitionRequest) -> Result&lt;BalanceDefinition, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates Balance definition
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .update_balance_definition(
            &"pid".to_string(),
            &"bdid".to_string(),
            &UpdateBalanceDefinitionRequest {
                name: "name".to_string(),
                unit: UpdateBalanceDefinitionRequestUnit::Points,
                balance_availability_duration_modifier: None,
                balance_availability_duration_unit: None,
                balance_availability_duration_value: None,
                balance_expiration_date: None,
                balance_option_amount_overtaking_strategy: None,
                balance_option_credit_rounding: None,
                balance_option_debit_rounding: None,
                description: None,
                image_ref: None,
                max_amount: None,
                max_credit_amount_limit: None,
                max_debit_amount_limit: None,
                meta: None,
                min_amount: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**bdid:** `String` — Balance Definition Id
    
</dd>
</dl>

<dl>
<dd>

**balance_availability_duration_modifier:** `Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationModifier>` — Defines when the balance expires within the selected duration.
    
</dd>
</dl>

<dl>
<dd>

**balance_availability_duration_unit:** `Option<UpdateBalanceDefinitionRequestBalanceAvailabilityDurationUnit>` — Unit of time for balance validity.
    
</dd>
</dl>

<dl>
<dd>

**balance_availability_duration_value:** `Option<i64>` — Number of time units before the balance expires.
    
</dd>
</dl>

<dl>
<dd>

**balance_expiration_date:** `Option<String>` — Expiration date (`dd/mm` format) or empty if not applicable.
    
</dd>
</dl>

<dl>
<dd>

**balance_option_amount_overtaking_strategy:** `Option<UpdateBalanceDefinitionRequestBalanceOptionAmountOvertakingStrategy>` — Defines whether partial credit is allowed when reaching max balance.
    
</dd>
</dl>

<dl>
<dd>

**balance_option_credit_rounding:** `Option<UpdateBalanceDefinitionRequestBalanceOptionCreditRounding>` — Rounding strategy for credit transactions.
    
</dd>
</dl>

<dl>
<dd>

**balance_option_debit_rounding:** `Option<UpdateBalanceDefinitionRequestBalanceOptionDebitRounding>` — Rounding strategy for debit transactions.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Short description of the balance definition.
    
</dd>
</dl>

<dl>
<dd>

**image_ref:** `Option<String>` — URL of an optional image reference.
    
</dd>
</dl>

<dl>
<dd>

**max_amount:** `Option<f64>` — Maximum allowable balance amount.
    
</dd>
</dl>

<dl>
<dd>

**max_credit_amount_limit:** `Option<f64>` — Maximum credit allowed per operation.
    
</dd>
</dl>

<dl>
<dd>

**max_debit_amount_limit:** `Option<f64>` — Maximum debit allowed per operation.
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<UpdateBalanceDefinitionRequestMeta>` — Optional metadata for the balance definition.
    
</dd>
</dl>

<dl>
<dd>

**min_amount:** `Option<f64>` — Minimum allowable balance amount.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the balance definition.
    
</dd>
</dl>

<dl>
<dd>

**unit:** `UpdateBalanceDefinitionRequestUnit` — Unit of balance measurement.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">delete_balance_definition</a>(pid: String, bdid: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete Balance definition
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .delete_balance_definition(&"pid".to_string(), &"bdid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**bdid:** `String` — Balance Definition Id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">create_balance_limit</a>(pid: String, bdid: String, request: CreateBalanceLimitRequest) -> Result&lt;BalanceLimit, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates balance limit and sends the created UUID along with the data
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .create_balance_limit(
            &"pid".to_string(),
            &"bdid".to_string(),
            &CreateBalanceLimitRequest {
                constraint_type: CreateBalanceLimitRequestConstraintType::Transaction,
                duration_unit: CreateBalanceLimitRequestDurationUnit::Day,
                duration_value: 1,
                transaction_type: CreateBalanceLimitRequestTransactionType::Credit,
                value: 1,
                sliding_schedule: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**bdid:** `String` — Balance Definition Id
    
</dd>
</dl>

<dl>
<dd>

**constraint_type:** `CreateBalanceLimitRequestConstraintType` — Defines whether the limit applies to transaction count or amount.
    
</dd>
</dl>

<dl>
<dd>

**duration_unit:** `CreateBalanceLimitRequestDurationUnit` — Unit of time for which the limit is applicable.
    
</dd>
</dl>

<dl>
<dd>

**duration_value:** `i64` — Number of time units for the balance limit.
    
</dd>
</dl>

<dl>
<dd>

**sliding_schedule:** `Option<bool>` — Determines if the limit resets on a rolling schedule.
    
</dd>
</dl>

<dl>
<dd>

**transaction_type:** `CreateBalanceLimitRequestTransactionType` — Specifies whether the limit applies to credit or debit transactions.
    
</dd>
</dl>

<dl>
<dd>

**value:** `i64` — Maximum allowed value for the specified constraint type.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">get_balance_limit</a>(pid: String, bdid: String, blid: String, version: Option&lt;Option&lt;GetBalanceLimitRequestVersion&gt;&gt;) -> Result&lt;BalanceLimit, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches balance limits and send the created UUID along with the data
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .get_balance_limit(
            &"pid".to_string(),
            &"bdid".to_string(),
            &"blid".to_string(),
            &GetBalanceLimitQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**bdid:** `String` — Balance Definition Id
    
</dd>
</dl>

<dl>
<dd>

**blid:** `String` — Balance Limit Id
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetBalanceLimitRequestVersion>` — Version
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">update_balance_limit</a>(pid: String, bdid: String, blid: String, request: UpdateBalanceLimitRequest) -> Result&lt;BalanceLimit, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates balance limit
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .update_balance_limit(
            &"pid".to_string(),
            &"bdid".to_string(),
            &"blid".to_string(),
            &UpdateBalanceLimitRequest {
                constraint_type: UpdateBalanceLimitRequestConstraintType::Transaction,
                duration_unit: UpdateBalanceLimitRequestDurationUnit::Day,
                duration_value: 1,
                transaction_type: UpdateBalanceLimitRequestTransactionType::Credit,
                value: 1,
                sliding_schedule: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**bdid:** `String` — Balance Definition Id
    
</dd>
</dl>

<dl>
<dd>

**blid:** `String` — Balance Limit Id
    
</dd>
</dl>

<dl>
<dd>

**constraint_type:** `UpdateBalanceLimitRequestConstraintType` — Defines whether the limit applies to transaction count or amount.
    
</dd>
</dl>

<dl>
<dd>

**duration_unit:** `UpdateBalanceLimitRequestDurationUnit` — Unit of time for which the limit is applicable.
    
</dd>
</dl>

<dl>
<dd>

**duration_value:** `i64` — Number of time units for the balance limit.
    
</dd>
</dl>

<dl>
<dd>

**sliding_schedule:** `Option<bool>` — Determines if the limit resets on a rolling schedule.
    
</dd>
</dl>

<dl>
<dd>

**transaction_type:** `UpdateBalanceLimitRequestTransactionType` — Specifies whether the limit applies to credit or debit transactions.
    
</dd>
</dl>

<dl>
<dd>

**value:** `i64` — Maximum allowed value for the specified constraint type.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">delete_balance_limit</a>(pid: String, bdid: String, blid: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete balance limit
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .delete_balance_limit(
            &"pid".to_string(),
            &"bdid".to_string(),
            &"blid".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**bdid:** `String` — Balance Definition Id
    
</dd>
</dl>

<dl>
<dd>

**blid:** `String` — Balance Limit Id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">get_contact_balances</a>(pid: String, include_internal: Option&lt;Option&lt;bool&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort: Option&lt;Option&lt;GetContactBalancesRequestSort&gt;&gt;, sort_field: Option&lt;Option&lt;GetContactBalancesRequestSortField&gt;&gt;, balance_definition_id: Option&lt;String&gt;) -> Result&lt;GetContactBalancesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns contact balances for a given balance definition across all subscriptions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .get_contact_balances(
            &"pid".to_string(),
            &GetContactBalancesQueryRequest {
                balance_definition_id: "balanceDefinitionId".to_string(),
                include_internal: None,
                limit: None,
                offset: None,
                sort: None,
                sort_field: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**include_internal:** `Option<bool>` — Include balances tied to internal definitions.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Limit the number of records returned
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Skip a number of records
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetContactBalancesRequestSort>` — Sort order
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<GetContactBalancesRequestSortField>` — Field to sort by
    
</dd>
</dl>

<dl>
<dd>

**balance_definition_id:** `String` — Balance Definition ID (required)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">create_balance_order</a>(pid: String, request: CreateBalanceOrderRequest) -> Result&lt;CreateBalanceOrderResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns created order
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .create_balance_order(
            &"pid".to_string(),
            &CreateBalanceOrderRequest {
                amount: 1.1,
                balance_definition_id: "balanceDefinitionId".to_string(),
                contact_id: 1,
                due_at: "dueAt".to_string(),
                source: CreateBalanceOrderRequestSource::Engine,
                expires_at: None,
                meta: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**amount:** `f64` — Order amount (must be non-zero).
    
</dd>
</dl>

<dl>
<dd>

**balance_definition_id:** `String` — Unique identifier (UUID) of the associated balance definition.
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `i64` — Unique identifier of the contact placing the order (must be ≥ 1).
    
</dd>
</dl>

<dl>
<dd>

**due_at:** `String` — RFC3339 timestamp specifying when the order is due.
    
</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<String>` — Optional RFC3339 timestamp defining order expiration.
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Optional metadata associated with the order.
    
</dd>
</dl>

<dl>
<dd>

**source:** `CreateBalanceOrderRequestSource` — Specifies the origin of the order.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">get_subscription_balances</a>(pid: String, cid: String, include_internal: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;GetSubscriptionBalancesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns subscription balances
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .get_subscription_balances(
            &"pid".to_string(),
            &"cid".to_string(),
            &GetSubscriptionBalancesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**cid:** `String` — Contact Id
    
</dd>
</dl>

<dl>
<dd>

**include_internal:** `Option<bool>` — Include balances tied to internal definitions.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">create_subscription_balances</a>(pid: String, cid: String, request: PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest) -> Result&lt;PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a balance for a contact
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .create_subscription_balances(
            &"pid".to_string(),
            &"cid".to_string(),
            &PostLoyaltyBalanceProgramsPidSubscriptionsCidBalancesRequest {
                balance_definition_id: "balanceDefinitionId".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**cid:** `String` — Contact Id
    
</dd>
</dl>

<dl>
<dd>

**balance_definition_id:** `String` — Unique identifier (UUID) of the balance definition associated with the new balance.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">get_transaction_history_api</a>(pid: String, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetLoyaltyBalanceProgramsPidTransactionHistoryRequestSort&gt;&gt;, contact_id: Option&lt;String&gt;, balance_definition_id: Option&lt;String&gt;, status: Option&lt;Option&lt;GetLoyaltyBalanceProgramsPidTransactionHistoryRequestStatus&gt;&gt;, transaction_type: Option&lt;Option&lt;GetLoyaltyBalanceProgramsPidTransactionHistoryRequestTransactionType&gt;&gt;, loyalty_subscription_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetLoyaltyBalanceProgramsPidTransactionHistoryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns transaction history
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .get_transaction_history_api(
            &"pid".to_string(),
            &GetTransactionHistoryApiQueryRequest {
                contact_id: 1000000,
                balance_definition_id: "balanceDefinitionId".to_string(),
                limit: None,
                offset: None,
                sort_field: None,
                sort: None,
                status: None,
                transaction_type: None,
                loyalty_subscription_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Limit the number of records returned
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Page number to retrieve
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<String>` — Field to sort by
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestSort>` — Sort order
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `String` — Contact ID
    
</dd>
</dl>

<dl>
<dd>

**balance_definition_id:** `String` — Balance Definition ID
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestStatus>` — Transaction status filter
    
</dd>
</dl>

<dl>
<dd>

**transaction_type:** `Option<GetLoyaltyBalanceProgramsPidTransactionHistoryRequestTransactionType>` — Transaction type filter
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — Loyalty Subscription ID filter
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">begin_transaction</a>(pid: String, request: BeginTransactionRequest) -> Result&lt;Transaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates new transaction and returns information
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .begin_transaction(
            &"pid".to_string(),
            &BeginTransactionRequest {
                amount: 1.1,
                balance_definition_id: "balanceDefinitionId".to_string(),
                transaction_type: None,
                contact_id: None,
                loyalty_subscription_id: None,
                meta: None,
                ttl: None,
                event_time: None,
                auto_complete: None,
                balance_expiry_in_minutes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**amount:** `f64` — Transaction amount. A positive value creates a credit transaction and a negative value creates a debit transaction (unless transactionType is explicitly provided).
    
</dd>
</dl>

<dl>
<dd>

**transaction_type:** `Option<BeginTransactionRequestTransactionType>` — Explicit transaction type. If not provided, the type is inferred from the sign of the amount (positive = credit, negative = debit).
    
</dd>
</dl>

<dl>
<dd>

**balance_definition_id:** `String` — Unique identifier (UUID) of the associated balance definition.
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `Option<String>` — Unique identifier of the contact involved in the transaction. Required unless `LoyaltySubscriptionId` is provided.
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — Unique identifier for the loyalty subscription. Required unless `contactId` is provided.
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Optional metadata associated with the transaction.
    
</dd>
</dl>

<dl>
<dd>

**ttl:** `Option<i64>` — Time-to-live for the transaction in seconds. Must be at least 10 seconds if provided.
    
</dd>
</dl>

<dl>
<dd>

**event_time:** `Option<String>` — Timestamp specifying when the transaction event occurred (ISO 8601 / RFC 3339 format).
    
</dd>
</dl>

<dl>
<dd>

**auto_complete:** `Option<bool>` — Whether the transaction should be automatically completed.
    
</dd>
</dl>

<dl>
<dd>

**balance_expiry_in_minutes:** `Option<i64>` — Expiry time for the balance in minutes. Must be greater than 0 if provided. Only applicable when autoComplete is true.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">cancel_transaction</a>(pid: String, tid: String) -> Result&lt;Transaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancels transaction
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .cancel_transaction(&"pid".to_string(), &"tid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**tid:** `String` — Transaction Id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.balance.<a href="/src/api/resources/balance/client.rs">complete_transaction</a>(pid: String, tid: String) -> Result&lt;Transaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Completes transaction
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .balance
        .complete_transaction(&"pid".to_string(), &"tid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program Id
    
</dd>
</dl>

<dl>
<dd>

**tid:** `String` — Transaction Id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Program
<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">get_lp_list</a>(limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;GetLpListRequestSortField&gt;&gt;, sort: Option&lt;Option&lt;GetLpListRequestSort&gt;&gt;) -> Result&lt;GetLpListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns list of loyalty programs
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .get_lp_list(
            &GetLpListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<i64>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<GetLpListRequestSortField>` — Sort documents by field
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetLpListRequestSort>` — Sort order
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">create_new_lp</a>(request: CreateNewLpRequest) -> Result&lt;LoyaltyProgram, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates loyalty program
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .create_new_lp(
            &CreateNewLpRequest {
                name: "name".to_string(),
                description: None,
                document_id: None,
                meta: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**description:** `Option<String>` — Optional description of the loyalty program (max 256 chars).
    
</dd>
</dl>

<dl>
<dd>

**document_id:** `Option<String>` — Optional unique document ID.
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Optional metadata related to the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Required name of the loyalty program (max 128 chars).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">get_loyalty_program_info</a>(pid: String) -> Result&lt;LoyaltyProgram, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns loyalty program
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .get_loyalty_program_info(&"pid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">update_loyalty_program</a>(pid: String, request: UpdateLoyaltyProgramRequest) -> Result&lt;LoyaltyProgram, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates loyalty program
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .update_loyalty_program(
            &"pid".to_string(),
            &UpdateLoyaltyProgramRequest {
                name: "name".to_string(),
                description: None,
                meta: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Loyalty Program description
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Loyalty Program meta data
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Loyalty Program name
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">delete_loyalty_program</a>(pid: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes Loyalty Program
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .delete_loyalty_program(&"pid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">partially_update_loyalty_program</a>(pid: String, request: PartiallyUpdateLoyaltyProgramRequest) -> Result&lt;LoyaltyProgram, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Partially updates loyalty program
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .partially_update_loyalty_program(
            &"pid".to_string(),
            &PartiallyUpdateLoyaltyProgramRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Loyalty Program description
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Loyalty Program meta data
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Loyalty Program name
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">get_parameter_subscription_info</a>(pid: String, contact_id: Option&lt;Option&lt;String&gt;&gt;, params: Option&lt;Option&lt;String&gt;&gt;, loyalty_subscription_id: Option&lt;Option&lt;String&gt;&gt;, include_internal: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;GetParameterSubscriptionInfoResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get Information of balances, tiers, rewards and subscription members for a subscription
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .get_parameter_subscription_info(
            &"pid".to_string(),
            &GetParameterSubscriptionInfoQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `Option<String>` — The contact ID to filter by.
    
</dd>
</dl>

<dl>
<dd>

**params:** `Option<String>` — A list of filter parameters for querying the subscription info.
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — The loyalty subscription ID to filter by.
    
</dd>
</dl>

<dl>
<dd>

**include_internal:** `Option<bool>` — Include balances tied to internal definitions.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">delete_contact_subscription</a>(pid: String, cid: i64) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete subscription for a contact
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .delete_contact_subscription(&"pid".to_string(), 1, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**cid:** `i64` — Contact ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">publish_loyalty_program</a>(pid: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Publishes loyalty program
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .publish_loyalty_program(&"pid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">subscribe_member_to_a_subscription</a>(pid: String, request: SubscribeMemberToASubscriptionRequest) -> Result&lt;SubscribeMemberToASubscriptionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add member to a subscription
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .subscribe_member_to_a_subscription(
            &"pid".to_string(),
            &SubscribeMemberToASubscriptionRequest {
                member_contact_ids: vec![1],
                contact_id: None,
                loyalty_subscription_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `Option<i64>` — Required if LoyaltySubscriptionId is not provided, must be greater than 0
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — Required if ContactId is not provided, max length 64
    
</dd>
</dl>

<dl>
<dd>

**member_contact_ids:** `Vec<i64>` — Required, each item must be greater than or equal to 1
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">delete_contact_members</a>(pid: String, member_contact_ids: Option&lt;String&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes member from a subscription
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .delete_contact_members(
            &"pid".to_string(),
            &DeleteContactMembersQueryRequest {
                member_contact_ids: "memberContactIds".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**member_contact_ids:** `String` — Comma-separated list of member contact IDs to delete from the subscription.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.program.<a href="/src/api/resources/program/client.rs">subscribe_to_loyalty_program</a>(pid: String, request: SubscribeToLoyaltyProgramRequest) -> Result&lt;SubscribeToLoyaltyProgramResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Subscribes to a loyalty program
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .program
        .subscribe_to_loyalty_program(
            &"pid".to_string(),
            &SubscribeToLoyaltyProgramRequest {
                contact_id: 1000000,
                loyalty_subscription_id: None,
                creation_date: None,
                meta: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID. A unique identifier for the loyalty program.
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `String` — Required contact ID; must be greater than 0.
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — Optional subscription ID (max length 64).
    
</dd>
</dl>

<dl>
<dd>

**creation_date:** `Option<String>` — Optional creation date in ISO 8601 format (YYYY-MM-DDThh:mm:ss.ffffff+HH:MM). Must be in the past.
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Optional metadata associated with the subscription.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Reward
<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">get_code_count</a>(pid: String, cpid: String) -> Result&lt;GetCodeCountResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get code count
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .get_code_count(&"pid".to_string(), &"cpid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**cpid:** `String` — Code Pool ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">get_reward_page_api</a>(pid: String, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, state: Option&lt;Option&lt;String&gt;&gt;, version: Option&lt;Option&lt;GetLoyaltyOfferProgramsPidOffersRequestVersion&gt;&gt;) -> Result&lt;GetLoyaltyOfferProgramsPidOffersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a reward page
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .get_reward_page_api(
            &"pid".to_string(),
            &GetRewardPageApiQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Page size
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Pagination offset
    
</dd>
</dl>

<dl>
<dd>

**state:** `Option<String>` — State of the reward
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetLoyaltyOfferProgramsPidOffersRequestVersion>` — Version
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">create_reward</a>(pid: String, request: CreateRewardRequest) -> Result&lt;CreateRewardResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new reward in the loyalty program.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .create_reward(
            &"pid".to_string(),
            &CreateRewardRequest {
                name: "name".to_string(),
                public_description: None,
                public_image: None,
                public_name: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Internal name of the reward
    
</dd>
</dl>

<dl>
<dd>

**public_description:** `Option<String>` — Public facing description of the reward
    
</dd>
</dl>

<dl>
<dd>

**public_image:** `Option<String>` — URL of the public image for the reward
    
</dd>
</dl>

<dl>
<dd>

**public_name:** `Option<String>` — Public facing name of the reward
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">create_voucher</a>(pid: String, request: CreateVoucherRequest) -> Result&lt;CreateVoucherResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a voucher and attribute it to a specific membership.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .create_voucher(
            &"pid".to_string(),
            &CreateVoucherRequest {
                reward_id: "rewardId".to_string(),
                code: None,
                contact_id: None,
                expiration_date: None,
                loyalty_subscription_id: None,
                meta: None,
                value: None,
                valid_from: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` — Code generated to attribute reward to a contact
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `Option<String>` — Contact to attribute the reward
    
</dd>
</dl>

<dl>
<dd>

**expiration_date:** `Option<String>` — Reward expiration date
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — One of contactId or loyaltySubscriptionId is required
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Offer meta information (key/value object)
    
</dd>
</dl>

<dl>
<dd>

**reward_id:** `String` — Reward id
    
</dd>
</dl>

<dl>
<dd>

**value:** `Option<f64>` — Value of the selected reward config
    
</dd>
</dl>

<dl>
<dd>

**valid_from:** `Option<String>` — Date from which the voucher becomes valid. Accepts RFC 3339 or DD/MM/YYYY HH:MM AM/PM format. Converted to UTC using the organization's timezone.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">redeem_voucher</a>(pid: String, request: RedeemVoucherRequest) -> Result&lt;Redeem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a request to redeem a voucher.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .redeem_voucher(
            &"pid".to_string(),
            &RedeemVoucherRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**attributed_reward_id:** `Option<String>` — Unique identifier for the attributed reward
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` — Redemption code for the reward
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `Option<String>` — Unique identifier for the contact
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — Identifier for the loyalty subscription
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Additional metadata associated with the redeem request
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<RedeemVoucherRequestOrder>` — Order details for the redemption
    
</dd>
</dl>

<dl>
<dd>

**reward_id:** `Option<String>` — Unique identifier for the reward
    
</dd>
</dl>

<dl>
<dd>

**ttl:** `Option<i64>` — Time to live in seconds for the redemption request
    
</dd>
</dl>

<dl>
<dd>

**auto_complete:** `Option<bool>` — Whether the redemption should be automatically completed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">complete_redeem_transaction</a>(pid: String, tid: String) -> Result&lt;Redeem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Completes voucher redeem request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .complete_redeem_transaction(&"pid".to_string(), &"tid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**tid:** `String` — Redeem transaction ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">revoke_vouchers</a>(pid: String, attributed_reward_ids: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Revoke attributed vouchers.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .revoke_vouchers(
            &"pid".to_string(),
            &RevokeVouchersQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**attributed_reward_ids:** `Option<String>` — Reward Attribution IDs (comma seperated)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">validate_reward</a>(pid: String, request: ValidateRewardRequest) -> Result&lt;ValidateRewardResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates a reward.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .validate_reward(
            &"pid".to_string(),
            &ValidateRewardRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**attributed_reward_id:** `Option<String>` — Unique identifier for the attributed reward
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` — Validation code for the reward
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `Option<String>` — Unique identifier for the contact
    
</dd>
</dl>

<dl>
<dd>

**loyalty_subscription_id:** `Option<String>` — Identifier for the loyalty subscription
    
</dd>
</dl>

<dl>
<dd>

**point_of_sell_id:** `Option<String>` — Identifier for the point of sale
    
</dd>
</dl>

<dl>
<dd>

**reward_id:** `Option<String>` — Unique identifier for the reward
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">get_reward_information</a>(pid: String, rid: String, version: Option&lt;Option&lt;GetLoyaltyOfferProgramsPidRewardsRidRequestVersion&gt;&gt;) -> Result&lt;GetLoyaltyOfferProgramsPidRewardsRidResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns reward information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .get_reward_information(
            &"pid".to_string(),
            &"rid".to_string(),
            &GetRewardInformationQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**rid:** `String` — Reward ID
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetLoyaltyOfferProgramsPidRewardsRidRequestVersion>` — Version
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reward.<a href="/src/api/resources/reward/client.rs">get_voucher_for_a_contact</a>(pid: String, limit: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort: Option&lt;Option&lt;GetLoyaltyOfferProgramsPidVouchersRequestSort&gt;&gt;, sort_field: Option&lt;Option&lt;GetLoyaltyOfferProgramsPidVouchersRequestSortField&gt;&gt;, contact_id: Option&lt;i64&gt;, metadata_key_value: Option&lt;Option&lt;String&gt;&gt;, reward_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetLoyaltyOfferProgramsPidVouchersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get voucher for a contact
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .reward
        .get_voucher_for_a_contact(
            &"pid".to_string(),
            &GetVoucherForAContactQueryRequest {
                contact_id: 1,
                limit: None,
                offset: None,
                sort: None,
                sort_field: None,
                metadata_key_value: None,
                reward_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Page size
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Pagination offset
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetLoyaltyOfferProgramsPidVouchersRequestSort>` — Sort order
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<GetLoyaltyOfferProgramsPidVouchersRequestSortField>` — Sort field
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `i64` — Contact ID
    
</dd>
</dl>

<dl>
<dd>

**metadata_key_value:** `Option<String>` — Metadata value for a Key filter
    
</dd>
</dl>

<dl>
<dd>

**reward_id:** `Option<String>` — Reward ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tier
<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">add_subscription_to_tier</a>(pid: String, cid: String, tid: String) -> Result&lt;AddSubscriptionToTierResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Manually assigns a tier to a specific membership.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .add_subscription_to_tier(
            &"pid".to_string(),
            &"cid".to_string(),
            &"tid".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**cid:** `String` — Contact ID
    
</dd>
</dl>

<dl>
<dd>

**tid:** `String` — Tier ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">get_list_of_tier_groups</a>(pid: String, version: Option&lt;Option&lt;GetListOfTierGroupsRequestVersion&gt;&gt;) -> Result&lt;GetListOfTierGroupsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the list of tier groups defined within the loyalty program.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .get_list_of_tier_groups(
            &"pid".to_string(),
            &GetListOfTierGroupsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetListOfTierGroupsRequestVersion>` — Select 'active' to retrieve list of all tier groups which are live for clients. Select draft to retrieve list of all non deleted tier groups.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">create_tier_group</a>(pid: String, request: CreateTierGroupRequest) -> Result&lt;TierGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new tier group in a loyalty program. *(The changes will take effect with the next publication of the loyalty program)*
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .create_tier_group(
            &"pid".to_string(),
            &CreateTierGroupRequest {
                name: "name".to_string(),
                downgrade_strategy: None,
                meta: None,
                tier_order: None,
                upgrade_strategy: None,
                upgrade_schedule: None,
                downgrade_schedule: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**downgrade_strategy:** `Option<CreateTierGroupRequestDowngradeStrategy>` — Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the tier group
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<CreateTierGroupRequestMeta>` — Additional metadata for the tier group.
    
</dd>
</dl>

<dl>
<dd>

**tier_order:** `Option<Vec<String>>` — Order of the tiers in the group in ascending order
    
</dd>
</dl>

<dl>
<dd>

**upgrade_strategy:** `Option<CreateTierGroupRequestUpgradeStrategy>` — Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    
</dd>
</dl>

<dl>
<dd>

**upgrade_schedule:** `Option<CreateTierGroupRequestUpgradeSchedule>` — Schedule configuration for tier upgrades. Required when upgradeStrategy is set to a schedule-based strategy.
    
</dd>
</dl>

<dl>
<dd>

**downgrade_schedule:** `Option<CreateTierGroupRequestDowngradeSchedule>` — Schedule configuration for tier downgrades. Required when downgradeStrategy is set to a schedule-based strategy.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">get_tier_group</a>(pid: String, gid: String, version: Option&lt;Option&lt;GetTierGroupRequestVersion&gt;&gt;) -> Result&lt;TierGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns tier group information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .get_tier_group(
            &"pid".to_string(),
            &"gid".to_string(),
            &GetTierGroupQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**gid:** `String` — Tier group ID
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetTierGroupRequestVersion>` — Select active to retrieve active version of tier group. Select draft to retrieve latest changes in tier group.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">update_tier_group</a>(pid: String, gid: String, request: UpdateTierGroupRequest) -> Result&lt;TierGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a tier group from a loyalty program. *(The changes will take effect with the next publication of the loyalty program)*
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .update_tier_group(
            &"pid".to_string(),
            &"gid".to_string(),
            &UpdateTierGroupRequest {
                downgrade_strategy: UpdateTierGroupRequestDowngradeStrategy::RealTime,
                name: "name".to_string(),
                tier_order: vec!["tierOrder".to_string()],
                upgrade_strategy: UpdateTierGroupRequestUpgradeStrategy::RealTime,
                meta: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**gid:** `String` — Tier group ID
    
</dd>
</dl>

<dl>
<dd>

**downgrade_strategy:** `UpdateTierGroupRequestDowngradeStrategy` — Select real_time to downgrade tier on real time balance updates. Select membership_anniversary to downgrade tier on subscription anniversary. Select tier_anniversary to downgrade tier on tier anniversary.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the tier group
    
</dd>
</dl>

<dl>
<dd>

**meta:** `Option<UpdateTierGroupRequestMeta>` — Additional metadata for the tier group.
    
</dd>
</dl>

<dl>
<dd>

**tier_order:** `Vec<String>` — Order of the tiers in the group in ascending order
    
</dd>
</dl>

<dl>
<dd>

**upgrade_strategy:** `UpdateTierGroupRequestUpgradeStrategy` — Select real_time to upgrade tier on real time balance updates. Select membership_anniversary to upgrade tier on subscription anniversary. Select tier_anniversary to upgrade tier on tier anniversary.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">delete_tier_group</a>(pid: String, gid: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a tier group from a loyalty program. *(The changes will take effect with the next publication of the loyalty program)*
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .delete_tier_group(&"pid".to_string(), &"gid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**gid:** `String` — Tier group ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">create_tier_for_tier_group</a>(pid: String, gid: String, request: CreateTierForTierGroupRequest) -> Result&lt;Tier, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new tier in a loyalty program tier group. *(The changes will take effect with the next publication of the loyalty program)*
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .create_tier_for_tier_group(
            &"pid".to_string(),
            &"gid".to_string(),
            &CreateTierForTierGroupRequest {
                access_conditions: vec![CreateTierForTierGroupRequestAccessConditionsItem {
                    ..Default::default()
                }],
                name: "name".to_string(),
                image_ref: None,
                tier_rewards: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**gid:** `String` — Tier group ID
    
</dd>
</dl>

<dl>
<dd>

**access_conditions:** `Vec<CreateTierForTierGroupRequestAccessConditionsItem>` 
    
</dd>
</dl>

<dl>
<dd>

**image_ref:** `Option<String>` — Image of the tier
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the tier to be created
    
</dd>
</dl>

<dl>
<dd>

**tier_rewards:** `Option<Vec<CreateTierForTierGroupRequestTierRewardsItem>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">get_loyalty_program_tier</a>(pid: String, version: Option&lt;Option&lt;GetLoyaltyProgramTierRequestVersion&gt;&gt;) -> Result&lt;GetLoyaltyProgramTierResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the list of tiers defined within the loyalty program.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .get_loyalty_program_tier(
            &"pid".to_string(),
            &GetLoyaltyProgramTierQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**version:** `Option<GetLoyaltyProgramTierRequestVersion>` — Select 'active' to retrieve list of all tiers which are live for clients. Select draft to retrieve list of all non deleted tiers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">update_tier</a>(pid: String, tid: String, request: UpdateTierRequest) -> Result&lt;Tier, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Modifies an existing tier for the specified tier group *(The changes will take effect with the next publication of the loyalty program)*
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .update_tier(
            &"pid".to_string(),
            &"tid".to_string(),
            &UpdateTierRequest {
                access_conditions: vec![UpdateTierRequestAccessConditionsItem {
                    ..Default::default()
                }],
                name: "name".to_string(),
                tier_rewards: vec![UpdateTierRequestTierRewardsItem {
                    ..Default::default()
                }],
                image_ref: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**tid:** `String` — Tier ID
    
</dd>
</dl>

<dl>
<dd>

**access_conditions:** `Vec<UpdateTierRequestAccessConditionsItem>` 
    
</dd>
</dl>

<dl>
<dd>

**image_ref:** `Option<String>` — Image of the tier
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the tier to be created
    
</dd>
</dl>

<dl>
<dd>

**tier_rewards:** `Vec<UpdateTierRequestTierRewardsItem>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tier.<a href="/src/api/resources/tier/client.rs">delete_tier</a>(pid: String, tid: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a tier from a loyalty program tier group. *(The changes will take effect with the next publication of the loyalty program)*
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tier
        .delete_tier(&"pid".to_string(), &"tid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pid:** `String` — Loyalty Program ID
    
</dd>
</dl>

<dl>
<dd>

**tid:** `String` — Tier ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Wallet
<details><summary><code>client.wallet.<a href="/src/api/resources/wallet/client.rs">get_wallet_pass_install_url</a>(pass_id: String, contact_id: String) -> Result&lt;WalletPassInstallUrl, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate a wallet installation URL for a specific contact. The returned URL points to the pass installation page and encodes the pass, contact and organization identifiers as an encrypted token, so it can be shared with the contact (email, SMS, QR code, ...) to add the pass to their Apple Wallet or Google Wallet.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .wallet
        .get_wallet_pass_install_url(&"passId".to_string(), 1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pass_id:** `String` — Pass ID. The unique identifier of the wallet pass for which to generate an installation URL.
    
</dd>
</dl>

<dl>
<dd>

**contact_id:** `String` — The Brevo contact ID the installation URL is generated for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.wallet.<a href="/src/api/resources/wallet/client.rs">redeem_wallet_pass</a>(installation_id: String, request: WalletPassRedeemRequest) -> Result&lt;WalletPassRedeemResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Record that a wallet pass installation was scanned or redeemed. This issues a `wallet_pass_scanned` event that can trigger automations and update contact segments (for example to track store visits, event attendance or loyalty check-ins). The request body is optional; any details supplied (location, amount spent, event tag, custom data) are carried on the event.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .wallet
        .redeem_wallet_pass(
            &"installationId".to_string(),
            &WalletPassRedeemRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**installation_id:** `String` — Installation ID. The unique identifier of the pass installation being redeemed (as issued when the contact added the pass to their wallet).
    
</dd>
</dl>

<dl>
<dd>

**amount_spent:** `Option<String>` — Amount spent during the visit or transaction. Useful for point-of-sale integrations and purchase tracking.
    
</dd>
</dl>

<dl>
<dd>

**custom_data:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Arbitrary key/value pairs merged into the scan event's properties, for use in automations and segmentation (e.g. register id, cashier, department). Reserved keys take precedence over the same keys supplied here: `passId` is always set by the endpoint, and `location`, `amountSpent` and `eventTag` are replaced by the matching top-level fields when those are sent.
    
</dd>
</dl>

<dl>
<dd>

**event_tag:** `Option<String>` — Free-form tag identifying the event or context of the scan (e.g. an event name a contact attended). Useful to segment or route automations.
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — Location where the scan occurred. Can be a store name, address, or any identifier relevant to your use case.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## EmailCampaigns
<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">get_email_campaigns</a>(type_: Option&lt;Option&lt;GetEmailCampaignsRequestType&gt;&gt;, status: Option&lt;Option&lt;GetEmailCampaignsRequestStatus&gt;&gt;, statistics: Option&lt;Option&lt;GetEmailCampaignsRequestStatistics&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetEmailCampaignsRequestSort&gt;&gt;, exclude_html_content: Option&lt;Option&lt;bool&gt;&gt;, exclude_pdf_attachment: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;GetEmailCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>The response payload for this endpoint has changed
You now need to specify which type of statistics you would like to retrieve. For more information visit [this page](https://developers.brevo.com/changelog/2023/2/7).</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .get_email_campaigns(
            &GetEmailCampaignsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_:** `Option<GetEmailCampaignsRequestType>` — Filter on the type of the campaigns
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<GetEmailCampaignsRequestStatus>` — Filter on the status of the campaign
    
</dd>
</dl>

<dl>
<dd>

**statistics:** `Option<GetEmailCampaignsRequestStatistics>` — Filter on the type of statistics required. Example: **globalStats** value will only fetch globalStats info of the campaign in the returned response. This option only returns data for events that occurred in the last 6 months. For older campaigns, it is advisable to use the **Get Campaign Report** endpoint.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. The date range between `startDate` and `endDate` must not exceed 2 years. `startDate` must not be in the future.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent email campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. The date range between `startDate` and `endDate` must not exceed 2 years. `endDate` must not be in the future.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetEmailCampaignsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**exclude_html_content:** `Option<bool>` — Use this flag to exclude htmlContent from the response body. If set to **true**, the htmlContent field will be returned as an empty string in the response body.
    
</dd>
</dl>

<dl>
<dd>

**exclude_pdf_attachment:** `Option<bool>` — Use this flag to filter out campaigns that have a PDF attachment. If set to **true**, only campaigns without a PDF attachment (or with no attachment at all) will be returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">create_email_campaign</a>(request: CreateEmailCampaignRequest) -> Result&lt;CreateEmailCampaignResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new email campaign. The campaign requires at minimum a name and sender details, and is created in draft status by default. You must provide email content via one of three mutually exclusive options: htmlContent (inline HTML), htmlUrl (remote URL), or templateId (existing template); additionally, A/B testing can be enabled by setting abTesting to true with subjectA and subjectB, but this is incompatible with sendAtBestTime.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .create_email_campaign(
            &CreateEmailCampaignRequest {
                name: "Newsletter - May 2017".to_string(),
                sender: CreateEmailCampaignRequestSender {
                    ..Default::default()
                },
                ab_testing: None,
                attachment_url: None,
                email_expiration_date: None,
                footer: None,
                header: None,
                html_content: None,
                html_url: None,
                increase_rate: None,
                initial_quota: None,
                inline_image_activation: None,
                ip_warmup_enable: None,
                mirror_active: None,
                params: None,
                preview_text: None,
                recipients: None,
                reply_to: None,
                scheduled_at: None,
                send_at_best_time: None,
                split_rule: None,
                subject: None,
                subject_a: None,
                subject_b: None,
                tag: None,
                template_id: None,
                to_field: None,
                unsubscription_page_id: None,
                update_form_id: None,
                utm_campaign: None,
                utm_content: None,
                utm_id: None,
                utm_medium: None,
                utm_source: None,
                utm_term: None,
                winner_criteria: None,
                winner_delay: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ab_testing:** `Option<bool>` — Status of A/B Test. abTesting = false means it is disabled & abTesting = true means it is enabled. **subjectA, subjectB, splitRule, winnerCriteria & winnerDelay** will be considered when abTesting is set to true. subjectA & subjectB are mandatory together & subject if passed is ignored. **Can be set to true only if sendAtBestTime is false**. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B
    
</dd>
</dl>

<dl>
<dd>

**attachment_url:** `Option<String>` — Absolute url of the attachment (no local file). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    
</dd>
</dl>

<dl>
<dd>

**email_expiration_date:** `Option<CreateEmailCampaignRequestEmailExpirationDate>` — To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy. Learn more about setting an email expiration date. For reference , ``https://help.brevo.com/hc/en-us/articles/4413566705298-Create-an-email-campaign``
    
</dd>
</dl>

<dl>
<dd>

**footer:** `Option<String>` — Footer of the email campaign
    
</dd>
</dl>

<dl>
<dd>

**header:** `Option<String>` — Header of the email campaign
    
</dd>
</dl>

<dl>
<dd>

**html_content:** `Option<String>` — **Mandatory if htmlUrl and templateId are empty.** Body of the message (HTML). Must have more than 10 characters and be less than 1MB in size. Cannot be used together with `htmlUrl` or `templateId`.
    
</dd>
</dl>

<dl>
<dd>

**html_url:** `Option<String>` — **Mandatory if htmlContent and templateId are empty.** URL to the message (HTML). Cannot be used together with `htmlContent` or `templateId`. For example: **https://html.domain.com**
    
</dd>
</dl>

<dl>
<dd>

**increase_rate:** `Option<String>` — **Mandatory if ipWarmupEnable is set to true**. Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%.
    
</dd>
</dl>

<dl>
<dd>

**initial_quota:** `Option<String>` — **Mandatory if ipWarmupEnable is set to true**. Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000.
    
</dd>
</dl>

<dl>
<dd>

**inline_image_activation:** `Option<bool>` — Use true to embedded the images in your email. Final size of the email should be less than **4MB**. Campaigns with embedded images can _not be sent to more than 5000 contacts_
    
</dd>
</dl>

<dl>
<dd>

**ip_warmup_enable:** `Option<bool>` — **Available for dedicated ip clients**. Set this to true if you wish to warm up your ip.
    
</dd>
</dl>

<dl>
<dd>

**mirror_active:** `Option<bool>` — Use true to enable the mirror link
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the campaign
    
</dd>
</dl>

<dl>
<dd>

**params:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Pass the set of attributes to customize the type classic campaign. For example: **{"FNAME":"Joe", "LNAME":"Doe"}**. Only available if **type** is **classic**. It's considered only if campaign is in _New Template Language format_. The New Template Language is dependent on the values of **subject, htmlContent/htmlUrl, sender.name & toField**
    
</dd>
</dl>

<dl>
<dd>

**preview_text:** `Option<String>` — Preview text or preheader of the email campaign
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<CreateEmailCampaignRequestRecipients>` — Segment ids and List ids to include/exclude from campaign
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<String>` — Email on which the campaign recipients will be able to reply to
    
</dd>
</dl>

<dl>
<dd>

**scheduled_at:** `Option<String>` — Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result**. If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part). For example: **2017-06-01T12:30:00+02:00**
    
</dd>
</dl>

<dl>
<dd>

**send_at_best_time:** `Option<bool>` — Set this to true if you want to send your campaign at best time.
    
</dd>
</dl>

<dl>
<dd>

**sender:** `CreateEmailCampaignRequestSender` — Sender details including id or email and name (optional). Only one of either Sender’s email or Sender’s ID shall be passed in one request at a time. Passing both `email` and `id` will result in an error. For example: **{"name":"xyz", "email":"example@abc.com"}** or **{"name":"xyz", "id":123}**
    
</dd>
</dl>

<dl>
<dd>

**split_rule:** `Option<String>` — Add the size of your test groups. **Mandatory if abTesting = true & 'recipients' is passed**. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<String>` — Subject of the campaign. **Mandatory if abTesting is false**. Ignored if abTesting is true.
    
</dd>
</dl>

<dl>
<dd>

**subject_a:** `Option<String>` — Subject A of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    
</dd>
</dl>

<dl>
<dd>

**subject_b:** `Option<String>` — Subject B of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Tag of the campaign
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `Option<String>` — **Mandatory if htmlContent and htmlUrl are empty.** Id of the transactional email template with status _active_. Used to copy only its content fetched from htmlContent/htmlUrl to an email campaign for RSS feature. Cannot be used together with `htmlContent` or `htmlUrl`.
    
</dd>
</dl>

<dl>
<dd>

**to_field:** `Option<String>` — To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    
</dd>
</dl>

<dl>
<dd>

**unsubscription_page_id:** `Option<String>` — Enter an unsubscription page id. The page id is a 24 digit alphanumeric id that can be found in the URL when editing the page. If not entered, then the default unsubscription page will be used.
    
</dd>
</dl>

<dl>
<dd>

**update_form_id:** `Option<String>` — **Mandatory if templateId is used containing the {{ update_profile }} tag**. Enter an update profile form id. The form id is a 24 digit alphanumeric id that can be found in the URL when editing the form. If not entered, then the default update profile form will be used.
    
</dd>
</dl>

<dl>
<dd>

**utm_campaign:** `Option<String>` — Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed
    
</dd>
</dl>

<dl>
<dd>

**utm_content:** `Option<String>` — Customize the utm_content value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    
</dd>
</dl>

<dl>
<dd>

**utm_id:** `Option<String>` — Customize the utm_id value. Appears on outgoing tracking links alongside utm_campaign. When omitted or empty, the utm_id entry from the account's global utm_settings is used if enabled; otherwise no utm_id parameter is emitted.
    
</dd>
</dl>

<dl>
<dd>

**utm_medium:** `Option<String>` — Customize the utm_medium value. When omitted or empty, the utm_medium entry from the account's global utm_settings is used if set; otherwise the default `email` is used.
    
</dd>
</dl>

<dl>
<dd>

**utm_source:** `Option<String>` — Customize the utm_source value. When omitted or empty, the utm_source entry from the account's global utm_settings is used if set; otherwise the account default (`brevo` or `sendinblue`) is used.
    
</dd>
</dl>

<dl>
<dd>

**utm_term:** `Option<String>` — Customize the utm_term value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    
</dd>
</dl>

<dl>
<dd>

**winner_criteria:** `Option<CreateEmailCampaignRequestWinnerCriteria>` — Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
    
</dd>
</dl>

<dl>
<dd>

**winner_delay:** `Option<String>` — Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerDelay` is ignored if passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">upload_image_to_gallery</a>(request: UploadImageToGalleryRequest) -> Result&lt;UploadImageToGalleryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload an image to your account''s image gallery by providing an absolute URL to the image. The maximum allowed image size is 2MB and supported formats are jpeg, jpg, png, bmp, and gif; local file uploads are not supported.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .upload_image_to_gallery(
            &UploadImageToGalleryRequest {
                image_url: "https://somedomain.com/image1.jpg".to_string(),
                name: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**image_url:** `String` — The absolute url of the image (**no local file**). Maximum allowed size for image is **2MB**. Allowed extensions for images are: #### jpeg, jpg, png, bmp, gif.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name shown in your Brevo image gallery. Include the file extension, e.g. product-banner.png
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">get_email_campaign</a>(campaign_id: String, statistics: Option&lt;Option&lt;GetEmailCampaignRequestStatistics&gt;&gt;, exclude_html_content: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;GetEmailCampaignResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed information about a specific email campaign by its ID, including recipients, statistics, and HTML content. Use the statistics query parameter to select which statistics to include (globalStats, linksStats, statsByDomain, statsByDevice, or statsByBrowser); statsByDevice and statsByBrowser are only available on this single-campaign endpoint. You can exclude HTML content from the response by setting excludeHtmlContent to true.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .get_email_campaign(
            1000000,
            &GetEmailCampaignQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>

<dl>
<dd>

**statistics:** `Option<GetEmailCampaignRequestStatistics>` — Filter on the type of statistics required. Example: **globalStats** value will only fetch globalStats info of the campaign in the returned response. `statsByDevice` and `statsByBrowser` are only available when retrieving a single campaign (not in the list endpoint).
    
</dd>
</dl>

<dl>
<dd>

**exclude_html_content:** `Option<bool>` — Use this flag to exclude htmlContent from the response body. If set to **true**, htmlContent field will be returned as empty string in the response body
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">update_email_campaign</a>(campaign_id: String, request: UpdateEmailCampaignRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing email campaign''s properties such as name, subject, content, sender, recipients, schedule, and A/B testing configuration. The campaign must exist and the request body must contain at least one valid field to update. Only draft or scheduled campaigns can be modified; if sendAtBestTime is enabled, IP warmup will be automatically disabled.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .update_email_campaign(
            1000000,
            &UpdateEmailCampaignRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>

<dl>
<dd>

**ab_testing:** `Option<bool>` — Status of A/B Test. abTesting = false means it is disabled & abTesting = true means it is enabled. **subjectA, subjectB, splitRule, winnerCriteria & winnerDelay** will be considered when abTesting is set to true. subjectA & subjectB are mandatory together & subject if passed is ignored. **Can be set to true only if sendAtBestTime is false**. You will be able to set up two subject lines for your campaign and send them to a random sample of your total recipients. Half of the test group will receive version A, and the other half will receive version B
    
</dd>
</dl>

<dl>
<dd>

**attachment_url:** `Option<String>` — Absolute url of the attachment (no local file). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps'
    
</dd>
</dl>

<dl>
<dd>

**email_expiration_date:** `Option<UpdateEmailCampaignRequestEmailExpirationDate>` — To reduce your carbon footprint, set an expiration date for your email. If supported, it will be automatically deleted from the recipient’s inbox, saving storage space and energy.
    
</dd>
</dl>

<dl>
<dd>

**footer:** `Option<String>` — Footer of the email campaign
    
</dd>
</dl>

<dl>
<dd>

**header:** `Option<String>` — Header of the email campaign
    
</dd>
</dl>

<dl>
<dd>

**html_content:** `Option<String>` — Body of the message (HTML version). If the campaign is designed using Drag & Drop editor via HTML content, then the design page will not have Drag & Drop editor access for that campaign. **REQUIRED if htmlUrl is empty**
    
</dd>
</dl>

<dl>
<dd>

**html_url:** `Option<String>` — Url which contents the body of the email message. **REQUIRED if htmlContent is empty**
    
</dd>
</dl>

<dl>
<dd>

**increase_rate:** `Option<String>` — Set a percentage increase rate for warming up your ip. We recommend you set the increase rate to 30% per day. If you want to send the same number of emails every day, set the daily increase value to 0%.
    
</dd>
</dl>

<dl>
<dd>

**initial_quota:** `Option<String>` — Set an initial quota greater than 1 for warming up your ip. We recommend you set a value of 3000.
    
</dd>
</dl>

<dl>
<dd>

**inline_image_activation:** `Option<bool>` — Status of inline image. inlineImageActivation = false means image can’t be embedded, & inlineImageActivation = true means image can be embedded, in the email. You cannot send a campaign of more than **4MB** with images embedded in the email. Campaigns with the images embedded in the email _must be sent to less than 5000 contacts_.
    
</dd>
</dl>

<dl>
<dd>

**ip_warmup_enable:** `Option<bool>` — **Available for dedicated ip clients**. Set this to true if you wish to warm up your ip.
    
</dd>
</dl>

<dl>
<dd>

**mirror_active:** `Option<bool>` — Status of mirror links in campaign. mirrorActive = false means mirror links are deactivated, & mirrorActive = true means mirror links are activated, in the campaign
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of the campaign
    
</dd>
</dl>

<dl>
<dd>

**params:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Pass the set of attributes to customize the type classic campaign. For example: **{"FNAME":"Joe", "LNAME":"Doe"}**. Only available if **type** is **classic**. It's considered only if campaign is in _New Template Language format_. The New Template Language is dependent on the values of **subject, htmlContent/htmlUrl, sender.name & toField**
    
</dd>
</dl>

<dl>
<dd>

**preview_text:** `Option<String>` — Preview text or preheader of the email campaign
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<UpdateEmailCampaignRequestRecipients>` — Segment ids and List ids to include/exclude from campaign
    
</dd>
</dl>

<dl>
<dd>

**recurring:** `Option<bool>` — **FOR TRIGGER ONLY !** Type of trigger campaign.recurring = false means contact can receive the same Trigger campaign only once, & recurring = true means contact can receive the same Trigger campaign several times
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<String>` — Email on which campaign recipients will be able to reply to
    
</dd>
</dl>

<dl>
<dd>

**scheduled_at:** `Option<String>` — UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.** If sendAtBestTime is set to true, your campaign will be sent according to the date passed (ignoring the time part).
    
</dd>
</dl>

<dl>
<dd>

**send_at_best_time:** `Option<bool>` — Set this to true if you want to send your campaign at best time. Note:- **if true, warmup ip will be disabled.**
    
</dd>
</dl>

<dl>
<dd>

**sender:** `Option<UpdateEmailCampaignRequestSender>` — Sender details including id or email and name (optional). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    
</dd>
</dl>

<dl>
<dd>

**split_rule:** `Option<String>` — Add the size of your test groups. **Mandatory if abTesting = true & 'recipients' is passed**. We'll send version A and B to a random sample of recipients, and then the winning version to everyone else
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<String>` — Subject of the campaign
    
</dd>
</dl>

<dl>
<dd>

**subject_a:** `Option<String>` — Subject A of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    
</dd>
</dl>

<dl>
<dd>

**subject_b:** `Option<String>` — Subject B of the campaign. **Mandatory if abTesting = true**. subjectA & subjectB should have unique value
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Tag of the campaign
    
</dd>
</dl>

<dl>
<dd>

**to_field:** `Option<String>` — To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    
</dd>
</dl>

<dl>
<dd>

**unsubscription_page_id:** `Option<String>` — Enter an unsubscription page id. The page id is a 24 digit alphanumeric id that can be found in the URL when editing the page.
    
</dd>
</dl>

<dl>
<dd>

**update_form_id:** `Option<String>` — **Mandatory if templateId is used containing the {{ update_profile }} tag**. Enter an update profile form id. The form id is a 24 digit alphanumeric id that can be found in the URL when editing the form.
    
</dd>
</dl>

<dl>
<dd>

**utm_campaign:** `Option<String>` — Customize the utm_campaign value. If this field is empty, the campaign name will be used. Only alphanumeric characters and spaces are allowed
    
</dd>
</dl>

<dl>
<dd>

**utm_content:** `Option<String>` — Customize the utm_content value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    
</dd>
</dl>

<dl>
<dd>

**utm_id:** `Option<String>` — Customize the utm_id value. Appears on outgoing tracking links alongside utm_campaign. When omitted or empty, the utm_id entry from the account's global utm_settings is used if enabled; otherwise no utm_id parameter is emitted.
    
</dd>
</dl>

<dl>
<dd>

**utm_medium:** `Option<String>` — Customize the utm_medium value. When omitted or empty, the utm_medium entry from the account's global utm_settings is used if set; otherwise the default `email` is used.
    
</dd>
</dl>

<dl>
<dd>

**utm_source:** `Option<String>` — Customize the utm_source value. When omitted or empty, the utm_source entry from the account's global utm_settings is used if set; otherwise the account default (`brevo` or `sendinblue`) is used.
    
</dd>
</dl>

<dl>
<dd>

**utm_term:** `Option<String>` — Customize the utm_term value. Appears on outgoing tracking links alongside utm_campaign. Only alphanumeric characters and spaces are allowed
    
</dd>
</dl>

<dl>
<dd>

**winner_criteria:** `Option<UpdateEmailCampaignRequestWinnerCriteria>` — Choose the metrics that will determinate the winning version. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerCriteria` is ignored if passed
    
</dd>
</dl>

<dl>
<dd>

**winner_delay:** `Option<String>` — Choose the duration of the test in hours. Maximum is 7 days, pass 24*7 = 168 hours. The winning version will be sent at the end of the test. **Mandatory if _splitRule_ >= 1 and < 50**. If splitRule = 50, `winnerDelay` is ignored if passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">delete_email_campaign</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an email campaign by its campaign ID. Only campaigns that have not been scheduled can be deleted; attempting to delete a campaign that has already been scheduled will return a 403 permission denied error. Related data in templates, newsletter builder, and schedule collections is also cleaned up.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .delete_email_campaign(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">get_ab_test_campaign_result</a>(campaign_id: String) -> Result&lt;GetAbTestCampaignResultResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Obtain winning version of an A/B test email campaign
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .get_ab_test_campaign_result(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the A/B test campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">email_export_recipients</a>(campaign_id: String, request: EmailExportRecipientsRequest) -> Result&lt;EmailExportRecipientsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export the recipients of a sent email campaign as an asynchronous process, filtered by recipient type (e.g. openers, clickers, hardBounces). The recipientsType field is required and determines which subset of recipients to export. An optional notifyURL webhook will be called once the export is complete, and the response returns a processId to track the export status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .email_export_recipients(
            1000000,
            &EmailExportRecipientsRequest {
                recipients_type: EmailExportRecipientsRequestRecipientsType::All,
                notify_url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>

<dl>
<dd>

**notify_url:** `Option<String>` — Webhook called once the export process is finished. For reference, https://help.brevo.com/hc/en-us/articles/360007666479
    
</dd>
</dl>

<dl>
<dd>

**recipients_type:** `EmailExportRecipientsRequestRecipientsType` — Type of recipients to export for a campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">send_email_campaign_now</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send an existing email campaign immediately by scheduling it for the current time. The campaign must have valid recipients and content configured before sending. The system verifies your account''s send limit and credit balance before dispatching; if credits are insufficient, a 402 error is returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .send_email_campaign_now(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">send_report</a>(campaign_id: String, request: SendReport) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

A PDF will be sent to the specified email addresses
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .send_report(
            1000000,
            &SendReport {
                email: SendReportEmail {
                    body: "Please find attached the report of our last email campaign.".to_string(),
                    to: vec!["jim.suehan@example.com".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">send_test_email</a>(campaign_id: String, request: SendTestEmail) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a test version of an email campaign to specified email addresses or your entire test list. If the emailTo array is left empty, the test mail will be sent to all addresses in your test list. You can send a maximum of 50 test emails per day.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .send_test_email(
            1000000,
            &SendTestEmail {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">get_shared_template_url</a>(campaign_id: String) -> Result&lt;GetSharedTemplateUrlResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a unique URL to share & import an email template from one Brevo account to another.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .get_shared_template_url(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign or template
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.email_campaigns.<a href="/src/api/resources/email_campaigns/client.rs">update_campaign_status</a>(campaign_id: String, request: UpdateCampaignStatus) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the status of an email campaign, such as suspending, archiving, or replicating it. Available status values include suspended, archive, darchive, sent, queued, replicate, replicateTemplate, cancel, and draft. Note that the replicateTemplate status is only available for template type campaigns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .email_campaigns
        .update_campaign_status(
            1000000,
            &UpdateCampaignStatus {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SmsCampaigns
<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">get_sms_campaigns</a>(status: Option&lt;Option&lt;GetSmsCampaignsRequestStatus&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetSmsCampaignsRequestSort&gt;&gt;) -> Result&lt;GetSmsCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of all your SMS campaigns with their statistics and recipient information. Results can be filtered by status and date range, with a default limit of 500 and maximum of 1000 per page. The sort order defaults to descending by creation date; date filters are only available when status is not passed or is set to sent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .get_sms_campaigns(
            &GetSmsCampaignsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**status:** `Option<GetSmsCampaignsRequestStatus>` — Status of campaign.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent SMS campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. `startDate` must not be in the future.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the sent SMS campaigns. **Prefer to pass your timezone in date-time format for accurate result.** Only available if `status` is not passed or is set to `sent`. `endDate` must not be in the future.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Beginning point in the list to retrieve from.
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetSmsCampaignsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">create_sms_campaign</a>(request: CreateSmsCampaignRequest) -> Result&lt;CreateSmsCampaignResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new SMS campaign with the required name, sender, and content fields. The sender name is limited to 11 alphanumeric characters or 15 numeric characters, and the content should stay within 160 characters per SMS segment. If a scheduledAt date is provided, listIds in recipients become mandatory; accounts under validation are limited to 4 total campaigns and campaigns with more than 10 recipients will be saved as draft.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .create_sms_campaign(
            &CreateSmsCampaignRequest {
                content: "Get a discount by visiting our NY store and saying : Happy Spring!"
                    .to_string(),
                name: "Spring Promo Code".to_string(),
                sender: "MyShop".to_string(),
                organisation_prefix: None,
                recipients: None,
                scheduled_at: None,
                unicode_enabled: None,
                unsubscribe_instruction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**content:** `String` — Content of the message. The **maximum characters used per SMS is 160**, if used more than that, it will be counted as more than one SMS.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the campaign
    
</dd>
</dl>

<dl>
<dd>

**organisation_prefix:** `Option<String>` — A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<CreateSmsCampaignRequestRecipients>` 
    
</dd>
</dl>

<dl>
<dd>

**scheduled_at:** `Option<String>` — UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**sender:** `String` — Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters**
    
</dd>
</dl>

<dl>
<dd>

**unicode_enabled:** `Option<bool>` — Format of the message. It indicates whether the content should be treated as unicode or not.
    
</dd>
</dl>

<dl>
<dd>

**unsubscribe_instruction:** `Option<String>` — Instructions to unsubscribe from future communications. Recommended by U.S. carriers. Must include **STOP** keyword. This will be added as instructions after the end of message content. **Prefer verifying maximum length of 160 characters including this instructions in message content to avoid multiple sending of same sms.**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">get_sms_campaign</a>(campaign_id: String) -> Result&lt;GetSmsCampaignResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve detailed information about a specific SMS campaign by its ID, including campaign content, sender, recipients with list names, statistics (delivered, sent, bounces, unsubscriptions, answered), and tags. Unlike the list endpoint, recipients are returned as objects with id and name fields rather than plain IDs.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.sms_campaigns.get_sms_campaign(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the SMS campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">update_sms_campaign</a>(campaign_id: String, request: UpdateSmsCampaignRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing SMS campaign''s properties such as name, sender, content, recipients, scheduled date, organisation prefix, and unsubscribe instructions. The request body must contain at least one valid field to update. The campaign must exist and must be of type SMS; if a scheduledAt is provided, valid recipients must be present either in the request or already configured on the campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .update_sms_campaign(
            1000000,
            &UpdateSmsCampaignRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the SMS campaign
    
</dd>
</dl>

<dl>
<dd>

**content:** `Option<String>` — Content of the message. The **maximum characters used per SMS is 160**, if used more than that, it will be counted as more than one SMS
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of the campaign
    
</dd>
</dl>

<dl>
<dd>

**organisation_prefix:** `Option<String>` — A recognizable prefix will ensure your audience knows who you are. Recommended by U.S. carriers. This will be added as your Brand Name before the message content. **Prefer verifying maximum length of 160 characters including this prefix in message content to avoid multiple sending of same sms.**
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<UpdateSmsCampaignRequestRecipients>` 
    
</dd>
</dl>

<dl>
<dd>

**scheduled_at:** `Option<String>` — UTC date-time on which the campaign has to run (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.**
    
</dd>
</dl>

<dl>
<dd>

**sender:** `Option<String>` — Name of the sender. **The number of characters is limited to 11 for alphanumeric characters and 15 for numeric characters**
    
</dd>
</dl>

<dl>
<dd>

**unicode_enabled:** `Option<bool>` — Format of the message. It indicates whether the content should be treated as unicode or not.
    
</dd>
</dl>

<dl>
<dd>

**unsubscribe_instruction:** `Option<String>` — Instructions to unsubscribe from future communications. Recommended by U.S. carriers. Must include **STOP** keyword. This will be added as instructions after the end of message content. **Prefer verifying maximum length of 160 characters including this instructions in message content to avoid multiple sending of same sms.**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">delete_sms_campaign</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an SMS campaign by its campaign ID. Only campaigns that have not been scheduled or sent can be deleted; attempting to delete a campaign that is queued, in process, or has been sent with recipients will return a 403 permission denied error.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .delete_sms_campaign(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the SMS campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">request_sms_recipient_export</a>(campaign_id: String, request: RequestSmsRecipientExportRequest) -> Result&lt;RequestSmsRecipientExportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

It returns the background process ID which on completion calls the notify URL that you have set in the input.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .request_sms_recipient_export(
            1000000,
            &RequestSmsRecipientExportRequest {
                recipients_type: RequestSmsRecipientExportRequestRecipientsType::All,
                notify_url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the campaign
    
</dd>
</dl>

<dl>
<dd>

**notify_url:** `Option<String>` — URL that will be called once the export process is finished. For reference, https://help.brevo.com/hc/en-us/articles/360007666479
    
</dd>
</dl>

<dl>
<dd>

**recipients_type:** `RequestSmsRecipientExportRequestRecipientsType` — Filter the recipients based on how they interacted with the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">send_sms_campaign_now</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send an existing SMS campaign immediately by scheduling it for the current time. The system verifies your account''s SMS credit balance before dispatching; if credits are insufficient or the remaining credit is less than the number of recipients, a 402 error is returned. The campaign must have valid recipients and content already configured.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .send_sms_campaign_now(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">send_sms_report</a>(campaign_id: String, request: SendReport) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send report of Sent and Archived campaign, to the specified email addresses, with respective data and a pdf attachment in detail.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .send_sms_report(
            1000000,
            &SendReport {
                email: SendReportEmail {
                    body: "Please find attached the report of our last email campaign.".to_string(),
                    to: vec!["jim.suehan@example.com".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">send_test_sms</a>(campaign_id: String, request: SendTestSmsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a test SMS to a specified phone number to preview the campaign before sending it to all recipients. The phone number must belong to one of your existing contacts in your Brevo account and must not be blacklisted. The number should include the country code (e.g. 33689965433).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .send_test_sms(
            1000000,
            &SendTestSmsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the SMS campaign
    
</dd>
</dl>

<dl>
<dd>

**phone_number:** `Option<String>` — Mobile number of the recipient with the country code. This number **must belong to one of your contacts in Brevo account and must not be blacklisted**
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">update_sms_campaign_status</a>(campaign_id: String, request: UpdateCampaignStatus) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the status of an SMS campaign, such as suspending, archiving, or replicating it. Available status values include suspended, archive, darchive, sent, queued, replicate, replicateTemplate, cancel, and draft. Note that the replicateTemplate status is only available for template type campaigns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .update_sms_campaign_status(
            1000000,
            &UpdateCampaignStatus {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## WhatsAppCampaigns
<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">get_whats_app_campaigns</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetWhatsAppCampaignsRequestSort&gt;&gt;) -> Result&lt;GetWhatsAppCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of all your WhatsApp campaigns with their statistics and metadata. Results can be filtered by creation date range using startDate and endDate, with a default limit of 50 and maximum of 100 per page. The sort order defaults to descending by modification date.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .get_whats_app_campaigns(
            &GetWhatsAppCampaignsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used**. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the WhatsApp campaigns created. **Prefer to pass your timezone in date-time format for accurate result**
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used**. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the WhatsApp campaigns created. **Prefer to pass your timezone in date-time format for accurate result**
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetWhatsAppCampaignsRequestSort>` — Sort the results in the ascending/descending order of record modification. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">create_whats_app_campaign</a>(request: CreateWhatsAppCampaignRequest) -> Result&lt;CreateWhatsAppCampaignResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
[Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
<Note>This API requires the List and Segment ids as recipients in Body params.You can use the below Contact endpoints to get the required information.
[Get all the Lists](https://developers.brevo.com/reference/getlists-1)
[Get all the Segments](https://developers.brevo.com/reference/getsegments)</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .create_whats_app_campaign(
            &CreateWhatsAppCampaignRequest {
                name: "Test Campaign".to_string(),
                recipients: CreateWhatsAppCampaignRequestRecipients {
                    ..Default::default()
                },
                scheduled_at: "2017-06-01T12:30:00+02:00".to_string(),
                template_id: 19,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — Name of the WhatsApp campaign creation
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `CreateWhatsAppCampaignRequestRecipients` — Segment ids and List ids to include/exclude from campaign
    
</dd>
</dl>

<dl>
<dd>

**scheduled_at:** `String` — Sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). **Prefer to pass your timezone in date-time format for accurate result.For example: **2017-06-01T12:30:00+02:00**
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `i64` — Id of the WhatsApp template in **approved** state
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">get_whats_app_config</a>() -> Result&lt;GetWhatsAppConfigResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
[Activating WhatsApp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.whats_app_campaigns.get_whats_app_config(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">create_whats_app_template</a>(request: CreateWhatsAppTemplateRequest) -> Result&lt;CreateWhatsAppTemplateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
[Activating WhatsApp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .create_whats_app_template(
            &CreateWhatsAppTemplateRequest {
                body_text: "making it look like readable English".to_string(),
                category: CreateWhatsAppTemplateRequestCategory::Marketing,
                language: "en".to_string(),
                name: "Test template".to_string(),
                header_text: None,
                media_url: None,
                source: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**body_text:** `String` — Body of the template. **Maximum allowed characters are 1024**
    
</dd>
</dl>

<dl>
<dd>

**category:** `CreateWhatsAppTemplateRequestCategory` — Category of the template
    
</dd>
</dl>

<dl>
<dd>

**header_text:** `Option<String>` — Text content of the header in the template.  **Maximum allowed characters are 45** **Use this field to add text content in template header and if mediaUrl is empty**
    
</dd>
</dl>

<dl>
<dd>

**language:** `String` 

Language of the template. For Example :
**en** for English
    
</dd>
</dl>

<dl>
<dd>

**media_url:** `Option<String>` — Absolute url of the media file **(no local file)** for the header. **Use this field in you want to add media in Template header and headerText is empty** Allowed extensions for media files are: #### jpeg | png | mp4 | pdf
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of the template
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<CreateWhatsAppTemplateRequestSource>` — source of the template
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">get_whats_app_templates</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetWhatsAppTemplatesRequestSort&gt;&gt;, source: Option&lt;Option&lt;GetWhatsAppTemplatesRequestSource&gt;&gt;) -> Result&lt;GetWhatsAppTemplatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of all your WhatsApp templates with their status, category, language, and metadata. Results can be filtered by creation date range and optionally by source (Automation or Conversations), with a default limit of 50 and maximum of 100 per page. The sort order defaults to descending by modification date.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .get_whats_app_templates(
            &GetWhatsAppTemplatesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used**. Starting (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the templates created. **Prefer to pass your timezone in date-time format for accurate result**
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used**. Ending (urlencoded) UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) to filter the templates created. **Prefer to pass your timezone in date-time format for accurate result**
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetWhatsAppTemplatesRequestSort>` — Sort the results in the ascending/descending order of record modification. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<GetWhatsAppTemplatesRequestSource>` — source of the template
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">send_whats_app_template_approval</a>(template_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
[Activating WhatsApp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .send_whats_app_template_approval(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `String` — id of the template
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">get_whats_app_campaign</a>(campaign_id: String) -> Result&lt;GetWhatsAppCampaignResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
[Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
<Note>This API requires the List and Segment ids as recipients in Body params.You can use the below Contact endpoints to get the required information.
[Get all the Lists](https://developers.brevo.com/reference/getlists-1)
[Get all the Segments](https://developers.brevo.com/reference/getsegments)</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .get_whats_app_campaign(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — Id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">update_whats_app_campaign</a>(campaign_id: String, request: UpdateWhatsAppCampaignRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
[Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
<Note>This API requires the List and Segment ids as recipients in Body params.You can use the below Contact endpoints to get the required information.
[Get all the Lists](https://developers.brevo.com/reference/getlists-1)
[Get all the Segments](https://developers.brevo.com/reference/getsegments)</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .update_whats_app_campaign(
            1000000,
            &UpdateWhatsAppCampaignRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the campaign
    
</dd>
</dl>

<dl>
<dd>

**campaign_name:** `Option<String>` — Name of the campaign
    
</dd>
</dl>

<dl>
<dd>

**campaign_status:** `Option<UpdateWhatsAppCampaignRequestCampaignStatus>` — Status of the campaign
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<UpdateWhatsAppCampaignRequestRecipients>` — Segment ids and List ids to include/exclude from campaign
    
</dd>
</dl>

<dl>
<dd>

**reschedule_for:** `Option<String>` — Reschedule the sending UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ) of campaign. **Prefer to pass your timezone in date-time format for accurate result.For example: **2017-06-01T12:30:00+02:00** Use this field to update the scheduledAt of any existing draft or scheduled WhatsApp campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.whats_app_campaigns.<a href="/src/api/resources/whats_app_campaigns/client.rs">delete_whats_app_campaign</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a WhatsApp campaign by its campaign ID. The campaign must exist; if the campaign ID is not found, a 404 error is returned. This action is permanent and cannot be undone.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .whats_app_campaigns
        .delete_whats_app_campaign(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — id of the campaign
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Companies
<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">get_all_companies</a>(filters_attributes_name: Option&lt;Option&lt;String&gt;&gt;, linked_contacts_ids: Option&lt;Option&lt;String&gt;&gt;, linked_deals_ids: Option&lt;Option&lt;String&gt;&gt;, modified_since: Option&lt;Option&lt;String&gt;&gt;, created_since: Option&lt;Option&lt;String&gt;&gt;, page: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetCompaniesRequestSort&gt;&gt;, sort_by: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetCompaniesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of companies with optional filtering, sorting, and search capabilities. Results are sorted by creation date in descending order by default, and can be filtered by attributes, linked contacts, linked deals, or modification/creation timestamps.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .get_all_companies(
            &GetAllCompaniesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**filters_attributes_name:** `Option<String>` — Filter by attributes. If you have a filter for the owner on your side please send it as filters[attributes.owner] and utilize the account email for the filtering.
    
</dd>
</dl>

<dl>
<dd>

**linked_contacts_ids:** `Option<String>` — Filter by linked contacts ids
    
</dd>
</dl>

<dl>
<dd>

**linked_deals_ids:** `Option<String>` — Filter by linked Deals ids
    
</dd>
</dl>

<dl>
<dd>

**modified_since:** `Option<String>` — Filter (urlencoded) the companies modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    
</dd>
</dl>

<dl>
<dd>

**created_since:** `Option<String>` — Filter (urlencoded) the companies created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetCompaniesRequestSort>` — Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field used to sort field names.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">create_a_company</a>(request: PostCompaniesRequest) -> Result&lt;PostCompaniesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new CRM company with the specified name, attributes, and optional associations to contacts and deals. The company name is required, and you can optionally provide a country code when a phone number attribute is included.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .create_a_company(
            &PostCompaniesRequest {
                name: "company".to_string(),
                attributes: None,
                country_code: None,
                linked_contacts_ids: None,
                linked_deals_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Attributes for company creation
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<String>` — Country code if phone_number is passed in attributes.
    
</dd>
</dl>

<dl>
<dd>

**linked_contacts_ids:** `Option<Vec<String>>` — Contact ids to be linked with company
    
</dd>
</dl>

<dl>
<dd>

**linked_deals_ids:** `Option<Vec<String>>` — Deal ids to be linked with company
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of company
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">import_companies_creation_and_updation</a>() -> Result&lt;PostCompaniesImportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Import companies from a CSV file with mapping options.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .import_companies_creation_and_updation(
            &ImportCompaniesCreationAndUpdationRequest {
                file: b"test file content".to_vec(),
                mapping: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">link_and_unlink_company_with_contact_and_deal</a>(id: String, request: PatchCompaniesLinkUnlinkIdRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Link or unlink contacts and deals with a specific company in a single request. You can simultaneously link new contacts/deals and unlink existing ones by providing the respective ID arrays in the request body.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .link_and_unlink_company_with_contact_and_deal(
            &"id".to_string(),
            &PatchCompaniesLinkUnlinkIdRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**link_contact_ids:** `Option<Vec<String>>` — Contact ids for contacts to be linked with company
    
</dd>
</dl>

<dl>
<dd>

**link_deals_ids:** `Option<Vec<String>>` — Deal ids for deals to be linked with company
    
</dd>
</dl>

<dl>
<dd>

**unlink_contact_ids:** `Option<Vec<String>>` — Contact ids for contacts to be unlinked from company
    
</dd>
</dl>

<dl>
<dd>

**unlink_deals_ids:** `Option<Vec<String>>` — Deal ids for deals to be unlinked from company
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">get_a_company</a>(id: String) -> Result&lt;Company, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the full details of a single company by its identifier, including its attributes, linked contacts, and linked deals. Returns a 404 error if the company does not exist, or a 403 error if the user lacks permission to view the company.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .get_a_company(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Get Company Details
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">delete_a_company</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a company by its identifier. The requesting user must be the company owner or have manage permission on companies; otherwise, a 403 Forbidden error is returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .delete_a_company(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Company ID to delete
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">update_a_company</a>(id: String, request: PatchCompaniesIdRequest) -> Result&lt;Company, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing company''s attributes, name, linked contacts, or linked deals. Note that passing `linkedContactsIds` or `linkedDealsIds` replaces the entire list of associations, so omitted IDs will be removed. The company name cannot be set to an empty string.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .update_a_company(
            &"id".to_string(),
            &PatchCompaniesIdRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Attributes for company update
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<String>` — Country code if phone_number is passed in attributes.
    
</dd>
</dl>

<dl>
<dd>

**linked_contacts_ids:** `Option<Vec<String>>` — Warning - Using PATCH on linkedContactIds replaces the list of linked contacts. Omitted IDs will be removed.
    
</dd>
</dl>

<dl>
<dd>

**linked_deals_ids:** `Option<Vec<String>>` — Warning - Using PATCH on linkedDealsIds replaces the list of linked deals. Omitted IDs will be removed.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of company
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">create_a_company_deal_attribute</a>(request: PostCrmAttributesRequest) -> Result&lt;PostCrmAttributesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new custom attribute for companies or deals. The attribute label must be unique within the object type, cannot exceed 50 characters, and cannot use reserved names. For `single-select` or `multi-choice` attribute types, you must also provide the `optionsLabels` array.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .create_a_company_deal_attribute(
            &PostCrmAttributesRequest {
                attribute_type: PostCrmAttributesRequestAttributeType::Text,
                label: "Attribute Label".to_string(),
                object_type: PostCrmAttributesRequestObjectType::Companies,
                description: None,
                options_labels: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attribute_type:** `PostCrmAttributesRequestAttributeType` — The type of attribute (must be one of the defined enums)
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — A description of the attribute
    
</dd>
</dl>

<dl>
<dd>

**label:** `String` — The label for the attribute (max 50 characters, cannot be empty)
    
</dd>
</dl>

<dl>
<dd>

**object_type:** `PostCrmAttributesRequestObjectType` — The type of object the attribute belongs to. Must be either `companies` or `deals`.
    
</dd>
</dl>

<dl>
<dd>

**options_labels:** `Option<Vec<String>>` — Options for multi-choice or single-select attributes
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">delete_an_attribute</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an existing custom attribute by its identifier. This permanently removes the attribute definition and cleans up all references to it across companies or deals. System-default and non-editable attributes cannot be deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .delete_an_attribute(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Attribute ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">update_an_attribute</a>(id: String, request: PatchCrmAttributesIdRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing custom attribute''s label or options. You can rename the attribute label or modify the available options for `single-select` and `multi-choice` attribute types. System-default attributes cannot be modified except for specific editable fields.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .companies
        .update_an_attribute(
            &"id".to_string(),
            &PatchCrmAttributesIdRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Attribute ID
    
</dd>
</dl>

<dl>
<dd>

**label:** `Option<String>` — Attribute display label
    
</dd>
</dl>

<dl>
<dd>

**options_labels:** `Option<Vec<PatchCrmAttributesIdRequestOptionsLabelsItem>>` — Updated labels for selectable options
    
</dd>
</dl>

<dl>
<dd>

**object_type:** `Option<PatchCrmAttributesIdRequestObjectType>` — The type of object the attribute belongs to, it cannot be updated after creation
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">get_company_attributes</a>() -> Result&lt;Vec&lt;GetCrmAttributesCompaniesResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the list of all attributes defined for companies, including both system-default and custom attributes. Each attribute includes its label, internal name, type, required status, and available options for select-type attributes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.companies.get_company_attributes(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Deals
<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">get_deal_attributes</a>() -> Result&lt;Vec&lt;GetCrmAttributesDealsResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the list of all attributes defined for deals, including both system-default and custom attributes. Each attribute includes its label, internal name, type, required status, and available options for select-type attributes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.deals.get_deal_attributes(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">get_all_deals</a>(filters_attributes_deal_name: Option&lt;Option&lt;String&gt;&gt;, filters_attributes_deal_owner: Option&lt;Option&lt;String&gt;&gt;, filters_attributes_deal_stage: Option&lt;Option&lt;String&gt;&gt;, filters_attributes_pipeline: Option&lt;Option&lt;String&gt;&gt;, filters_linked_companies_ids: Option&lt;Option&lt;String&gt;&gt;, filters_linked_contacts_ids: Option&lt;Option&lt;String&gt;&gt;, modified_since: Option&lt;Option&lt;String&gt;&gt;, created_since: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetCrmDealsRequestSort&gt;&gt;, sort_by: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetCrmDealsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of deals with optional filtering, sorting, and search capabilities. Results can be filtered by attributes such as deal name or owner, linked companies, linked contacts, or modification/creation timestamps. Default sort order is descending by creation date.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .deals
        .get_all_deals(
            &GetAllDealsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**filters_attributes_deal_name:** `Option<String>` — Filter by attributes. If you have a filter for the owner on your end, please send it as filters[attributes.deal_owner] and utilize the account email for the filtering.
    
</dd>
</dl>

<dl>
<dd>

**filters_attributes_deal_owner:** `Option<String>` — Filter by the deal owner. Pass the account email address of the deal owner.
    
</dd>
</dl>

<dl>
<dd>

**filters_attributes_deal_stage:** `Option<String>` — Filter by the deal stage. Pass the stage id, retrievable from GET /crm/pipeline/details/{pipelineID}.
    
</dd>
</dl>

<dl>
<dd>

**filters_attributes_pipeline:** `Option<String>` — Filter by the pipeline. Pass the pipeline id, retrievable from GET /crm/pipeline/details/{pipelineID}.
    
</dd>
</dl>

<dl>
<dd>

**filters_linked_companies_ids:** `Option<String>` — Filter by linked companies ids
    
</dd>
</dl>

<dl>
<dd>

**filters_linked_contacts_ids:** `Option<String>` — Filter by linked contacts ids
    
</dd>
</dl>

<dl>
<dd>

**modified_since:** `Option<String>` — Filter (urlencoded) the deals modified after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    
</dd>
</dl>

<dl>
<dd>

**created_since:** `Option<String>` — Filter (urlencoded) the deals created after a given UTC date-time (YYYY-MM-DDTHH:mm:ss.SSSZ). Prefer to pass your timezone in date-time format for accurate result.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetCrmDealsRequestSort>` — Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field used to sort field names.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">create_a_deal</a>(request: PostCrmDealsRequest) -> Result&lt;PostCrmDealsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new deal in the CRM with the specified name, attributes, and optional associations to contacts and companies. You can assign the deal to a specific pipeline and stage by providing `pipeline` and `deal_stage` attribute IDs, which can be retrieved from the pipeline details endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .deals
        .create_a_deal(
            &PostCrmDealsRequest {
                name: "Deal: Connect with company".to_string(),
                attributes: None,
                linked_companies_ids: None,
                linked_contacts_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Attributes for deal creation To assign owner of a Deal you can send attributes.deal_owner and utilize the account email or ID. If you want to create a deal on a specific pipeline and stage you can use the following attributes `pipeline` and `deal_stage`. Pipeline and deal_stage are ids you can fetch using this endpoint `/crm/pipeline/details/{pipelineID}`
    
</dd>
</dl>

<dl>
<dd>

**linked_companies_ids:** `Option<Vec<String>>` — Company ids to be linked with deal
    
</dd>
</dl>

<dl>
<dd>

**linked_contacts_ids:** `Option<Vec<String>>` — Contact ids to be linked with deal
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of deal
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">import_deals_creation_and_updation</a>() -> Result&lt;PostCrmDealsImportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Import deals from a CSV file with mapping options.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .deals
        .import_deals_creation_and_updation(
            &ImportDealsCreationAndUpdationRequest {
                file: b"test file content".to_vec(),
                mapping: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">link_and_unlink_a_deal_with_contacts_and_companies</a>(id: String, request: PatchCrmDealsLinkUnlinkIdRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Link or unlink contacts and companies with a specific deal in a single request. You can simultaneously link new contacts/companies and unlink existing ones by providing the respective ID arrays in the request body.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .deals
        .link_and_unlink_a_deal_with_contacts_and_companies(
            &"id".to_string(),
            &PatchCrmDealsLinkUnlinkIdRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**link_company_ids:** `Option<Vec<String>>` — Company ids to be linked with deal
    
</dd>
</dl>

<dl>
<dd>

**link_contact_ids:** `Option<Vec<String>>` — Contact ids for contacts to be linked with deal
    
</dd>
</dl>

<dl>
<dd>

**unlink_company_ids:** `Option<Vec<String>>` — Company ids to be unlinked from deal
    
</dd>
</dl>

<dl>
<dd>

**unlink_contact_ids:** `Option<Vec<String>>` — Contact ids for contacts to be unlinked from deal
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">get_a_deal</a>(id: String) -> Result&lt;Deal, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the full details of a single deal by its identifier, including its attributes, pipeline stage, linked contacts, and linked companies. Returns a 404 error if the deal does not exist.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.deals.get_a_deal(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">delete_a_deal</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a deal by its identifier. The requesting user must be the deal owner or have manage permission on deals; otherwise, a 403 Forbidden error is returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.deals.delete_a_deal(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">update_a_deal</a>(id: String, request: PatchCrmDealsIdRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing deal''s attributes, name, linked contacts, or linked companies. Note that passing `linkedContactsIds` or `linkedCompaniesIds` replaces the entire list of associations, so omitted IDs will be removed. To move a deal to a different pipeline or stage, provide both the `pipeline` and `deal_stage` attribute IDs.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .deals
        .update_a_deal(
            &"id".to_string(),
            &PatchCrmDealsIdRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**attributes:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Attributes for deal update To assign owner of a Deal you can send attributes.deal_owner and utilize the account email or ID. If you wish to update the pipeline of a deal you need to provide the `pipeline` and the `deal_stage` Pipeline and deal_stage are ids you can fetch using this endpoint `/crm/pipeline/details/{pipelineID}`
    
</dd>
</dl>

<dl>
<dd>

**linked_companies_ids:** `Option<Vec<String>>` — Warning - Using PATCH on linkedCompaniesIds replaces the list of linked companies. Omitted IDs will be removed.
    
</dd>
</dl>

<dl>
<dd>

**linked_contacts_ids:** `Option<Vec<String>>` — Warning - Using PATCH on linkedContactIds replaces the list of linked contacts. Omitted IDs will be removed.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of deal
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">get_pipeline_stages</a>() -> Result&lt;Pipeline, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint is deprecated. Prefer /crm/pipeline/details/{pipelineID} instead.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.deals.get_pipeline_stages(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">get_all_pipelines</a>() -> Result&lt;Pipelines, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the list of all deal pipelines configured for your account, including each pipeline''s stages and settings. If no pipelines have been configured yet, a default pipeline is automatically created and returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.deals.get_all_pipelines(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deals.<a href="/src/api/resources/deals/client.rs">get_a_pipeline</a>(pipeline_id: String) -> Result&lt;Pipelines, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the details of a specific deal pipeline by its identifier, including its stages, stage ordering, and configuration. Use this endpoint to obtain the pipeline and stage IDs needed when creating or updating deals.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .deals
        .get_a_pipeline(&"pipelineID".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pipeline_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Files
<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">get_all_files</a>(entity: Option&lt;Option&lt;GetCrmFilesRequestEntity&gt;&gt;, entity_ids: Option&lt;Option&lt;String&gt;&gt;, date_from: Option&lt;Option&lt;i64&gt;&gt;, date_to: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetCrmFilesRequestSort&gt;&gt;) -> Result&lt;Vec&lt;FileData&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of CRM files with optional filtering by entity type, entity IDs, and date range. Results are sorted by creation date in descending order by default, with a default limit of 50 files per page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .files
        .get_all_files(
            &GetAllFilesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity:** `Option<GetCrmFilesRequestEntity>` — Filter by file entity type
    
</dd>
</dl>

<dl>
<dd>

**entity_ids:** `Option<String>` — Filter by file entity IDs
    
</dd>
</dl>

<dl>
<dd>

**date_from:** `Option<i64>` — dateFrom to date range filter type (timestamp in milliseconds)
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `Option<i64>` — dateTo to date range filter type (timestamp in milliseconds)
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetCrmFilesRequestSort>` — Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">upload_a_file</a>() -> Result&lt;FileData, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload a file and associate it with a contact, company, or deal. The file must be sent as multipart form data with a maximum size of 10 MB. You can optionally link the file to a specific entity by providing the corresponding entity ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .files
        .upload_a_file(
            &UploadAFileRequest {
                file: b"test file content".to_vec(),
                company_id: None,
                contact_id: None,
                deal_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">download_a_file</a>(id: String) -> Result&lt;GetCrmFilesIdResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a temporary download URL for a CRM file by its identifier. The returned URL is valid for 5 minutes only and provides direct access to the file content.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.files.download_a_file(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — File id to download.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">delete_a_file</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a CRM file by its identifier. This removes the file from storage and unlinks it from any associated contacts, companies, or deals.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.files.delete_a_file(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — File id to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">get_file_details</a>(id: String) -> Result&lt;FileData, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the metadata and details of a specific CRM file by its identifier. This returns information such as the file name, size, type, creation date, and associated entities, but does not include the file content itself.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.files.get_file_details(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — File id to get file data.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Notes
<details><summary><code>client.notes.<a href="/src/api/resources/notes/client.rs">get_all_notes</a>(entity: Option&lt;Option&lt;GetCrmNotesRequestEntity&gt;&gt;, entity_ids: Option&lt;Option&lt;String&gt;&gt;, date_from: Option&lt;Option&lt;i64&gt;&gt;, date_to: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetCrmNotesRequestSort&gt;&gt;) -> Result&lt;Vec&lt;Note&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of CRM notes with optional filtering by entity type, entity IDs, and date range. Results are sorted by creation date in descending order by default, with a default limit of 50 notes per page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .notes
        .get_all_notes(
            &GetAllNotesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity:** `Option<GetCrmNotesRequestEntity>` — Filter by note entity type
    
</dd>
</dl>

<dl>
<dd>

**entity_ids:** `Option<String>` — Filter by note entity IDs
    
</dd>
</dl>

<dl>
<dd>

**date_from:** `Option<i64>` — dateFrom to date range filter type (timestamp in milliseconds)
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `Option<i64>` — dateTo to date range filter type (timestamp in milliseconds)
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetCrmNotesRequestSort>` — Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notes.<a href="/src/api/resources/notes/client.rs">create_a_note</a>(request: NoteData) -> Result&lt;PostCrmNotesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new CRM note and optionally associate it with contacts, companies, or deals. The note text content is required, and you can link the note to multiple entities simultaneously during creation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.notes.create_a_note(&NoteData {
        text: "<p>Meeting notes: <b>Action item</b> - visit <a href=\"https://www.brevo.com/\">Brevo</a> for details.</p>".to_string(),
        ..Default::default()
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notes.<a href="/src/api/resources/notes/client.rs">get_a_note</a>(id: String) -> Result&lt;Note, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the full details of a single CRM note by its identifier. The response includes the note''s text content, creation date, author, and any associated contacts, companies, or deals.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.notes.get_a_note(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Note ID to get
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notes.<a href="/src/api/resources/notes/client.rs">delete_a_note</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a CRM note by its identifier. This removes the note and unlinks it from any associated contacts, companies, or deals.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.notes.delete_a_note(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Note ID to delete
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notes.<a href="/src/api/resources/notes/client.rs">update_a_note</a>(id: String, request: NoteData) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing CRM note''s text content and its associations with contacts, companies, or deals. You can modify the note text, change the pinned status, or update the linked entities.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.notes.update_a_note(&"id".to_string(), &NoteData {
        text: "<p>Meeting notes: <b>Action item</b> - visit <a href=\"https://www.brevo.com/\">Brevo</a> for details.</p>".to_string(),
        ..Default::default()
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Note ID to update
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tasks
<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">get_all_tasks</a>(filter_type: Option&lt;Option&lt;String&gt;&gt;, filter_status: Option&lt;Option&lt;GetCrmTasksRequestFilterStatus&gt;&gt;, filter_date: Option&lt;Option&lt;GetCrmTasksRequestFilterDate&gt;&gt;, filter_assign_to: Option&lt;Option&lt;String&gt;&gt;, filter_contacts: Option&lt;Option&lt;String&gt;&gt;, filter_deals: Option&lt;Option&lt;String&gt;&gt;, filter_companies: Option&lt;Option&lt;String&gt;&gt;, date_from: Option&lt;Option&lt;i64&gt;&gt;, date_to: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetCrmTasksRequestSort&gt;&gt;, sort_by: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetCrmTasksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of CRM tasks with optional filtering by task type, status, date range, assignee, and linked entities (contacts, deals, companies). Results are sorted by creation date in descending order by default, with a default limit of 50 tasks per page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tasks
        .get_all_tasks(
            &GetAllTasksQueryRequest {
                sort_by: Some("name".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**filter_type:** `Option<String>` — Filter by task type (ID)
    
</dd>
</dl>

<dl>
<dd>

**filter_status:** `Option<GetCrmTasksRequestFilterStatus>` — Filter by task status
    
</dd>
</dl>

<dl>
<dd>

**filter_date:** `Option<GetCrmTasksRequestFilterDate>` — Filter by date
    
</dd>
</dl>

<dl>
<dd>

**filter_assign_to:** `Option<String>` — Filter by the "assignTo" ID. You can utilize account emails for the "assignTo" attribute.
    
</dd>
</dl>

<dl>
<dd>

**filter_contacts:** `Option<String>` — Filter by contact ids
    
</dd>
</dl>

<dl>
<dd>

**filter_deals:** `Option<String>` — Filter by deals ids
    
</dd>
</dl>

<dl>
<dd>

**filter_companies:** `Option<String>` — Filter by companies ids
    
</dd>
</dl>

<dl>
<dd>

**date_from:** `Option<i64>` — dateFrom to date range filter type (timestamp in milliseconds)
    
</dd>
</dl>

<dl>
<dd>

**date_to:** `Option<i64>` — dateTo to date range filter type (timestamp in milliseconds)
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetCrmTasksRequestSort>` — Sort the results in the ascending/descending order. Default order is **descending** by creation if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field used to sort field names.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">create_a_task</a>(request: PostCrmTasksRequest) -> Result&lt;PostCrmTasksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new CRM task with the specified name, type, due date, and optional associations to contacts, companies, or deals. A task requires a name, task type ID, and due date at minimum. You can also set a duration, notes, a reminder, and assign the task to a specific user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tasks
        .create_a_task(
            &PostCrmTasksRequest {
                date: DateTime::parse_from_rfc3339("2021-11-01T17:44:54Z").unwrap(),
                name: "Task: Connect with client".to_string(),
                task_type_id: "61a5cd07ca1347c82306ad09".to_string(),
                assign_to_id: None,
                companies_ids: None,
                contacts_ids: None,
                deals_ids: None,
                done: None,
                duration: None,
                notes: None,
                reminder: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**assign_to_id:** `Option<String>` — To assign a task to a user you can use either the account email or ID.
    
</dd>
</dl>

<dl>
<dd>

**companies_ids:** `Option<Vec<String>>` — Companies ids for companies a task is linked to
    
</dd>
</dl>

<dl>
<dd>

**contacts_ids:** `Option<Vec<i64>>` — Contact ids for contacts linked to this task
    
</dd>
</dl>

<dl>
<dd>

**date:** `String` — Task due date and time
    
</dd>
</dl>

<dl>
<dd>

**deals_ids:** `Option<Vec<String>>` — Deal ids for deals a task is linked to
    
</dd>
</dl>

<dl>
<dd>

**done:** `Option<bool>` — Task marked as done
    
</dd>
</dl>

<dl>
<dd>

**duration:** `Option<String>` — Duration of task in milliseconds [1 minute = 60000 ms]
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name of task
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` — Notes added to a task
    
</dd>
</dl>

<dl>
<dd>

**reminder:** `Option<TaskReminder>` 
    
</dd>
</dl>

<dl>
<dd>

**task_type_id:** `String` — Id for type of task e.g Call / Email / Meeting etc.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">get_a_task</a>(id: String) -> Result&lt;Task, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the full details of a single CRM task by its identifier. The response includes the task''s name, type, status, due date, duration, notes, assignee, reminder settings, and linked contacts, companies, or deals.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.tasks.get_a_task(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">delete_a_task</a>(id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a CRM task by its identifier. This removes the task and cancels any associated reminders. The requesting user must be the task assignee or have manage permission on tasks.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.tasks.delete_a_task(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">update_a_task</a>(id: String, request: PatchCrmTasksIdRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing CRM task''s properties such as name, type, due date, status, duration, notes, assignee, reminder, or linked entities. Only the fields provided in the request body will be updated; omitted fields remain unchanged.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .tasks
        .update_a_task(
            &"id".to_string(),
            &PatchCrmTasksIdRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**assign_to_id:** `Option<String>` — To assign a task to a user you can use either the account email or ID.
    
</dd>
</dl>

<dl>
<dd>

**companies_ids:** `Option<Vec<String>>` — Companies ids for companies a task is linked to
    
</dd>
</dl>

<dl>
<dd>

**contacts_ids:** `Option<Vec<i64>>` — Contact ids for contacts linked to this task
    
</dd>
</dl>

<dl>
<dd>

**date:** `Option<String>` — Task date/time
    
</dd>
</dl>

<dl>
<dd>

**deals_ids:** `Option<Vec<String>>` — Deal ids for deals a task is linked to
    
</dd>
</dl>

<dl>
<dd>

**done:** `Option<bool>` — Task marked as done
    
</dd>
</dl>

<dl>
<dd>

**duration:** `Option<String>` — Duration of task in milliseconds [1 minute = 60000 ms]
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Name of task
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` — Notes added to a task
    
</dd>
</dl>

<dl>
<dd>

**reminder:** `Option<TaskReminder>` 
    
</dd>
</dl>

<dl>
<dd>

**task_type_id:** `Option<String>` — Id for type of task e.g Call / Email / Meeting etc.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">get_all_task_types</a>() -> Result&lt;Vec&lt;GetCrmTasktypesResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the list of all available task types, such as Email, Call, Meeting, Todo, Lunch, Deadline, and LinkedIn. If no task types exist yet, the default set is automatically created and returned. Use the task type ID when creating or updating tasks.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.tasks.get_all_task_types(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TransactionalWhatsApp
<details><summary><code>client.transactional_whats_app.<a href="/src/api/resources/transactional_whats_app/client.rs">send_whatsapp_message</a>(request: SendWhatsappMessageRequest) -> Result&lt;SendWhatsappMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>You can use this API for WhatsApp only if you have setup your WhatsApp account on Brevo platform. To setup your WhatsApp account, follow the steps in the guide below.
[Activating Whatsapp](https://developers.brevo.com/docs/whatsapp-campaigns-1) in your account</Note>
This endpoint is used to send a WhatsApp message. <br/>(**The first message you send using the API must contain a Template ID. You must create a template on WhatsApp on the Brevo platform to fetch the Template ID.**)
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_whats_app
        .send_whatsapp_message(
            &SendWhatsappMessageRequest::SendWhatsappMessageRequestParams(
                SendWhatsappMessageRequestParams {
                    contact_numbers: vec!["contactNumbers".to_string()],
                    sender_number: "senderNumber".to_string(),
                    template_id: 123,
                    ..Default::default()
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_whats_app.<a href="/src/api/resources/transactional_whats_app/client.rs">get_whatsapp_event_report</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, days: Option&lt;Option&lt;String&gt;&gt;, contact_number: Option&lt;Option&lt;String&gt;&gt;, event: Option&lt;Option&lt;GetWhatsappEventReportRequestEvent&gt;&gt;, sort: Option&lt;Option&lt;GetWhatsappEventReportRequestSort&gt;&gt;) -> Result&lt;GetWhatsappEventReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will show the unaggregated statistics for WhatsApp activity (30 days by default if `startDate` and `endDate` or `days` is not passed. The date range can not exceed 90 days)
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_whats_app
        .get_whatsapp_event_report(
            &GetWhatsappEventReportQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number limitation for the result returned
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Beginning point in the list to retrieve from
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    
</dd>
</dl>

<dl>
<dd>

**days:** `Option<String>` — Number of days in the past including today (positive integer). _Not compatible with 'startDate' and 'endDate'_
    
</dd>
</dl>

<dl>
<dd>

**contact_number:** `Option<String>` — Filter results for specific contact (WhatsApp Number with country code. Example, 85264318721)
    
</dd>
</dl>

<dl>
<dd>

**event:** `Option<GetWhatsappEventReportRequestEvent>` — Filter the report for a specific event type
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetWhatsappEventReportRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TransactionalEmails
<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_transac_blocked_contacts</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetTransacBlockedContactsRequestSort&gt;&gt;) -> Result&lt;GetTransacBlockedContactsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of transactional contacts that have been blocked or unsubscribed, along with the reason for blocking (e.g. hard bounce, admin blocked, spam complaint, or unsubscription via email/API/Marketing Automation). Both `startDate` and `endDate` must be provided together when filtering by date range, and neither date can be in the future. Results default to 50 per page (max 100) and are sorted in descending order of record creation unless overridden with the `sort` parameter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_transac_blocked_contacts(
            &GetTransacBlockedContactsQueryRequest {
                start_date: None,
                end_date: None,
                limit: None,
                offset: None,
                senders: vec![],
                sort: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) from which you want to fetch the blocked or unsubscribed contacts
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) till which you want to fetch the blocked or unsubscribed contacts
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document on the page
    
</dd>
</dl>

<dl>
<dd>

**senders:** `Option<String>` — Comma separated list of emails of the senders from which contacts are blocked or unsubscribed
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetTransacBlockedContactsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">unblock_or_resubscribe_a_transactional_contact</a>(email: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unblock or resubscribe a transactional contact by removing their email address from the blacklist. The email address must be URL-encoded in the path parameter and must be a valid email format. If the contact is not found in the blocklist, a 404 error is returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .unblock_or_resubscribe_a_transactional_contact(&"email".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `String` — contact email (urlencoded) to unblock.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_blocked_domains</a>() -> Result&lt;GetBlockedDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the list of blocked domains
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.transactional_emails.get_blocked_domains(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">block_new_domain</a>(request: BlockNewDomainRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Blocks a new domain in order to avoid messages being sent to the same
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .block_new_domain(
            &BlockNewDomainRequest {
                domain: "example.com".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain:** `String` — name of the domain to be blocked
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">delete_blocked_domain</a>(domain: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unblocks an existing domain from the list of blocked domains
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .delete_blocked_domain(&"domain".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain:** `String` — The name of the domain to be deleted
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">delete_hardbounces</a>(request: DeleteHardbouncesRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete hardbounces. To use carefully (e.g. in case of temporary ISP failures)
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .delete_hardbounces(
            &DeleteHardbouncesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**contact_email:** `Option<String>` — Target a specific email address
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — Ending date (YYYY-MM-DD) of the time period for deletion. The hardbounces until this date will be deleted. Must be greater than or equal to the startDate
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Starting date (YYYY-MM-DD) of the time period for deletion. The hardbounces occurred after this date will be deleted. Must be less than or equal to the endDate
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">send_transac_email</a>(request: SendTransacEmailRequest) -> Result&lt;SendTransacEmailResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a transactional email to one or more recipients, either using inline HTML content or a pre-built template via `templateId`. You can schedule emails for future delivery using `scheduledAt` (UTC, up to 5-minute delay), send multiple personalized versions with `messageVersions` (max 2000 total recipients, 99 per version), and attach files via URL or base64-encoded content. A `sender` and `subject` are required when no `templateId` is provided; when a `templateId` is used, the template''s sender and subject are applied unless overridden.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client.transactional_emails.send_transac_email(&SendTransacEmailRequest {
        html_content: Some("<html><head></head><body><p>Hello,</p>This is my first transactional email sent from Brevo.</p></body></html>".to_string()),
        sender: Some(SendTransacEmailRequestSender {
            email: Some("hello@brevo.com".to_string()),
            name: Some("Alex from Brevo".to_string()),
            ..Default::default()
        }),
        subject: Some("Hello from Brevo!".to_string()),
        to: Some(vec![SendTransacEmailRequestToItem {
            email: "johndoe@example.com".to_string(),
            name: Some("John Doe".to_string()),
            ..Default::default()
        }]),
        ..Default::default()
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attachment:** `Option<Vec<SendTransacEmailRequestAttachmentItem>>` — Array of attachment objects. Each attachment must include either an absolute URL (no local file paths) or base64-encoded content, along with the attachment filename. The `name` field is required when `content` is provided. Supported file extensions: xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub, eps, odt, mp3, m4a, m4v, wma, ogg, flac, wav, aif, aifc, aiff, mp4, mov, avi, mkv, mpeg, mpg, wmv, pkpass, xlsm. When `templateId` is specified: if the template uses the New Template Language format, both `url` and `content` attachment types are supported; if the template uses the Old Template Language format, the `attachment` parameter is ignored.
    
</dd>
</dl>

<dl>
<dd>

**batch_id:** `Option<String>` — UUIDv4 identifier for the scheduled batch of transactional emails. If omitted, a valid UUIDv4 batch identifier is automatically generated.
    
</dd>
</dl>

<dl>
<dd>

**bcc:** `Option<Vec<SendTransacEmailRequestBccItem>>` — Array of BCC recipient objects. Each object contains an email address and an optional name.
    
</dd>
</dl>

<dl>
<dd>

**cc:** `Option<Vec<SendTransacEmailRequestCcItem>>` — Array of CC recipient objects. Each object contains an email address and an optional name.
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Custom email headers (non-standard headers) to include in the email. The `sender.ip` header can be set to specify the IP address used for sending transactional emails (dedicated IP users only). Header names must use Title-Case-Format (words separated by hyphens with the first letter of each word capitalized). Headers not in this format are automatically converted. Standard email headers are not supported. Example: `{"sender.ip":"1.2.3.4", "X-Mailin-custom":"some_custom_value", "IdempotencyKey":"abc-123"}`
    
</dd>
</dl>

<dl>
<dd>

**html_content:** `Option<String>` — HTML body content of the email. Required when `templateId` is not provided. Ignored when `templateId` is provided.
    
</dd>
</dl>

<dl>
<dd>

**message_versions:** `Option<Vec<SendTransacEmailRequestMessageVersionsItem>>` — Array of message version objects for sending customized email variants. The `templateId` can be customized per version only if a global `templateId` is provided. The `htmlContent` and `textContent` can be customized per version only if at least one of these is present in the global parameters. Global parameters such as `to` (required), `bcc`, `cc`, `replyTo`, and `subject` can be customized per version. Maximum total recipients per API request is 2000. Maximum recipients per message version is 99. Individual `params` objects must not exceed 100 KB. Cumulative `params` across all versions must not exceed 1000 KB. See https://developers.brevo.com/docs/batch-send-transactional-emails for detailed usage instructions.
    
</dd>
</dl>

<dl>
<dd>

**params:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Key-value pairs for template variable substitution. Only applicable when the template uses the New Template Language format.
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<SendTransacEmailRequestReplyTo>` — Reply-to email address (required) and optional display name. Recipients will use this address when replying to the email.
    
</dd>
</dl>

<dl>
<dd>

**scheduled_at:** `Option<String>` — UTC date-time when the email should be sent (format: YYYY-MM-DDTHH:mm:ss.SSSZ). Include timezone information in the date-time value. Scheduled emails may be delayed by up to 5 minutes.
    
</dd>
</dl>

<dl>
<dd>

**sender:** `Option<SendTransacEmailRequestSender>` — Sender information. Required when `templateId` is not provided. Specify either an email address (with optional name) or a sender ID. The `name` field is ignored when `id` is provided.
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<String>` — Email subject line. Required when `templateId` is not provided.
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Vec<String>>` — Array of tags for categorizing and filtering emails
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `Option<String>` — Template identifier
    
</dd>
</dl>

<dl>
<dd>

**text_content:** `Option<String>` — Plain text body content of the email. Ignored when `templateId` is provided.
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<Vec<SendTransacEmailRequestToItem>>` — Array of recipient objects. Each object contains an email address and an optional display name. Required when `messageVersions` is not provided. Ignored when `messageVersions` is provided. Example: `[{"name":"Jimmy", "email":"jimmy@example.com"}, {"name":"Joe", "email":"joe@example.com"}]`
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">delete_scheduled_email_by_id</a>(identifier: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete scheduled batch of emails by batchId or single scheduled email by messageId
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .delete_scheduled_email_by_id(&"4320f270-a4e3-4a2e-b591-edfe30a5e627".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `String` — The `batchId` of scheduled emails batch (must be a valid UUIDv4) or the `messageId` of scheduled email (enclosed in angle brackets with @ sign, e.g. `<...@domain>`).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_scheduled_email_by_id</a>(identifier: String, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetScheduledEmailByIdRequestSort&gt;&gt;, status: Option&lt;Option&lt;GetScheduledEmailByIdRequestStatus&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetScheduledEmailByIdResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetch scheduled batch of emails by batchId or single scheduled email by messageId (Can retrieve data upto 30 days old)
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_scheduled_email_by_id(
            &"4320f270-a4e3-4a2e-b591-edfe30a5e627".to_string(),
            &GetScheduledEmailByIdQueryRequest {
                start_date: Some(NaiveDate::parse_from_str("2022-02-02", "%Y-%m-%d").unwrap()),
                end_date: Some(NaiveDate::parse_from_str("2022-03-02", "%Y-%m-%d").unwrap()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `String` — The `batchId` of scheduled emails batch (must be a valid UUIDv4) or the `messageId` of scheduled email (enclosed in angle brackets with @ sign, e.g. `<...@domain>`). When using `messageId`, the `limit`, `offset`, `sort`, and `status` query parameters are ignored.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — Mandatory if `endDate` is used. Starting date (YYYY-MM-DD) from which you want to fetch the list. Cannot be more than 30 days older than the current date.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — Mandatory if `startDate` is used. Ending date (YYYY-MM-DD) till which you want to fetch the list. Maximum time period that can be selected is one month.
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetScheduledEmailByIdRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed. Not valid when identifier is `messageId`.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<GetScheduledEmailByIdRequestStatus>` — Filter the records by `status` of the scheduled email batch or message. Not valid when identifier is `messageId`.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page. Not valid when identifier is `messageId`.
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document on the page.  Not valid when identifier is `messageId`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_transac_emails_list</a>(email: Option&lt;Option&lt;String&gt;&gt;, template_id: Option&lt;Option&lt;String&gt;&gt;, message_id: Option&lt;Option&lt;String&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetTransacEmailsListRequestSort&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetTransacEmailsListResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will show the list of emails for past 30 days by default. To retrieve emails before that time, please pass startDate and endDate in query filters.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_transac_emails_list(
            &GetTransacEmailsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `Option<String>` — **Mandatory if templateId and messageId are not passed in query filters.** Email address to which transactional email has been sent.
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `Option<String>` — **Mandatory if email and messageId are not passed in query filters.** Id of the template that was used to compose transactional email.
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `Option<String>` — **Mandatory if templateId and email are not passed in query filters.** Message ID of the transactional email sent.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) from which you want to fetch the list. **Maximum time period that can be selected is one month**.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) till which you want to fetch the list. **Maximum time period that can be selected is one month.**
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetTransacEmailsListRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_transac_email_content</a>(uuid: String) -> Result&lt;GetTransacEmailContentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note title="How to get uuid">You can get the uuid using either of the following methods:
Send a GET request to https://api.brevo.com/v3/smtp/emails and pass the message_id in the url. Use your api-key to authenticate the request and you will get your uuid as a response.
The uuid can also be fetched from the transactional logs page in your Brevo account, from the address URL.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_transac_email_content(&"uuid".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**uuid:** `String` — Unique id of the transactional email that has been sent to a particular contact
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">delete_an_smtp_transactional_log</a>(identifier: String, from_date: Option&lt;Option&lt;String&gt;&gt;, to_date: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete SMTP transactional log entries identified by a message ID (enclosed in angle brackets with an @ sign) or a valid email address. Optionally narrow the deletion to a specific date range using `from_date` and `to_date` query parameters (YYYY-MM-DD format). The operation also removes any associated stored email preview content.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .delete_an_smtp_transactional_log(
            &"identifier".to_string(),
            &DeleteAnSmtpTransactionalLogQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `String` — MessageId or email address of the transactional log(s) to delete. Must be a valid message ID (enclosed in angle brackets with @ sign) or a valid email address.
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `Option<String>` — Starting date (YYYY-MM-DD) to narrow down logs for deletion
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `Option<String>` — Ending date (YYYY-MM-DD) to narrow down logs for deletion
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_aggregated_smtp_report</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, days: Option&lt;Option&lt;String&gt;&gt;, tag: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetAggregatedSmtpReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will show the aggregated stats for past 90 days by default if `startDate` and `endDate` OR `days` is not passed. The date range can not exceed 90 days
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_aggregated_smtp_report(
            &GetAggregatedSmtpReportQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    
</dd>
</dl>

<dl>
<dd>

**days:** `Option<String>` — Number of days in the past including today (positive integer, maximum 90). _Not compatible with 'startDate' and 'endDate'_. Defaults to 90 if neither dates nor days are provided.
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Tag of the emails
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_email_event_report</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, days: Option&lt;Option&lt;String&gt;&gt;, email: Option&lt;Option&lt;String&gt;&gt;, event: Option&lt;Option&lt;GetEmailEventReportRequestEvent&gt;&gt;, tags: Option&lt;Option&lt;String&gt;&gt;, message_id: Option&lt;Option&lt;String&gt;&gt;, template_id: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetEmailEventReportRequestSort&gt;&gt;) -> Result&lt;GetEmailEventReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will show the aggregated stats for past 30 days by default if `startDate` and `endDate` OR `days` is not passed. The date range can not exceed 90 days
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_email_event_report(
            &GetEmailEventReportQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number limitation for the result returned
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Beginning point in the list to retrieve from.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD). Must be lower than equal to endDate
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD). Must be greater than equal to startDate
    
</dd>
</dl>

<dl>
<dd>

**days:** `Option<String>` — Number of days in the past including today (positive integer, maximum 90). _Not compatible with 'startDate' and 'endDate'_. Defaults to 30 if neither dates nor days are provided.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Filter the report for a specific email addresses
    
</dd>
</dl>

<dl>
<dd>

**event:** `Option<GetEmailEventReportRequestEvent>` — Filter the report for a specific event type
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<String>` — Filter the report for tags (serialized and urlencoded array)
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `Option<String>` — Filter on a specific message id
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `Option<String>` — Filter on a specific template id
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetEmailEventReportRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_smtp_report</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, days: Option&lt;Option&lt;String&gt;&gt;, tag: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetSmtpReportRequestSort&gt;&gt;) -> Result&lt;GetSmtpReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint will show the aggregated stats per day for the past 10 days by default if `startDate` and `endDate` OR `days` is not passed. The date range can not exceed 30 days.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_smtp_report(
            &GetSmtpReportQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document on the page
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date of the report (YYYY-MM-DD)
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date of the report (YYYY-MM-DD)
    
</dd>
</dl>

<dl>
<dd>

**days:** `Option<String>` — Number of days in the past including today (positive integer, maximum 30). _Not compatible with 'startDate' and 'endDate'_
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Tag of the emails
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetSmtpReportRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">post_preview_smtp_email_templates</a>(request: PostPreviewSmtpEmailTemplatesRequest) -> Result&lt;PostPreviewSmtpEmailTemplatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate a fully rendered preview of a transactional email template by resolving dynamic variables. Provide either an `email` address (to populate variables from the contact''s attributes) or a `params` object with key-value pairs for manual substitution; at least one of these is required alongside the mandatory `templateId`. The response includes the rendered HTML, subject, sender details, preview text, and any feed names used in the template.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .post_preview_smtp_email_templates(
            &PostPreviewSmtpEmailTemplatesRequest::Unknown(serde_json::json!({"key":"value"})),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_smtp_templates</a>(template_status: Option&lt;Option&lt;bool&gt;&gt;, limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetSmtpTemplatesRequestSort&gt;&gt;, editor_type: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetSmtpTemplatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of all transactional email templates (including automation templates) with their details such as name, subject, sender, status, HTML content, and timestamps. Results default to 50 per page (max 1000) and are sorted in descending creation order unless overridden. You can filter by active/inactive status using `templateStatus` and by editor type using `editorType` (currently only `richTextEditor` is supported).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_smtp_templates(
            &GetSmtpTemplatesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_status:** `Option<bool>` — Filter on the status of the template. Active = true, inactive = false
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetSmtpTemplatesRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>

<dl>
<dd>

**editor_type:** `Option<String>` — Filter on the editor type used to create the template. Currently only `richTextEditor` is supported as a filter value.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">create_smtp_template</a>(request: CreateSmtpTemplateRequest) -> Result&lt;CreateSmtpTemplateResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new transactional email template with the specified sender, subject, and content. The `sender`, `subject`, and `templateName` fields are required. Template content can be provided via `htmlContent` (minimum 10 characters) or `htmlUrl`; at least one must be supplied. Templates are created as inactive by default unless `isActive` is explicitly set to `true`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .create_smtp_template(
            &CreateSmtpTemplateRequest {
                sender: CreateSmtpTemplateRequestSender {
                    ..Default::default()
                },
                subject: "Thanks for your purchase !".to_string(),
                template_name: "Order Confirmation - EN".to_string(),
                attachment_url: None,
                html_content: None,
                html_url: None,
                is_active: None,
                reply_to: None,
                tag: None,
                to_field: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attachment_url:** `Option<String>` — Absolute url of the attachment (**no local file**). Extension allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps'
    
</dd>
</dl>

<dl>
<dd>

**html_content:** `Option<String>` — Body of the message (HTML version). The field must have more than 10 characters. **REQUIRED if htmlUrl is empty**
    
</dd>
</dl>

<dl>
<dd>

**html_url:** `Option<String>` — Url which contents the body of the email message. REQUIRED if htmlContent is empty
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` — Status of template. isActive = true means template is active and isActive = false means template is inactive
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<String>` — Email on which campaign recipients will be able to reply to
    
</dd>
</dl>

<dl>
<dd>

**sender:** `CreateSmtpTemplateRequestSender` — Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    
</dd>
</dl>

<dl>
<dd>

**subject:** `String` — Subject of the template
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Tag of the template
    
</dd>
</dl>

<dl>
<dd>

**template_name:** `String` — Name of the template
    
</dd>
</dl>

<dl>
<dd>

**to_field:** `Option<String>` — To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">get_smtp_template</a>(template_id: GetSmtpTemplateRequestTemplateId) -> Result&lt;GetSmtpTemplateOverview, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the full details of a specific transactional email template by its numeric ID or custom template identifier string. The response includes the template name, subject, sender information, HTML content, active status, creation and modification timestamps, reply-to address, tag, and a `doiTemplate` flag indicating whether the template is a double opt-in template (detected by the presence of optin-related tags or variables in the content).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .get_smtp_template(&GetSmtpTemplateRequestTemplateId::Long(1000000), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `GetSmtpTemplateRequestTemplateId` — ID of the template. Can be a numeric template ID or a custom template identifier string (alphanumeric, hyphens, and underscores only, max 64 characters, must start with a letter).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">update_smtp_template</a>(template_id: UpdateSmtpTemplateRequestTemplateId, request: UpdateSmtpTemplateRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an existing transactional email template by its numeric ID or custom template identifier string. All fields in the request body are optional; only the provided fields will be updated. You can update the template name, subject, sender, reply-to address, HTML content (via `htmlContent` or `htmlUrl`), active status, tag, attachment URL, and the personalized `toField`. Only one of sender email or sender ID should be provided per request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .update_smtp_template(
            &UpdateSmtpTemplateRequestTemplateId::Long(1000000),
            &UpdateSmtpTemplateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `UpdateSmtpTemplateRequestTemplateId` — ID of the template. Can be a numeric template ID or a custom template identifier string.
    
</dd>
</dl>

<dl>
<dd>

**attachment_url:** `Option<String>` — Absolute url of the attachment (**no local file**). Extensions allowed: #### xlsx, xls, ods, docx, docm, doc, csv, pdf, txt, gif, jpg, jpeg, png, tif, tiff, rtf, bmp, cgm, css, shtml, html, htm, zip, xml, ppt, pptx, tar, ez, ics, mobi, msg, pub and eps
    
</dd>
</dl>

<dl>
<dd>

**html_content:** `Option<String>` — **Required if htmlUrl is empty**. If the template is designed using Drag & Drop editor via HTML content, then the design page will not have Drag & Drop editor access for that template. Body of the message (HTML must have more than 10 characters)
    
</dd>
</dl>

<dl>
<dd>

**html_url:** `Option<String>` — **Required if htmlContent is empty**. URL to the body of the email (HTML)
    
</dd>
</dl>

<dl>
<dd>

**is_active:** `Option<bool>` — Status of the template. isActive = false means template is inactive, isActive = true means template is active
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<String>` — Email on which campaign recipients will be able to reply to
    
</dd>
</dl>

<dl>
<dd>

**sender:** `Option<UpdateSmtpTemplateRequestSender>` — Sender details including id or email and name (_optional_). Only one of either Sender's email or Sender's ID shall be passed in one request at a time. For example: **{"name":"xyz", "email":"example@abc.com"}** **{"name":"xyz", "id":123}**
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<String>` — Subject of the email
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Tag of the template
    
</dd>
</dl>

<dl>
<dd>

**template_name:** `Option<String>` — Name of the template
    
</dd>
</dl>

<dl>
<dd>

**to_field:** `Option<String>` — To personalize the **To** Field. If you want to include the first name and last name of your recipient, add **{FNAME} {LNAME}**. These contact attributes must already exist in your Brevo account. If input parameter **params** used please use **{{contact.FNAME}} {{contact.LNAME}}** for personalization
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">delete_smtp_template</a>(template_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a transactional email template by its numeric ID. Only inactive templates can be deleted; attempting to delete an active template returns a 405 error. To deactivate a template before deletion, use `PUT /smtp/templates/{templateId}` with `isActive` set to `false`. Deletion also removes associated newsletter template data and triggers asynchronous cleanup of shared assets.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .delete_smtp_template(1000000, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `String` — id of the template
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_emails.<a href="/src/api/resources/transactional_emails/client.rs">send_test_template</a>(template_id: String, request: SendTestEmail) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a test email of the specified transactional template to one or more recipients. Provide an array of email addresses in the `emailTo` field; if left empty, the test mail is sent to your entire test list. You can send a maximum of 50 test emails per day, and all provided email addresses must be valid.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_emails
        .send_test_template(
            1000000,
            &SendTestEmail {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `String` — Id of the template
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TransactionalSms
<details><summary><code>client.transactional_sms.<a href="/src/api/resources/transactional_sms/client.rs">send_async_transactional_sms</a>(request: SendTransacSms) -> Result&lt;SendAsyncTransactionalSmsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Note>If the user includes stop code in the Transactional SMS, then it will be switched to Marketing SMS automatically and it will be interpreted as a Marketing SMS. To send Transactional SMS as Transactional, it is important not to use stop code.
Note: For adding a stop code, client has to add reply STOP to [STOP_CODE] and the [STOP_CODE] will be replaced with the number.</Note>
<Note title="For end users in France">Transactional SMS can be sent at any time without time restrictions. However, if a message is categorized as Marketing, it must adhere to specific time restrictions. Messages sent outside of these restricted hours will experience delays and will be processed during allowable times. Specifically, Marketing SMS cannot be processed between 10pm and 8am, on Sundays, and on French public holidays.</Note>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_sms
        .send_async_transactional_sms(
            &SendTransacSms {
                recipient: "33689965433".to_string(),
                sender: "MyShop".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_sms.<a href="/src/api/resources/transactional_sms/client.rs">send_transac_sms</a>(request: SendTransacSms) -> Result&lt;SendTransacSmsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a transactional SMS message to a single mobile number. The `sender`, `recipient`, and either `content` or `templateId` fields are required. The sender name is limited to 11 alphanumeric characters or 15 numeric characters, and the recipient must be a valid international phone number (6-15 digits, optional leading +). Tags can be a string or an array of up to 10 strings. The SMS type defaults to `transactional` but can be set to `marketing`; if the content includes a stop code, it is automatically treated as marketing. Returns the message ID, SMS count, credits used, and remaining credits.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_sms
        .send_transac_sms(
            &SendTransacSms {
                recipient: "33689965433".to_string(),
                sender: "MyShop".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_sms.<a href="/src/api/resources/transactional_sms/client.rs">get_transac_aggregated_sms_report</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, days: Option&lt;Option&lt;String&gt;&gt;, tag: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetTransacAggregatedSmsReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve an aggregated report of your transactional SMS activity over a specified time period, including counts for requests, delivered, hard bounces, soft bounces, blocked, unsubscribed, replied, accepted, rejected, and skipped messages. Filter by date range using `startDate` and `endDate` (both required together, YYYY-MM-DD format) or by a number of past `days` (not compatible with date range). You can further narrow results by `tag`. If no date filter is provided, the report covers all available data and returns the auto-detected date range.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_sms
        .get_transac_aggregated_sms_report(
            &GetTransacAggregatedSmsReportQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    
</dd>
</dl>

<dl>
<dd>

**days:** `Option<String>` — Number of days in the past including today (positive integer). **Not compatible with startDate and endDate**
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Filter on a tag
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_sms.<a href="/src/api/resources/transactional_sms/client.rs">get_sms_events</a>(limit: Option&lt;Option&lt;String&gt;&gt;, start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, days: Option&lt;Option&lt;String&gt;&gt;, phone_number: Option&lt;Option&lt;String&gt;&gt;, event: Option&lt;Option&lt;GetSmsEventsRequestEvent&gt;&gt;, tags: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetSmsEventsRequestSort&gt;&gt;) -> Result&lt;GetSmsEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a paginated list of individual SMS event records (unaggregated), including event type, phone number, message ID, timestamp, tag, and reason or reply content where applicable. Results default to 50 per page (max 100) and are sorted in descending order unless overridden. Filter by date range (`startDate`/`endDate`), past `days` (not compatible with date range), specific `event` type (e.g. delivered, bounces, replies), `phoneNumber`, or `tags`. Bounce events include the failure reason, and reply events include the reply content.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_sms
        .get_sms_events(
            &GetSmsEventsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents per page
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document of the page
    
</dd>
</dl>

<dl>
<dd>

**days:** `Option<String>` — Number of days in the past including today (positive integer). **Not compatible with 'startDate' and 'endDate'**
    
</dd>
</dl>

<dl>
<dd>

**phone_number:** `Option<String>` — Filter the report for a specific phone number
    
</dd>
</dl>

<dl>
<dd>

**event:** `Option<GetSmsEventsRequestEvent>` — Filter the report for specific events
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<String>` — Filter the report for specific tags passed as a serialized urlencoded array
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetSmsEventsRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transactional_sms.<a href="/src/api/resources/transactional_sms/client.rs">get_transac_sms_report</a>(start_date: Option&lt;Option&lt;String&gt;&gt;, end_date: Option&lt;Option&lt;String&gt;&gt;, days: Option&lt;Option&lt;String&gt;&gt;, tag: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetTransacSmsReportRequestSort&gt;&gt;) -> Result&lt;GetTransacSmsReportResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a day-by-day breakdown of your transactional SMS activity, with each entry containing the date and counts for requests, delivered, hard bounces, soft bounces, blocked, unsubscribed, replied, accepted, rejected, and skipped messages. Filter by date range using `startDate` and `endDate` (both required together, YYYY-MM-DD format), by a number of past `days` (not compatible with date range), or by `tag`. Results are sorted in descending order by default unless overridden with the `sort` parameter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .transactional_sms
        .get_transac_sms_report(
            &GetTransacSmsReportQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**start_date:** `Option<String>` — **Mandatory if endDate is used.** Starting date (YYYY-MM-DD) of the report
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` — **Mandatory if startDate is used.** Ending date (YYYY-MM-DD) of the report
    
</dd>
</dl>

<dl>
<dd>

**days:** `Option<String>` — Number of days in the past including today (positive integer). **Not compatible with 'startDate' and 'endDate'**
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — Filter on a tag
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetTransacSmsReportRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SmsTemplates
<details><summary><code>client.sms_templates.<a href="/src/api/resources/sms_templates/client.rs">get_sms_templates</a>(limit: Option&lt;Option&lt;String&gt;&gt;, offset: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;GetSmsTemplatesRequestSort&gt;&gt;) -> Result&lt;GetSmsTemplatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .sms_templates
        .get_sms_templates(
            &GetSmsTemplatesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<String>` — Number of documents returned per page
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<String>` — Index of the first document in the page
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<GetSmsTemplatesRequestSort>` — Sort the results in the ascending/descending order of record creation. Default order is **descending** if `sort` is not passed
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## OAuth
<details><summary><code>client.o_auth.<a href="/src/api/resources/o_auth/client.rs">get_o_auth_m2m_token</a>(request: GetOAuthM2MTokenRequest) -> Result&lt;GetOAuthM2MTokenResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Exchanges an app's client_id/client_secret for a short-lived access token using the OAuth 2.0 client_credentials grant (RFC 6749 §4.4). Tokens are valid for about one hour; there is no refresh token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");
    client
        .o_auth
        .get_o_auth_m2m_token(
            &GetOAuthM2MTokenRequest {
                grant_type: "client_credentials".to_string(),
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
                scope: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**grant_type:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**client_secret:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**scope:** `Option<String>` — Space-separated list of requested scopes.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>


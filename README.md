# Brevo Rust SDK

![](https://raw.githubusercontent.com/getbrevo/brevo-node/v4/banner.png)

[![crates.io](https://img.shields.io/crates/v/brevo-rust)](https://crates.io/crates/brevo-rust)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Official Rust SDK for the [Brevo API](https://developers.brevo.com): send transactional email and SMS, manage contacts and campaigns, and more.

## Quick facts

For people and AI coding agents who want the essentials first.

| | |
|---|---|
| Crate name | `brevo-rust` (`cargo add brevo-rust`) |
| Import path | `use brevo_rust::prelude::*;` |
| Client | `BrevoClient`, built with `BrevoClient::new(ClientConfig { .. })` |
| Authentication | API key, sent as the `api-key` header (OAuth is not supported in this SDK yet) |
| Base URL | `https://api.brevo.com/v3` |
| Call shape | `client.<resource>.<method>(&request, None).await` returning `Result<T, ApiError>` |
| Runtime | async, uses `tokio` and `reqwest` with `rustls` (no system OpenSSL) |
| Full method list | [`reference.md`](reference.md) |
| API docs | <https://developers.brevo.com> |

## Installation

```sh
cargo add brevo-rust
```

or add it to your `Cargo.toml`:

```toml
[dependencies]
brevo-rust = "1.0.0"
```

## Quick start

Send your first transactional email:

```rust
use brevo_rust::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some(std::env::var("BREVO_API_KEY").expect("BREVO_API_KEY not set")),
        ..Default::default()
    };
    let client = BrevoClient::new(config).expect("Failed to build client");

    let result = client
        .transactional_emails
        .send_transac_email(
            &SendTransacEmailRequest {
                subject: Some("Hello from Brevo!".to_string()),
                html_content: Some(
                    "<html><body><p>Hello,</p><p>This is my first transactional email.</p></body></html>"
                        .to_string(),
                ),
                sender: Some(SendTransacEmailRequestSender {
                    name: Some("Alex from Brevo".to_string()),
                    email: Some("hello@brevo.com".to_string()),
                    ..Default::default()
                }),
                to: Some(vec![SendTransacEmailRequestToItem {
                    email: "johndoe@example.com".to_string(),
                    name: Some("John Doe".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            None,
        )
        .await;

    match result {
        Ok(response) => println!("Sent: {:?}", response),
        Err(e) => eprintln!("Request failed: {e}"),
    }
}
```

## Authentication

The SDK authenticates with your Brevo [API key](https://developers.brevo.com/docs/api-key-authentication), which it sends in the `api-key` header. Keep the key out of source control, for example in an environment variable as above.

You can override the key for a single call with `RequestOptions::new().api_key("...")`.

## Configuration

`ClientConfig` controls the client. Common fields:

| Field | Default | Purpose |
|---|---|---|
| `api_key` | `None` | Your Brevo API key |
| `base_url` | `https://api.brevo.com/v3` | API base URL |
| `timeout` | 60 seconds | Request timeout (`std::time::Duration`) |
| `max_retries` | 3 | Retries for failed requests |
| `custom_headers` | SDK identification headers | Extra headers sent on every request |
| `user_agent` | `brevo-rust/<version>` | `User-Agent` header |
| `reqwest_client` | `None` | Your own `reqwest::Client` (proxies, TLS, and so on) |

A builder is also available:

```rust
use brevo_rust::ApiClientBuilder;
use std::time::Duration;

let client = ApiClientBuilder::new("https://api.brevo.com/v3")
    .api_key("your-api-key")
    .timeout(Duration::from_secs(30))
    .max_retries(5)
    .build()
    .expect("Failed to build client");
```

## Error handling

Every method returns `Result<T, ApiError>`. `ApiError` has a variant for each error the API documents, plus transport and serialization errors, so you can match on the ones you care about:

```rust
match client.account.get_account(None).await {
    Ok(account) => println!("{:?}", account),
    Err(ApiError::UnauthorizedError { message, .. }) => eprintln!("Check your API key: {message}"),
    Err(ApiError::TooManyRequestsError { message, .. }) => eprintln!("Rate limited: {message}"),
    Err(e) => eprintln!("Request failed: {e}"),
}
```

See [`src/error.rs`](src/error.rs) for all variants.

## Retries and timeouts

Failed requests are retried automatically, up to 3 times by default, and requests time out after 60 seconds. Change either per call:

```rust
let response = client
    .transactional_emails
    .send_transac_email(
        &request,
        Some(RequestOptions::new().max_retries(5).timeout_seconds(30)),
    )
    .await;
```

## Request options

`RequestOptions` also lets you add headers, query parameters or body properties to a single call:

```rust
let options = RequestOptions::new()
    .additional_header("X-My-Header", "value")
    .additional_query_param("debug", "true");
```

## Custom HTTP client

Provide your own `reqwest::Client` to control proxies, TLS roots or connection settings. When set, it owns all transport-level configuration:

```rust
use brevo_rust::ApiClientBuilder;

let http = reqwest::Client::builder().build().expect("reqwest client");
let client = ApiClientBuilder::new("https://api.brevo.com/v3")
    .api_key("your-api-key")
    .reqwest_client(http)
    .build()
    .expect("Failed to build client");
```

## Reference

Every resource and method is listed in [`reference.md`](reference.md). Resources are fields on `BrevoClient`, such as `client.contacts`, `client.email_campaigns` and `client.transactional_emails`.

## Contributing

This library is generated programmatically. Changes made directly to the generated code would be overwritten. Please open an issue first to discuss changes.

Contributions to this README are always welcome!

## Support

- [API Reference](https://developers.brevo.com)
- [GitHub Issues](https://github.com/getbrevo/brevo-rust/issues)
- [Email Support](mailto:support@brevo.com)

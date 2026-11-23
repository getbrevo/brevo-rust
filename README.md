# Brevo API Rust Library

![](https://raw.githubusercontent.com/getbrevo/brevo-node/v4/banner.png)

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=https%3A%2F%2Fgithub.com%2Fgetbrevo%2Fbrevo-rust)
[![crates.io shield](https://img.shields.io/crates/v/brevo-rust)](https://crates.io/crates/brevo-rust)

Official SDK for the Brevo API.

## Table of Contents

- [Documentation](#documentation)
- [Installation](#installation)
- [Reference](#reference)
- [Usage](#usage)
- [Environments](#environments)
- [Errors](#errors)
- [Request Types](#request-types)
- [Advanced](#advanced)
  - [Retries](#retries)
  - [Timeouts](#timeouts)
  - [Additional Headers](#additional-headers)
  - [Additional Query String Parameters](#additional-query-string-parameters)
  - [Additional Body Properties](#additional-body-properties)
  - [Custom Client](#custom-client)

## Documentation

API reference documentation is available [here](https://developers.brevo.com).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
brevo-rust = "1.0.0"
```

Or install via cargo:

```sh
cargo add brevo-rust
```

## Reference

A full reference for this library is available [here](https://github.com/getbrevo/brevo-rust/blob/HEAD/./reference.md).

## Usage

Instantiate and use the client with the following:

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

## Environments

This SDK allows you to configure different environments for API requests.

```rust
use brevo_rust::prelude::{*};

let config = ClientConfig {
    base_url: Environment::Default.url().to_string(),
    ..Default::default()
};
let client = Client::new(config).expect("Failed to build client");
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
match client.transactional_emails.send_transac_email(None)?.await {
    Ok(response) => {
        println!("Success: {:?}", response);
    },
    Err(ApiError::HTTP { status, message }) => {
        println!("API Error {}: {:?}", status, message);
    },
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Request Types

The SDK exports all request types as Rust structs. Simply import them from the crate to access them:

```rust
use brevo_rust::prelude::{*};

let request = PostCorporateGroupRequest {
    ...
};
```

## Advanced

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 2).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) (Internal Server Error)

The `retryStatusCodes` configuration controls which [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) status codes are retried:

- `legacy` (default): Retries `408`, `429`, and all `>= 500`
- `recommended`: Retries `408`, `429`, `502`, `503`, `504` only (excludes `500 Internal Server Error` to avoid retrying non-idempotent failures)

Use the `max_retries` method to configure this behavior.

```rust
let response = client.transactional_emails.send_transac_email(
    Some(RequestOptions::new().max_retries(3))
)?.await;
```

### Timeouts

The SDK defaults to a 30 second timeout. Use the `timeout` method to configure this behavior.

```rust
let response = client.transactional_emails.send_transac_email(
    Some(RequestOptions::new().timeout_seconds(30))
)?.await;
```

### Additional Headers

You can add custom headers to requests using `RequestOptions`.

```rust
let response = client.transactional_emails.send_transac_email(
    Some(
        RequestOptions::new()
            .additional_header("X-Custom-Header", "custom-value")
            .additional_header("X-Another-Header", "another-value")
    )
)?
.await;
```

### Additional Query String Parameters

You can add custom query parameters to requests using `RequestOptions`.

```rust
let response = client.transactional_emails.send_transac_email(
    Some(
        RequestOptions::new()
            .additional_query_param("filter", "active")
            .additional_query_param("sort", "desc")
    )
)?
.await;
```

### Additional Body Properties

You can add properties to the request body using `RequestOptions`, e.g. to send fields the SDK
doesn't model yet. Keys are sent as-is, and a property set here overrides a field of the same name.
If the endpoint has no body, a JSON object containing only these properties is sent. Additional
properties apply to JSON and form-urlencoded bodies; multipart (file upload) and raw bytes bodies
are sent unchanged.

```rust
let response = client.transactional_emails.send_transac_email(
    Some(
        RequestOptions::new()
            .additional_body_param("beta_feature", true)
            .additional_body_param("metadata", serde_json::json!({ "source": "sdk" }))
    )
)?
.await;
```

### Custom Client

The SDK builds its own `reqwest` client by default, but you can supply your own through
`ClientConfig.reqwest_client` (or `ApiClientBuilder::reqwest_client`) when you need control over the
transport — custom root certificates, client certificates, proxies or connection tuning. The supplied
client is used as-is; authentication, custom headers and retries are still applied by the SDK.

```rust
use brevo_rust::prelude::*;

let certificate = reqwest::Certificate::from_pem(&std::fs::read("ca.pem")?)?;
let reqwest_client = reqwest::Client::builder()
    .add_root_certificate(certificate)
    .build()
    .expect("Failed to build reqwest client");
let config = ClientConfig {
    reqwest_client: Some(reqwest_client),
    ..Default::default()
};
let client = BrevoClient::new(config).expect("Failed to build client");
```


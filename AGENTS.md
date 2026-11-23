# AGENTS.md: brevo-rust

Instructions for AI coding agents working in, or with, this repository. For human-oriented docs see [README.md](README.md).

## What this is

`brevo-rust` is the official Rust SDK for the [Brevo API](https://developers.brevo.com). It is **generated** from Brevo's OpenAPI definition with [Fern](https://buildwithfern.com). Most of the code under `src/` is generated and is overwritten on every regeneration.

- Crate: `brevo-rust` on crates.io; import as `brevo_rust`
- Main type: `BrevoClient`, created with `BrevoClient::new(ClientConfig { .. })`
- Authentication: API key in the `api-key` header (no OAuth in this SDK yet)
- Base URL: `https://api.brevo.com/v3`

## Using the SDK (for code that depends on it)

- Import with `use brevo_rust::prelude::*;`.
- Resources are fields on the client: `client.contacts`, `client.transactional_emails`, `client.email_campaigns`, and so on. See [reference.md](reference.md) for every method and its request and response types.
- Calls are async and return `Result<T, ApiError>`: `client.<resource>.<method>(&request, None).await`. The last argument is `Option<RequestOptions>`.
- Request structs implement `Default`, so build them with `..Default::default()` and set only the fields you need.
- Match on `ApiError` variants (for example `UnauthorizedError`, `TooManyRequestsError`); see `src/error.rs`.
- Never hard-code API keys. Read them from the environment.

Machine-readable docs: <https://developers.brevo.com/llms.txt> (append `.md` to any docs page URL for clean Markdown).

## Repository layout

```
src/api/resources/   one module per API area (generated)
src/api/types/       request and response types (generated)
src/core/            HTTP client, retries, request options (generated)
src/config.rs        ClientConfig and defaults (generated)
src/error.rs         ApiError (generated)
reference.md         method reference (generated)
README.md            hand-written
AGENTS.md            hand-written
changelog.md         hand-written
LICENSE              hand-written
.fernignore          lists the hand-written files Fern must not overwrite
```

## Working in this repo

**Do not edit generated files** (everything under `src/`, `reference.md`, `Cargo.toml`, `.github/`). Changes there are overwritten by the next regeneration. If generated code is wrong, open an issue; the fix belongs in the generator configuration.

Files listed in `.fernignore` are maintained by hand and survive regeneration. If you add another hand-written file, add it to `.fernignore`.

Commands:

```sh
cargo build    # compile
cargo test     # unit tests
cargo check    # fast type check
```

## Releasing

Maintainers release by pushing a version tag such as `v1.0.0`; CI publishes the crate when a tag is pushed. Do not create tags, releases or publish the crate unless a maintainer asks you to. The first publish of a crate name is permanent.

## Boundaries

- Do not commit secrets, tokens or API keys.
- Do not push directly to `main`; open a pull request.
- Do not change `.github/workflows/` by hand; it is generated.

# reqwest-rewire

`reqwest-rewire` is a lightweight wrapper around [`reqwest`](https://docs.rs/reqwest) that transparently **rewrites outgoing request URLs** based on user-defined rules.

It is designed primarily for **testing and local development**, allowing you to redirect HTTP requests to mock servers **without changing application code**.

---

## Installation

    [dependencies]
    reqwest-rewire = "0.1"

---

## Motivation

In many applications, you want to:

- Use real APIs in production
- Redirect those same calls to mock servers in tests
- Avoid changing request code just to mock HTTP

`reqwest-rewire` solves this by acting as a **rewiring layer** in front of `reqwest`.

---

## Core Concept

You define a set of **redirect rules**:

    https://real-api.com/api/
    → http://localhost:3000/api-mock/

Any request matching the left-hand side is transparently rewritten before being sent.

The **most specific rule always wins**.

---

## Usage

### 1. Use the `TestableClient` trait

Your application code depends on a trait, not a concrete client:

    use crate::TestableClient;

    fn fetch_data(client: &dyn TestableClient) {
        client
            .get("https://real-api.com/api/users")
            .send()
            .unwrap();
    }

---

### 2. Use `reqwest::Client` in production

    let client = reqwest::Client::new();
    fetch_data(&client);

---

### 3. Use `RewireClient` in tests

    use std::collections::HashMap;
    use reqwest_rewire::RewireClient;

    let mut redirects = HashMap::new();
    redirects.insert(
        "https://real-api.com/api/".to_string(),
        "http://localhost:3000/api-mock/".to_string(),
    );

    let client = RewireClient::new(redirects);

    fetch_data(&client);

No application code changes required 🎉

---

## Rewrite Behavior

- URLs are parsed using `url::Url`
- Rewrites match on:
  - scheme
  - host
  - **path prefix**
- The rule with the **longest matching path** is selected
- Query strings and fragments are preserved
- Invalid URLs fall back to the original request

### Example

    Request:
    https://real-api.com/api/v1/users?id=42

    Rule:
    https://real-api.com/api/
    → http://localhost:3000/api-mock/

    Result:
    http://localhost:3000/api-mock/v1/users?id=42

---

## Non-Goals

- HTTP mocking or response stubbing
- Network interception or proxying
- Wildcard URLs (`*` is not valid in URLs)

For full HTTP mocking, consider tools like `wiremock`, `mockito`, or `httpmock`.

---

## Testing Philosophy

`reqwest-rewire` is intentionally minimal:
- It rewrites requests
- It does not alter responses
- It does not simulate servers

This makes it ideal as a **composable building block** in larger test setups.

---

## API Stability

- The crate follows semantic versioning
- `0.x` releases may introduce breaking changes
- `1.0` will guarantee API stability

---

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

You may choose either.

---

## Contributing

Issues, ideas, and pull requests are welcome!  
Please open an issue before making large changes.

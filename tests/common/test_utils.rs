use httpmock::Mock;
use httpmock::prelude::*;
use reqwest_rewire::testable_client::TestableClient;
use std::collections::HashMap;

fn build_query_string(query_args: &HashMap<&str, &str>) -> String {
    if query_args.is_empty() {
        return String::new();
    }

    let query = query_args
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");

    format!("?{}", query)
}
pub fn build_rewire_test_params(server: &MockServer) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert(server.url("/request"), server.url("/rewire_request"));
    params.insert(server.url("/request/*"), server.url("/rewire_request/*"));
    params
}

fn build_mock<'a>(
    server: &'a MockServer,
    path: &str,
    method: &Method,
    query_args: Option<HashMap<&str, &str>>,
    request_body: Option<&str>,
    response_body: String,
) -> Mock<'a> {
    let method = match method {
        GET => GET,
        Method::HEAD => Method::HEAD,
        POST => POST,
        PUT => PUT,
        DELETE => DELETE,
        Method::CONNECT => Method::CONNECT,
        OPTIONS => OPTIONS,
        Method::TRACE => Method::TRACE,
        PATCH => PATCH,
    };
    server.mock(|when, then| {
        let mut when_builder = when.method(method).path(path);
        if let Some(query_args) = query_args.clone() {
            for (k, v) in query_args.clone() {
                when_builder = when_builder.query_param(k, v);
            }
        }
        if let Some(body) = request_body.clone() {
            when_builder.body(body);
        }
        then.status(200)
            .header("Content-Type", "text/html")
            .body(response_body);
    })
}

fn get_mock<'a>(
    server: &'a MockServer,
    method: &Method,
    query_args: Option<HashMap<&str, &str>>,
    request_body: Option<&str>,
    rewire: bool,
) -> Mock<'a> {
    if rewire {
        build_mock(
            server,
            "/rewire_request",
            method,
            query_args,
            request_body,
            "rewire_client".to_string(),
        )
    } else {
        build_mock(
            server,
            "/request",
            method,
            query_args,
            request_body,
            "reqwest_client".to_string(),
        )
    }
}

async fn test_method(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    body: Option<&'static str>,
    rewire: bool,
    server: &MockServer,
    method: Method,
) {
    let mock = get_mock(server, &method, query_args.clone(), None, rewire);

    let query_args_string = match query_args {
        None => "".to_string(),
        Some(q) => build_query_string(&q),
    };

    let request = match method {
        GET => {
            client
                .get(&server.url(format!("/request{}", query_args_string)))
                .send()
                .await
        }
        Method::HEAD => {
            client
                .head(&server.url(format!("/request{}", query_args_string)))
                .send()
                .await
        }
        POST => {
            client
                .post(&server.url(format!("/request{}", query_args_string)))
                .body(body.unwrap_or(""))
                .send()
                .await
        }
        PUT => {
            client
                .put(&server.url(format!("/request{}", query_args_string)))
                .body(body.unwrap_or(""))
                .send()
                .await
        }
        DELETE => {
            client
                .delete(&server.url(format!("/request{}", query_args_string)))
                .body(body.unwrap_or(""))
                .send()
                .await
        }
        PATCH => {
            client
                .patch(&server.url(format!("/request{}", query_args_string)))
                .body(body.unwrap_or(""))
                .send()
                .await
        }
        _ => unreachable!(),
    };

    assert!(request.is_ok());
    let request = request.unwrap();
    assert_eq!(request.status(), 200);
    let response_body = request.text().await;
    assert!(response_body.is_ok());
    let response_body = response_body.unwrap();

    if let Method::HEAD = method {
        return assert_eq!(response_body, "");
    }

    mock.assert();
    if rewire {
        assert_eq!(response_body, "rewire_client");
    } else {
        assert_eq!(response_body, "reqwest_client");
    }
}

pub async fn test_get(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    rewire: bool,
    server: &MockServer,
) {
    test_method(client, query_args, None, rewire, server, GET).await;
}

pub async fn test_post(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    body: Option<&'static str>,
    rewire: bool,
    server: &MockServer,
) {
    test_method(client, query_args, body, rewire, server, POST).await;
}

pub async fn test_put(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    body: Option<&'static str>,
    rewire: bool,
    server: &MockServer,
) {
    test_method(client, query_args, body, rewire, server, PUT).await;
}

pub async fn test_patch(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    body: Option<&'static str>,
    rewire: bool,
    server: &MockServer,
) {
    test_method(client, query_args, body, rewire, server, PATCH).await;
}

pub async fn test_delete(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    body: Option<&'static str>,
    rewire: bool,
    server: &MockServer,
) {
    test_method(client, query_args, body, rewire, server, DELETE).await;
}

pub async fn test_head(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    rewire: bool,
    server: &MockServer,
) {
    test_method(client, query_args, None, rewire, server, Method::HEAD).await;
}

pub async fn test_request(
    client: impl TestableClient,
    query_args: Option<HashMap<&str, &str>>,
    request_body: Option<&'static str>,
    rewire: bool,
    server: &MockServer,
    method: http::Method,
) {
    let httpmock_method = match method {
        http::Method::GET => GET,
        http::Method::POST => POST,
        http::Method::PUT => PUT,
        http::Method::DELETE => DELETE,
        http::Method::HEAD => Method::HEAD,
        http::Method::PATCH => PATCH,
        _ => unreachable!(),
    };

    let mock = get_mock(server, &httpmock_method, query_args.clone(), request_body.clone(), rewire);

    let query_args_string = match query_args {
        None => "".to_string(),
        Some(q) => build_query_string(&q),
    };

    let request = client.request(
        method.clone(),
        &server.url(format!("/request{}", query_args_string)),
    ).body(request_body.unwrap_or_default()).send().await;

    assert!(request.is_ok());
    let request = request.unwrap();
    assert_eq!(request.status(), 200);
    let response_body = request.text().await;
    assert!(response_body.is_ok());
    let response_body = response_body.unwrap();

    if let http::Method::HEAD = method {
        return assert_eq!(response_body, "");
    }

    mock.assert();
    if rewire {
        assert_eq!(response_body, "rewire_client");
    } else {
        assert_eq!(response_body, "reqwest_client");
    }
}

pub async fn test_request_with_nested_path(
    client: impl TestableClient,
    rewire: bool,
    server: &MockServer,
    method: http::Method,
) {
    let httpmock_method = match method {
        http::Method::GET => GET,
        http::Method::POST => POST,
        http::Method::PUT => PUT,
        http::Method::DELETE => DELETE,
        http::Method::HEAD => Method::HEAD,
        http::Method::PATCH => PATCH,
        _ => unreachable!(),
    };

    let mock = match rewire {
        true => build_mock(server,
                           "/rewire_request/nested_path",
                           &httpmock_method,
                           None,
                           None,
                           "rewire_client".to_string(),
        ),
        false => build_mock(server,
                            "/request/nested_path",
                            &httpmock_method,
                            None,
                            None,
                            "reqwest_client".to_string(),
        ),
    };

    let request = client.request(
        method.clone(),
        &server.url("/request/nested_path"),
    ).send().await;

    assert!(request.is_ok());
    let request = request.unwrap();
    assert_eq!(request.status(), 200);
    let response_body = request.text().await;
    assert!(response_body.is_ok());
    let response_body = response_body.unwrap();

    if let http::Method::HEAD = method {
        return assert_eq!(response_body, "");
    }

    mock.assert();
    if rewire {
        assert_eq!(response_body, "rewire_client");
    } else {
        assert_eq!(response_body, "reqwest_client");
    }
}
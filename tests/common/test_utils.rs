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
    params
}

pub async fn test_get(client: impl TestableClient, query_args: Option<HashMap<&str, &str>>, rewire: bool, server: &MockServer) {
    let reqwest_mock = server.mock(|when, then| {
        let mut when_builder = when.method(GET).path("/request");
        if let Some(query_args) = query_args.clone() {
            for (k, v) in query_args.clone() {
                when_builder = when_builder.query_param(k, v);
            }
        }
        then.status(200)
            .header("Content-Type", "text/html")
            .body("reqwest_client");
    });

    let rewire_mock = server.mock(|when, then| {
        let mut when_builder = when.method(GET).path("/rewire_request");
        if let Some(query_args) = query_args.clone() {
            for (k, v) in query_args.clone() {
                when_builder = when_builder.query_param(k, v);
            }
        }
        then.status(200)
            .header("Content-Type", "text/html")
            .body("rewire_client");
    });
    
    let query_args_string = match query_args {
        None => "".to_string(),
        Some(q) => build_query_string(&q),
    };

    let request = client.get(&server.url(format!("/request{}", query_args_string)))
        .send()
        .await;

    assert!(request.is_ok());
    let request = request.unwrap();
    assert_eq!(request.status(), 200);
    let response_body = request.text().await;
    assert!(response_body.is_ok());
    let response_body = response_body.unwrap();

    if rewire {
        rewire_mock.assert();
        assert_eq!(response_body, "rewire_client");
    } else {
        reqwest_mock.assert();
        assert_eq!(response_body, "reqwest_client");
    }
}

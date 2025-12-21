mod common;

use std::collections::HashMap;
use httpmock::MockServer;
use common::test_utils::*;
use reqwest_rewire::rewire_client::RewireClient;

#[tokio::test]
async fn test_get_request_rewire() {
    let server = MockServer::start();
    let rewire_params = build_rewire_test_params(&server);
    let client = RewireClient::new(rewire_params);
    test_get(client, None, true, &server).await;
}

#[tokio::test]
async fn test_get_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_get(client, None, false, &server).await;
}

#[tokio::test]
async fn test_get_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_get(client, Some(query_args), true, &server).await;
}

#[tokio::test]
async fn test_get_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_get(client, Some(query_args), false, &server).await;
}

#[tokio::test]
async fn test_head_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_head(client, None, true, &server).await;
}

#[tokio::test]
async fn test_head_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_head(client, None, false, &server).await;
}

#[tokio::test]
async fn test_head_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_head(client, Some(query_args), true, &server).await;
}

#[tokio::test]
async fn test_head_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_head(client, Some(query_args), false, &server).await;
}

#[tokio::test]
async fn test_post_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_post(client, None, None, true, &server).await;
}

#[tokio::test]
async fn test_post_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_post(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_post_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_post(client, Some(query_args), None, true, &server).await;
}

#[tokio::test]
async fn test_post_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_post(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_post_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_post(client, None, Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_post_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_post(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_post_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_post(client, Some(query_args), Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_post_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_post(client, Some(query_args), Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_put_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_put(client, None, None, true, &server).await;
}

#[tokio::test]
async fn test_put_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_put(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_put_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_put(client, Some(query_args), None, true, &server).await;
}

#[tokio::test]
async fn test_put_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_put(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_put_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_put(client, None, Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_put_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_put(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_put_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_put(client, Some(query_args), Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_put_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_put(client, Some(query_args), Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_patch_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_patch(client, None, None, true, &server).await;
}

#[tokio::test]
async fn test_patch_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_patch(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_patch(client, Some(query_args), None, true, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_patch(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_patch(client, None, Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_patch(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_patch(client, Some(query_args), Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_patch(client, Some(query_args), Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_delete_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_delete(client, None, None, true, &server).await;
}

#[tokio::test]
async fn test_delete_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_delete(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_delete(client, Some(query_args), None, true, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_delete(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_delete(client, None, Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_delete(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_delete(client, Some(query_args), Some("This is a string body"), true, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_delete(client, Some(query_args), Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_request_get_request_rewire() {
    let server = MockServer::start();
    let rewire_params = build_rewire_test_params(&server);
    let client = RewireClient::new(rewire_params);
    test_request(client, None, None, true, &server, http::Method::GET).await;
}

#[tokio::test]
async fn test_request_get_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, None, false, &server, http::Method::GET).await;
}

#[tokio::test]
async fn test_request_get_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, true, &server, http::Method::GET).await;
}

#[tokio::test]
async fn test_request_get_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::GET).await;
}

#[tokio::test]
async fn test_request_head_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, None, true, &server, http::Method::HEAD).await;
}

#[tokio::test]
async fn test_request_head_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, None, false, &server, http::Method::HEAD).await;
}

#[tokio::test]
async fn test_request_head_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, true, &server, http::Method::HEAD).await;
}

#[tokio::test]
async fn test_request_head_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::HEAD).await;
}

#[tokio::test]
async fn test_request_post_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, None, true, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, None, false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, true, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, Some("What are you looking at"), true, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, Some("What are you looking at"), false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("What are you looking at"), true, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("What are you looking at"), false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_put_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, None, true, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, None, false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, true, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, Some("This is a String body"), true, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, Some("This is a String body"), false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("I was crazy once"), true, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("I was crazy once"), false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_patch_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, None, true, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, None, false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, true, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, Some("This is a String body"), true, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, Some("This is a String body"), false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("Is this enough testing?..."), true, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("NO! KEEP GOING!!"), false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_delete_request_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, None, true, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, None, false, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, true, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_body_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request(client, None, Some("Oh dear, that's the last method right?"), true, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_body() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request(client, None, Some("Wait, I think it is"), false, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_body_and_query_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("It's almost over!!"), true, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("YES FINALLY"), false, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_with_nested_args() {
    let server = MockServer::start();
    let client = RewireClient::new(HashMap::new());
    test_request_with_nested_path(client, false, &server, http::Method::GET).await;
}

#[tokio::test]
async fn test_with_nested_args_rewire() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    test_request_with_nested_path(client, true, &server, http::Method::GET).await;
}
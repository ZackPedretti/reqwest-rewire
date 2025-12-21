mod common;

use std::collections::HashMap;
use httpmock::MockServer;
use common::test_utils::*;

#[tokio::test]
async fn test_get_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_get(client, None, false, &server).await;
}

#[tokio::test]
async fn test_get_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_get(client, Some(query_args), false, &server).await;
}

#[tokio::test]
async fn test_head_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_head(client, None, false, &server).await;
}

#[tokio::test]
async fn test_head_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_head(client, Some(query_args), false, &server).await;
}

#[tokio::test]
async fn test_post_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_post(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_post_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_post(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_post_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_post(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_post_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_post(client, Some(query_args), Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_put_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_put(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_put_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_put(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_put_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_put(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_put_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_put(client, Some(query_args), Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_patch_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_patch(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_patch(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_patch(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_patch_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_patch(client, Some(query_args), Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_delete_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_delete(client, None, None, false, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_delete(client, Some(query_args), None, false, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_delete(client, None, Some("This is a string body"), false, &server).await;
}

#[tokio::test]
async fn test_delete_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_delete(client, Some(query_args), Some("This is a string body"), false, &server).await;
}
#[tokio::test]
async fn test_request_get_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, None, false, &server, http::Method::GET).await;
}

#[tokio::test]
async fn test_request_get_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::GET).await;
}

#[tokio::test]
async fn test_request_head_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, None, false, &server, http::Method::HEAD).await;
}

#[tokio::test]
async fn test_request_head_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, None, None, false, &server, http::Method::HEAD).await;
}

#[tokio::test]
async fn test_request_post_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, None, false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, Some("I'm tired of writing tests..."), false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_post_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("When will this end"), false, &server, http::Method::POST).await;
}

#[tokio::test]
async fn test_request_put_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, None, false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, Some("This is a body... Here we go again"), false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_put_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("This is a string body"), false, &server, http::Method::PUT).await;
}

#[tokio::test]
async fn test_request_patch_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, None, false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, Some("Body that I used to know"), false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_patch_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("Nekalakanenahapenenaweewatsnothin"), false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_delete_request() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, None, false, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), None, false, &server, http::Method::DELETE).await;
}

#[tokio::test]
async fn test_request_delete_request_with_body() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request(client, None, Some("This is a String body... Or is it?!"), false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_request_delete_request_with_body_and_query_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_request(client, Some(query_args), Some("Vsauce, Michael here"), false, &server, http::Method::PATCH).await;
}

#[tokio::test]
async fn test_with_nested_args() {
    let server = MockServer::start();
    let client = reqwest::Client::new();
    test_request_with_nested_path(client, false, &server, http::Method::GET).await;
}
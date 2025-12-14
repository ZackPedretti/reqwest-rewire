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
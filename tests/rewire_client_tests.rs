mod common;

use std::collections::HashMap;
use httpmock::MockServer;
use common::test_utils::*;
use reqwest_rewire::rewire_client::RewireClient;

#[tokio::test]
async fn test_get_request() {
    let server = MockServer::start();
    let rewire_params = build_rewire_test_params(&server);
    dbg!(&rewire_params);
    let client = RewireClient::new(rewire_params);
    test_get(client, None, true, &server).await;
}

#[tokio::test]
async fn test_get_request_with_query_args() {
    let server = MockServer::start();
    let client = RewireClient::new(build_rewire_test_params(&server));
    let mut query_args = HashMap::new();
    query_args.insert("stop", "stop_area:GST:SA:HLCRT");
    query_args.insert("line", "T1");
    test_get(client, Some(query_args), true, &server).await;
}
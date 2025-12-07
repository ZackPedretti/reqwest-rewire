use std::collections::HashMap;
use std::fmt::Display;
use std::pin::Pin;
use http::Method;
use reqwest::{Error, Request, RequestBuilder, Response};
use crate::testable_client::TestableClient;

pub struct RewireClient {
    redirects: HashMap<String, String>,
    client: reqwest::Client,
}

impl RewireClient {
    pub fn new(redirects: HashMap<String, String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            redirects,
        }
    }

    pub fn from_reqwest_client(client: reqwest::Client, redirects: HashMap<String, String>) -> Self {
        Self {
            client,
            redirects,
        }
    }
}

impl RewireClient {
    fn get_url(&self, url: &str) -> &str {
        self.redirects.get(url).map(|s| s.as_str()).unwrap_or(url)
    }
}

impl TestableClient for RewireClient {
    fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(self.get_url(url))
    }

    fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(self.get_url(url))
    }

    fn put(&self, url: &str) -> RequestBuilder {
        self.client.put(self.get_url(url))
    }

    fn patch(&self, url: &str) -> RequestBuilder {
        self.client.patch(self.get_url(url))
    }

    fn delete(&self, url: &str) -> RequestBuilder {
        self.client.delete(self.get_url(url))
    }

    fn head(&self, url: &str) -> RequestBuilder {
        self.client.head(self.get_url(url))
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client.request(method, self.get_url(url))
    }

    fn execute<'life0, 'async_trait>(&'life0 self, request: Request) -> Pin<Box<dyn Future<Output=Result<Response, Error>> + Send + 'async_trait>> {
        self.client.execute(request)
    }
}
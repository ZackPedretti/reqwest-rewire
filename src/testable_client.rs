use std::pin::Pin;
use http::Method;
use reqwest::{RequestBuilder, Response};

pub trait TestableClient {
    fn get(&self, url: &str) -> RequestBuilder;
    fn post(&self, url: &str) -> RequestBuilder;
    fn put(&self, url: &str) -> RequestBuilder;
    fn patch(&self, url: &str) -> RequestBuilder;
    fn delete(&self, url: &str) -> RequestBuilder;
    fn head(&self, url: &str) -> RequestBuilder;
    fn request(&self, method: Method, url: &str) -> RequestBuilder;
    fn execute<'life0, 'async_trait>(
        &'life0 self,
        request: reqwest::Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, reqwest::Error>> + Send + 'async_trait>>;
}

impl TestableClient for reqwest::Client {
    fn get(&self, url: &str) -> RequestBuilder {
        self.get(url)
    }

    fn post(&self, url: &str) -> RequestBuilder {
        self.post(url)
    }

    fn put(&self, url: &str) -> RequestBuilder {
        self.put(url)
    }

    fn patch(&self, url: &str) -> RequestBuilder {
        self.patch(url)
    }

    fn delete(&self, url: &str) -> RequestBuilder {
        self.delete(url)
    }

    fn head(&self, url: &str) -> RequestBuilder {
        self.head(url)
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.request(method, url)
    }

    fn execute<'life0, 'async_trait>(
        &'life0 self,
        request: reqwest::Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, reqwest::Error>> + Send + 'async_trait>> {
        self.execute(request)
    }
}
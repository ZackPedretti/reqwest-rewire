use http::Method;
use reqwest::{RequestBuilder};

pub trait TestableClient {
    fn get(&self, url: &str) -> RequestBuilder;
    fn post(&self, url: &str) -> RequestBuilder;
    fn put(&self, url: &str) -> RequestBuilder;
    fn patch(&self, url: &str) -> RequestBuilder;
    fn delete(&self, url: &str) -> RequestBuilder;
    fn head(&self, url: &str) -> RequestBuilder;
    fn request(&self, method: Method, url: &str) -> RequestBuilder;
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
}
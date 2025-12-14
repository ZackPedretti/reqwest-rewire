use std::borrow::Cow;
use crate::testable_client::TestableClient;
use http::Method;
use reqwest::RequestBuilder;
use std::collections::HashMap;

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

    pub fn from_reqwest_client(
        client: reqwest::Client,
        redirects: HashMap<String, String>,
    ) -> Self {
        Self { client, redirects }
    }
}

impl RewireClient {
    fn get_url<'a>(&'a self, url: &'a str) -> Cow<'a, str> {

        let (base_url, query_string) = url
            .split_once('?')
            .map(|(u, q)| (u, format!("?{}", q)))
            .unwrap_or((url, "".to_string()));
        
        match self.redirects.get(base_url) {
            None => Cow::Borrowed(url),
            Some(rewire_url) => {
                Cow::Owned(format!("{}{}", rewire_url, query_string))
            }
        }
    }
}

impl TestableClient for RewireClient {
    fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(self.get_url(url).as_ref())
    }

    fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(self.get_url(url).as_ref())
    }

    fn put(&self, url: &str) -> RequestBuilder {
        self.client.put(self.get_url(url).as_ref())
    }

    fn patch(&self, url: &str) -> RequestBuilder {
        self.client.patch(self.get_url(url).as_ref())
    }

    fn delete(&self, url: &str) -> RequestBuilder {
        self.client.delete(self.get_url(url).as_ref())
    }

    fn head(&self, url: &str) -> RequestBuilder {
        self.client.head(self.get_url(url).as_ref())
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client.request(method, self.get_url(url).as_ref())
    }
}

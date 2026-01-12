use crate::RewireClient;
use crate::TestableClient;

pub enum Client {
    ReqwestClient(reqwest::Client),
    TestClient(RewireClient),
}

impl TestableClient for Client {
    fn get(&self, url: &str) -> reqwest::RequestBuilder {
        match self {
            Client::ReqwestClient(client) => client.get(url),
            Client::TestClient(rewire_client) => rewire_client.get(url),
        }
    }

    fn post(&self, url: &str) -> reqwest::RequestBuilder {
        match self {
            Client::ReqwestClient(client) => client.post(url),
            Client::TestClient(rewire_client) => rewire_client.post(url),
        }
    }

    fn put(&self, url: &str) -> reqwest::RequestBuilder {
        match self {
            Client::ReqwestClient(client) => client.put(url),
            Client::TestClient(rewire_client) => rewire_client.put(url),
        }
    }

    fn patch(&self, url: &str) -> reqwest::RequestBuilder {
        match self {
            Client::ReqwestClient(client) => client.patch(url),
            Client::TestClient(rewire_client) => rewire_client.patch(url),
        }
    }

    fn delete(&self, url: &str) -> reqwest::RequestBuilder {
        match self {
            Client::ReqwestClient(client) => client.delete(url),
            Client::TestClient(rewire_client) => rewire_client.delete(url),
        }
    }

    fn head(&self, url: &str) -> reqwest::RequestBuilder {
        match self {
            Client::ReqwestClient(client) => client.head(url),
            Client::TestClient(rewire_client) => rewire_client.head(url),
        }
    }

    fn request(&self, method: http::Method, url: &str) -> reqwest::RequestBuilder {
        match self {
            Client::ReqwestClient(client) => client.request(method, url),
            Client::TestClient(rewire_client) => rewire_client.request(method, url),
        }
    }
}

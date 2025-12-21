use std::borrow::Cow;
use crate::TestableClient;
use http::Method;
use reqwest::{RequestBuilder, Url};
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
    /// Gets the rewired URL from the request-rewire client instance's `redirects` propriety.
    /// If the request URL is not in the `redirects` hashmap, the request URL is unchanged.
    /// 
    /// The request methods (.get(), .post(), .put()...) of the request-rewire client all use this method internally.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.get("http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.get("http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.get("http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn get_url<'a>(&'a self, url: &'a str) -> Cow<'a, str> {

        let parsed = match Url::parse(url) {
            Ok(u) => u,
            Err(_) => return Cow::Borrowed(url),
        };

        let key = self.redirects.keys()
            .flat_map(|r| Url::parse(r.as_str()))
            .filter(|u| u.host_str().is_some() && u.host_str() == parsed.host_str() && parsed.path().starts_with(u.path()) )
            .max_by(|a, b| a.path().len().cmp(&b.path().len()));

        if key.is_none() {
            return Cow::Borrowed(url);
        }

        let key = key.unwrap();
        let rewire = Url::parse(self.redirects.get(key.as_str()).unwrap());
        let rewire = match rewire {
            Ok(r) => r,
            Err(_) => return Cow::Borrowed(url)
        };

        let segment_count = rewire.path_segments().map(|s| s.count()).unwrap_or(0);
        let rest_of_path = parsed
            .path_segments()
            .map(|segments| segments.skip(segment_count).collect::<Vec<_>>().join("/"))
            .unwrap_or_default();

        let mut rewire = rewire.to_string();
        if !rest_of_path.is_empty() {
            rewire.push('/');
            rewire.push_str(&rest_of_path);
        }
        let query_string = parsed.query().unwrap_or_default();
        if !query_string.is_empty() {
            rewire.push('?');
            rewire.push_str(query_string);
        }
        Cow::Owned(rewire)
    }
}

impl TestableClient for RewireClient {
    
    /// Sends a GET request to a rewired URL if it is in the client's `redirects` hashmap, or else to the URL in method call.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.get("http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.get("http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.get("http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(self.get_url(url).as_ref())
    }
    
    /// Sends a POST request to a rewired URL if it is in the client's `redirects` hashmap, or else to the URL in method call.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.post("http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.post("http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.post("http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(self.get_url(url).as_ref())
    }

    /// Sends a PUT request to a rewired URL if it is in the client's `redirects` hashmap, or else to the URL in method call.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.put("http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.put("http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.put("http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn put(&self, url: &str) -> RequestBuilder {
        self.client.put(self.get_url(url).as_ref())
    }

    /// Sends a PATCH request to a rewired URL if it is in the client's `redirects` hashmap, or else to the URL in method call.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.patch("http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.patch("http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.patch("http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn patch(&self, url: &str) -> RequestBuilder {
        self.client.patch(self.get_url(url).as_ref())
    }

    /// Sends a DELETE request to a rewired URL if it is in the client's `redirects` hashmap, or else to the URL in method call.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.delete("http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.delete("http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.delete("http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn delete(&self, url: &str) -> RequestBuilder {
        self.client.delete(self.get_url(url).as_ref())
    }

    /// Sends a HEAD request to a rewired URL if it is in the client's `redirects` hashmap, or else to the URL in method call.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.head("http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.head("http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.head("http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn head(&self, url: &str) -> RequestBuilder {
        self.client.head(self.get_url(url).as_ref())
    }

    /// Sends a request to a rewired URL if it is in the client's `redirects` hashmap, or else to the URL in method call.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use reqwest_rewire::rewire_client::RewireClient;
    /// use reqwest_rewire::testable_client::TestableClient;
    ///
    /// let mut redirects: HashMap<String, String> = HashMap::new();
    /// redirects.insert("http://localhost:8080/example".into_string(), "http://localhost:8080/rewired".into_string());
    /// let client = RewireClient::new(redirects);
    /// client.request(http::Method::GET, "http://localhost:8080/example").send().await.unwrap(); // hits http://localhost:8080/rewired
    /// client.request(http::Method::GET, "http://localhost:8080/example/nested_path").send().await.unwrap(); // hits http://localhost:8080/rewired/nested_path
    /// client.request(http::Method::GET, "http://localhost:8080/example?foo=bar").send().await.unwrap(); // hits http://localhost:8080/rewired?foo=bar
    /// ```
    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client.request(method, self.get_url(url).as_ref())
    }
}

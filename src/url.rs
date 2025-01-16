mod parser;
mod url_search_params;

pub use url_search_params::*;

use parser::*;

#[derive(Debug)]
pub struct URL {
  hash: Option<String>,
  pathname: String,
  port: Option<String>,
  protocol: String,
  username: Option<String>,
  password: Option<String>,
  hostname: String,
  pub search_params: URLSearchParams,
}

impl URL {
  /// 解析URL字符串，返回URL结构体的实例
  /// 
  /// # Example
  /// ```
  /// use fetch_js::url::URL;
  /// let url = URL::new("https://user:pass@example.com:8080/path/to/file.html?query=string#hash");
  /// assert_eq!(url.get_href(), "https://user:pass@example.com:8080/path/to/file.html?query=string#hash");
  /// assert_eq!(url.get_protocol(), "https");
  /// ```
  pub fn new(url: &str) -> Self {
    let mut url = url.to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);
    Self {
      hash,
      pathname,
      port,
      protocol,
      username,
      password,
      hostname,
      search_params,
    }
  }
}

impl URL {
  pub fn get_protocol(&self) -> String {
    self.protocol.clone()
  }
  
  pub fn set_protocol(&mut self, protocol: &str) {
    self.protocol = protocol.to_string();
  }
  
  pub fn get_username(&self) -> Option<String> {
    self.username.clone()
  }
  
  pub fn set_username(&mut self, username: &str) {
    self.username = Some(username.to_string());
  }
  
  pub fn get_password(&self) -> Option<String> {
    self.password.clone()
  }
  
  pub fn set_password(&mut self, password: &str) {
    self.password = Some(password.to_string());
  }
  
  pub fn get_hostname(&self) -> String {
    self.hostname.clone()
  }
  
  pub fn set_hostname(&mut self, hostname: &str) {
    self.hostname = hostname.to_string();
  }
  
  pub fn get_port(&self) -> Option<String> {
    self.port.clone()
  }
  
  pub fn set_port(&mut self, port: &str) {
    self.port = Some(port.to_string());
  }
  
  pub fn get_pathname(&self) -> String {
    self.pathname.clone()
  }
  
  pub fn set_pathname(&mut self, pathname: &str) {
    self.pathname = pathname.to_string();
  }
  
  pub fn get_hash(&self) -> Option<String> {
    self.hash.clone()
  }
  
  pub fn set_hash(&mut self, hash: &str) {
    self.hash = Some(hash.to_string());
  }
}

impl URL {
  pub fn get_href(&self) -> String {
    todo!()
  }
}


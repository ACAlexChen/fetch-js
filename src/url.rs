use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub struct URL {
  hash: String,
  pathname: String,
  port: String,
  protocol: String,
  username: String,
  password: String,
  hostname: String,
  pub search_params: URLSearchParams,
}

impl URL {
  fn parse_protocol(url: &mut String) -> Result<String, Box<dyn std::error::Error>> {
    let protocol_end = url.find("//").ok_or("Invalid URL")?;
    let protocol = &url.clone()[..protocol_end];
    *url = url[protocol_end + 2..].to_string();
    Ok(protocol.to_string())
  }
  
  fn parse_username_and_password(url: &mut String) -> Result<(String, String), Box<dyn std::error::Error>> {
    let username_end = url.find(":").ok_or("Invalid URL")?;
    let username = &url.clone()[..username_end];
    *url = url[username_end + 1..].to_string();
    let password_end = url.find("@").ok_or("Invalid URL")?;
    let password = &url.clone()[..password_end];
    *url = url[password_end + 1..].to_string();
    Ok((username.to_string(), password.to_string()))
  }
  
  fn parse_hostname_and_port(url: &mut String) -> Result<(String, String), Box<dyn std::error::Error>> {
    let hostname_end = url.find(":").ok_or("Invalid URL")?;
    let hostname = &url.clone()[..hostname_end];
    *url = url[hostname_end + 1..].to_string();
    let port_end = url.find("/").ok_or("Invalid URL")?;
    let port = &url.clone()[..port_end];
    *url = url[port_end + 1..].to_string();
    Ok((hostname.to_string(), port.to_string()))
  }
  
  fn parse_pathname(url: &mut String) -> Result<String, Box<dyn std::error::Error>> {
    let pathname_end = url.find("?").ok_or("Invalid URL")?;
    let pathname = &url.clone()[..pathname_end];
    *url = url[pathname_end + 1..].to_string();
    Ok(pathname.to_string())
  }
  
  fn parse_search_params_and_hash(url: &mut String) -> Result<(URLSearchParams, String), Box<dyn std::error::Error>> {
    let search_end = url.find("#").ok_or("Invalid URL")?;
    let search = &url.clone()[..search_end];
    *url = url[search_end + 1..].to_string();
    let hash = url.clone();
    Ok((URLSearchParams::new(search), hash.to_string()))
  }
  
  pub fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let mut url = url.to_string();
    let protocol = Self::parse_protocol(&mut url).unwrap_or_else(|_| "http".to_string());
    let (username, password) = Self::parse_username_and_password(&mut url).unwrap_or_else(|_| ("".to_string(), "".to_string()));
    let (hostname, port) = Self::parse_hostname_and_port(&mut url).unwrap_or_else(|_| ("".to_string(), "".to_string()));
    let pathname = Self::parse_pathname(&mut url).unwrap_or_else(|_| "".to_string());
    let (search_params, hash) = Self::parse_search_params_and_hash(&mut url).unwrap_or_else(|_| (URLSearchParams::new(""), "".to_string()));
    Ok(Self {
      hash,
      pathname,
      port,
      protocol,
      username,
      password,
      hostname,
      search_params, 
    })
  }
}

impl URL {
  pub fn get_protocol(&self) -> String {
    self.protocol.clone()
  }
  
  pub fn set_protocol(&mut self, protocol: &str) {
    self.protocol = protocol.to_string();
  }
  
  pub fn get_username(&self) -> String {
    self.username.clone()
  }
  
  pub fn set_username(&mut self, username: &str) {
    self.username = username.to_string();
  }
  
  pub fn get_password(&self) -> String {
    self.password.clone()
  }
  
  pub fn set_password(&mut self, password: &str) {
    self.password = password.to_string();
  }
  
  pub fn get_hostname(&self) -> String {
    self.hostname.clone()
  }
  
  pub fn set_hostname(&mut self, hostname: &str) {
    self.hostname = hostname.to_string();
  }
  
  pub fn get_port(&self) -> String {
    self.port.clone()
  }
  
  pub fn set_port(&mut self, port: &str) {
    self.port = port.to_string();
  }
  
  pub fn get_pathname(&self) -> String {
    self.pathname.clone()
  }
  
  pub fn set_pathname(&mut self, pathname: &str) {
    self.pathname = pathname.to_string();
  }
  
  pub fn get_hash(&self) -> String {
    self.hash.clone()
  }
  
  pub fn set_hash(&mut self, hash: &str) {
    self.hash = hash.to_string();
  }
}

impl URL {
  pub fn get_href(&self) -> String {
    format!("{}://{}:{}{}?{}#{}", self.protocol, self.hostname, self.port, self.pathname, self.search_params, self.hash)
  }
}

#[derive(Debug)]
pub struct URLSearchParams {
  params: HashMap<String, String>,
}

impl URLSearchParams {
  pub fn new(param_str: &str) -> Self {
    let param_strings = param_str.split('&').collect::<Vec<&str>>();
    let mut params = HashMap::new();
    for param in param_strings {
      let parts = param.split('=').collect::<Vec<&str>>();
      params.insert(parts[0].to_string(), parts[1].to_string());
    };
    Self {
      params,
    }
  }
}

impl URLSearchParams {
  pub fn get(&self, name: &str) -> Option<String> {
    self.params.get(name).map(|v| v.to_string())
  }

  pub fn set(&mut self, name: &str, value: &str) {
    self.params.insert(name.to_string(), value.to_string());
  }

  pub fn has(&self, name: &str) -> bool {
    self.params.contains_key(name)
  }

  pub fn delete(&mut self, name: &str) {
    self.params.remove(name);
  }

  pub fn entries(&self) -> SearchParamsIter {
    SearchParamsIter::new(self.params.clone())
  }

  pub fn for_each<F>(&self, callback: F) where F: Fn(&str, &str, &Self) {
    for (k, v) in self.params.iter() {
      callback(k, v, self);
    }
  }

  pub fn keys(&self) -> StringIter {
    StringIter::new(self.params.keys().map(|k| k.to_string()).collect())
  }

  pub fn values(&self) -> StringIter {
    StringIter::new(self.params.values().map(|v| v.to_string()).collect())
  }
}

impl IntoIterator for URLSearchParams {
  type Item = (String, String);
  type IntoIter = SearchParamsIter;
  fn into_iter(self) -> Self::IntoIter {
    SearchParamsIter::new(self.params)
  }
}

impl Display for URLSearchParams {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut param_strings = vec![];
    for (k, v) in self.params.iter() {
      param_strings.push(format!("{}={}", k, v));
    };
    write!(f, "{}", param_strings.join("&"))
  }
}

#[derive(Debug)]
pub struct SearchParamsIter {
  params: Vec<(String, String)>,
  index: usize,
}

impl SearchParamsIter {
  pub fn new(params: HashMap<String, String>) -> Self {
    let mut param_vec = vec![];
    for (k, v) in params.iter() {
      param_vec.push((k.to_string(), v.to_string()));
    };
    Self {
      params: param_vec,
      index: 0,
    }
  }
}

impl Iterator for SearchParamsIter {
  type Item = (String, String);
  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.params.len() {
      let param = self.params[self.index].clone();
      self.index += 1;
      Some(param)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct StringIter {
  strings: Vec<String>,
  index: usize,
}

impl StringIter {
  pub fn new(strings: Vec<String>) -> Self {
    Self {
      strings,
      index: 0,
    }
  }
}

impl Iterator for StringIter {
  type Item = String;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index < self.strings.len() {
      let string = self.strings[self.index].clone();
      self.index += 1;
      Some(string)
    } else {
      None
    }
  }
}
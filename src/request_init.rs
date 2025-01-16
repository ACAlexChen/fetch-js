use std::collections::HashMap;

pub enum Method {
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS,
}

pub(crate) fn get_method_string(method: &Method) -> String {
  match method {
    Method::GET => "GET".to_string(),
    Method::POST => "POST".to_string(),
    Method::PUT => "PUT".to_string(),
    Method::DELETE => "DELETE".to_string(),
    Method::PATCH => "PATCH".to_string(),
    Method::HEAD => "HEAD".to_string(),
    Method::OPTIONS => "OPTIONS".to_string(),
  }
}

pub(crate) fn header_sort(headers: &HashMap<String, String>) -> Vec<String> {
  let sorted_headers = headers.iter().collect::<Vec<(&String, &String)>>();
  sorted_headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect()
}

pub struct RequestInit {
  pub method: Method,
  pub headers: HashMap<String, String>,
  pub body: Option<String>,
}

impl Default for RequestInit {
  fn default() -> Self {
    RequestInit {
      method: Method::GET,
      headers: HashMap::new(),
      body: None,
    }
  }
}
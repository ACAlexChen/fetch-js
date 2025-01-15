pub mod url;

use std::collections::HashMap;
use crate::url::URL;

pub enum Method {
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS,
}

fn get_method_string(method: &Method) -> String {
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

fn header_sort(headers: &HashMap<String, String>) -> Vec<String> {
  let sorted_headers = headers.iter().collect::<Vec<(&String, &String)>>();
  sorted_headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect()
}

pub struct RequestInit {
  method: Method,
  headers: HashMap<String, String>,
  body: Option<String>,
}

#[cfg(feature = "tokio-fetch")]
use tokio::{
  net::TcpStream,
  io::{AsyncWriteExt, AsyncReadExt},
};

#[cfg(feature = "tokio-fetch")]
pub struct Response {
  stream: TcpStream,
}
#[cfg(feature = "tokio-fetch")]
impl Response {
  pub async fn text(&mut self) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = vec![0; 1024];
    let mut text = String::new();
    loop {
      let n = self.stream.read(&mut buffer).await?;
      if n == 0 {
        break;
      }
      text.push_str(&String::from_utf8_lossy(&buffer[..n]));
    }
    Ok(text)
  }
}

#[cfg(feature = "tokio-fetch")]
pub async fn fetch(input: URL, init: RequestInit) -> Result<Response, Box<dyn std::error::Error>> {
  let mut stream = TcpStream::connect(input.get_hostname()).await?;
  let request = format!(
    "{} {} HTTP/1.1\r\nHost: {}\r\n{}\r\n\r\n{}",
    get_method_string(&init.method),
    input.get_pathname(),
    input.get_hostname(),
    header_sort(&init.headers).join("\r\n"),
    init.body.unwrap_or("".to_string())
  );
  stream.write_all(request.as_bytes()).await?;
  Ok(Response { stream })
}


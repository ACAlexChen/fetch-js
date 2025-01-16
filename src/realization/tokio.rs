use crate::request_init::{RequestInit, get_method_string, header_sort};
use crate::url::URL;


use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


pub struct Response {
  stream: TcpStream,
}

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


/// 向服务器发送请求并获取响应
///
/// # Example
/// ```
/// use fetch_js::url::URL;
/// use fetch_js::request_init::RequestInit;
/// use fetch_js::fetch;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let url = URL::new("https://example.com")?;
///   let init = RequestInit::default();
///   let mut response = fetch(url, init).await?;
///   let text = response.text().await?;
///   println!("{}", text);
///   Ok(())
/// }
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


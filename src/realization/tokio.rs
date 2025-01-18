mod response;

use crate::request_init::RequestInit;
use crate::url::URL;
use thiserror::Error;

pub use response::Response;

#[derive(Debug, Error)]
pub enum FetchError {}

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
///   let url = URL::new("https://example.com");
///   let init = RequestInit::default();
///   let mut response = fetch(url, init).await?;
///   let text = response.text().await?;
///   println!("{}", text);
///   Ok(())
/// }
pub async fn fetch(input: URL, init: RequestInit) -> Result<Response, FetchError> {
  todo!()
}


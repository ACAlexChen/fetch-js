use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

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
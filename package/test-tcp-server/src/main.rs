use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Read, Write};

fn main() {
  let v = vec![1, 2, 3];
  let iter = v.into_iter();
  let listener = TcpListener::bind("127.0.0.1:2010").unwrap();
  for stream in listener.incoming() {
    println!("New connection: {:?}", stream);
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  println!("Request: {:#?}", http_request);
  let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nHello, world!";
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

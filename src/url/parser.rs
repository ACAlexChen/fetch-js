use super::URLSearchParams;



fn str_interceptor(str: &mut String, start: usize, end: usize) -> String {
  let iter = str.drain(start..end);
  let mut result = String::new();
  for c in iter {
    result.push(c);
  };
  result
}

pub(in super) fn intercept_protocol(url: &mut String) -> String {
  let end = match url.find("//") {
    Some(i) => i,
    None => return "http:".to_string(),
  };
  let protocol = str_interceptor(url, 0, end);
  url.drain(0..2);
  protocol
}

pub(in super) fn intercept_username(url: &mut String) -> Option<String> {
  let end = match url.find("@") {
    Some(_) => {
      match url.find(":") {
        Some(i) => i,
        None => return None,
      }
    },
    None => return None,
  };
  let username = str_interceptor(url, 0, end);
  url.drain(0..1);
  Some(username)
}

pub(in super) fn intercept_password(url: &mut String) -> Option<String> {
  let end = match url.find("@") {
    Some(i) => i,
    None => return None,
  };
  let password = str_interceptor(url, 0, end);
  url.drain(0..1);
  Some(password)
}

pub(in super) fn intercept_hostname(url: &mut String) -> String {
  let end = match url.find(":") {
    Some(i) => i,
    None => {
      match url.find("/") {
        Some(i) => i,
        None => return url.to_string(),
      }
    },
  };
  str_interceptor(url, 0, end)
}

pub(in super) fn intercept_port(url: &mut String) -> Option<String> {
  let end = if url.starts_with(":") {
    match url.find("/") {
      Some(i) => i,
      None => {
        match url.find("?") {
          Some(i) => i,
          None => {
            match url.find("#") {
              Some(i) => i,
              None => url.len()
            }
          }
        }
      },
    }
  } else {
    return None;
  };
  let port = str_interceptor(url, 1, end);
  url.drain(0..1);
  Some(port)
}

pub(in super) fn intercept_pathname(url: &mut String) -> String {
  let end = match url.find("?") {
    Some(i) => i,
    None => {
      match url.find("#") {
        Some(i) => i,
        None => url.len()
      }
    }
  };
  if url.starts_with("/") {
    str_interceptor(url, 0, end)
  } else {
    "/".to_string()
  }
}

pub(in super) fn intercept_search_params(url: &mut String) -> URLSearchParams {
  let end = match url.find("#") {
    Some(i) => i,
    None => url.len(),
  };
  if url.starts_with("?") {
    let search_params = str_interceptor(url, 1, end);
    url.drain(0..1);
    URLSearchParams::new(&search_params)
  } else {
    URLSearchParams::new("")
  }
}

pub(in super) fn intercept_hash(url: &mut String) -> Option<String> {
  let end = url.len();
  if url.starts_with("#") {
    Some(str_interceptor(url, 0, end))
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn all() {
    let mut url = "https://user:pass@example.com:8080/path/to/file.html?query=string#hash".to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);

    assert_eq!(protocol, "https:");
    assert_eq!(username, Some("user".to_string()));
    assert_eq!(password, Some("pass".to_string()));
    assert_eq!(hostname, "example.com");
    assert_eq!(port, Some("8080".to_string()));
    assert_eq!(pathname, "/path/to/file.html");
    assert_eq!(search_params.to_string(), "query=string");
    assert_eq!(hash, Some("#hash".to_string()));
  }

  #[test]
  fn without_protocol() {
    let mut url = "example.com:8080/path/to/file.html?query=string#hash".to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);

    assert_eq!(protocol, "http:");
    assert_eq!(username, None);
    assert_eq!(password, None);
    assert_eq!(hostname, "example.com");
    assert_eq!(port, Some("8080".to_string()));
    assert_eq!(pathname, "/path/to/file.html");
    assert_eq!(search_params.to_string(), "query=string");
    assert_eq!(hash, Some("#hash".to_string()));
  }

  #[test]
  fn without_username_and_password() {
    let mut url = "https://example.com:8080/path/to/file.html?query=string#hash".to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);

    assert_eq!(protocol, "https:");
    assert_eq!(username, None);
    assert_eq!(password, None);
    assert_eq!(hostname, "example.com");
    assert_eq!(port, Some("8080".to_string()));
    assert_eq!(pathname, "/path/to/file.html");
    assert_eq!(search_params.to_string(), "query=string");
    assert_eq!(hash, Some("#hash".to_string()));
  }

  #[test]
  fn without_port() {
    let mut url = "https://user:pass@example.com/path/to/file.html?query=string#hash".to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);

    assert_eq!(protocol, "https:");
    assert_eq!(username, Some("user".to_string()));
    assert_eq!(password, Some("pass".to_string()));
    assert_eq!(hostname, "example.com");
    assert_eq!(port, None);
    assert_eq!(pathname, "/path/to/file.html");
    assert_eq!(search_params.to_string(), "query=string");
    assert_eq!(hash, Some("#hash".to_string()));
  }

  #[test]
  fn without_pathname() {
    let mut url = "https://user:pass@example.com:8080?query=string#hash".to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);

    assert_eq!(protocol, "https:");
    assert_eq!(username, Some("user".to_string()));
    assert_eq!(password, Some("pass".to_string()));
    assert_eq!(hostname, "example.com");
    assert_eq!(port, Some("8080".to_string()));
    assert_eq!(pathname, "/");
    assert_eq!(search_params.to_string(), "query=string");
    assert_eq!(hash, Some("#hash".to_string()));
  }

  #[test]
  fn without_search_params() {
    let mut url = "https://user:pass@example.com:8080/path/to/file.html#hash".to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);

    assert_eq!(protocol, "https:");
    assert_eq!(username, Some("user".to_string()));
    assert_eq!(password, Some("pass".to_string()));
    assert_eq!(hostname, "example.com");
    assert_eq!(port, Some("8080".to_string()));
    assert_eq!(pathname, "/path/to/file.html");
    assert_eq!(search_params.to_string(), "");
    assert_eq!(hash, Some("#hash".to_string()));
  }

  #[test]
  fn without_hash() {
    let mut url = "https://user:pass@example.com:8080/path/to/file.html?query=string".to_string();
    let protocol = intercept_protocol(&mut url);
    let username = intercept_username(&mut url);
    let password = intercept_password(&mut url);
    let hostname = intercept_hostname(&mut url);
    let port = intercept_port(&mut url);
    let pathname = intercept_pathname(&mut url);
    let search_params = intercept_search_params(&mut url);
    let hash = intercept_hash(&mut url);

    assert_eq!(protocol, "https:");
    assert_eq!(username, Some("user".to_string()));
    assert_eq!(password, Some("pass".to_string()));
    assert_eq!(hostname, "example.com");
    assert_eq!(port, Some("8080".to_string()));
    assert_eq!(pathname, "/path/to/file.html");
    assert_eq!(search_params.to_string(), "query=string");
    assert_eq!(hash, None);
  }
}
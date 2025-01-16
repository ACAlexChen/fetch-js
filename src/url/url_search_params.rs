use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub struct URLSearchParams {
  params: HashMap<String, String>,
}

impl URLSearchParams {
  /// 解析URL查询字符串，返回URLSearchParams结构体的实例
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let search_params = URLSearchParams::new("query=string&key=value");
  /// assert_eq!(search_params.get("query").unwrap(), "string");
  /// assert_eq!(search_params.get("key").unwrap(), "value");
  /// ```
  pub fn new(param_str: &str) -> Self {
    let param_strings = param_str.split('&').collect::<Vec<&str>>();
    if param_strings.len() == 1 && param_strings[0].is_empty() {
      return Self {
        params: HashMap::new(),
      };
    };
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
  /// 获取查询参数的值
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let search_params = URLSearchParams::new("query=string&key=value");
  /// assert_eq!(search_params.get("query").unwrap(), "string");
  /// assert_eq!(search_params.get("key").unwrap(), "value");
  /// ```
  pub fn get(&self, name: &str) -> Option<String> {
    self.params.get(name).map(|v| v.to_string())
  }

  /// 设置查询参数的值
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let mut search_params = URLSearchParams::new("query=string&key=value");
  /// search_params.set("key", "new_value");
  /// assert_eq!(search_params.get("query").unwrap(), "string");
  /// assert_eq!(search_params.get("key").unwrap(), "new_value");
  /// ```
  pub fn set(&mut self, name: &str, value: &str) {
    self.params.insert(name.to_string(), value.to_string());
  }

  /// 判断是否存在查询参数
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let search_params = URLSearchParams::new("query=string&key=value");
  /// assert_eq!(search_params.has("query"), true);
  /// assert_eq!(search_params.has("key"), true);
  /// assert_eq!(search_params.has("not_exist"), false);
  /// ```
  pub fn has(&self, name: &str) -> bool {
    self.params.contains_key(name)
  }

  /// 删除查询参数
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let mut search_params = URLSearchParams::new("query=string&key=value");
  /// search_params.delete("key");
  /// assert_eq!(search_params.get("query").unwrap(), "string");
  /// assert_eq!(search_params.get("key"), None);
  /// ```
  pub fn delete(&mut self, name: &str) {
    self.params.remove(name);
  }

  /// 获取查询参数的迭代器
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let search_params = URLSearchParams::new("query=string&key=value");
  /// for (k, v) in search_params.entries() {
  ///   println!("{}: {}", k, v);
  /// }
  /// ```
  pub fn entries(&self) -> SearchParamsIter {
    SearchParamsIter::new(self.params.clone())
  }

  /// 遍历查询参数
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let search_params = URLSearchParams::new("query=string&key=value");
  /// search_params.for_each(|k, v, search_params| {
  ///   println!("{}: {}", k, v);
  /// });
  /// ```
  pub fn for_each<F>(&self, callback: F) where F: Fn(&str, &str, &Self) {
    for (k, v) in self.params.iter() {
      callback(k, v, self);
    }
  }

  /// 获取查询参数的键的迭代器
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let search_params = URLSearchParams::new("query=string&key=value");
  /// for k in search_params.keys() {
  ///   println!("{}", k);
  /// }
  /// ```
  pub fn keys(&self) -> StringIter {
    StringIter::new(self.params.keys().map(|k| k.to_string()).collect())
  }

  /// 获取查询参数的值的迭代器
  ///
  /// # Example
  /// ```
  /// use fetch_js::url::URLSearchParams;
  /// let search_params = URLSearchParams::new("query=string&key=value");
  /// for v in search_params.values() {
  ///   println!("{}", v);
  /// }
  /// ```
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
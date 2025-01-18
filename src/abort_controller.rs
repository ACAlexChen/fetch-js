type ListenerFn = Box<dyn Fn(Option<String>)>;

pub struct AbortController {
  pub signal: AbortSignal,
}

impl AbortController {
  pub fn new() -> Self {
    Self {
      signal: AbortSignal::new(),
    }
  }

  pub fn abort(&mut self, reason: Option<String>) -> () {
    self.signal.abort(reason);
  }
}

pub struct AbortSignal {
  pub aborted: bool,
  pub reason: Option<String>,
  listeners: Vec<ListenerFn>,
}

impl AbortSignal {
  pub fn new() -> Self {
    Self {
      aborted: false,
      listeners: Vec::new(),
      reason: None,
    }
  }

  pub fn add_event_listener(&mut self, listener: ListenerFn) -> usize {
    self.listeners.push(listener);
    self.listeners.len() - 1
  }
  
  pub fn remove_event_listener(&mut self, index: usize) {
    let _ = self.listeners.remove(index);
  }
  
  pub(in self) fn abort(&mut self, reason: Option<String>) -> () {
    self.aborted = true;
    self.reason = reason;
    for listener in self.listeners.iter_mut() {
      listener(self.reason.clone());
    }
  }
}


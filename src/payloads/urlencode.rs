pub trait UrlParams {
  fn to_url_params(&self) -> String;
}

impl<T: serde::Serialize> UrlParams for T {
  fn to_url_params(&self) -> String {
      serde_urlencoded::to_string(self).expect("Failed to encode params")
  }
}
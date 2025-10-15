// Performance optimization utilities

pub fn preload_resource(_url: &str, _resource_type: &str) {
  #[cfg(target_arch = "wasm32")]
  {
    use leptos::web_sys::window;

    if let Some(window) = window() {
      if let Some(document) = window.document() {
        if let Some(head) = document.head() {
          if let Ok(link) = document.create_element("link") {
            let _ = link.set_attribute("rel", "preload");
            let _ = link.set_attribute("href", _url);
            let _ = link.set_attribute("as", _resource_type);
            let _ = head.append_child(&link);
          }
        }
      }
    }
  }
}

pub fn add_prefetch_link(_url: &str) {
  #[cfg(target_arch = "wasm32")]
  {
    use leptos::web_sys::window;

    if let Some(window) = window() {
      if let Some(document) = window.document() {
        if let Some(head) = document.head() {
          if let Ok(link) = document.create_element("link") {
            let _ = link.set_attribute("rel", "prefetch");
            let _ = link.set_attribute("href", _url);
            let _ = head.append_child(&link);
          }
        }
      }
    }
  }
}

pub fn optimize_images() {
  #[cfg(target_arch = "wasm32")]
  {
    use leptos::web_sys::window;

    if let Some(window) = window() {
      if let Some(document) = window.document() {
        let images = document.get_elements_by_tag_name("img");
        for i in 0..images.length() {
          if let Some(img) = images.item(i) {
            let _ = img.set_attribute("loading", "lazy");
            let _ = img.set_attribute("decoding", "async");
          }
        }
      }
    }
  }
}

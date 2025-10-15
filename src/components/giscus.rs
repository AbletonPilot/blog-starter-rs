use leptos::prelude::*;

#[component]
pub fn Giscus() -> impl IntoView {
  #[cfg(target_arch = "wasm32")]
  let get_current_theme = || -> String {
    if let Some(window) = web_sys::window() {
      if let Some(document) = window.document() {
        if let Some(body) = document.body() {
          let is_light = body.class_list().contains("light-mode");
          return if is_light {
            "noborder_light".to_string()
          } else {
            "noborder_dark".to_string()
          };
        }
      }
    }
    "noborder_dark".to_string()
  };

  // Load Giscus script with current theme
  Effect::new(move |_| {
    #[cfg(target_arch = "wasm32")]
    {
      use wasm_bindgen::JsCast;

      let window = web_sys::window().expect("no global `window` exists");
      let document = window.document().expect("should have a document on window");

      // Remove existing Giscus script and widget if present
      if let Some(existing_script) = document
        .query_selector("script[src*='giscus.app']")
        .ok()
        .flatten()
      {
        existing_script.remove();
      }
      if let Some(existing_widget) = document.query_selector(".giscus").ok().flatten() {
        existing_widget.remove();
      }

      // Small delay to ensure DOM is ready
      let document_clone = document.clone();
      let current_theme = get_current_theme();

      let timeout_closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        // Create new Giscus script
        if let Ok(script) = document_clone.create_element("script") {
          let script = script.dyn_into::<web_sys::HtmlScriptElement>().unwrap();

          script.set_src("https://giscus.app/client.js");
          script
            .set_attribute("data-repo", "YOUR_USERNAME/YOUR_REPO")
            .ok();
          script.set_attribute("data-repo-id", "YOUR_REPO_ID").ok();
          script.set_attribute("data-category", "General").ok();
          script
            .set_attribute("data-category-id", "YOUR_CATEGORY_ID")
            .ok();
          script.set_attribute("data-mapping", "pathname").ok();
          script.set_attribute("data-strict", "0").ok();
          script.set_attribute("data-reactions-enabled", "0").ok();
          script.set_attribute("data-emit-metadata", "0").ok();
          script.set_attribute("data-input-position", "bottom").ok();
          script.set_attribute("data-theme", &current_theme).ok();
          script.set_attribute("data-lang", "en").ok();
          script.set_attribute("crossorigin", "anonymous").ok();
          script.set_async(true);

          if let Some(giscus_container) = document_clone.get_element_by_id("giscus-container") {
            giscus_container.append_child(&script).ok();
          }
        }
      }) as Box<dyn FnMut()>);

      window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
          timeout_closure.as_ref().unchecked_ref(),
          100,
        )
        .ok();
      timeout_closure.forget();
    }
  });

  // Listen for theme changes via MutationObserver
  Effect::new(move |_| {
    #[cfg(target_arch = "wasm32")]
    {
      use wasm_bindgen::prelude::*;
      use wasm_bindgen::JsCast;

      if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
          if let Some(body) = document.body() {
            // Create MutationObserver to watch for class changes on body
            let callback = Closure::wrap(Box::new(move |_mutations: JsValue| {
              if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                  if let Some(body) = document.body() {
                    let is_light = body.class_list().contains("light-mode");
                    let theme = if is_light {
                      "noborder_light"
                    } else {
                      "noborder_dark"
                    };

                    // Send message to Giscus iframe immediately to change theme
                    if let Some(iframe) = document
                      .query_selector("iframe.giscus-frame")
                      .ok()
                      .flatten()
                    {
                      let iframe_el = iframe.dyn_into::<web_sys::HtmlIFrameElement>().ok();
                      if let Some(iframe_element) = iframe_el {
                        if let Some(content_window) = iframe_element.content_window() {
                          // Create message: { "giscus": { "setConfig": { "theme": "noborder_dark" } } }
                          let message = js_sys::Object::new();
                          let giscus = js_sys::Object::new();
                          let set_config = js_sys::Object::new();

                          js_sys::Reflect::set(
                            &set_config,
                            &"theme".into(),
                            &JsValue::from_str(theme),
                          )
                          .ok();
                          js_sys::Reflect::set(&giscus, &"setConfig".into(), &set_config).ok();
                          js_sys::Reflect::set(&message, &"giscus".into(), &giscus).ok();

                          content_window
                            .post_message(&message, "https://giscus.app")
                            .ok();
                        }
                      }
                    }
                  }
                }
              }
            }) as Box<dyn FnMut(JsValue)>);

            let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()).ok();
            if let Some(obs) = observer {
              let options = web_sys::MutationObserverInit::new();
              options.set_attributes(true);
              options.set_attribute_filter(&js_sys::Array::of1(&"class".into()));
              obs.observe_with_options(&body, &options).ok();
            }

            callback.forget();
          }
        }
      }
    }
  });

  view! {
    <div class="giscus-wrapper">
      <div id="giscus-container"></div>
    </div>
  }
}

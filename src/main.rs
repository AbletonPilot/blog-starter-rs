#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  use axum::{
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
  };
  // IMPORTANT: If you changed the package name in Cargo.toml, update these imports
  // Replace blog_starter_rs with your_package_name (hyphens become underscores)
  use blog_starter_rs::app::*;
  use blog_starter_rs::posts::load_posts;
  use blog_starter_rs::rss::generate_rss;
  use blog_starter_rs::sitemap::{generate_robots_txt, generate_sitemap};
  use leptos::logging::log;
  use leptos::prelude::*;
  use leptos_axum::{generate_route_list, LeptosRoutes};

  let conf = get_configuration(None).unwrap();
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  // Generate the list of routes in your Leptos App
  let routes = generate_route_list(App);

  // Cache control middleware for static assets
  async fn cache_middleware(req: axum::extract::Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let mut response = next.run(req).await;

    // Apply cache headers to static assets
    if path.starts_with("/pkg/")
      || path.ends_with(".css")
      || path.ends_with(".js")
      || path.ends_with(".wasm")
    {
      response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000, immutable"),
      );
    } else if path.ends_with(".xml") || path.ends_with(".txt") {
      response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600"),
      );
    }

    response
  }

  // RSS handler
  async fn rss_handler() -> Response {
    let posts = load_posts();
    let rss_content = generate_rss(&posts);
    (
      StatusCode::OK,
      [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
      rss_content,
    )
      .into_response()
  }

  // Sitemap handler
  async fn sitemap_handler() -> Response {
    let posts = load_posts();
    let sitemap_content = generate_sitemap(&posts);
    (
      StatusCode::OK,
      [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
      sitemap_content,
    )
      .into_response()
  }

  // Robots.txt handler
  async fn robots_handler() -> Response {
    let robots_content = generate_robots_txt();
    (
      StatusCode::OK,
      [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
      robots_content,
    )
      .into_response()
  }

  let app = Router::new()
    .route("/rss.xml", axum::routing::get(rss_handler))
    .route("/sitemap.xml", axum::routing::get(sitemap_handler))
    .route("/robots.txt", axum::routing::get(robots_handler))
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .layer(middleware::from_fn(cache_middleware))
    .with_state(leptos_options);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
  // no client-side main function
  // unless we want this to work with e.g., Trunk for pure client-side testing
  // see lib.rs for hydration function instead
}

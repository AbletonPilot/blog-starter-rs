use crate::components::{AboutPage, ArchivePage, Giscus, PostSummaryCard};
use crate::posts::{Post, PostSummary};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  path, StaticSegment,
};

// Global search context
#[derive(Clone, Copy)]
pub struct SearchContext {
  pub query: RwSignal<String>,
  pub current_page: RwSignal<usize>,
}

#[server]
pub async fn get_post_summaries() -> Result<Vec<PostSummary>, ServerFnError> {
  Ok(crate::posts::load_post_summaries())
}

#[server]
pub async fn get_posts_by_tag_summaries(tag: String) -> Result<Vec<PostSummary>, ServerFnError> {
  let posts = crate::posts::load_post_summaries();
  Ok(
    posts
      .into_iter()
      .filter(|p| p.metadata.tags.iter().any(|t| t == &tag))
      .collect(),
  )
}

#[server]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
  Ok(crate::posts::load_posts())
}

#[server]
pub async fn get_post_by_slug(slug: String) -> Result<Option<Post>, ServerFnError> {
  let posts = crate::posts::load_posts();
  Ok(posts.into_iter().find(|p| p.slug == slug))
}

#[server]
pub async fn get_posts_by_tag(tag: String) -> Result<Vec<Post>, ServerFnError> {
  let posts = crate::posts::load_posts();
  Ok(
    posts
      .into_iter()
      .filter(|p| p.metadata.tags.iter().any(|t| t == &tag))
      .collect(),
  )
}

#[component]
fn SiteHeader() -> impl IntoView {
  // Detect system theme preference
  let initial_dark = {
    #[cfg(target_arch = "wasm32")]
    {
      let win = window();
      let prefers_dark = win
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
        .map(|mql| mql.matches())
        .unwrap_or(true);

      // Check localStorage first (user preference)
      if let Some(storage) = win.local_storage().ok().flatten() {
        if let Ok(Some(theme)) = storage.get_item("theme") {
          theme == "dark"
        } else {
          prefers_dark
        }
      } else {
        prefers_dark
      }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
      true // SSR default to dark
    }
  };

  let is_dark = RwSignal::new(initial_dark);
  let search_ctx = expect_context::<SearchContext>();
  let navigate = leptos_router::hooks::use_navigate();

  // Apply initial theme on mount
  #[cfg(target_arch = "wasm32")]
  {
    use leptos::prelude::Effect;
    Effect::new(move || {
      let win = window();
      if let Some(document) = win.document() {
        if let Some(body) = document.body() {
          let html = document.document_element();
          if !is_dark.get() {
            let _ = body.class_list().add_1("light-mode");
            if let Some(html_el) = html {
              let _ = html_el.class_list().add_1("light-mode");
            }
          }
        }
      }
    });
  }

  let toggle_theme = move |_| {
    is_dark.update(|dark| *dark = !*dark);

    #[cfg(target_arch = "wasm32")]
    {
      let win = window();
      if let Some(document) = win.document() {
        let html = document.document_element();
        if let Some(body) = document.body() {
          if is_dark.get() {
            let _ = body.class_list().remove_1("light-mode");
            if let Some(html_el) = html {
              let _ = html_el.class_list().remove_1("light-mode");
            }
          } else {
            let _ = body.class_list().add_1("light-mode");
            if let Some(html_el) = html {
              let _ = html_el.class_list().add_1("light-mode");
            }
          }
        }
      }

      // Save theme to localStorage
      if let Some(storage) = win.local_storage().ok().flatten() {
        let theme = if is_dark.get() { "dark" } else { "light" };
        let _ = storage.set_item("theme", theme);
      }
    }
  };
  let (menu_open, set_menu_open) = signal(false);

  view! {
    <header class="site-header">
      <nav class="container">
        <div class="nav-brand">
          <a href="/">"Your Blog Name"</a>
        </div>

        // Desktop navigation
        <div class="nav-left desktop-nav">
          <ul class="nav-links">
            <li><a href="/archive">"Archive"</a></li>
            <li><a href="/about">"About"</a></li>
          </ul>
        </div>

        <div class="nav-right">
          <div class="search-container">
            <input
              type="text"
              placeholder="Search..."
              class="search-input"
              on:input=move |ev| {
                search_ctx.query.set(event_target_value(&ev));
                search_ctx.current_page.set(1);
                // Navigate to home page when searching from other pages
                navigate("/", Default::default());
              }
              prop:value=move || search_ctx.query.get()
            />
            <svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"></circle>
              <path d="m21 21-4.35-4.35"></path>
            </svg>
          </div>

          <button class="theme-toggle" on:click=toggle_theme aria-label="Toggle theme">
            <svg
              class="theme-icon moon-icon"
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
            </svg>
            <svg
              class="theme-icon sun-icon"
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <circle cx="12" cy="12" r="5"></circle>
              <line x1="12" y1="1" x2="12" y2="3"></line>
              <line x1="12" y1="21" x2="12" y2="23"></line>
              <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
              <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
              <line x1="1" y1="12" x2="3" y2="12"></line>
              <line x1="21" y1="12" x2="23" y2="12"></line>
              <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
              <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
            </svg>
          </button>

          <button
            class="hamburger-btn mobile-nav"
            on:click=move |_| set_menu_open.update(|open| *open = !*open)
          >
            <span class="hamburger-line"></span>
            <span class="hamburger-line"></span>
            <span class="hamburger-line"></span>
          </button>
        </div>

        // Mobile menu
        <div class=move || format!("mobile-menu {}", if menu_open.get() { "open" } else { "" })>
          <ul class="mobile-nav-links">
            <li><a href="/archive" on:click=move |_| set_menu_open.set(false)>"Archive"</a></li>
            <li><a href="/about" on:click=move |_| set_menu_open.set(false)>"About"</a></li>
          </ul>
        </div>
      </nav>
    </header>
  }
}

#[component]
fn CookieConsent() -> impl IntoView {
  let (show_banner, set_show_banner) = signal(true);

  // Check localStorage on mount
  Effect::new(move |_| {
    #[cfg(target_arch = "wasm32")]
    {
      if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
          if storage.get_item("cookie-consent").ok().flatten().is_some() {
            set_show_banner.set(false);
          }
        }
      }
    }
  });

  let accept_cookies = move |_| {
    #[cfg(target_arch = "wasm32")]
    {
      if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
          let _ = storage.set_item("cookie-consent", "accepted");
        }
      }
    }
    set_show_banner.set(false);
  };

  view! {
    <Show when=move || show_banner.get()>
      <div class="cookie-banner">
        <div class="cookie-content">
          <p>
            "This site uses cookies to analyze traffic and improve your experience. "
            "By clicking Accept, you consent to the use of Google Analytics."
          </p>
          <button class="cookie-accept" on:click=accept_cookies>
            "Accept"
          </button>
        </div>
      </div>
    </Show>
  }
}

#[component]
fn SiteFooter() -> impl IntoView {
  view! {
    <footer class="site-footer">
      <div class="container">
      <p>
        "Built with Rust and Leptos | "
        <span class="footer-links">
          <a href="https://github.com/AbletonPilot" target="_blank">
            "GitHub"
          </a>
          <span>" | "</span>
          <a href="https://linkedin.com/in/YOUR_PROFILE" target="_blank">
            "LinkedIn"
          </a>
          <span>" | "</span>
          <a href="https://twitter.com/YOUR_HANDLE" target="_blank">
            "Twitter"
          </a>
        </span>
      </p>
      <div>
        <p>"© 2025 AbletonPilot. All rights reserved."</p>
      </div>
      </div>
    </footer>
  }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>

        <script>
          {r#"
          (function() {
            try {
              const theme = localStorage.getItem('theme');
              const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
              const isDark = theme ? theme === 'dark' : prefersDark;
              if (!isDark) {
                document.documentElement.classList.add('light-mode');
              }
            } catch (e) {
              console.error('Theme initialization error:', e);
            }
          })();
          "#}
        </script>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin=""/>
        <link rel="alternate" type="application/rss+xml" title="Your Blog RSS Feed" href="/rss.xml"/>
        <AutoReload options=options.clone() />
        <HydrationScripts options/>
        <MetaTags/>
      </head>
      <body>
        <App/>
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  // Provide global search context
  let search_ctx = SearchContext {
    query: RwSignal::new(String::new()),
    current_page: RwSignal::new(1),
  };
  provide_context(search_ctx);

  view! {
    // content for this welcome page
    <Router>
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/blog-starter-rs.css"/>
      // sets the document title
      <Title text="Your Blog Name"/>
      <SiteHeader/>
      <main>
        <Routes fallback=|| view! {
          <div class="container">
            <div class="not-found">
              <h1>"404"</h1>
              <p>"Page not found."</p>
              <a href="/">"← Back to home"</a>
            </div>
          </div>
        }.into_view()>
          <Route path=StaticSegment("") view=HomePage/>
          <Route path=StaticSegment("archive") view=ArchivePage/>
          <Route path=StaticSegment("about") view=AboutPage/>
          <Route path=path!("/posts/:slug") view=PostPage/>
          <Route path=path!("/tags/:tag") view=TagPage/>
        </Routes>
      </main>
      <SiteFooter/>
      <CookieConsent/>
    </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
  let posts = Resource::new(
    || (),
    |_| async move { get_post_summaries().await.unwrap_or_default() },
  );
  let search_ctx = expect_context::<SearchContext>();
  let posts_per_page = 10;

  view! {
    <Title text="Your Blog Name - Thoughts on programming and technology"/>
    <Meta name="description" content="A blog about programming, technology, and software development. Sharing insights and experiences in web development, Rust, and more."/>
    <Meta name="keywords" content="programming, technology, software development, rust, web development, leptos"/>
    <Meta name="author" content="Your Name"/>
    <Meta property="og:type" content="website"/>
    <Meta property="og:title" content="Your Blog Name"/>
    <Meta property="og:description" content="A blog about programming, technology, and software development"/>
    <Meta property="og:url" content="https://your-domain.com/"/>
    <Meta property="og:site_name" content="Your Blog Name"/>
    <Meta property="og:locale" content="en_US"/>
    <Meta name="twitter:card" content="summary"/>
    <Meta name="twitter:title" content="Your Blog Name"/>
    <Meta name="twitter:description" content="A blog about programming, technology, and software development"/>
    <Meta name="twitter:site" content="@YourName"/>
    <Meta name="application-name" content="Your Name Blog"/>
    <link rel="canonical" href="https://your-domain.com/"/>


    <div class="container">
      <header class="blog-header">
        <div class="header-intro">
          <span class="wave">"👋"</span>
          <span class="greeting">" Welcome to Your Blog"</span>
        </div>
        <h1>"Your Blog Name"</h1>
        <p class="tagline">"Thoughts on programming and technology"</p>
        <div class="social-icons">
          <a href="https://github.com/YOUR_USERNAME" target="_blank" rel="noopener noreferrer" aria-label="GitHub" class="social-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
          </a>
          <a href="https://linkedin.com/in/YOUR_PROFILE" target="_blank" rel="noopener noreferrer" aria-label="LinkedIn" class="social-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
              <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
            </svg>
          </a>
          <a href="https://twitter.com/YOUR_HANDLE" target="_blank" rel="noopener noreferrer" aria-label="X (Twitter)" class="social-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
              <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
            </svg>
          </a>
        </div>
      </header>

      <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
        {move || {
          posts.get().map(|posts| {
            let query = search_ctx.query.get().to_lowercase();
            let filtered_posts: Vec<PostSummary> = if query.is_empty() {
              posts
            } else {
              posts.into_iter().filter(|post| {
                post.metadata.title.to_lowercase().contains(&query) ||
                post.metadata.description.to_lowercase().contains(&query) ||
                post.metadata.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
              }).collect()
            };

            if filtered_posts.is_empty() {
              view! {
                <div class="no-posts">
                  <p>"No posts found matching your search."</p>
                </div>
              }.into_any()
            } else {
              let total_posts = filtered_posts.len();
              let total_pages = (total_posts + posts_per_page - 1) / posts_per_page;
              let current = search_ctx.current_page.get();
              let start_idx = (current - 1) * posts_per_page;
              let paginated_posts: Vec<PostSummary> = filtered_posts.into_iter().skip(start_idx).take(posts_per_page).collect();

              view! {
                <div class="posts-list">
                  {paginated_posts.iter().map(|post| view! { <PostSummaryCard post=post.clone() /> }).collect_view()}
                </div>
                {if total_pages > 1 {
                  view! {
                    <div class="pagination">
                      <button
                        class="pagination-btn"
                        disabled=move || search_ctx.current_page.get() == 1
                        on:click=move |_| search_ctx.current_page.update(|p| *p = (*p - 1).max(1))
                      >
                        "Previous"
                      </button>
                      <span class="pagination-info">
                        {format!("Page {} of {}", current, total_pages)}
                      </span>
                      <button
                        class="pagination-btn"
                        disabled=move || search_ctx.current_page.get() >= total_pages
                        on:click=move |_| search_ctx.current_page.update(|p| *p = (*p + 1).min(total_pages))
                      >
                        "Next"
                      </button>
                    </div>
                  }.into_any()
                } else {
                  view! { <div></div> }.into_any()
                }}
              }.into_any()
            }
          })
        }}
      </Suspense>
    </div>
  }
}

#[component]
fn PostPage() -> impl IntoView {
  let params = leptos_router::hooks::use_params_map();
  let slug = move || params.read().get("slug").unwrap_or_default();

  let post = Resource::new(
    move || slug(),
    |slug| async move { get_post_by_slug(slug).await.ok().flatten() },
  );

  view! {
    <div class="container">
      <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
        {move || {
          post.get().map(|post_opt| {
            match post_opt {
              Some(post) => {
                let title = post.metadata.title.clone();
                let date = post.metadata.date.clone();
                let tags = post.metadata.tags.clone();
                let content = post.content.clone();
                let description = post.metadata.description.clone();
                let preview = post.preview.clone();
                let page_title = format!("{} - Your Blog", title);
                let og_url = format!("https://your-domain.com/posts/{}", post.slug);

                // Combine description and preview for better SEO
                let full_description = if description.is_empty() {
                  preview.clone()
                } else if preview.is_empty() {
                  description.clone()
                } else {
                  format!("{} {}", description, preview)
                };

                // Structured Data (JSON-LD) for SEO
                let schema_json = format!(
                  r#"{{
                    "@context": "https://schema.org",
                    "@type": "BlogPosting",
                    "headline": "{}",
                    "description": "{}",
                    "datePublished": "{}",
                    "author": {{
                      "@type": "Person",
                      "name": "Your Name",
                      "url": "https://your-domain.com/about"
                    }},
                    "publisher": {{
                      "@type": "Organization",
                      "name": "Your Blog Name",
                      "url": "https://your-domain.com"
                    }},
                    "mainEntityOfPage": {{
                      "@type": "WebPage",
                      "@id": "{}"
                    }},
                    "keywords": "{}"
                  }}"#,
                  title.replace('"', "\\\""),
                  description.replace('"', "\\\""),
                  date,
                  og_url,
                  tags.join(", ")
                );

                view! {
                  <Title text=page_title.clone()/>
                  <Meta name="description" content=full_description.clone()/>
                  <Meta name="keywords" content=tags.join(", ")/>
                  <Meta property="og:type" content="article"/>
                  <Meta property="og:title" content=title.clone()/>
                  <Meta property="og:description" content=full_description.clone()/>
                  <Meta property="og:url" content=og_url.clone()/>
                  <Meta property="og:site_name" content="Your Blog Name"/>
                  <Meta property="article:published_time" content=date.clone()/>
                  <Meta property="article:author" content="Your Name"/>
                  <Meta property="article:tag" content=tags.join(", ")/>
                  <Meta name="twitter:card" content="summary"/>
                  <Meta name="twitter:title" content=title.clone()/>
                  <Meta name="twitter:description" content=full_description/>
                  <Meta name="twitter:url" content=og_url/>

                  <article class="post-detail">
                    <script type="application/ld+json" inner_html=schema_json></script>
                    <header>
                      <h1>{title}</h1>
                      <div class="post-meta">
                        <span class="date">{date}</span>
                        <span class="tags">
                          {tags.iter().map(|tag| {
                            let tag_text = tag.clone();
                            let tag_link = tag.clone();
                            view! {
                              <a href=format!("/tags/{}", tag_link) class="tag">{tag_text}</a>
                            }
                          }).collect_view()}
                        </span>
                      </div>
                    </header>
                    <div class="post-content" inner_html=content></div>
                    <a href="/" class="back-link">"← Back to posts"</a>

                    // Comments section
                    <div class="comments-section">
                      <h2>"Comments"</h2>
                      <Giscus/>
                    </div>
                  </article>
                }.into_any()
              },
              None => view! {
                <Title text="Post Not Found - Your Blog"/>
                <Meta name="description" content="The requested blog post could not be found"/>

                <div class="not-found">
                  <h1>"Post Not Found"</h1>
                  <p>"The post you are looking for does not exist."</p>
                  <a href="/">"← Back to posts"</a>
                </div>
              }.into_any(),
            }
          })
        }}
      </Suspense>
    </div>
  }
}

#[component]
fn TagPage() -> impl IntoView {
  let params = leptos_router::hooks::use_params_map();
  let tag = move || params.read().get("tag").unwrap_or_default();

  let posts = Resource::new(
    move || tag(),
    |tag| async move { get_posts_by_tag_summaries(tag).await.unwrap_or_default() },
  );

  view! {
    {move || {
      let current_tag = tag();
      let page_title = format!("Posts tagged with '{}' - Your Blog", current_tag);
      let description = format!("All blog posts tagged with '{}' on Your Blog", current_tag);

      view! {
        <Title text=page_title/>
        <Meta name="description" content=description/>
        <Meta name="keywords" content=format!("{}, programming, technology", current_tag)/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content=format!("Posts tagged with '{}'", current_tag)/>
        <Meta property="og:description" content=format!("All blog posts tagged with '{}' on Your Blog", current_tag)/>
        <Meta property="og:site_name" content="Your Blog Name"/>
        <Meta name="twitter:card" content="summary"/>
        <Meta name="twitter:title" content=format!("Posts tagged with '{}'", current_tag)/>
        <Meta name="twitter:description" content=format!("All blog posts tagged with '{}' on Your Blog", current_tag)/>
      }
    }}

    <div class="container">
      <header class="tag-header">
        <h1>"Posts tagged with: " {move || tag()}</h1>
        <a href="/" class="back-link">"← All posts"</a>
      </header>

      <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
        {move || {
          posts.get().map(|posts| {
            if posts.is_empty() {
              view! {
                <div class="no-posts">
                  <p>"No posts found with this tag."</p>
                </div>
              }.into_any()
            } else {
              view! {
                <div class="posts-list">
                  {posts.iter().map(|post| view! { <PostSummaryCard post=post.clone() /> }).collect_view()}
                </div>
              }.into_any()
            }
          })
        }}
      </Suspense>
    </div>
  }
}

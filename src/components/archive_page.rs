use crate::posts::PostSummary;
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use std::collections::BTreeMap;

#[server]
pub async fn get_posts_for_archive() -> Result<Vec<PostSummary>, ServerFnError> {
  Ok(crate::posts::load_post_summaries())
}

#[component]
pub fn ArchivePage() -> impl IntoView {
  let posts = Resource::new(
    || (),
    |_| async move { get_posts_for_archive().await.unwrap_or_default() },
  );

  view! {
    <Title text="Archive - Your Blog"/>
    <Meta name="description" content="Archive of all blog posts organized by year"/>
    <Meta name="keywords" content="archive, blog posts, programming, technology"/>
    <Meta property="og:type" content="website"/>
    <Meta property="og:title" content="Archive - Your Blog"/>
    <Meta property="og:description" content="Archive of all blog posts organized by year"/>
    <Meta property="og:url" content="https://your-domain.com/archive"/>
    <Meta property="og:site_name" content="Your Blog Name"/>
    <Meta name="twitter:card" content="summary"/>
    <Meta name="twitter:title" content="Archive - Your Blog"/>
    <Meta name="twitter:description" content="Archive of all blog posts organized by year"/>
    <link rel="canonical" href="https://your-domain.com/archive"/>

    <div class="container">
      <div class="archive-page">
        <header class="archive-header">
          <h1>
            "Archive "
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
              <line x1="8" y1="21" x2="16" y2="21"></line>
              <line x1="12" y1="17" x2="12" y2="21"></line>
            </svg>
          </h1>
        </header>

        <Suspense fallback=move || view! { <p>"Loading archive..."</p> }>
          {move || {
            posts.get().map(|posts| {
              let posts_by_year = create_posts_by_year(&posts);

              view! {
                <div class="archive-content">
                  {posts_by_year.into_iter().map(|(year, year_posts)| {
                    view! {
                      <section class="year-section">
                        <h2 class="year-title">{year}</h2>
                        <div class="posts-list">
                          {year_posts.into_iter().map(|post| {
                            let date_parts: Vec<&str> = post.metadata.date.split('-').collect();
                            let month = if date_parts.len() >= 2 {
                              get_month_name(date_parts[1])
                            } else {
                              "Unknown"
                            };

                            view! {
                              <article class="archive-post">
                                <div class="post-date">
                                  <span class="month">{month}</span>
                                </div>
                                <div class="post-info">
                                  <h3><a href=format!("/posts/{}", post.slug)>{post.metadata.title}</a></h3>
                                  <p class="post-description">{post.metadata.description}</p>
                                </div>
                              </article>
                            }
                          }).collect_view()}
                        </div>
                      </section>
                    }
                  }).collect_view()}
                </div>
              }
            })
          }}
        </Suspense>

        <a href="/" class="back-link">"← Back to posts"</a>
      </div>
    </div>
  }
}
fn create_posts_by_year(posts: &[PostSummary]) -> BTreeMap<String, Vec<PostSummary>> {
  let mut posts_by_year: BTreeMap<String, Vec<PostSummary>> = BTreeMap::new();

  for post in posts {
    let year = post
      .metadata
      .date
      .split('-')
      .next()
      .unwrap_or("Unknown")
      .to_string();
    posts_by_year
      .entry(year)
      .or_insert_with(Vec::new)
      .push(post.clone());
  }

  // Sort by year descending
  posts_by_year
    .into_iter()
    .collect::<BTreeMap<_, _>>()
    .into_iter()
    .rev()
    .collect()
}

fn get_month_name(month_num: &str) -> &'static str {
  match month_num {
    "01" => "January",
    "02" => "February",
    "03" => "March",
    "04" => "April",
    "05" => "May",
    "06" => "June",
    "07" => "July",
    "08" => "August",
    "09" => "September",
    "10" => "October",
    "11" => "November",
    "12" => "December",
    _ => "Unknown",
  }
}

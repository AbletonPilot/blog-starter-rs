use crate::posts::PostSummary;
use leptos::prelude::*;

#[component]
pub fn PostSummaryCard(post: PostSummary) -> impl IntoView {
  let slug = post.slug.clone();
  let title = post.metadata.title.clone();
  let date = post.metadata.date.clone();
  let description = post.metadata.description.clone();
  let tags = post.metadata.tags.clone();
  let thumbnail = post.thumbnail.clone();
  let has_thumbnail = thumbnail.is_some();

  view! {
    <article class=move || if has_thumbnail { "post-card has-thumbnail" } else { "post-card" }>
      {thumbnail.map(|thumb_url| view! {
        <div class="post-thumbnail">
          <a href=format!("/posts/{}", slug.clone())>
            <img src=thumb_url alt=title.clone() loading="lazy"/>
          </a>
        </div>
      })}
      <div class="post-content">
        <h2><a href=format!("/posts/{}", slug)>{title}</a></h2>
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
        <p class="description">{description}</p>
      </div>
    </article>
  }
}

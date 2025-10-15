use crate::posts::PostSummary;
use leptos::prelude::*;

#[component]
pub fn PostSummaryCard(post: PostSummary) -> impl IntoView {
  let slug = post.slug.clone();
  let title = post.metadata.title.clone();
  let date = post.metadata.date.clone();
  let description = post.metadata.description.clone();
  let tags = post.metadata.tags.clone();

  view! {
    <article class="post-card">
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
    </article>
  }
}

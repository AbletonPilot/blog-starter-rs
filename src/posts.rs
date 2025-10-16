use serde::{Deserialize, Serialize};

/// Extract the first image URL from markdown content
fn extract_thumbnail(markdown: &str) -> Option<String> {
  for line in markdown.lines() {
    let trimmed = line.trim();
    // Match markdown image: ![alt](url)
    if trimmed.starts_with("!") && trimmed.contains("](") {
      if let Some(start) = trimmed.find("](") {
        if let Some(end) = trimmed[start + 2..].find(")") {
          let url = &trimmed[start + 2..start + 2 + end];
          return Some(url.to_string());
        }
      }
    }
    // Match HTML img tag: <img src="url"
    if trimmed.contains("<img") && trimmed.contains("src=") {
      if let Some(start) = trimmed.find("src=\"") {
        if let Some(end) = trimmed[start + 5..].find("\"") {
          let url = &trimmed[start + 5..start + 5 + end];
          return Some(url.to_string());
        }
      }
    }
  }
  None
}

/// Extract plain text from markdown content (removes formatting)
fn extract_text_preview(markdown: &str, max_chars: usize) -> String {
  let mut result = String::new();
  let mut in_code_block = false;
  let mut chars_count = 0;
  
  for line in markdown.lines() {
    // Skip code blocks
    if line.trim().starts_with("```") {
      in_code_block = !in_code_block;
      continue;
    }
    if in_code_block {
      continue;
    }
    
    // Skip images
    if line.trim().starts_with("!") && line.contains("](") {
      continue;
    }
    
    // Skip empty lines
    let trimmed = line.trim();
    if trimmed.is_empty() {
      continue;
    }
    
    // Remove markdown formatting
    let clean_line = trimmed
      .trim_start_matches(|c| c == '#' || c == ' ')
      .replace("**", "")
      .replace("*", "")
      .replace("__", "")
      .replace("_", "");
    
    // Add to result
    if !result.is_empty() {
      result.push(' ');
    }
    result.push_str(&clean_line);
    
    chars_count = result.chars().count();
    if chars_count >= max_chars {
      break;
    }
  }
  
  // Truncate to max_chars
  if chars_count > max_chars {
    result.chars().take(max_chars).collect::<String>() + "..."
  } else {
    result
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostMetadata {
  pub title: String,
  pub date: String,
  pub tags: Vec<String>,
  pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
  pub slug: String,
  pub metadata: PostMetadata,
  pub content: String,
  pub preview: String, // Text preview from content for SEO
  pub thumbnail: Option<String>, // First image URL for thumbnails
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostSummary {
  pub slug: String,
  pub metadata: PostMetadata,
  pub thumbnail: Option<String>, // First image URL for thumbnails
}

impl Post {
  pub fn from_markdown(slug: String, markdown_content: &str) -> Result<Self, String> {
    let parts: Vec<&str> = markdown_content.split("---").collect();

    if parts.len() < 3 {
      return Err("Invalid markdown format: missing front matter".to_string());
    }

    let front_matter = parts[1].trim();
    let content = parts[2..].join("---").trim().to_string();

    let metadata: PostMetadata = serde_yaml::from_str(front_matter)
      .map_err(|e| format!("Failed to parse front matter: {}", e))?;

    let html_content = markdown_to_html(&content);
    
    // Extract preview text from markdown content (max 160 chars for SEO)
    let preview = extract_text_preview(&content, 160);
    
    // Extract first image URL for thumbnail
    let thumbnail = extract_thumbnail(&content);

    Ok(Post {
      slug,
      metadata,
      content: html_content,
      preview,
      thumbnail,
    })
  }
}

fn markdown_to_html(markdown: &str) -> String {
  use pulldown_cmark::{html, Options, Parser};

  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_TASKLISTS);
  options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
  options.insert(Options::ENABLE_SMART_PUNCTUATION);
  options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
  options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

  #[cfg(feature = "ssr")]
  {
    use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};
    use syntect::highlighting::ThemeSet;
    use syntect::html::highlighted_html_for_string;
    use syntect::parsing::SyntaxSet;

    let parser = Parser::new_ext(markdown, options);
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    // Use a neutral theme that works in both light and dark modes // base16-ocean.dark, base16-ocean.light, InspiredGitHub, Solarized (dark), Solarized (light)
    let _theme = &ts.themes["Solarized (dark)"];

    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();

    let events: Vec<Event> = parser
      .filter_map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
          in_code_block = true;
          code_block_lang = lang.to_string();
          code_block_content.clear();
          None
        }
        Event::End(TagEnd::CodeBlock) if in_code_block => {
          in_code_block = false;
          let syntax = ss
            .find_syntax_by_token(&code_block_lang)
            .unwrap_or_else(|| ss.find_syntax_plain_text());

          // Generate dark theme version // base16-ocean.dark, base16-ocean.light, InspiredGitHub, Solarized (dark), Solarized (light)
          let dark_theme = &ts.themes["Solarized (dark)"];
          let mut dark_html = highlighted_html_for_string(&code_block_content, &ss, syntax, dark_theme)
            .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", code_block_content));
          dark_html = dark_html.replace(
            r#"<pre style="background-color:"#,
            r#"<pre class="code-dark" style="padding:1rem;border-radius:8px;overflow-x:auto;background-color:"#,
          );

          // Generate light theme version // base16-ocean.dark, base16-ocean.light, InspiredGitHub, Solarized (dark), Solarized (light)
          let light_theme = &ts.themes["Solarized (light)"];
          let mut light_html = highlighted_html_for_string(&code_block_content, &ss, syntax, light_theme)
            .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", code_block_content));
          light_html = light_html.replace(
            r#"<pre style="background-color:"#,
            r#"<pre class="code-light" style="padding:1rem;border-radius:8px;overflow-x:auto;background-color:"#,
          );
          
          // Combine both versions
          let combined_html = format!("{}{}", dark_html, light_html);
          Some(Event::Html(combined_html.into()))
        }
        Event::Text(text) if in_code_block => {
          code_block_content.push_str(&text);
          None
        }
        // Add target="_blank" and rel attributes to external links
        Event::Start(Tag::Link {
          link_type,
          dest_url,
          title,
          id,
        }) => {
          if dest_url.starts_with("http://") || dest_url.starts_with("https://") {
            let modified_html = format!(
              r#"<a href="{}" target="_blank" rel="noopener noreferrer"{}>"#,
              dest_url,
              if title.is_empty() {
                String::new()
              } else {
                format!(r#" title="{}""#, title)
              }
            );
            Some(Event::Html(modified_html.into()))
          } else {
            Some(Event::Start(Tag::Link {
              link_type,
              dest_url,
              title,
              id,
            }))
          }
        }
        Event::End(TagEnd::Link) => Some(Event::Html("</a>".into())),
        _ => Some(event),
      })
      .collect();

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    html_output
  }

  #[cfg(not(feature = "ssr"))]
  {
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
  }
}

#[cfg(feature = "ssr")]
pub fn load_posts() -> Vec<Post> {
  use std::fs;
  use std::path::Path;
  use std::sync::{LazyLock, Mutex};

  static POSTS_CACHE: LazyLock<Mutex<Option<Vec<Post>>>> = LazyLock::new(|| Mutex::new(None));

  // Check if posts are already cached
  if let Ok(cache) = POSTS_CACHE.lock() {
    if let Some(ref cached_posts) = *cache {
      return cached_posts.clone();
    }
  }

  let posts_dir = Path::new("posts");

  if !posts_dir.exists() {
    eprintln!("Posts directory does not exist");
    return vec![];
  }

  let mut posts = Vec::new();

  if let Ok(entries) = fs::read_dir(posts_dir) {
    for entry in entries.flatten() {
      let path = entry.path();

      if path.extension().and_then(|s| s.to_str()) == Some("md") {
        if let Ok(content) = fs::read_to_string(&path) {
          let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

          match Post::from_markdown(filename, &content) {
            Ok(post) => posts.push(post),
            Err(e) => eprintln!("Error parsing post {}: {}", path.display(), e),
          }
        }
      }
    }
  }

  posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

  // Cache the posts
  if let Ok(mut cache) = POSTS_CACHE.lock() {
    *cache = Some(posts.clone());
  }

  posts
}

#[cfg(feature = "ssr")]
pub fn load_post_summaries() -> Vec<PostSummary> {
  load_posts()
    .into_iter()
    .map(|post| PostSummary {
      slug: post.slug,
      metadata: post.metadata,
      thumbnail: post.thumbnail,
    })
    .collect()
}

#[cfg(not(feature = "ssr"))]
pub fn load_post_summaries() -> Vec<PostSummary> {
  vec![]
}

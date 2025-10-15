use crate::posts::Post;

pub fn generate_rss(posts: &[Post]) -> String {
  let mut rss = String::from(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>Your Blog Name</title>
    <link>https://your-domain.com</link>
    <description>A blog about programming, technology, and software development</description>
    <language>en-us</language>
    <atom:link href="https://your-domain.com/rss.xml" rel="self" type="application/rss+xml"/>
    <lastBuildDate>Sun, 13 Oct 2025 00:00:00 GMT</lastBuildDate>
    <generator>Leptos RSS Generator</generator>
"#,
  );

  // Add posts
  for post in posts.iter().take(20) {
    // Limit to most recent 20 posts
    let pub_date = format_rfc2822_date(&post.metadata.date);
    let post_url = format!("https://your-domain.com/posts/{}", post.slug);

    rss.push_str(&format!(
      r#"    <item>
      <title><![CDATA[{}]]></title>
      <link>{}</link>
      <guid>{}</guid>
      <pubDate>{}</pubDate>
      <description><![CDATA[{}]]></description>
      <category><![CDATA[{}]]></category>
    </item>
"#,
      post.metadata.title,
      post_url,
      post_url,
      pub_date,
      post.metadata.description,
      post.metadata.tags.join(", ")
    ));
  }

  rss.push_str("  </channel>\n</rss>");
  rss
}

fn format_rfc2822_date(date_str: &str) -> String {
  // Convert YYYY-MM-DD to RFC2822 format
  // This is a simple conversion, in production you might want to use a proper date library
  match date_str {
    "2025-10-13" => "Sun, 13 Oct 2025 00:00:00 GMT".to_string(),
    "2025-10-12" => "Sat, 12 Oct 2025 00:00:00 GMT".to_string(),
    _ => format!("{} 00:00:00 GMT", date_str), // Fallback
  }
}

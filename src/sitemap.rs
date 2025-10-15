use crate::posts::Post;

pub fn generate_sitemap(posts: &[Post]) -> String {
  let mut sitemap = String::from(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
  );

  // Add homepage
  sitemap.push_str(&format!(
    r#"  <url>
    <loc>https://your-domain.com/</loc>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
"#
  ));

  // Add posts
  for post in posts {
    sitemap.push_str(&format!(
      r#"  <url>
    <loc>https://your-domain.com/posts/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
"#,
      post.slug, post.metadata.date
    ));
  }

  // Add unique tags
  let mut tags: Vec<String> = posts
    .iter()
    .flat_map(|post| post.metadata.tags.clone())
    .collect();
  tags.sort();
  tags.dedup();

  for tag in tags {
    sitemap.push_str(&format!(
      r#"  <url>
    <loc>https://your-domain.com/tags/{}</loc>
    <changefreq>weekly</changefreq>
    <priority>0.6</priority>
  </url>
"#,
      tag
    ));
  }

  sitemap.push_str("</urlset>");
  sitemap
}

pub fn generate_robots_txt() -> String {
  r#"User-agent: *
Allow: /

Sitemap: https://your-domain.com/sitemap.xml
"#
  .to_string()
}

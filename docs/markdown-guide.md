# Markdown Guide for Blog Posts

This guide provides a comprehensive reference for writing blog posts using Markdown syntax.

## Front Matter

Every blog post must start with front matter (YAML format):

```yaml
---
title: "Your Post Title"
date: 2025-10-14
tags: [tag1, tag2, tag3]
description: "A brief description of your post (used for SEO and search)"
---
```

**Guidelines:**
- `title`: Clear, descriptive title
- `date`: Format YYYY-MM-DD
- `tags`: Lowercase, relevant keywords (2-5 tags recommended)
- `description`: 1-2 sentences summarizing the post (important for search!)

---

## 1. Headings

```markdown
# H1 Heading (avoid - reserved for post title)
## H2 Section Heading
### H3 Subsection Heading
#### H4 Minor Heading
##### H5 Smallest Heading
###### H6 Smallest Heading
```

**Best Practice**: Start with H2 (##) for main sections.

---

## 2. Text Formatting

```markdown
**Bold text** for emphasis
*Italic text* for subtle emphasis
***Bold and Italic*** for strong emphasis
~~Strikethrough~~ for corrections
`inline code` for code snippets or technical terms
```

**Examples:**
- **Important**: This is critical information
- *Note*: A side comment
- Use `cargo build` to compile
- ~~Old information~~ Updated information

---

## 3. Lists

### Unordered Lists
```markdown
- First item
- Second item
  - Nested item 2.1
  - Nested item 2.2
    - Deeper nesting 2.2.1
- Third item
```

### Ordered Lists
```markdown
1. First step
2. Second step
   1. Sub-step 2.1
   2. Sub-step 2.2
3. Third step
```

### Task Lists (Checklists)
```markdown
- [x] Completed task
- [ ] Pending task
- [ ] Another pending task
```

---

## 4. Links

```markdown
[Link text](https://example.com)
[Link with title](https://example.com "Hover text")
<https://example.com> (automatic link)

[Reference-style link][1]
[1]: https://example.com "Reference"
```

**Examples:**
- [Rust Official Site](https://www.rust-lang.org/)
- Check out <https://github.com>

---

## 5. Images

### Local Images
```markdown
![Alt text](/path/to/image.png "Optional title")
![Local image example](/2025-10-14/screenshot.png)
```

**Organization:**
- Create folder: `public/YYYY-MM-DD/`
- Place images: `public/2025-10-14/image.png`
- Reference: `![Description](/2025-10-14/image.png)`

### External Images
```markdown
![External image](https://example.com/image.jpg)
```

**Tips:**
- Optimize images before uploading (compress to 200-500KB)
- Use descriptive alt text for accessibility
- Consider using WebP format for better compression

---

## 6. Videos

### Local Video (HTML5)
```html
<video controls style="width: 100%; height: auto;">
  <source src="/2025-10-14/video.mp4" type="video/mp4">
  Your browser does not support the video tag.
</video>
```

### YouTube Embed
```html
<div style="position: relative; padding-bottom: 56.25%; height: 0; overflow: hidden;">
  <iframe 
    style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;" 
    src="https://www.youtube.com/embed/VIDEO_ID" 
    frameborder="0" 
    allowfullscreen>
  </iframe>
</div>
```

**Recommendation**: Use YouTube for large videos to save storage space.

---

## 7. Code Blocks

### Inline Code
```markdown
Use `cargo build` to compile your project.
```

### Code Blocks with Syntax Highlighting

````markdown
```rust
fn main() {
    println!("Hello, World!");
}
```
````

**Supported Languages:**
- `rust`, `javascript`, `python`, `sql`, `bash`, `html`, `css`, `json`, `yaml`, etc.

### Examples

**Rust:**
````markdown
```rust
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```
````

**JavaScript:**
````markdown
```javascript
const quickSort = (arr) => {
  if (arr.length <= 1) return arr;
  const pivot = arr[Math.floor(arr.length / 2)];
  const left = arr.filter(x => x < pivot);
  const right = arr.filter(x => x > pivot);
  return [...quickSort(left), pivot, ...quickSort(right)];
};
```
````

**Python:**
````markdown
```python
def merge_sort(arr):
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    return merge(left, right)
```
````

---

## 8. Blockquotes

```markdown
> This is a simple quote.

> Multi-line quote
> continues here.

> **Tip**: You can use **formatting** inside quotes!
>
> ```rust
> // Even code blocks!
> fn example() {}
> ```
```

**Use Cases:**
- Highlight important information
- Quote external sources
- Add tips or warnings

**Example:**
> **💡 Pro Tip**: Always test your code locally with `cargo leptos watch` before pushing!

---

## 9. Tables

### Basic Table
```markdown
| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| Row 1    | Data     | Value    |
| Row 2    | Data     | Value    |
```

### Aligned Tables
```markdown
| Left Aligned | Center Aligned | Right Aligned |
|:-------------|:--------------:|--------------:|
| Left         | Center         | Right         |
| Text         | Text           | Text          |
```

**Example:**
| Language   | Type       | Difficulty |
|:-----------|:----------:|-----------:|
| Rust       | Compiled   | High       |
| Python     | Interpreted| Low        |
| JavaScript | Interpreted| Medium     |

---

## 10. Horizontal Rules

```markdown
---
***
___
```

Use horizontal rules to separate major sections.

---

## 11. Special Characters

Escape special characters with backslash:

```markdown
\* Asterisk
\_ Underscore
\# Hash
\[ Bracket
\] Bracket
\( Parenthesis
\) Parenthesis
```

---

## 12. Emojis

You can use emojis directly:

```markdown
😀 💻 🚀 ⭐ 🎉 👍 ❤️ 🔥 📝 ✨
```

**Common Use Cases:**
- 💡 Tips
- ⚠️ Warnings
- ✅ Completed
- ❌ Error
- 🚀 New feature

---

## Best Practices

### Content Structure
1. Start with clear introduction
2. Use H2 (##) for main sections
3. Keep paragraphs short (3-5 sentences)
4. Use lists for easier scanning
5. Add code examples for technical content

### SEO & Discoverability
- Write descriptive `title` and `description`
- Use relevant `tags` (2-5 tags)
- Include keywords naturally in content
- Use descriptive image alt text

### Code Examples
- Always specify language for syntax highlighting
- Keep examples concise and focused
- Add comments for complex code
- Test code before publishing

### Images & Media
- Optimize images (target: 200-500KB)
- Use descriptive filenames
- Organize by date: `public/YYYY-MM-DD/`
- Consider external CDN for large files

### Formatting
- Use **bold** for emphasis, not CAPS
- Break long paragraphs into shorter ones
- Use lists for multiple items
- Add whitespace for readability

---

## Publishing Workflow

1. **Create markdown file**: `posts/YYYY-MM-DD-slug.md`
2. **Add front matter** (title, date, tags, description)
3. **Write content** using this guide
4. **Add media files** to `public/YYYY-MM-DD/`
5. **Test locally**: `cargo leptos watch` → visit `localhost:3000`
6. **Commit and push**:
   ```bash
   git add posts/ public/
   git commit -m "Add new post: slug-name"
   git push origin main
   ```
7. **Wait for deployment** (2-5 minutes)
8. **Verify** at your deployed site

---

## Example Post Template

```markdown
---
title: "Getting Started with Rust"
date: 2025-10-14
tags: [rust, programming, tutorial]
description: "A beginner-friendly guide to getting started with Rust programming language"
---

## Introduction

Brief introduction to your topic. Explain what readers will learn.

## Main Section 1

Content here with examples:

```rust
fn main() {
    println!("Hello, Rust!");
}
```

### Subsection

More detailed content.

## Main Section 2

Continue your content...

## Conclusion

Summarize key points and provide next steps.
```

---

## Quick Reference

| Element | Syntax |
|---------|--------|
| Bold | `**text**` |
| Italic | `*text*` |
| Code | `` `code` `` |
| Link | `[text](url)` |
| Image | `![alt](/path)` |
| Heading | `## text` |
| List | `- item` |
| Quote | `> text` |

---

**Questions?** Refer to this guide when writing posts!

**Happy blogging! 📝✨**

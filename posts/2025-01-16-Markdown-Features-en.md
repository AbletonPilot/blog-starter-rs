---
title: "Markdown Features Demo"
date: "2025-01-16"
description: "A showcase of all the Markdown features supported in this blog"
tags: ["markdown", "tutorial", "demo"]
---

# Markdown Features Demo

This post demonstrates all the Markdown features supported in this blog.

## Text Formatting

You can use **bold text**, *italic text*, or ***both***.

~~Strikethrough~~ is also supported.

## Lists

### Unordered Lists

- Item 1
- Item 2
  - Nested item 2.1
  - Nested item 2.2
- Item 3

### Ordered Lists

1. First item
2. Second item
3. Third item
   1. Nested item 3.1
   2. Nested item 3.2

## Code

### Inline Code

Use `inline code` for short snippets.

### Code Blocks

```rust
// Rust example with syntax highlighting
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("Fibonacci(10) = {}", fibonacci(10));
}
```

```javascript
// JavaScript example
function greet(name) {
  console.log(`Hello, ${name}!`);
}

greet('World');
```

## Blockquotes

> This is a blockquote.
> 
> It can span multiple lines.

> You can also nest blockquotes
> > Like this!

## Links

[Visit the Leptos website](https://leptos.dev)

[Relative link to documentation](../docs/markdown-guide.md)

## Tables

| Feature | Supported | Notes |
|---------|-----------|-------|
| Headers | ✅ | H1 through H6 |
| Lists | ✅ | Ordered and unordered |
| Code | ✅ | Inline and blocks |
| Tables | ✅ | Full support |
| Images | ✅ | Local and remote |

## Horizontal Rules

---

Content above the line

---

Content below the line

## Task Lists

- [x] Set up blog
- [x] Write first post
- [ ] Add custom styling
- [ ] Deploy to production

## Headings

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

## Emphasis

*Italic* or _italic_

**Bold** or __bold__

***Bold and italic*** or ___bold and italic___

## Escaping Characters

You can escape special characters: \*not italic\*, \`not code\`

## Conclusion

These are all the basic Markdown features you can use in your blog posts. Happy writing! 🚀

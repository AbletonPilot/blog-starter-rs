# Blog Starter RS

A modern, blazingly fast blog starter template built with Rust and Leptos.

🚀 **Quick Start**: Clone, customize, and deploy your own blog in minutes!

---

## ✨ Features

- 🌓 **Dark/Light Mode** - Beautiful themes that are easy on the eyes
- 🔍 **Search** - Find posts by title, description, or tags
- 💬 **Comments** - Giscus integration for GitHub Discussions-powered comments
- 📱 **Mobile Friendly** - Responsive design that looks great on any device
- 🎨 **Clean Design** - Focus on content, not clutter
- ⚡ **Blazingly Fast** - Built with Rust and Leptos for optimal performance
- 📝 **Markdown Posts** - Write your posts in Markdown with YAML frontmatter
- 🌍 **Multi-language Support** - Built-in support for multiple languages
- 🎯 **SEO Optimized** - Meta tags, Open Graph, and sitemap generation
- 📊 **RSS Feed** - Automatically generated RSS feed for your posts

---

## 🛠️ Tech Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Leptos](https://leptos.dev/)** - Reactive web framework
- **[Axum](https://github.com/tokio-rs/axum)** - Web server
- **[Pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark)** - Markdown parser
- **[Syntect](https://github.com/trishume/syntect)** - Syntax highlighting
- **[SCSS](https://sass-lang.com/)** - CSS preprocessor

---

## 🚀 Quick Start

### Prerequisites

#### 1. Install Rust

If you don't have Rust installed, install it using rustup:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions, then restart your terminal or run:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

For Windows, download and run [rustup-init.exe](https://rustup.rs/)

#### 2. Install cargo-leptos

```bash
cargo install cargo-leptos
```

#### 3. Add WebAssembly target

```bash
rustup target add wasm32-unknown-unknown
```

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/AbletonPilot/blog-starter-rs.git
cd blog-starter-rs
```

2. **Run in development mode**
```bash
cargo leptos watch
```

3. **Open your browser**
Navigate to `http://localhost:3000`

---

## 📝 Creating Posts

Posts are written in Markdown and stored in the `posts/` directory.

### Post Format

Create a file with the naming convention: `YYYY-MM-DD-title-lang.md`

Example: `2025-01-15-My-First-Post-en.md`

```markdown
---
title: "My First Post"
date: "2025-01-15"
description: "This is my first blog post"
tags: ["rust", "leptos", "web"]
---

Your content here...
```

### Supported Languages

- `en` - English
- `kr` - Korean
- Add more in `src/posts.rs`

---

## ⚙️ Configuration

### 1. Update Project Name

Edit `Cargo.toml`:
```toml
[package]
name = "your-blog-name"
```

**Important:** If you change the project name, you must also update `src/main.rs`:
```rust
// Change all occurrences of blog_starter_rs to your_blog_name
use blog_starter_rs::app::*;  // Change this
use blog_starter_rs::posts::load_posts;  // Change this
// ... etc
```

Note: Replace hyphens with underscores (e.g., `your-blog-name` becomes `your_blog_name`)

### 2. Configure Giscus Comments

1. Enable GitHub Discussions on your repository
2. Visit [giscus.app](https://giscus.app)
3. Follow the configuration steps
4. Update `src/components/giscus.rs` with your settings:

```rust
script.set_attribute("data-repo", "YOUR_USERNAME/YOUR_REPO").ok();
script.set_attribute("data-repo-id", "YOUR_REPO_ID").ok();
script.set_attribute("data-category", "YOUR_CATEGORY").ok();
script.set_attribute("data-category-id", "YOUR_CATEGORY_ID").ok();
```

See `docs/giscus-setup-guide.md` for detailed instructions.

### 3. Update About Page

Edit `src/components/about_page.rs` to customize your about page with your information.

### 4. Update Meta Tags

Search for hardcoded URLs in `src/` and update them:
- `https://YOUR_DOMAIN.com`
- Update site name, descriptions, etc.

---

## 🏗️ Building for Production

```bash
cargo leptos build --release
```

The output will be in `target/site/` and `target/release/`.

---

## 📦 Deployment

After building for production, you'll have:
- Binary: `target/release/blog-starter-rs`
- Static files: `target/site/`

### General Deployment Steps

1. **Build the project**
```bash
cargo leptos build --release
```

2. **Deploy both the binary and static files to your hosting platform**

3. **Set environment variable** (if needed)
```
LEPTOS_SITE_ADDR=0.0.0.0:3000
```

4. **Run the binary**
```bash
./target/release/blog-starter-rs
```

### Platform Options

You can deploy to any platform that supports:
- Rust binaries
- Static file serving
- Web servers

Popular options include:
- **VPS** (DigitalOcean, Linode, Vultr, etc.)
- **Cloud Platforms** (AWS, GCP, Azure)
- **Platform-as-a-Service** (Render, Railway, Fly.io, etc.)
- **Containerized Deployment** (Docker, Kubernetes)

---

## 📚 Documentation

- [Markdown Guide](docs/markdown-guide.md) - Learn about supported Markdown features
- [Giscus Setup Guide](docs/giscus-setup-guide.md) - Set up comments

---

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Submit pull requests
- Improve documentation

---

## 📄 License

MIT License - Feel free to use this template for your own blog!

**Important**: Please keep the copyright notice in the footer (`© 2025 AbletonPilot`) as credit to the original author. This is required by the MIT License.

---

## 🙏 Acknowledgments

- Original template by [AbletonPilot](https://github.com/AbletonPilot)

- [Leptos](https://leptos.dev/) - Amazing reactive web framework
- [Rust](https://www.rust-lang.org/) - The best systems programming language
- All the open-source projects that make this possible

---

## 📞 Support

If you find this project helpful, please ⭐ star it on GitHub!

For questions or issues, please open an issue on GitHub.

---

*Built with Rust 🦀 and a lot of coffee ☕*

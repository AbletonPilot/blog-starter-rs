use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn AboutPage() -> impl IntoView {
  view! {
    <Title text="About - Your Name"/>
    <Meta name="description" content="About Your Name, software developer and technology enthusiast"/>
    <Meta name="keywords" content="about, your-name, software developer, programming, technology"/>
    <Meta property="og:type" content="website"/>
    <Meta property="og:title" content="About - Your Name"/>
    <Meta property="og:description" content="About Your Name, software developer and technology enthusiast"/>
    <Meta property="og:url" content="https://your-domain.com/about"/>
    <Meta property="og:site_name" content="Your Blog Name"/>
    <Meta name="twitter:card" content="summary"/>
    <Meta name="twitter:title" content="About - Your Name"/>
    <Meta name="twitter:description" content="About Your Name, software developer and technology enthusiast"/>
    <link rel="canonical" href="https://your-domain.com/about"/>

    <div class="container">
      <article class="about-page">
        <header>
          <h1>"About Me"</h1>
        </header>

        <div class="about-content">
          <section class="intro">
            <h2>"Hello, I'm [Your Name]"</h2>
            <p>"I'm a software developer passionate about building efficient and elegant solutions.
                I enjoy exploring new technologies and sharing my experiences through this blog."</p>
            <p>"TODO: Replace this with your own introduction!"</p>
          </section>

          <section class="experience">
            <h2>"What I Do"</h2>
            <ul>
              <li>"Full-stack web development with modern frameworks"</li>
              <li>"System architecture and performance optimization"</li>
              <li>"Open source contributions and community involvement"</li>
              <li>"Technical writing and knowledge sharing"</li>
            </ul>
            <p>"TODO: Customize this list with your own skills and experience!"</p>
          </section>

          <section class="technologies">
            <h2>"Technologies I Work With"</h2>
            <div class="tech-list">
              <span class="tech-item">"Rust"</span>
              <span class="tech-item">"Leptos"</span>
              <span class="tech-item">"JavaScript/TypeScript"</span>
              <span class="tech-item">"React"</span>
              <span class="tech-item">"Node.js"</span>
              <span class="tech-item">"PostgreSQL"</span>
              <span class="tech-item">"Add your own!"</span>
            </div>
            <p>"TODO: Replace with technologies you actually use!"</p>
          </section>

          <section class="blog-info">
            <h2>"About This Blog"</h2>
            <p>"This blog is built with Rust and Leptos, showcasing modern web development techniques.
               Here I share my thoughts on programming, technology trends, and lessons learned from 
               various projects."</p>
            <p>"TODO: Customize this section to describe what your blog is about!"</p>
          </section>

          <section class="connect">
            <h2>"Let's Connect"</h2>
            <p>"Feel free to reach out if you'd like to discuss technology, collaborate on projects,
               or just say hello!"</p>
            <div class="social-links">
              <a href="https://github.com/YOUR_USERNAME" target="_blank" rel="noopener noreferrer">"GitHub"</a>
              <a href="https://linkedin.com/in/YOUR_PROFILE" target="_blank" rel="noopener noreferrer">"LinkedIn"</a>
              <a href="https://twitter.com/YOUR_HANDLE" target="_blank" rel="noopener noreferrer">"Twitter"</a>
            </div>
            <p>"TODO: Update these links with your actual social media profiles!"</p>
          </section>
        </div>

        <a href="/" class="back-link">"← Back to posts"</a>
      </article>
    </div>
  }
}

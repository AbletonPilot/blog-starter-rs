# Giscus Setup Guide

This guide will help you set up GitHub Discussions and Giscus for blog comments.

## Step 1: Enable GitHub Discussions

1. Go to your GitHub repository: https://github.com/YOUR_USERNAME/YOUR_REPO
2. Click on **Settings** tab
3. Scroll down to **Features** section
4. Check the box for **Discussions**
5. Click **Set up discussions** if prompted

## Step 2: Create Announcements Category

1. Go to the **Discussions** tab in your repository
2. Click on **Categories** (or the gear icon)
3. Look for the **Announcements** category
4. If it doesn't exist, create a new category:
   - Name: `Announcements`
   - Description: `Blog post comments powered by Giscus`
   - Discussion format: `Announcement`
5. Make sure only maintainers can create new discussions in this category

## Step 3: Configure Giscus

1. Visit https://giscus.app
2. Fill in the configuration:

### Repository
```
YOUR_USERNAME/YOUR_REPO
```

### Page ↔️ Discussions Mapping
- Choose: **Discussion title contains page `pathname`**
- Or: **Discussion title contains page `<meta>` tag**

### Discussion Category
- Choose: **Announcements**
- ✅ Only search for discussions in this category

### Features
- ✅ Enable reactions for the main post
- Choose position: **bottom** (comments below post)

### Theme
- Choose: **Preferred color scheme** (for automatic dark mode)

### Language
- Choose: **한국어 (Korean)**

## Step 4: Copy Configuration

After configuring, giscus.app will generate a script tag like this:

```html
<script src="https://giscus.app/client.js"
        data-repo="YOUR_USERNAME/YOUR_REPO"
        data-repo-id="R_kgDO..." 
        data-category="Announcements"
        data-category-id="DIC_kwDO..."
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="ko"
        crossorigin="anonymous"
        async>
</script>
```

## Step 5: Update giscus.rs

Copy the `data-repo-id` and `data-category-id` values from the generated script and update them in:

**File:** `src/components/giscus.rs`

Replace:
- `YOUR_REPO_ID` with the actual `data-repo-id` value
- `YOUR_CATEGORY_ID` with the actual `data-category-id` value

Example:
```rust
script.set_attribute("data-repo-id", "R_kgDOxxxxxxxx").ok();
script.set_attribute("data-category-id", "DIC_kwDOxxxxxxxx").ok();
```

## Step 6: Test Locally

1. Build and run your blog locally:
   ```bash
   cargo leptos watch
   ```

2. Navigate to any blog post
3. Scroll down to the comments section
4. You should see the Giscus widget load
5. Try posting a test comment (you'll need to be logged into GitHub)

## Step 7: Verify on GitHub

1. Go to your repository's Discussions tab
2. Look in the Announcements category
3. You should see a new discussion created for the blog post you commented on
4. The discussion title should match the post's pathname

## Troubleshooting

### Giscus widget doesn't appear
- Check browser console for errors
- Verify repository is public
- Ensure Discussions are enabled
- Check that repo-id and category-id are correct

### Comments don't save
- Make sure you're logged into GitHub
- Check repository permissions
- Verify the Announcements category allows discussions

### Dark mode doesn't work
- Theme is set to `preferred_color_scheme`
- Should automatically follow browser/system preference
- Check body class toggle in your app

## Additional Configuration

### Moderation
- You can moderate comments directly in GitHub Discussions
- Edit, delete, or lock discussions as needed
- Set up repository moderation settings

### Notifications
- Configure GitHub notification settings
- You'll receive notifications for new comments
- Can set up email alerts for new discussions

## Security Notes

- Giscus uses GitHub OAuth for authentication
- All comments are stored in GitHub Discussions
- Comments are tied to GitHub accounts
- Repository must be public for Giscus to work

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based web server project for Eternal Cataclysm Studios that serves a modeling portfolio website. The project supports both development server mode and static site generation for GitHub Pages deployment.

## Technology Stack

- **Language**: Rust (2024 edition)
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio 1.0
- **HTTP Utilities**: Tower 0.4, Tower-HTTP 0.5
- **Template System**: Custom HTML template replacement system

## Common Commands

### Development
- `cargo run --bin server` - Start the development server (runs on http://127.0.0.1:3000)
- `cargo run --bin generate-static` - Generate static HTML files for GitHub Pages
- `cargo check` - Check code for compilation errors without building
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version

### Code Quality
- `cargo test` - Run all tests
- `cargo clippy` - Run the Rust linter
- `cargo fmt` - Format code according to Rust standards

## Project Structure

- `src/main.rs` - Development server with dynamic route handlers
- `src/generate_static.rs` - Static site generator for GitHub Pages
- `templates/` - HTML template files with placeholder substitution
  - `base.html` - Base template with `{{TITLE}}`, `{{CONTENT}}`, navigation, and footer
  - `index.html` - Home page content template
  - `styles.css` - Shared CSS styles for all pages
  - `global-images/` - Images used across multiple pages
  - `contact/contact.html` - Contact page with form
  - `modeling/modeling.html` - Unified modeling page with category dropdown
  - `modeling/{category}/` - Category-specific folders containing:
    - `images/` - Portfolio images for that category
    - `images/Links.txt` - Optional file mapping image names to URLs (format: `imagename,https://...`)
    - `subtitle.txt` - Optional custom subtitle text
- `docs/` - Generated static HTML files and assets for GitHub Pages
- `.github/workflows/deploy.yml` - GitHub Actions workflow for automated deployment

## Architecture

### Template System
The project uses a custom template replacement system:
- `base.html` provides the layout with `{{TITLE}}` and `{{CONTENT}}` placeholders
- `base.html` includes the site footer (appears on all pages automatically)
- `base.html` links to `styles.css` for consistent styling across all pages
- Page-specific templates are embedded into the base template
- Modeling page uses `{{CATEGORIES_JSON}}` placeholder for all category data

### Unified Modeling Page
- Single `/modeling/` route serves all categories
- Category dropdown selector to switch between headshots, groupshots, etc.
- All category data (images, subtitles, links) embedded as JSON in the page
- Images are clickable:
  - If `Links.txt` contains a mapping for the image, opens that URL
  - Otherwise, opens the image in a new tab

### Links.txt Format
Place in `templates/modeling/{category}/images/Links.txt`:
```
imagename,https://example.com/product-link
another-image,https://example.com/another-link
```
- Image name should match filename without extension
- One mapping per line, comma-separated

### Development Server (src/main.rs)
- Multi-route Axum application with separate handlers per page
- Template compilation happens at runtime using `include_str!` macros
- Static file serving from `templates/` directory
- **No-cache headers**: Browser caching disabled for instant updates during development
- Routes:
  - `/` - Home page
  - `/contact/` - Contact page (GET shows form, POST handles submission)
  - `/modeling/` - Unified modeling portfolio with category dropdown

### Static Site Generator (src/generate_static.rs)
- Compiles templates into static HTML files in `docs/` directory
- Automatically scans for images in modeling category folders and copies them to `docs/`
- Reads `Links.txt` files and embeds link data in generated pages
- **Cache busting**: Appends git commit hash to CSS links (`styles.css?v=abc1234`)
- Updates all navigation links and asset paths for GitHub Pages deployment

### Caching Strategy
- **Development**: No-cache headers ensure changes appear immediately
- **Production**: Git commit hash appended to CSS URL forces browser refresh on new deployments

### GitHub Pages Deployment
- Automated deployment via GitHub Actions on push to main
- Uses Rust toolchain with dependency caching
- Builds static files and deploys to GitHub Pages
- CSS version changes automatically on each commit (cache busting)

## Development Workflow

1. **Adding New Pages**: Add route handler in `main.rs` and corresponding entry in `generate_static.rs`
2. **Template Updates**: Modify templates in `templates/` directory - changes affect both dev server and static generation
3. **Adding Images**: Place images in `templates/modeling/{category}/images/` - they'll be auto-discovered and copied to `docs/` by static generator
4. **Adding Image Links**: Create/edit `templates/modeling/{category}/images/Links.txt` with `imagename,url` mappings
5. **Testing Changes**: Use development server for rapid iteration (no need to hard-refresh), then generate static files to test final output
6. **Footer/Navigation Changes**: Edit `templates/base.html` - changes apply to all pages

## Deployment Setup

To configure GitHub Pages:
1. Go to your repository Settings → Pages
2. Set source to "GitHub Actions"
3. Push to main branch to trigger automatic deployment

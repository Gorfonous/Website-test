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
  - `base.html` - Base template with {{TITLE}} and {{CONTENT}} placeholders
  - `index.html` - Home page content template
  - `headshots/headshots.html` - Headshots portfolio page with {{IMAGE_PATHS}} and {{CUSTOM_TITLE}} placeholders
  - `styles.css` - Shared CSS styles for all pages
  - `global-images/` - Images used across multiple pages
  - `{category}/` - Category-specific folders containing:
    - `images/` - Portfolio images for that category
    - `Background/` - Optional background image (only one allowed per category)
    - `subtitle.txt` - Optional custom subtitle text
- `docs/` - Generated static HTML files and assets for GitHub Pages
- `.github/workflows/deploy.yml` - GitHub Actions workflow for automated deployment

## Architecture

### Template System
The project uses a custom template replacement system:
- `base.html` provides the layout with `{{TITLE}}` and `{{CONTENT}}` placeholders
- Page-specific templates are embedded into the base template
- Special handling for modeling pages with dynamic image gallery generation
- Image paths are automatically discovered and injected as JavaScript arrays
- Modeling pages support custom titles via `{{CUSTOM_TITLE}}` placeholder
- Background images can be set per category (only one image allowed per category)
- Static generation updates navigation links and asset paths for GitHub Pages deployment

### Development Server (src/main.rs)
- Multi-route Axum application with separate handlers per page
- Template compilation happens at runtime using `include_str!` macros
- Static file serving from `docs/` directory for images
- Routes:
  - `/` - Home page
  - `/modeling/headshots/` - Headshots portfolio page

### Static Site Generator (src/generate_static.rs)
- Compiles templates into static HTML files in `docs/` directory
- Automatically scans for images in modeling category folders and copies them to `docs/`
- Creates necessary directory structure for nested pages
- Special logic for modeling pages to inject image paths from filesystem
- Validates background images (only one allowed per category, fails build if multiple found)
- Updates all navigation links and asset paths for GitHub Pages deployment

### GitHub Pages Deployment
- Automated deployment via GitHub Actions on push to main
- Uses Rust toolchain with dependency caching
- Builds static files and deploys to GitHub Pages
- Supports both main branch pushes and pull request previews

## Development Workflow

1. **Adding New Pages**: Add route handler in `main.rs` and corresponding entry in `generate_static.rs`
2. **Template Updates**: Modify templates in `templates/` directory - changes affect both dev server and static generation
3. **Adding Images**: Place images in `templates/{category}/images/` - they'll be auto-discovered and copied to `docs/` by static generator
4. **Testing Changes**: Use development server for rapid iteration, then generate static files to test final output

## Deployment Setup

To configure GitHub Pages:
1. Go to your repository Settings → Pages
2. Set source to "GitHub Actions"
3. Push to main branch to trigger automatic deployment
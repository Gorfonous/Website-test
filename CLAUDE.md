# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based web server project for Eternal Cataclysm Studios. It provides a simple "Hello World" website using the Axum web framework.

## Technology Stack

- **Language**: Rust (2024 edition)
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio 1.0
- **HTTP Utilities**: Tower 0.4

## Common Commands

### Development
- `cargo run --bin server` - Start the development server (runs on http://127.0.0.1:3000)
- `cargo run --bin generate-static` - Generate static HTML files for GitHub Pages
- `cargo check` - Check code for compilation errors without building
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version

### GitHub Pages Deployment
- Static files are automatically generated and deployed via GitHub Actions
- Manual generation: `cargo run --bin generate-static` (outputs to `docs/` directory)

### Testing
- `cargo test` - Run all tests
- `cargo clippy` - Run the Rust linter
- `cargo fmt` - Format code according to Rust standards

## Project Structure

- `src/main.rs` - Main application entry point with web server setup
- `build.rs` - Static site generator for GitHub Pages
- `docs/` - Static HTML files for GitHub Pages deployment
- `.github/workflows/deploy.yml` - GitHub Actions workflow for automated deployment
- `Cargo.toml` - Project dependencies and configuration
- `target/` - Build artifacts (auto-generated)

## Architecture

### Development Server
The application uses Axum as the web framework with the following setup:
- Single route handler at `/` serving HTML content
- Async/await pattern with Tokio runtime
- TCP listener bound to localhost:3000
- Simple HTML response with company branding

### GitHub Pages Deployment
- Static HTML files are generated using a custom build script
- Files are output to the `docs/` directory
- GitHub Actions automatically builds and deploys on push to main
- Styled with CSS for a professional appearance

## Deployment Setup

To configure GitHub Pages:
1. Go to your repository Settings → Pages
2. Set source to "GitHub Actions"
3. Push to main branch to trigger automatic deployment
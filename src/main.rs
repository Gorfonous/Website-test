use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use std::fs;
use std::path::Path;

fn generate_page(title: &str, content: &str) -> String {
    let base_template = include_str!("../templates/base.html");
    base_template
        .replace("{{TITLE}}", title)
        .replace("{{CONTENT}}", content)
}

fn get_image_list(images_dir: &Path) -> Vec<String> {
    let mut images = Vec::new();
    
    if images_dir.exists() {
        if let Ok(entries) = fs::read_dir(images_dir) {
            for entry in entries.flatten() {
                if let Some(extension) = entry.path().extension() {
                    if extension.to_str() == Some("png") || extension.to_str() == Some("jpg") || extension.to_str() == Some("jpeg") {
                        if let Some(filename) = entry.file_name().to_str() {
                            images.push(format!("/templates/{}/images/{}", images_dir.parent().unwrap().file_name().unwrap().to_str().unwrap(), filename));
                        }
                    }
                }
            }
        }
    }
    
    // Sort images naturally (1.png, 2.png, 3.png, etc.)
    images.sort();
    images
}

fn check_background_image_dev(background_dir: &Path) -> Option<String> {
    if !background_dir.exists() {
        return None;
    }
    
    if let Ok(entries) = fs::read_dir(background_dir) {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension.to_str() == Some("png") || extension.to_str() == Some("jpg") || extension.to_str() == Some("jpeg") {
                    if let Some(filename) = entry.file_name().to_str() {
                        let category = background_dir.parent().unwrap().file_name().unwrap().to_str().unwrap();
                        return Some(format!("/templates/{}/Background/{}", category, filename));
                    }
                }
            }
        }
    }
    
    None
}

fn generate_modeling_page(category: &str, title: &str, content: &str) -> String {
    let templates_images_dir = Path::new("templates").join(category).join("images");
    let templates_background_dir = Path::new("templates").join(category).join("Background");
    let templates_title_file = Path::new("templates").join(category).join("title.txt");
    let image_list = get_image_list(&templates_images_dir);
    
    // Read custom title if it exists
    let custom_title = if templates_title_file.exists() {
        match fs::read_to_string(&templates_title_file) {
            Ok(content) => Some(content.trim().to_string()),
            Err(_) => None,
        }
    } else {
        None
    };
    
    // Check for background image (for dev server, we don't fail on multiple images)
    let background_image = check_background_image_dev(&templates_background_dir);
    
    let image_paths_js = if image_list.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", image_list.iter().map(|img| format!("'{}'", img)).collect::<Vec<_>>().join(", "))
    };
    
    let mut updated_content = content.replace("{{IMAGE_PATHS}}", &image_paths_js);
    
    // Replace custom title if it exists, otherwise use default
    if let Some(ref custom_text) = custom_title {
        updated_content = updated_content.replace("{{CUSTOM_TITLE}}", custom_text);
    } else {
        updated_content = updated_content.replace("{{CUSTOM_TITLE}}", "Professional portrait photography for actors, models, and business professionals");
    }
    
    let base_template = include_str!("../templates/base.html");
    let mut final_html = base_template
        .replace("{{TITLE}}", title)
        .replace("{{CONTENT}}", &updated_content);
    
    // Replace background if one exists
    if let Some(bg_path) = background_image {
        final_html = final_html.replace(
            "background: linear-gradient(45deg, #ff6b9d, #c44faf, #8b5fbf, #6b73ff);",
            &format!("background: url('{}') center center/cover no-repeat fixed;", bg_path)
        );
        // Remove the background animation properties since we have a static image
        final_html = final_html.replace("background-size: 400% 400%;", "");
        final_html = final_html.replace("animation: gradientShift 15s ease infinite;", "");
    }
    
    final_html
}

async fn home_page() -> Html<String> {
    let content = include_str!("../templates/index.html");
    Html(generate_page("Home", content))
}

// Modeling subcategory pages
async fn headshots_page() -> Html<String> {
    let content = include_str!("../templates/headshots/headshots.html");
    Html(generate_modeling_page("headshots", "Headshots", content))
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        // Home page
        .route("/", get(home_page))
        
        // Headshots page
        .route("/modeling/headshots/", get(headshots_page))
        
        // Serve static files (images) from docs directory and templates
        .nest_service("/docs", ServeDir::new("docs"))
        .nest_service("/templates", ServeDir::new("templates"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("Server running on http://127.0.0.1:3000");
    println!("Available pages:");
    println!("  - / (Home)");
    println!("  - /modeling/headshots/");
    
    axum::serve(listener, app).await.unwrap();
}

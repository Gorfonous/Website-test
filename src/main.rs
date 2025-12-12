use axum::{extract::Path as AxumPath, response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn url_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '+' => "%2B".to_string(),
            '#' => "%23".to_string(),
            '&' => "%26".to_string(),
            '=' => "%3D".to_string(),
            '?' => "%3F".to_string(),
            c if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

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
                            let url_encoded_filename = url_encode(filename);
                            images.push(format!("/templates/{}/images/{}", images_dir.parent().unwrap().file_name().unwrap().to_str().unwrap(), url_encoded_filename));
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

#[derive(Clone, Debug)]
struct PageTemplate {
    title: String,
    content: String,
    is_modeling: bool,
    category: Option<String>,
}

fn discover_templates() -> Result<HashMap<String, PageTemplate>, Box<dyn std::error::Error>> {
    let mut templates = HashMap::new();
    let templates_dir = Path::new("templates");
    
    // Handle index.html as root page
    let index_path = templates_dir.join("index.html");
    if index_path.exists() {
        let content = fs::read_to_string(&index_path)?;
        templates.insert("/".to_string(), PageTemplate {
            title: "Home".to_string(),
            content,
            is_modeling: false,
            category: None,
        });
    }
    
    // Recursively scan for other HTML files
    scan_directory(&templates_dir, &mut templates, "")?;
    
    Ok(templates)
}

fn scan_directory(dir: &Path, templates: &mut HashMap<String, PageTemplate>, base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.exists() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let folder_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            
            // Skip certain directories
            if folder_name == "images" || folder_name == "Background" {
                continue;
            }
            
            let new_base_path = if base_path.is_empty() {
                folder_name.to_string()
            } else {
                format!("{}/{}", base_path, folder_name)
            };
            
            scan_directory(&path, templates, &new_base_path)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("html") {
            let filename = path.file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            
            // Skip base.html and index.html (index is handled separately)
            if filename == "base" || (filename == "index" && base_path.is_empty()) {
                continue;
            }
            
            let content = fs::read_to_string(&path)?;
            
            // Determine route path
            let route_path = if base_path.is_empty() {
                format!("/{}/", filename)
            } else {
                format!("/modeling/{}/", base_path)
            };
            
            // Check if this is a modeling page (in a subfolder of templates)
            let is_modeling = !base_path.is_empty();
            let category = if is_modeling {
                Some(base_path.to_string())
            } else {
                None
            };
            
            // Generate title from path
            let title = if is_modeling {
                // Capitalize first letter of category
                let mut chars = base_path.chars();
                match chars.next() {
                    None => "Portfolio".to_string(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            } else {
                // Capitalize first letter of filename
                let mut chars = filename.chars();
                match chars.next() {
                    None => "Page".to_string(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            };
            
            templates.insert(route_path.clone(), PageTemplate {
                title,
                content,
                is_modeling,
                category,
            });
        }
    }
    
    Ok(())
}

fn generate_modeling_page(category: &str, title: &str, content: &str) -> String {
    let templates_images_dir = Path::new("templates").join(category).join("images");
    let templates_background_dir = Path::new("templates").join(category).join("Background");
    let templates_title_file = Path::new("templates").join(category).join("subtitle.txt");
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

// Dynamic page handler for modeling pages
async fn dynamic_modeling_handler(AxumPath(category): AxumPath<String>, templates: axum::extract::State<HashMap<String, PageTemplate>>) -> Result<Html<String>, axum::response::Response> {
    let normalized_path = format!("/modeling/{}/", category);
    
    if let Some(template) = templates.get(&normalized_path) {
        let html_content = if template.is_modeling {
            if let Some(ref category) = template.category {
                generate_modeling_page(category, &template.title, &template.content)
            } else {
                generate_page(&template.title, &template.content)
            }
        } else {
            generate_page(&template.title, &template.content)
        };
        
        Ok(Html(html_content))
    } else {
        // Return 404 response
        let not_found_html = generate_page("404 - Page Not Found", 
            "<div style='text-align: center; padding: 50px;'>
                <h1>404 - Page Not Found</h1>
                <p>The page you're looking for doesn't exist.</p>
                <a href='/'>Return to Home</a>
             </div>");
        
        Err(axum::response::Response::builder()
            .status(404)
            .header("content-type", "text/html")
            .body(not_found_html.into())
            .unwrap())
    }
}

// Home page handler (special case for root)
async fn home_page_handler(templates: axum::extract::State<HashMap<String, PageTemplate>>) -> Html<String> {
    if let Some(template) = templates.get("/") {
        Html(generate_page(&template.title, &template.content))
    } else {
        Html(generate_page("Error", "<h1>Home page template not found</h1>"))
    }
}


#[tokio::main]
async fn main() {
    // Discover all templates at startup
    let templates = match discover_templates() {
        Ok(templates) => templates,
        Err(e) => {
            eprintln!("Error discovering templates: {}", e);
            std::process::exit(1);
        }
    };
    
    // Print discovered routes
    println!("Discovered templates:");
    let mut sorted_paths: Vec<_> = templates.keys().collect();
    sorted_paths.sort();
    for path in &sorted_paths {
        let template = &templates[*path];
        let page_type = if template.is_modeling { "modeling" } else { "standard" };
        println!("  - {} ({}) - {}", path, page_type, template.title);
    }
    
    let app = Router::new()
        // Home page (special route)
        .route("/", get(home_page_handler))
        
        // Dynamic routes for modeling pages
        .route("/modeling/:category/", get(dynamic_modeling_handler))
        
        // Serve static files (images) from docs directory and templates
        .nest_service("/docs", ServeDir::new("docs"))
        .nest_service("/templates", ServeDir::new("templates"))
        
        // Share templates state with handlers
        .with_state(templates.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("\nServer running on http://127.0.0.1:3000");
    println!("Available pages:");
    for path in sorted_paths {
        println!("  - http://127.0.0.1:3000{}", if path == "/" { "" } else { &path[..path.len()-1] });
    }
    
    axum::serve(listener, app).await.unwrap();
}

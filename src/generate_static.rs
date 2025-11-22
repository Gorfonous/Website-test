use std::fs;
use std::path::Path;

fn generate_page(_page_name: &str, title: &str, content: &str) -> String {
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
                            images.push(format!("./templates/{}/images/{}", images_dir.parent().unwrap().file_name().unwrap().to_str().unwrap(), filename));
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

fn generate_modeling_page(category: &str, title: &str, content: &str, _docs_dir: &Path) -> String {
    let templates_images_dir = Path::new("templates").join(category).join("images");
    let image_list = get_image_list(&templates_images_dir);
    
    let image_paths_js = if image_list.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", image_list.iter().map(|img| format!("'{}'", img)).collect::<Vec<_>>().join(", "))
    };
    
    let updated_content = content.replace("{{IMAGE_PATHS}}", &image_paths_js);
    
    let base_template = include_str!("../templates/base.html");
    base_template
        .replace("{{TITLE}}", title)
        .replace("{{CONTENT}}", &updated_content)
}

fn create_dir_if_not_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create directory: {:?}", path));
    }
}

fn main() {
    let docs_dir = Path::new("docs");
    create_dir_if_not_exists(docs_dir);

    // Create modeling subdirectory
    create_dir_if_not_exists(&docs_dir.join("modeling"));
    
    // Create headshots directory with images folder
    let headshots_path = docs_dir.join("modeling").join("headshots");
    create_dir_if_not_exists(&headshots_path);
    create_dir_if_not_exists(&headshots_path.join("images"));

    // Define pages with their folder paths and content
    let pages = vec![
        ("index.html", "Home", include_str!("../templates/index.html")),
        ("modeling/headshots/index.html", "Headshots", include_str!("../templates/headshots/headshots.html")),
    ];

    // Generate all pages
    for (filepath, title, content) in pages {
        let html_content = if filepath.starts_with("modeling/") && filepath != "modeling/index.html" {
            // Extract category name from filepath (e.g., "modeling/headshots/index.html" -> "headshots")
            let parts: Vec<&str> = filepath.split('/').collect();
            if parts.len() >= 2 {
                let category = parts[1];
                generate_modeling_page(category, title, content, docs_dir)
            } else {
                generate_page("", title, content)
            }
        } else {
            generate_page("", title, content)
        };
        
        let file_path = docs_dir.join(filepath);
        
        fs::write(&file_path, html_content)
            .expect(&format!("Failed to write {}", filepath));
        
        println!("Generated {}", filepath);
    }

    println!("Static files generated for Home page and Headshots page");
}
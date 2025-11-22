use std::fs;
use std::path::Path;

fn generate_page(_page_name: &str, title: &str, content: &str) -> String {
    let base_template = include_str!("../templates/base.html");
    base_template
        .replace("{{TITLE}}", title)
        .replace("{{CONTENT}}", content)
}


fn get_image_list_for_web(images_dir: &Path, _category: &str) -> Vec<String> {
    let mut images = Vec::new();
    
    if images_dir.exists() {
        if let Ok(entries) = fs::read_dir(images_dir) {
            for entry in entries.flatten() {
                if let Some(extension) = entry.path().extension() {
                    if extension.to_str() == Some("png") || extension.to_str() == Some("jpg") || extension.to_str() == Some("jpeg") {
                        if let Some(filename) = entry.file_name().to_str() {
                            images.push(format!("./images/{}", filename));
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

fn copy_images(source_dir: &Path, dest_dir: &Path) {
    if !source_dir.exists() {
        return;
    }
    
    // Create destination directory if it doesn't exist
    if let Err(_) = fs::create_dir_all(dest_dir) {
        println!("Failed to create directory: {:?}", dest_dir);
        return;
    }
    
    // Copy all image files
    if let Ok(entries) = fs::read_dir(source_dir) {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension.to_str() == Some("png") || extension.to_str() == Some("jpg") || extension.to_str() == Some("jpeg") {
                    if let Some(filename) = entry.file_name().to_str() {
                        let source_file = source_dir.join(filename);
                        let dest_file = dest_dir.join(filename);
                        
                        if let Err(e) = fs::copy(&source_file, &dest_file) {
                            println!("Failed to copy {:?} to {:?}: {}", source_file, dest_file, e);
                        } else {
                            println!("Copied image: {}", filename);
                        }
                    }
                }
            }
        }
    }
}

fn generate_modeling_page(category: &str, title: &str, content: &str, docs_dir: &Path) -> String {
    let templates_images_dir = Path::new("templates").join(category).join("images");
    let docs_images_dir = docs_dir.join("modeling").join(category).join("images");
    
    // Copy images from templates to docs directory
    copy_images(&templates_images_dir, &docs_images_dir);
    
    let image_list = get_image_list_for_web(&templates_images_dir, category);
    
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
use std::fs;
use std::path::Path;

fn generate_page(_page_name: &str, title: &str, content: &str) -> String {
    let base_template = include_str!("../templates/base.html");
    let mut final_html = base_template
        .replace("{{TITLE}}", title)
        .replace("{{CONTENT}}", content);
    
    // Update navigation links for GitHub Pages (static generation)
    final_html = final_html.replace(
        r#"<a href="/" class="nav-item">Home</a>"#,
        r#"<a href="/Website-test/index.html" class="nav-item">Home</a>"#
    );
    final_html = final_html.replace(
        r#"<a href="/modeling/headshots/">Headshots</a>"#,
        r#"<a href="/Website-test/modeling/headshots/index.html">Headshots</a>"#
    );
    
    // Update image paths for GitHub Pages deployment
    final_html = final_html.replace(
        r#"src="/templates/global-images/"#,
        r#"src="/Website-test/global-images/"#
    );
    final_html = final_html.replace(
        r#"album-cover.png"#,
        r#"album-cover.png"#
    );
    
    final_html
}


fn get_image_list_for_web(images_dir: &Path, _category: &str) -> Vec<String> {
    let mut images = Vec::new();
    
    if images_dir.exists() {
        if let Ok(entries) = fs::read_dir(images_dir) {
            for entry in entries.flatten() {
                if let Some(extension) = entry.path().extension() {
                    if matches!(extension.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
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
                if matches!(extension.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
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

fn check_background_image(background_dir: &Path, location: &str) -> Result<Option<String>, String> {
    if !background_dir.exists() {
        return Ok(None);
    }
    
    let mut background_images = Vec::new();
    
    if let Ok(entries) = fs::read_dir(background_dir) {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if matches!(extension.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
                    if let Some(filename) = entry.file_name().to_str() {
                        background_images.push(filename.to_string());
                    }
                }
            }
        }
    }
    
    match background_images.len() {
        0 => Ok(None),
        1 => Ok(Some(format!("./Background/{}", background_images[0]))),
        _ => Err(format!("ERROR: Multiple background images found in {}: {:?}", location, background_images)),
    }
}

fn generate_modeling_page(category: &str, title: &str, content: &str, docs_dir: &Path) -> Result<String, String> {
    let templates_images_dir = Path::new("templates").join(category).join("images");
    let templates_background_dir = Path::new("templates").join(category).join("Background");
    let templates_title_file = Path::new("templates").join(category).join("subtitle.txt");
    let docs_images_dir = docs_dir.join("modeling").join(category).join("images");
    let docs_background_dir = docs_dir.join("modeling").join(category).join("Background");
    
    // Read custom title if it exists
    let custom_title = if templates_title_file.exists() {
        match fs::read_to_string(&templates_title_file) {
            Ok(content) => Some(content.trim().to_string()),
            Err(_) => None,
        }
    } else {
        None
    };
    
    // Check for background image
    let background_image = check_background_image(&templates_background_dir, &format!("templates/{}/Background", category))?;
    
    // Copy images from templates to docs directory
    copy_images(&templates_images_dir, &docs_images_dir);
    
    // Copy background image if it exists
    if background_image.is_some() {
        copy_images(&templates_background_dir, &docs_background_dir);
    }
    
    let image_list = get_image_list_for_web(&templates_images_dir, category);
    
    let image_paths_js = if image_list.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", image_list.iter().map(|img| format!("'{}'", img)).collect::<Vec<_>>().join(", "))
    };
    
    let mut updated_content = content.replace("{{IMAGE_PATHS}}", &image_paths_js);
    
    // Replace custom title if it exists, otherwise use default
    if let Some(ref custom_text) = custom_title {
        updated_content = updated_content.replace("{{CUSTOM_TITLE}}", custom_text);
        println!("Applied custom title from subtitle.txt: {}", custom_text);
    } else {
        updated_content = updated_content.replace("{{CUSTOM_TITLE}}", "Professional portrait photography for actors, models, and business professionals");
    }
    
    let base_template = include_str!("../templates/base.html");
    let mut final_html = base_template
        .replace("{{TITLE}}", title)
        .replace("{{CONTENT}}", &updated_content);
    
    // Update navigation links for GitHub Pages (static generation)
    final_html = final_html.replace(
        r#"<a href="/" class="nav-item">Home</a>"#,
        r#"<a href="/Website-test/index.html" class="nav-item">Home</a>"#
    );
    final_html = final_html.replace(
        r#"<a href="/modeling/headshots/">Headshots</a>"#,
        r#"<a href="/Website-test/modeling/headshots/index.html">Headshots</a>"#
    );
    
    // Replace background if one exists
    if let Some(bg_path) = background_image {
        final_html = final_html.replace(
            "background: linear-gradient(45deg, #ff6b9d, #c44faf, #8b5fbf, #6b73ff);",
            &format!("background: url('{}') center center/cover no-repeat fixed;", bg_path)
        );
        // Remove the background animation properties since we have a static image
        final_html = final_html.replace("background-size: 400% 400%;", "");
        final_html = final_html.replace("animation: gradientShift 15s ease infinite;", "");
        println!("Applied background image: {}", bg_path);
    }
    
    Ok(final_html)
}

fn create_dir_if_not_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path).expect(&format!("Failed to create directory: {:?}", path));
    }
}

fn main() {
    let docs_dir = Path::new("docs");
    
    // Clean and rebuild the entire docs directory structure
    if docs_dir.exists() {
        fs::remove_dir_all(docs_dir).expect("Failed to remove existing docs directory");
        println!("Cleaned existing docs directory");
    }
    
    create_dir_if_not_exists(docs_dir);
    
    // Copy CSS file from templates to docs
    let templates_css = Path::new("templates").join("styles.css");
    let docs_css = docs_dir.join("styles.css");
    
    if templates_css.exists() {
        if let Err(e) = fs::copy(&templates_css, &docs_css) {
            println!("Failed to copy CSS file: {}", e);
        } else {
            println!("Copied styles.css to docs directory");
        }
    }
    
    // Copy global images folder from templates to docs
    let templates_global_images = Path::new("templates").join("global-images");
    let docs_global_images = docs_dir.join("global-images");
    
    if templates_global_images.exists() {
        create_dir_if_not_exists(&docs_global_images);
        copy_images(&templates_global_images, &docs_global_images);
    }

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
                match generate_modeling_page(category, title, content, docs_dir) {
                    Ok(html) => html,
                    Err(error) => {
                        eprintln!("{}", error);
                        eprintln!("Failing to generate: {}", filepath);
                        std::process::exit(1);
                    }
                }
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
use std::fs;
use std::path::Path;

fn generate_page(title: &str, content: &str) -> String {
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
        r#"<a href="/contact/" class="nav-item">Contact</a>"#,
        r#"<a href="/Website-test/contact/index.html" class="nav-item">Contact</a>"#
    );
    final_html = final_html.replace(
        r#"<a href="/modeling/" class="nav-item">Modeling</a>"#,
        r#"<a href="/Website-test/modeling/index.html" class="nav-item">Modeling</a>"#
    );

    // Update image paths for GitHub Pages deployment
    final_html = final_html.replace(
        r#"src="/templates/global-images/"#,
        r#"src="/Website-test/global-images/"#
    );

    // Update CSS path for GitHub Pages deployment
    final_html = final_html.replace(
        r#"href="/templates/styles.css""#,
        r#"href="/Website-test/styles.css""#
    );

    final_html
}

fn get_image_list_for_web(images_dir: &Path, category: &str) -> Vec<String> {
    let mut images = Vec::new();

    if images_dir.exists() {
        if let Ok(entries) = fs::read_dir(images_dir) {
            for entry in entries.flatten() {
                if let Some(extension) = entry.path().extension() {
                    if matches!(extension.to_str(), Some("png") | Some("jpg") | Some("jpeg")) {
                        if let Some(filename) = entry.file_name().to_str() {
                            images.push(format!("./{}/images/{}", category, filename));
                        }
                    }
                }
            }
        }
    }

    images.sort();
    images
}

fn copy_images(source_dir: &Path, dest_dir: &Path) {
    if !source_dir.exists() {
        return;
    }

    if fs::create_dir_all(dest_dir).is_err() {
        println!("Failed to create directory: {:?}", dest_dir);
        return;
    }

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

use std::collections::HashMap;

struct CategoryData {
    title: String,
    subtitle: String,
    images: Vec<String>,
    links: HashMap<String, String>,
}

fn read_links_file(images_dir: &Path) -> HashMap<String, String> {
    let mut links = HashMap::new();
    let links_file = images_dir.join("Links.txt");

    if links_file.exists() {
        if let Ok(content) = fs::read_to_string(&links_file) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some((name, url)) = line.split_once(',') {
                    links.insert(name.trim().to_string(), url.trim().to_string());
                }
            }
        }
    }

    links
}

fn discover_modeling_categories(docs_dir: &Path) -> Vec<(String, CategoryData)> {
    let mut categories = Vec::new();
    let modeling_dir = Path::new("templates").join("modeling");

    if !modeling_dir.exists() {
        return categories;
    }

    if let Ok(entries) = fs::read_dir(&modeling_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let category_name = entry.file_name().to_str().unwrap_or("").to_string();

                let images_dir = entry.path().join("images");
                if !images_dir.exists() {
                    continue;
                }

                let title = {
                    let mut chars = category_name.chars();
                    match chars.next() {
                        None => continue,
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                };

                let subtitle_file = entry.path().join("subtitle.txt");
                let subtitle = if subtitle_file.exists() {
                    fs::read_to_string(&subtitle_file)
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|_| format!("Professional {} photography", category_name))
                } else {
                    format!("Professional {} photography", category_name)
                };

                let docs_images_dir = docs_dir.join("modeling").join(&category_name).join("images");
                copy_images(&images_dir, &docs_images_dir);

                let images = get_image_list_for_web(&images_dir, &category_name);
                let links = read_links_file(&images_dir);

                categories.push((category_name, CategoryData {
                    title,
                    subtitle,
                    images,
                    links,
                }));
            }
        }
    }

    categories.sort_by(|a, b| a.0.cmp(&b.0));
    categories
}

fn generate_categories_json(categories: &[(String, CategoryData)]) -> String {
    let mut json_parts = Vec::new();

    for (key, data) in categories {
        let images_json: Vec<String> = data.images.iter().map(|img| format!("\"{}\"", img)).collect();
        let escaped_subtitle = data.subtitle
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", " ")
            .replace("\r", "");

        let links_json: Vec<String> = data.links.iter()
            .map(|(k, v)| format!("\"{}\": \"{}\"", k, v.replace("\"", "\\\"")))
            .collect();

        json_parts.push(format!(
            "\"{}\": {{\"title\": \"{}\", \"subtitle\": \"{}\", \"images\": [{}], \"links\": {{{}}}}}",
            key,
            data.title,
            escaped_subtitle,
            images_json.join(", "),
            links_json.join(", ")
        ));
    }

    format!("{{{}}}", json_parts.join(", "))
}

fn generate_modeling_page(content: &str, categories: &[(String, CategoryData)]) -> String {
    let categories_json = generate_categories_json(categories);
    let updated_content = content.replace("{{CATEGORIES_JSON}}", &categories_json);

    let base_template = include_str!("../templates/base.html");
    let mut final_html = base_template
        .replace("{{TITLE}}", "Modeling Portfolio")
        .replace("{{CONTENT}}", &updated_content);

    // Update navigation links for GitHub Pages (static generation)
    final_html = final_html.replace(
        r#"<a href="/" class="nav-item">Home</a>"#,
        r#"<a href="/Website-test/index.html" class="nav-item">Home</a>"#
    );
    final_html = final_html.replace(
        r#"<a href="/contact/" class="nav-item">Contact</a>"#,
        r#"<a href="/Website-test/contact/index.html" class="nav-item">Contact</a>"#
    );
    final_html = final_html.replace(
        r#"<a href="/modeling/" class="nav-item">Modeling</a>"#,
        r#"<a href="/Website-test/modeling/index.html" class="nav-item">Modeling</a>"#
    );

    // Update CSS path for GitHub Pages deployment
    final_html = final_html.replace(
        r#"href="/templates/styles.css""#,
        r#"href="/Website-test/styles.css""#
    );

    final_html
}

fn create_dir_if_not_exists(path: &Path) {
    if !path.exists() {
        fs::create_dir_all(path).unwrap_or_else(|_| panic!("Failed to create directory: {:?}", path));
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

    // Discover modeling categories and copy their images
    let categories = discover_modeling_categories(docs_dir);
    println!("\nModeling categories discovered:");
    for (name, data) in &categories {
        println!("  - {} ({} images)", name, data.images.len());
    }

    // Generate home page
    let home_content = include_str!("../templates/index.html");
    let home_html = generate_page("Home", home_content);
    let home_file_path = docs_dir.join("index.html");
    fs::write(&home_file_path, home_html).expect("Failed to write index.html");
    println!("Generated index.html");

    // Generate unified modeling page
    let modeling_content = include_str!("../templates/modeling/modeling.html");
    let modeling_html = generate_modeling_page(modeling_content, &categories);
    let modeling_path = docs_dir.join("modeling").join("index.html");
    fs::write(&modeling_path, modeling_html).expect("Failed to write modeling/index.html");
    println!("Generated modeling/index.html");

    // Generate contact page
    let contact_dir = docs_dir.join("contact");
    create_dir_if_not_exists(&contact_dir);

    let contact_path = Path::new("templates").join("contact").join("contact.html");
    if contact_path.exists() {
        match fs::read_to_string(&contact_path) {
            Ok(content) => {
                let html = generate_page("Contact", &content);
                let file_path = contact_dir.join("index.html");
                fs::write(&file_path, html).expect("Failed to write contact/index.html");
                println!("Generated contact/index.html");
            },
            Err(e) => {
                println!("Failed to read contact template: {}", e);
            }
        }
    }

    println!("\nStatic files generated successfully!");
}

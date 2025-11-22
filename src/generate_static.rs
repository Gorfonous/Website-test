use std::fs;
use std::path::Path;

fn main() {
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir).expect("Failed to create docs directory");
    }

    let html_content = include_str!("../templates/index.html");

    fs::write(docs_dir.join("index.html"), html_content)
        .expect("Failed to write index.html");

    println!("Static files generated in docs/ directory");
}
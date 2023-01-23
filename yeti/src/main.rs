use colored::*;
use std::path::Path;
use walkdir::WalkDir;

const OUTPUT_DIR: &str = "dist";

fn main() {
    let path = Path::new("../templates");

    for entry in WalkDir::new(&path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            match path.extension() {
                Some(ext) if ext == "html" => {
                    if path.file_name().unwrap().to_str().unwrap().starts_with("_") {
                        continue;
                    }
                    println!(
                        "compiling {} to {}",
                        path.display().to_string().yellow(),
                        OUTPUT_DIR.green()
                    );
                }
                _ => {
                    println!(
                        "copying {} to {}",
                        path.display().to_string().yellow(),
                        OUTPUT_DIR.green()
                    );
                }
            }
        }
    }
}

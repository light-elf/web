use colored::*;
use path_absolutize::Absolutize;
use std::{fs::copy, fs::create_dir_all, fs::write, path::Path};
use tera::{Context, Tera};
use walkdir::WalkDir;

const OUTPUT_DIR: &str = "../dist";
const TEMPLATES_DIR: &str = "../templates";

fn main() {
    let templates_path = Path::new(&TEMPLATES_DIR).absolutize().unwrap();
    let build_path = Path::new(&OUTPUT_DIR).absolutize().unwrap();
    let t = Tera::new(
        format!(
            "{}/{}",
            &templates_path.to_string_lossy(),
            "**/*.html".to_string()
        )
        .as_str(),
    )
    .unwrap();

    for entry in WalkDir::new(&templates_path) {
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
                    let file = path.strip_prefix(templates_path.as_os_str()).unwrap();
                    let result = t
                        .render(&file.display().to_string(), &Context::new())
                        .unwrap();
                    write(&build_path.join(&file), result.as_bytes()).unwrap();
                }
                _ => {
                    copy_to_dist(path, &templates_path, &build_path);
                }
            }
        }
    }
}

fn copy_to_dist(
    path: &Path,
    templates_path: &std::borrow::Cow<Path>,
    build_path: &std::borrow::Cow<Path>,
) {
    let file = path.strip_prefix(templates_path.as_os_str()).unwrap();
    let output_file = build_path.join(&file);
    println!(
        "copying {} to {}",
        path.display().to_string().yellow(),
        &output_file.display().to_string().green()
    );

    create_dir_all(&output_file.parent().unwrap()).expect("failed to create output directory");
    copy(&path, &output_file).expect(
        format!(
            "ERROR: failed to copy {} to {}",
            path.display().to_string().red(),
            &output_file.display().to_string().red()
        )
        .as_str(),
    );
}

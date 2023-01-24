use colored::*;
use path_absolutize::Absolutize;
use std::fs::{copy, create_dir_all, write};
use std::io;
use std::path::Path;
use tera::{Context, Error, Tera};
use walkdir::WalkDir;

const OUTPUT_DIR: &str = "../dist";
const TEMPLATES_DIR: &str = "../templates";

fn main() {
    let templates_path = Path::new(&TEMPLATES_DIR).absolutize().unwrap();
    let build_path = Path::new(&OUTPUT_DIR).absolutize().unwrap();
    let template_engine = get_template_engine(&templates_path).unwrap();

    for entry in WalkDir::new(&templates_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        let file = path.strip_prefix(templates_path.as_os_str()).unwrap();
        let output_file = build_path.join(&file);
        let source_file = templates_path.join(&file);
        if path.is_file() {
            match path.extension() {
                Some(ext) if ext == "html" => {
                    if path.file_name().unwrap().to_str().unwrap().starts_with("_") {
                        continue;
                    }
                    println!(
                        "compiling {} to {}",
                        file.display().to_string().yellow(),
                        OUTPUT_DIR.green()
                    );
                    render_template(&template_engine, file, &build_path).unwrap();
                }
                _ => {
                    println!(
                        "copying {} to {}",
                        file.display().to_string().yellow(),
                        OUTPUT_DIR.green()
                    );
                    copy_to_dist(&source_file, &output_file).unwrap();
                }
            }
        }
    }
}

fn get_template_engine(templates_path: &Path) -> Result<Tera, Error> {
    Tera::new(
        format!(
            "{}/{}",
            &templates_path.to_string_lossy(),
            "**/*.html".to_string()
        )
        .as_str(),
    )
}

fn render_template(template_engine: &Tera, file: &Path, build_path: &Path) -> io::Result<()> {
    match template_engine.render(&file.display().to_string(), &Context::new()) {
        Ok(result) => write(&build_path.join(&file), result.as_bytes()),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to render template: {}", e),
            ))
        }
    }
}

fn copy_to_dist(file: &Path, output_file: &Path) -> io::Result<()> {
    let parent = match output_file.parent() {
        Some(parent) => parent,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to get parent directory",
            ))
        }
    };
    match create_dir_all(parent) {
        Ok(_) => match copy(file, &output_file) {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "failed to copy file from {} to {} ERROR: {}",
                        file.display(),
                        &output_file.display(),
                        e
                    ),
                ))
            }
        },
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to create directory: {}", e),
            ))
        }
    }
}

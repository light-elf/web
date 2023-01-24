use colored::*;
use path_absolutize::Absolutize;
use std::borrow::Cow;
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
    let t = get_template_engine(&templates_path).unwrap();

    for entry in WalkDir::new(&templates_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        let file = path.strip_prefix(templates_path.as_os_str()).unwrap();
        if path.is_file() {
            match path.extension() {
                Some(ext) if ext == "html" => {
                    if path.file_name().unwrap().to_str().unwrap().starts_with("_") {
                        continue;
                    }
                    render_template(&t, file, &build_path).unwrap();
                }
                _ => {
                    copy_to_dist(file, &build_path).unwrap();
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

fn render_template(t: &Tera, file: &Path, build_path: &Path) -> io::Result<()> {
    println!(
        "compiling {} to {}",
        file.display().to_string().yellow(),
        OUTPUT_DIR.green()
    );
    let Ok(result) = t
        .render(&file.display().to_string(), &Context::new()) else {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to render template"));
        };
    write(&build_path.join(&file), result.as_bytes())
}

fn copy_to_dist(file: &Path, build_path: &Cow<Path>) -> io::Result<()> {
    let output_file = build_path.join(&file);
    println!(
        "copying {} to {}",
        file.display().to_string().yellow(),
        &output_file.display().to_string().green()
    );

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
                        file.display().to_string().red(),
                        &output_file.display().to_string().red(),
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

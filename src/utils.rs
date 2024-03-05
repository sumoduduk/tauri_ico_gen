use std::{fs::create_dir_all, path::Path};

use eyre::Report;

pub fn check_file(src: &Path) -> Option<&Path> {
    if src.is_file() {
        let extension = src.extension().and_then(std::ffi::OsStr::to_str);

        match extension {
            Some("jpg") | Some("jpeg") | Some("png") => Some(src),
            _ => None,
        }
    } else {
        None
    }
}

pub fn create_folder(path: &Path) {
    if !path.exists() {
        create_dir_all(path).expect("Failed to create output folder")
    }
}

pub fn join_path(out_folder: &Path, file_name: &str) -> std::path::PathBuf {
    out_folder.join(file_name)
}

pub fn print_err(res: Result<(), Report>) {
    match res {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

pub fn format_name_logo(num: u32) -> String {
    format!("Square{num}x{num}Logo.png")
}

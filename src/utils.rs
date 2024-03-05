use std::path::Path;

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

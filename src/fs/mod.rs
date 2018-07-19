use std::path::Path;

/// Ensure existing file
pub fn is_file(path: String) -> Result<(), String> {
    if Path::new(&path).exists() {
        return Ok(());
    }
    Err(format!("{} is not an existing file", &path))
}

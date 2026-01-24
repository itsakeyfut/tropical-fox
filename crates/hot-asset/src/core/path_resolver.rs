use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Path not found: {0}")]
    NotFound(PathBuf),

    #[error("Failed to canonicalize path: {0}")]
    Canonicalize(std::io::Error),
}

pub struct PathResolver {
    assets_dir: PathBuf,
}

impl PathResolver {
    pub fn new() -> Self {
        Self {
            assets_dir: PathBuf::from("assets"),
        }
    }

    pub fn with_assets_dir(assets_dir: PathBuf) -> Self {
        Self { assets_dir }
    }

    pub fn resolve(&self, relative_path: &str) -> Result<PathBuf, PathError> {
        let path_buf = PathBuf::from(relative_path);

        // If absolute, use as-is
        if path_buf.is_absolute() {
            return Ok(path_buf);
        }

        // Otherwise, resolve relative to assets/
        let full_path = self.assets_dir.join(&path_buf);

        if !full_path.exists() {
            return Err(PathError::NotFound(full_path));
        }

        Ok(full_path)
    }
}

impl Default for PathResolver {
    fn default() -> Self {
        Self::new()
    }
}

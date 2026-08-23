use std::{fs};

use crate::{Detector, manifest_discovery};

pub struct ProjectBoundary {
    pub root: String,
}

impl Default for ProjectBoundary {
    fn default() -> Self {
        let root = String::new();
        Self { root }
    }
}

impl Detector for ProjectBoundary {
    type Output = Self;
    fn detect(request: &crate::ScanRequest) -> Result<Self::Output, String> {
        let mut boundary = ProjectBoundary::default();

        let (manifest, dotnet_manifest) = manifest_discovery();

        if let Some(path) = request.target_path.clone() {
            let canonical = fs::canonicalize(path.clone()).unwrap_or_else(|_| path.clone().to_path_buf());
            for ancestor in canonical.ancestors() {
                if let Ok(entries) = fs::read_dir(ancestor) {
                    for entry in entries.flatten() {
                        if let Some(file_name) = entry.file_name().to_str() {
                            if
                            (
                                manifest.contains(file_name) ||
                                dotnet_manifest.iter().any(|&f| file_name.ends_with(f))
                            ) &&
                                entry.path().is_file()
                            {
                                boundary.root = ancestor.to_string_lossy().to_string();
                                return Ok(boundary);
                            };
                        }
                    }
                }
            };
            return Err(format!("\tCan't find project boundary for path: {:?}", canonical));
        }
        Err(format!("\tCan't specify root project boundary for --target path: {:?}", request.target_path.clone()))
    }
}
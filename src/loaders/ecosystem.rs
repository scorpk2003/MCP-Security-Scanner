use std::{fs::{ReadDir, read_dir}, path::PathBuf};

use crate::{Detector, ProjectBoundary};


#[derive(Debug, Clone)]
pub struct Ecosystem {
    pub language: Language,
    pub dependencies: Vec<String>,
}

impl Default for Ecosystem {
    fn default() -> Self {
        let language = Language::DEFAULT;
        let dependencies = Vec::new();
        Self { language, dependencies }
    }
}

impl Ecosystem {
    fn dependencies_discovery(&mut self, entries: ReadDir) -> Result<(), String> {
        match self.language {
            Language::DONTRECONIZE => {
                return Err(format!("\t[Project Manifest] Language not recognized, can't detect dependencies!!!"));
            },
            Language::DEFAULT => {
                return Err(format!("\t[Project Manifest] Language not detected, can't detect dependencies!!!"));
            },
            _ => {}
        }
        Ok(())
    }

    fn language_detect(&mut self, entries: ReadDir) {
        let mut path = entries.flatten();
        match self.language {
            Language::PYTHON => {
                if let Some(pyproject) = path.find(|x| x.file_name().to_str() == Some("pyproject.toml")) {
                    self.dependencies.push(pyproject.path().to_string_lossy().to_string());
                }
            },
            _ => {}
        }
    }
}

impl Detector for Ecosystem {
    type Output = Self;
    fn detect(request: &crate::ScanRequest) -> Result<Self::Output, String> {
        let mut ecosystem = Ecosystem::default();
        let boundary = match ProjectBoundary::detect(request) {
            Ok(bound) => bound,
            Err(e) => return Err(e),
        };

        let bound_path = PathBuf::from(boundary.root.clone());
        let path_entries = match read_dir(bound_path){
            Ok(entries) => entries,
            Err(e) => return Err(format!("\t[Project Manifest] Read directory failed: `  {:?}  `", e)),
        };
        let (manifest, dotnet_manifest) = crate::manifest_discovery();
        for entry in path_entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if manifest.contains(file_name) {
                    ecosystem.language = match file_name {
                        "Cargo.toml" => Language::RUST,
                        "package.json" => Language::JAVASCRIPT,
                        "pom.xml" => Language::JAVA,
                        "build.gradle" => Language::JAVA,
                        "requirements.txt" => Language::PYTHON,
                        "go.mod" => Language::GO,
                        "CMakeLists.txt" => Language::CPLUSPLUS,
                        _ => Language::DONTRECONIZE,
                    };
                } else if dotnet_manifest.iter().any(|&f| file_name.ends_with(f)) {
                    ecosystem.language = Language::CSHARP;
                }
            }
        }
        Ok(ecosystem)
    }
}

#[derive(Debug, Clone)]
pub enum Language {
    DEFAULT,
    RUST,
    PYTHON,
    JAVASCRIPT,
    JAVA,
    CSHARP,
    GO,
    CPLUSPLUS,
    DONTRECONIZE,
}
use std::{
    fs::{ReadDir, read_dir},
    path::PathBuf,
};

use aho_corasick::AhoCorasick;

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
        Self {
            language,
            dependencies,
        }
    }
}

impl Ecosystem {
    pub fn ecosystem_detect(&mut self, entries: ReadDir) {
        let path = entries
            .flatten()
            .map(|f| f.path().to_string_lossy().to_string())
            .collect::<Vec<_>>();
        let patterns = match self.language.language_ecosystem() {
            Ok(ecosystem) => ecosystem,
            Err(e) => {
                panic!("Cant specified ecosytem: \n{}", e);
            }
        };

        match self.language {
            Language::CSHARP => {
                let file_name = path
                    .iter()
                    .filter(|&f| patterns.iter().any(|&file| f.ends_with(file)))
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>();

                self.dependencies = file_name;
            }
            _ => {
                let haystack = path.join(" ");
                let builder = AhoCorasick::builder()
                    .ascii_case_insensitive(true)
                    .match_kind(aho_corasick::MatchKind::LeftmostFirst)
                    .build(patterns)
                    .unwrap();

                for matched in builder.find_iter(&haystack) {
                    let matched_pattern = patterns[matched.pattern()];
                    self.dependencies.push(matched_pattern.to_string());
                }
            }
        };
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
        let path_entries = match read_dir(bound_path) {
            Ok(entries) => entries,
            Err(e) => {
                return Err(format!(
                    "\t[Project Manifest] Read directory failed: `  {:?}  `",
                    e
                ));
            }
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
                        _ => {
                            println!("\n[Ecosystem] Language not recognized: {:?}\n", file_name);
                            Language::DONTRECONIZE
                        }
                    };
                } else if dotnet_manifest.iter().any(|&f| file_name.ends_with(f)) {
                    ecosystem.language = Language::CSHARP;
                }
            }
            match ecosystem.language {
                Language::DEFAULT => continue,
                _ => break,
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

impl Language {
    pub fn language_ecosystem(&self) -> Result<&[&'static str], String> {
        match self {
            Language::CPLUSPLUS => Ok(&[
                "CMakeLists.txt",
                "vcpkg.json",
                "conanfile.txt",
                "Makefile",
                "xmake.lua",
            ]),
            Language::PYTHON => Ok(&[
                "pyproject.toml",
                "requirements.txt",
                "uv.lock",
                "setup.py",
                "setup.cfg",
                "Pipfile",
                "Poetry.lock",
            ]),
            Language::JAVASCRIPT => Ok(&[
                "package.json",
                "yarn.lock",
                "pnpm-lock.yaml",
                "package-lock.json",
                "tsconfig.json",
                "bun.lockb",
            ]),
            Language::RUST => Ok(&["Cargo.toml", "Cargo.lock"]),
            Language::GO => Ok(&["go.mod", "go.sum"]),
            Language::JAVA => Ok(&[
                "pom.xml",
                "build.gradle",
                "build.gradle.kts",
                "settings.gradle",
                "settings.gradle.kts",
            ]),
            Language::CSHARP => Ok(&[".csproj", ".sln"]),
            _ => Err(format!(
                "\t[Project Manifest] Language not recognized, can't detect dependencies!!!"
            )),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_language_detect() {
        use super::Detector;
        use crate::cli::{request::ScanRequest, schemas::Cli};
        use clap::Parser;

        let args = vec!["mcp-security", "scan", "--target", "src/test/c#/"];
        let args = match Cli::try_parse_from(args) {
            Ok(cli) => match cli.command {
                crate::schemas::Cmd::Scan(req) => req,
                _ => panic!("Expected Scan command"),
            },
            Err(e) => panic!("Error parsing CLI arguments: {}", e),
        };

        let request = ScanRequest::mapping_args(&args);
        let mut ecosystem = match super::Ecosystem::detect(&request) {
            Ok(ecosystem) => ecosystem,
            Err(e) => panic!("Error detecting ecosystem: {}", e),
        };
        let entries = match std::fs::read_dir(request.target_path.unwrap()) {
            Ok(entries) => entries,
            Err(e) => panic!("Error reading directory: {}", e),
        };
        ecosystem.ecosystem_detect(entries);

        println!("\nEcosystem Detect Result: \n\t{:?}\n", ecosystem);
    }
}

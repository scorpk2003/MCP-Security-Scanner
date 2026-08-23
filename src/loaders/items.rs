use std::collections::HashSet;

pub const PROJECT_BOUNDARY_DISCOVERY: &[&str] = &[
    ".git",
    "pom.xml",
    "package.json",
    "Cargo.toml",
    "pyproject.toml",
    "requirements.txt",
    "Go.mod",
    "build.gradle",
    "CMakeLists.txt",
];

pub const DOTNET_DISCOVERY: &[&str] = &[
    ".csproj",
    ".sln",
];

pub const BONUS_DISCOVERY: &[&str] = &[
    ".gitignore",
    "dockerfile",
    "docker-compose",
    ".git",
];

pub fn manifest_discovery() -> (HashSet<&'static str>, HashSet<&'static str>) {
    let manifest = PROJECT_BOUNDARY_DISCOVERY.iter().cloned().collect::<HashSet<&str>>();
    let dotnet_manifest = DOTNET_DISCOVERY.iter().cloned().collect::<HashSet<&str>>();
    (manifest, dotnet_manifest)
}
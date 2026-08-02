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
use log::debug;
use std::fs::read_to_string;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Represents the contents of a Cargo.toml file.
#[derive(serde::Deserialize)]
pub struct CargoToml {
    pub(crate) file: Option<PathBuf>,
    /// The name of the cargo package.
    pub name: String,
    /// The description of the cargo package.
    pub description: Option<String>,
    /// The version of the cargo package.
    pub version: String,
}

/// Intermediate structure to deserialize the `package` section of Cargo.toml.
#[derive(serde::Deserialize)]
struct DefaultCargoToml {
    package: CargoToml,
}

impl CargoToml {
    /// Creates a new `CargoToml` instance from a given Cargo.toml file.
    ///
    /// # Arguments
    ///
    /// * `file` - A path reference to the Cargo.toml file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn Error>>` - On success, returns an instance of `CargoToml`.
    ///   On failure, returns an error wrapped in a `Box<dyn Error>`.
    pub fn new(file: impl AsRef<Path>) -> anyhow::Result<Self> {
        // Read the contents of the file.
        let content = read_to_string(&file)?;

        // Deserialize the TOML content into `DefaultCargoToml`.
        let data: DefaultCargoToml = toml::from_str(&content)?;

        // Construct and return a `CargoToml` instance.
        Ok(Self {
            file: Some(file.as_ref().to_owned()),
            name: data.package.name,
            description: data.package.description,
            version: data.package.version,
        })
    }

    pub fn increment_version(&mut self) -> anyhow::Result<()> {
        let mut version_parts = self.version.split('.');
        let mut major: u32 = version_parts
            .next()
            .ok_or(anyhow::Error::msg(
                "Failed to parse the major string to u32",
            ))?
            .parse()
            .unwrap_or(0);
        let mut minor: u32 = version_parts
            .next()
            .ok_or(anyhow::Error::msg(
                "Failed to parse the minor string to u32",
            ))?
            .parse()
            .unwrap_or(0);
        let mut patch: u32 = version_parts
            .next()
            .ok_or(anyhow::Error::msg(
                "Failed to parse the patch string to u32",
            ))?
            .parse()
            .unwrap_or(0);
        if patch + 1 > 9 {
            patch = 0;
            if minor + 1 > 9 {
                minor = 0;
                major = major.wrapping_add(1);
            } else {
                minor += 1;
            }
        } else {
            patch += 1;
        }
        self.version = format!("{}.{}.{}", major, minor, patch);
        debug!("New version: {}", self.version);
        if let Some(file) = &self.file {
            let file_content = read_to_string(file)?;
            let version_pattern = regex::Regex::new(r#"(?m)^version\s*=\s*"[^"]*""#)?;
            let new_version = format!(r#"version = "{}""#, self.version);
            let updated_content = version_pattern.replace(&file_content, new_version);
            let mut file = std::fs::File::create(file)?;
            file.write_all(updated_content.as_bytes())?;
        }

        Ok(())
    }

    pub fn create_tag(&self) -> anyhow::Result<()> {
        let tag_name = format!("v{}", self.version);
        let tag_message = format!("Version {}", self.version);
        let tag_result = std::process::Command::new("git")
            .arg("tag")
            .arg("-a")
            .arg(&tag_name)
            .arg("-m")
            .arg(&tag_message)
            .output()?;

        if !tag_result.status.success() {
            return Err(anyhow::Error::msg("Failed to create tag!"));
        }
        Ok(())
    }
}

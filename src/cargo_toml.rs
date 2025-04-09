use std::fs::read_to_string;
use std::path::Path;

/// Represents the contents of a Cargo.toml file.
pub struct CargoToml {
    /// The name of the cargo package.
    pub name: String,
    /// The description of the cargo package
    pub description: Option<String>,
}

/// Intermediate structure to deserialize the `package` section of Cargo.toml.
#[derive(serde::Deserialize)]
struct DefaultCargoToml {
    package: Package,
}

/// Structure to represent the package section in Cargo.toml.
#[derive(serde::Deserialize)]
struct Package {
    /// The name of the package.
    name: String,
    /// The description of the cargo package
    description: Option<String>,
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
    ///    On failure, returns an error wrapped in a `Box<dyn Error>`.
    pub fn new(file: impl AsRef<Path>) -> anyhow::Result<Self> {
        // Read the contents of the file.
        let content = read_to_string(file)?;

        // Deserialize the TOML content into `DefaultCargoToml`.
        let data: DefaultCargoToml = toml::from_str(&content)?;

        // Construct and return a `CargoToml` instance.
        Ok(Self {
            name: data.package.name,
            description: data.package.description,
        })
    }
}

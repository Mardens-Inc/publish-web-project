use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

/// Represents the contents of a Cargo.toml file.
pub struct CargoToml {
	/// The name of the cargo package.
	pub name: String,
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
	pub fn new(file: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
		// Read the contents of the file.
		let content = match read_to_string(file) {
			Ok(content) => content,
			Err(e) => return Err(Box::new(e)),
		};

		// Deserialize the TOML content into `DefaultCargoToml`.
		let data: DefaultCargoToml = match toml::from_str(&content) {
			Ok(data) => data,
			Err(e) => return Err(Box::new(e)),
		};

		// Construct and return a `CargoToml` instance.
		Ok(Self {
			name: data.package.name,
		})
	}
}
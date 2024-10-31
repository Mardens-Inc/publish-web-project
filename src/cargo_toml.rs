use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;

pub struct CargoToml {
	pub name: String,
}

#[derive(serde::Deserialize)]
struct DefaultCargoToml {
	package: Package
}

#[derive(serde::Deserialize)]
struct Package {
	name: String,
}


impl CargoToml {
	pub fn new(file: impl AsRef<Path>) -> Result<Self, Box<dyn Error>>
	{
		let content = match read_to_string(file)
		{
			Ok(content) => content,
			Err(e) => return Err(Box::new(e))
		};
		let data: DefaultCargoToml = match toml::from_str(&*content)
		{
			Ok(data) => data,
			Err(e) => return Err(Box::new(e))
		};

		Ok(Self {
			name: data.package.name,
		})
	}
}
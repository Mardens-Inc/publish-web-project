use clap::Parser;

#[derive(Debug, Parser)]
#[command(
	version,
	about = "A CLI for automating the publishing process for Actix-web projects to the Mardens Ubuntu server.",
	name = "Publish Web Project"
)]
pub struct PWPArgs {
	/// Hostname or IP address of the server
	#[arg(short='j', long)]
	pub host: String,

	/// Username for SSH authentication
	#[arg(short, long)]
	pub username: String,

	/// Port number for SSH connection
	#[arg(short, long)]
	pub port: u16,

	/// Path to the authentication file (e.g., private key)
	#[arg(short, long = "auth")]
	pub auth_file: Option<String>,

	/// Password for SSH authentication (conflicts with auth_file)
	#[arg(short, long, conflicts_with = "auth")]
	pub password: Option<String>,

	/// Name of the binary to be published
	#[arg(short, long = "binary")]
	pub binary_name: Option<String>,

	/// Name of the systemd service to be managed
	#[arg(short, long = "service")]
	pub service_name: Option<String>,

	/// Flag indicating whether to build before publishing
	#[arg(short, long)]
	pub build: bool,

	/// Command to build the project (used if 'build' is true)
	#[arg(short, long, default_value = "cargo build --release", requires = "build")]
	pub build_command: String,
}
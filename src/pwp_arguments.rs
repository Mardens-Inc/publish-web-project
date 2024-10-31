use clap::Parser;

#[derive(Debug, Parser)]
#[command(
	version,
	about = "A cli for automating the publishing process for actix-web projects to the mardens ubuntu server.",
	name = "Publish Web Project"
)]
pub struct PWPArgs {
	#[arg(short, long)]
	pub host: String,
	#[arg(short, long)]
	pub username: String,
	#[arg(short, long)]
	pub port: u16,
	#[arg(short, long = "auth")]
	pub auth_file: Option<String>,
	#[arg(short, long, conflicts_with = "auth")]
	pub password: Option<String>,
	#[arg(short, long = "binary")]
	pub binary_name: Option<String>,
	#[arg(short, long = "service")]
	pub service_name: Option<String>,
	#[arg(short, long)]
	pub build: bool,
	#[arg(short, long, default_value = "cargo build --release", requires = "build")]
	pub build_command: String,
}
use clap::Args;

#[derive(Debug, Args)]
pub struct PWPArgs {
	#[clap(short, long)]
	pub host: String,
	#[clap(short, long)]
	pub port: u16,
	#[clap(short, long)]
	pub auth_file: String,
	#[clap(short, long)]
	pub service_file: Option<String>,

}
use crate::pwp_arguments::PWPArgs;
use clap::Parser;
use log::{debug, error, info, warn};
use pretty_env_logger::env_logger;
use std::os::windows::process::CommandExt;

mod pwp_arguments;
mod cargo_toml;
mod ssh;

fn main() {
	std::env::set_var("RUST_LOG", "trace");
	env_logger::init();
	info!("Starting up");
	let args = PWPArgs::parse();
	let mut session = ssh::create_connection(&args).unwrap();
	if let Some(ref service) = args.service_name
	{
		debug!("Attempting to stop service: {}", service);
		let stop_command = format!("sudo systemctl stop {}", service);
		let output = ssh::execute_command(stop_command, &mut session);
		match output
		{
			Ok(output) => debug!("Output: {}", output),
			Err(e) => debug!("Error: {}", e),
		}
	}

	if args.build {
		warn!("Building binary");
		debug!("Executing command: {}", args.build_command);
		let build_cmd_parts = &mut args.build_command.split_whitespace();
		let command_executable = match build_cmd_parts.next() {
			Some(command) => command,
			None => {
				error!("No executable found in build command: {}", args.build_command);
				std::process::exit(1);
			},
		};

		let build_args = &args.build_command[command_executable.len()..];

		match std::process::Command::new(command_executable)
			.raw_arg(&build_args)
			.output()
		{
			Ok(output) => {
				if output.status.success() {
					info!("Build successful");
				} else {
					error!("Build failed: {}", String::from_utf8_lossy(&output.stderr));
					std::process::exit(1);
				}
			},
			Err(e) => {
				error!("Error building binary: {}", e);
				std::process::exit(1);
			},
		}
	}

	let binary_name = args.binary_name.unwrap_or_else(|| cargo_toml::CargoToml::new("./Cargo.toml").unwrap().name);
	let binary_path = format!("./target/release/{}", binary_name);
	let remote_path = format!("/usr/bin/{}", binary_name);
	ssh::upload_file(binary_path, remote_path, &mut session).unwrap();
	if let Some(service) = args.service_name
	{
		debug!("Attempting to start service: {}", service);
		let start_command = format!("sudo systemctl start {}", service);
		let output = ssh::execute_command(start_command, &mut session);
		match output
		{
			Ok(output) => debug!("Output: {}", output),
			Err(e) => debug!("Error: {}", e),
		}
	}
}

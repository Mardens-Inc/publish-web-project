use crate::pwp_arguments::PWPArgs;
use clap::Parser;
use log::{debug, error, info, warn};
use pretty_env_logger::env_logger;
use std::process::Command;

mod pwp_arguments;
mod cargo_toml;
mod ssh;

fn main() {
	// Parse CLI arguments.
	let args = PWPArgs::parse();

	// Initialize logging with the trace level.
	std::env::set_var("RUST_LOG", "trace");
	env_logger::init();
	info!("Starting up");


	// Establish an SSH connection using the parsed arguments.
	let mut session = ssh::create_connection(&args).unwrap();

	// If the build flag is set, build the binary using the specified build command.
	if args.build {
		warn!("Building binary");
		debug!("Executing command: {}", args.build_command);

		// Execute the build command and handle the result.
		match Command::new("cmd")
			.args(["/C", &args.build_command])
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


	// Determine the binary name, either from the arguments or from Cargo.toml.
	let binary_name = args.binary_name.unwrap_or_else(|| cargo_toml::CargoToml::new("./Cargo.toml").unwrap().name);
	let binary_path = format!("./target/release/{}", binary_name);
	let remote_path = format!("/home/{}/.local/bin/", args.username);


	match ssh::execute_command(format!("mkdir {}", remote_path), &mut session) {
		Ok(_) => info!("Directory created"),
		Err(e) => {
			if e.to_string().contains("File exists") {
				info!("Directory already exists");
			} else {
				debug!("Error: {}", e);
			}
		},
	}

	// If a service name is provided, attempt to stop the service on the remote server.
	if let Some(ref service) = &args.service_name {
		debug!("Attempting to stop service: {}", service);
		let stop_command = format!("sudo systemctl stop {}", service);
		let output = ssh::execute_command(stop_command, &mut session);
		match output {
			Ok(output) => {
				if output.is_empty() {
					info!("Service started successfully");
				} else {
					error!("Service failed to start: {}", output);
				}
			},
			Err(e) => debug!("Error: {}", e),
		}
	}

	// Upload the binary to the remote server.
	match ssh::upload_file(binary_path, format!("{}{}", remote_path, &binary_name), &mut session){
		Ok(_) => info!("Binary uploaded successfully"),
		Err(e) => error!("Error uploading binary: {}", e),
	}

	// If a service name is provided, attempt to start the service on the remote server.
	if let Some(service) = args.service_name {
		debug!("Attempting to start service: {}", service);
		let start_command = format!("sudo systemctl start {}", service);
		let output = ssh::execute_command(start_command, &mut session);
		match output {
			Ok(output) => {
				if output.is_empty() {
					info!("Service started successfully");
				} else {
					error!("Service failed to start: {}", output);
				}
			},
			Err(e) => debug!("Error: {}", e),
		}
	}
}

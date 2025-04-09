use crate::pwp_arguments::PWPArgs;
use clap::Parser;
use log::{debug, error, info, warn, LevelFilter};
use pretty_env_logger::env_logger;
use std::path::Path;
use std::process::Command;

mod cargo_toml;
mod pwp_arguments;
mod service_file;
mod ssh;
use crate::service_file::ServiceFile;
use anyhow::Result;

fn main() -> Result<()> {
    // Parse CLI arguments.
    let args = PWPArgs::parse();

    // Initialize logging with the trace level.
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(LevelFilter::Trace)
        .init();
    info!("Starting up");

    let cargo = cargo_toml::CargoToml::new("./Cargo.toml")?;

    // Establish an SSH connection using the parsed arguments.
    let mut session = ssh::create_connection(&args)?;

    // If the build flag is set, build the binary using the specified build command.
    if args.build {
        warn!("Building binary");
        debug!("Executing command: {}", args.build_command);

        // Execute the build command and handle the result.
        let output = Command::new("cmd")
            .args(["/C", &args.build_command])
            .output()?;

        if output.status.success() {
            info!("Build successful");
        } else {
            error!("Build failed: {}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
    }

    // Determine the binary name, either from the arguments or from Cargo.toml.
    let binary_name = args.binary_name.unwrap_or(cargo.name);
    let binary_path = format!("./target/release/{}", binary_name);
    let remote_path = format!("/home/{}/.local/bin/", args.username);

    if let Err(e) = ssh::execute_command(format!("mkdir {}", remote_path), &mut session) {
        if e.to_string().contains("File exists") {
            info!("Directory already exists");
        } else {
            debug!("Error: {}", e);
        }
    } else {
        info!("Directory created");
    }

    // If a service name is provided, attempt to stop the service on the remote server.
    if let Some(service) = &args.service_name {
        debug!("Attempting to stop service: {}", service);
        let stop_command = format!("sudo systemctl stop {}", service);
        let output = ssh::execute_command(stop_command, &mut session);

        match output {
            Ok(output) => {
                if output.is_empty() {
                    info!("Service stopped successfully");
                } else {
                    error!("Service failed to stop: {}", output);
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    // Upload the binary to the remote server.
    ssh::upload_file(
        &binary_path,
        format!("{}{}", remote_path, &binary_name),
        &mut session,
    )?;

    // If a service name is provided, attempt to start the service on the remote server.
    if let Some(service) = args.service_name {
        if args.install_service {
            let username = args.username;
            let working_dir = args
                .working_directory
                .unwrap_or(format!("/home/{}/.local/{}", username, &binary_name));

            let service_file = ServiceFile::new(
                cargo.description.unwrap_or(service.clone()),
                working_dir,
                binary_path,
            );

            service_file.install(&service, &mut session)?;
        }

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
            }
            Err(e) => debug!("Error: {}", e),
        }
    }
    Ok(())
}

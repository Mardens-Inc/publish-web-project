use anyhow::Result;
use crate::pwp_arguments::PWPArgs;
use log::debug;
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

/// Creates an SSH connection using the provided arguments.
///
/// # Arguments
///
/// * `args` - A reference to `PWPArgs`, which contains all required connection parameters.
///
/// # Returns
///
/// A result containing an `ssh2::Session` if successful.
pub fn create_connection(args: &PWPArgs) -> Result<Session> {
	debug!("Creating connection to {}:{}", args.host, args.port);
	let host = &args.host;
	let port = args.port;
	let username = &args.username;

	// Establish a TCP connection to the host and port.
	let tcp = TcpStream::connect(format!("{}:{}", host, port))?;

	// Create a new SSH session and set its underlying TCP stream.
	let mut session = Session::new()?;
	session.set_tcp_stream(tcp);
	session.handshake()?;

	// Authenticate using either a public key file or a password.
	if let Some(auth_file) = &args.auth_file {
		session.userauth_pubkey_file(username, None, auth_file.as_ref(), args.password.as_deref())?;
	} else if let Some(password) = &args.password {
		session.userauth_password(username, password)?;
	}

	Ok(session)
}

/// Uploads a file to a remote server using an existing SSH session.
///
/// # Arguments
///
/// * `local_path` - The local path of the file to be uploaded.
/// * `remote_path` - The remote path where the file should be uploaded.
/// * `session` - A mutable reference to an existing `ssh2::Session`.
///
/// # Returns
///
/// A result indicating success.
pub fn upload_file(local_path: impl AsRef<Path>, remote_path: impl AsRef<Path>, session: &mut Session) -> Result<()> {
	debug!("Uploading file from {:?} to {:?}", local_path.as_ref(), remote_path.as_ref());
	let local_path = local_path.as_ref();
	let remote_path = remote_path.as_ref();

	// Create an SFTP session.
	let sftp = session.sftp().map_err(|e| anyhow::anyhow!("Unable to create sftp connection: {:?}", e))?;

	// Open the remote file for writing.
	let mut remote_file = sftp.create(remote_path).map_err(|e| anyhow::anyhow!("Failed to create remote path: {:?} - {:?}", remote_path, e))?;

	// Open the local file for reading.
	let mut local_file = std::fs::File::open(local_path).map_err(|e| anyhow::anyhow!("Failed to read local file: {:?} - {:?}", local_path, e))?;

	// Copy the contents of the local file to the remote file.
	std::io::copy(&mut local_file, &mut remote_file).map_err(|e| anyhow::anyhow!("Failed to copy file from local filesystem to remote filesystem: {:?} -> {:?} - {:?}", local_path, remote_path, e))?;
	execute_command(format!(r#"chmod +x "{}""#, remote_path.to_str().unwrap()), session)?;
	Ok(())
}

/// Executes a command on a remote server using an existing SSH session and returns the output.
///
/// # Arguments
///
/// * `command` - The command to be executed on the remote server.
/// * `session` - A mutable reference to an existing `ssh2::Session`.
///
/// # Returns
///
/// A result containing the command's output as a `String`.
pub fn execute_command(command: impl AsRef<str>, session: &mut Session) -> Result<String> {
	debug!("Executing command: `{}`", command.as_ref());

	// Open a new channel for the command execution.
	let mut channel = session.channel_session()?;
	channel.exec(command.as_ref())?;

	// Read the command's output into a string.
	let mut s = String::new();
	channel.read_to_string(&mut s)?;
	Ok(s)
}
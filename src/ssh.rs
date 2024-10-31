use crate::pwp_arguments::PWPArgs;
use log::debug;
use ssh2::Session;
use std::error::Error;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

pub fn create_connection(args: &PWPArgs) -> Result<Session, Box<dyn Error>>
{
	debug!("Creating connection to {}:{}", args.host, args.port);
	let host = &args.host;
	let port = args.port;
	let username = &args.username;
	let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
	let mut session = Session::new()?;
	session.set_tcp_stream(tcp);
	session.handshake()?;


	if let Some(auth_file) = &args.auth_file
	{
		session.userauth_pubkey_file(&username, None, auth_file.as_ref(), args.password.as_deref())?;
	} else if let Some(password) = &args.password
	{
		session.userauth_password(&username, &password)?;
	}

	Ok(session)
}

pub fn upload_file(local_path: impl AsRef<Path>, remote_path: impl AsRef<Path>, session: &mut Session) -> Result<(), Box<dyn Error>>
{
	debug!("Uploading file from {:?} to {:?}", local_path.as_ref(), remote_path.as_ref());
	let sftp = session.sftp()?;
	let mut remote_file = sftp.create(remote_path.as_ref())?;
	let mut local_file = std::fs::File::open(local_path)?;
	std::io::copy(&mut local_file, &mut remote_file)?;
	Ok(())
}

pub fn execute_command(command: String, session: &mut Session) -> Result<String, Box<dyn Error>>
{
	debug!("Executing command: `{}`", command);
	let mut channel = session.channel_session()?;
	channel.exec(&command)?;
	let mut s = String::new();
	channel.read_to_string(&mut s)?;
	Ok(s)
}
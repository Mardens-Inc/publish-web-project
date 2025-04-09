use clap::Parser;

#[derive(Debug, Parser)]
#[command(
	version,
	about = "A CLI for automating the publishing process for Actix-web projects to the Mardens Ubuntu server.",
	name = "Publish Web Project",
	styles=get_styles(),
)]
pub struct PWPArgs {
    /// Hostname or IP address of the server
    #[arg(short = 'H', long)]
    pub host: String,

    /// Username for SSH authentication
    #[arg(short, long)]
    pub username: String,

    /// Port number for SSH connection
    #[arg(short, long, default_value = "22")]
    pub port: u16,

    /// Path to the authentication file (e.g., private key)
    #[arg(short, long = "auth")]
    pub auth_file: Option<String>,

    /// Password for SSH authentication (conflicts with auth_file)
    #[arg(short = 'P', long, conflicts_with = "auth_file")]
    pub password: Option<String>,

    /// Name of the binary to be published
    #[arg(short, long = "binary")]
    pub binary_name: Option<String>,

    /// Name of the systemd service to be managed
    #[arg(short, long = "service")]
    pub service_name: Option<String>,

    /// Flag indicating whether to build before publishing
    #[arg(long)]
    pub build: bool,

    /// Flag indicating whether to create a linux service.
    #[arg(long, requires = "service_name")]
    pub install_service: bool,

    /// Sets the working directory in the service file,
    /// and the default will be the same path as the uploaded binary.
    #[arg(long, requires = "install_service")]
    pub working_directory: Option<String>,

    /// Command to build the project (used if 'build' is true)
    #[arg(long, default_value = "cargo build --release", requires = "build")]
    pub build_command: String,
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightCyan))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightBlue))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

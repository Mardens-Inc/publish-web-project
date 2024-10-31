
# Publish Web Project

## Table of Contents

- [Usage](#usage)
- [Building](#building)

## Usage

This section covers how to use the CLI executable once it is built.

```sh
Publish Web Project

A CLI for automating the publishing process for Actix-web projects to the Mardens Ubuntu server.

Usage: pwp --host <HOST> --username <USERNAME> --port <PORT> [OPTIONS]

Options:
  -j, --host <HOST>              Hostname or IP address of the server
  -u, --username <USERNAME>      Username for SSH authentication
  -p, --port <PORT>              Port number for SSH connection
  -a, --auth-file <auth-file>    Path to the authentication file (e.g., private key)
  -P, --password <PASSWORD>      Password for SSH authentication (conflicts with auth-file)
  -b, --binary <binary>          Name of the binary to be published
  -s, --service <service>        Name of the systemd service to be managed
  --build                        Flag indicating whether to build before publishing
  --build-command <BUILD_COMMAND> Command to build the project (default: "cargo build --release")
  -h, --help                         Print help information
  -V, --version                      Print version information
```

To use the CLI, run it with the appropriate options. For example:

```sh
pwp --host 192.168.1.100 --username user --port 22 --auth-file ~/.ssh/id_rsa --binary my_app --service my_service --build
```

This command will:
- Connect to the server at `192.168.1.100` on port `22`.
- Authenticate using the private key located at `~/.ssh/id_rsa`.
- Build the binary named `my_app` if the `--build` flag is provided.
- Upload the binary to the remote server.
- Stop the systemd service `my_service` on the remote server.
- Start the service after uploading the binary.

## Building

To build the project for personal use, follow these steps:

1. **Install Rust:**
   Make sure you have Rust installed on your system. You can install Rust by following the instructions at [rustup.rs](https://rustup.rs).

2. **Clone the Repository:**
   ```sh
   git clone <repository-url>
   cd <repository-directory>
   ```

3. **Build the Project:**
   Use the following command to build the project in release mode:
   ```sh
   cargo build --release
   ```

4. **Run the Executable:**
   The built executable will be located in the `./target/release/` directory. You can run it by specifying the necessary arguments as shown in the [Usage](#usage) section.

```sh
./target/release/pwp --host 192.168.1.100 --username user --port 22 --auth-file ~/.ssh/id_rsa --binary my_app --service my_service --build
```
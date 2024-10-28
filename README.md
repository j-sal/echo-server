# Echo Server for Windows

This `echo_server` application is a simple HTTP server built with Rust for Windows that listens on a specified port and echoes back the content of any HTTP requests it receives.

## Features
- Listens on `localhost` (127.0.0.1) on a specified port (default: 3000).
- Echoes back HTTP POST request bodies.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- For displaying `pretty_env_logger` output on your console, run the command `$env:RUST_LOG="info"` prior to running the application. Note: If the script `install_script-JS-EchoServerInstaller.nsi` is used to pack the binary into an installer, then this step is will be done automatically.

## Build Instructions

To build the `echo_server` application, follow these steps:

1. **Clone the Repository**:
    `git clone https://github.com/your-username/echo_server.git && cd echo_server`
2. **Build it**:
    For an optimized executable, build in release mode with
    `cargo build --release`
    Which will be found in the target/release directory `target/release/echo_server`.
3. **Run it**:
    `cargo run`
4. **Send a request for it to echo**:
    On a separate terminal, PowerShell, or Git Bash window, run
    `curl -X POST http://127.0.0.1:3000 -d "Hello, Echo Server!"`
5. **Test it**:
    `cargo test`

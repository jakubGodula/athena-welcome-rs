# Athena Welcome (Rust Port)

A modern welcome application for Athena OS, written in Rust using GTK4.

## Features

- Modern GTK4-based user interface
- Role-based system configuration
- Multi-page navigation with smooth transitions
- System information display
- Credits page with project information
- HTB integration

## Requirements

- Rust (latest stable)
- GTK4 development libraries
- System with X11 or Wayland display server

## Building

```bash
# Clone the repository
git clone https://github.com/jakubGodula/athena-welcome-rs.git
cd athena-welcome-rs

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Configuration

The application uses a configuration file located at `/etc/athena-welcome/roles.conf` for managing user roles and preferences.

## Development Status

Current development stage: 0.1

### Known Issues
- HTB button functionality needs implementation
- Image scaling requires optimization
- UI quality of service improvements pending

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

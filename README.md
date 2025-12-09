# DNsight

A Windows GUI application for easily managing DNS server settings on your network adapter.

## Features

- **Quick DNS Provider Selection**: Choose from popular DNS providers:
  - Electro (78.157.42.100 / 78.157.42.101)
  - Radar (10.202.10.10 / 10.202.10.11)
  - Shekan (178.22.122.100 / 185.51.200.2)
  - Bogzar (185.55.226.26 / 185.55.225.25)
  - Quad9 (9.9.9.9 / 149.112.112.112)
- **Custom DNS Configuration**: Set any custom DNS servers
- **Save Custom DNS Entries**: Save and manage your custom DNS configurations
- **Clear DNS Settings**: Revert to automatic/default DNS configuration
- **Test DNS**: Verify your current DNS server configuration
- **Modern GUI**: Clean, transparent interface built with egui
- **Real-time Ping Monitoring**: Monitor network latency

## Requirements

- Windows 7 SP1 or later (64-bit)
- Administrator privileges (required to modify network settings)

## Installation

1. Download the installer from the releases page
2. Run `DNsight-Setup-1.0.0.exe`
3. Follow the installation wizard
4. Launch DNsight from the Start Menu or desktop shortcut

**Note**: The application requires administrator privileges to modify network settings. You may need to run the installer as administrator.

## Usage

1. Launch DNsight (requires administrator privileges)
2. Select a DNS provider from the list or configure custom DNS servers
3. Click "Set DNS" to apply the changes
4. Use "Clear DNS" to revert to automatic DNS configuration
5. Use "Test DNS" to verify your current DNS settings
6. Save custom DNS entries for quick access later

## Building from Source

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))
- Windows SDK

### Build Steps

```bash
# Clone the repository
git clone <repository-url>
cd DNsight

# Build in release mode
cargo build --release
```

The executable will be in `target/release/dnsight.exe`

### Creating an Installer

**Quick Method:**
```powershell
.\build-release.ps1
```

**Manual Method:**
1. Install [Inno Setup](https://jrsoftware.org/isinfo.php)
2. Download VC++ Redistributable to `redist\vc_redist.x64.exe` (see `redist\README.md`)
3. Open `dnsight.iss` in Inno Setup Compiler
4. Build the installer (F9)
5. The installer will be created in the `installer/` directory

**For updating and publishing new versions, see `UPDATE.md`**

## Version

1.0.0

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

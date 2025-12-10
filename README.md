# DNsight

A Windows GUI application for easily managing DNS server settings on your network adapter.

## 📥 Download

**Get the latest release from the [GitHub Releases page](https://github.com/RezaPourdast/DNsight/releases)**

### Latest Release (v1.0.0)

- **Installer** (Recommended): [Download DNsight-Setup-1.0.0.exe](https://github.com/RezaPourdast/DNsight/releases/download/v1.0.0/DNsight-Setup-1.0.0.exe)
- **Portable ZIP**: [Download DNsight-1.0.0-portable.zip](https://github.com/RezaPourdast/DNsight/releases/download/v1.0.0/DNsight-1.0.0-portable.zip)

📦 Visit the [Releases page](https://github.com/RezaPourdast/DNsight/releases) for all versions, release notes, and changelog.

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

### Using the Installer (Recommended)

1. Download `DNsight-Setup-1.0.0.exe` from the releases page
2. Run the installer (it will request administrator privileges)
3. Follow the installation wizard
4. Launch DNsight from the Start Menu or desktop shortcut

### Using the Portable Version

1. Download `DNsight-1.0.0-portable.zip` from the releases page
2. Extract the ZIP file to any location
3. Run `DNsight.exe` (it will automatically request administrator privileges)

**Note**: The application requires administrator privileges to modify network settings. Windows will prompt you for elevation when launching the app.

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

## Disclaimer

**DNS Provider Information**: The DNS providers listed in this application (Electro, Radar, Shekan, Bogzar, Quad9) are third-party services. DNsight is not affiliated with, endorsed by, or sponsored by any DNS provider. The application simply provides a convenient interface to configure publicly available DNS servers. Users are responsible for their DNS configuration choices and should review each provider's terms of service and privacy policy before use.

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

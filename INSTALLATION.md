# Systrix Installation Guide

## Quick Install

### From GitHub (Recommended)

```bash
# Clone the repository
git clone https://github.com/Kazeku-06/systrix.git
cd systrix

# Build release binary
cargo build --release

# The binary will be at:
# - Linux/macOS: ./target/release/systrix
# - Windows: .\target\release\systrix.exe
```

### From Source (Alternative)

If you already have the source code:

```bash
cd systrix
cargo build --release
```

## System Requirements

### Minimum Requirements
- **Rust**: 1.70 or later
- **RAM**: 512 MB
- **Disk**: 50 MB for binary + dependencies
- **OS**: Linux, macOS, or Windows

### Build Tools Required

**Linux:**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"

# Arch
sudo pacman -S base-devel
```

**macOS:**
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

**Windows:**
- Option 1: [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
  - Select "Desktop development with C++"
- Option 2: [MSYS2](https://www.msys2.org/) with MinGW

## Installing Rust

If you don't have Rust installed:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
- Download from: https://rustup.rs/
- Run the installer
- Restart terminal

Verify installation:
```bash
rustc --version
cargo --version
```

## Building Systrix

### Standard Build

```bash
# Navigate to project directory
cd systrix

# Build release version (optimized)
cargo build --release

# Build with all features
cargo build --release --all-features
```

### Feature Flags

```bash
# TUI only (default)
cargo build --release --features tui

# With GPU monitoring (requires NVIDIA drivers)
cargo build --release --features gpu

# With remote agent
cargo build --release --features remote

# With dynamic plugins
cargo build --release --features dynamic-plugins

# All features
cargo build --release --all-features
```

## Installing to System

### Linux

**Option 1: Install to /usr/local/bin (system-wide)**
```bash
sudo cp target/release/systrix /usr/local/bin/
sudo chmod +x /usr/local/bin/systrix
systrix version
```

**Option 2: Install to ~/.local/bin (user-only)**
```bash
mkdir -p ~/.local/bin
cp target/release/systrix ~/.local/bin/
# Add to PATH if not already (add to ~/.bashrc or ~/.zshrc)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
systrix version
```

**Option 3: Using cargo install**
```bash
cargo install --path .
systrix version
```

### macOS

Same as Linux:
```bash
# System-wide
sudo cp target/release/systrix /usr/local/bin/
systrix version

# User-only
mkdir -p ~/.local/bin
cp target/release/systrix ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
systrix version
```

### Windows

**Option 1: Add to System PATH (Permanent)**
```powershell
# Run PowerShell as Administrator
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
$systrixPath = "D:\local\systrix\target\release"
$newPath = "$currentPath;$systrixPath"
[Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")

# Restart terminal, then:
systrix version
```

**Option 2: Add to User PATH (No Admin)**
```powershell
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
$systrixPath = "D:\local\systrix\target\release"
$newPath = "$currentPath;$systrixPath"
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")

# Restart terminal, then:
systrix version
```

**Option 3: Copy to Windows directory**
```powershell
# Run as Administrator
Copy-Item target\release\systrix.exe C:\Windows\System32\
systrix version
```

## Verifying Installation

After installation, verify it works:

```bash
# Check version
systrix version

# Test system info
systrix info

# Test process list
systrix ps --limit 5

# Test TUI (press 'q' to quit)
systrix tui
```

## Updating Systrix

To update to the latest version:

```bash
# Navigate to project directory
cd systrix

# Pull latest changes
git pull origin main

# Rebuild
cargo build --release

# Reinstall (if installed to system)
sudo cp target/release/systrix /usr/local/bin/  # Linux/macOS
# or
# Copy to PATH on Windows
```

## Uninstalling

### If installed to system

**Linux/macOS:**
```bash
sudo rm /usr/local/bin/systrix
# or
rm ~/.local/bin/systrix
```

**Windows:**
```powershell
# Remove from PATH (reverse of installation)
# or
Remove-Item C:\Windows\System32\systrix.exe
```

### If using from project directory

Simply delete the project folder:
```bash
rm -rf systrix  # Linux/macOS
```
```powershell
Remove-Item -Recurse -Force systrix  # Windows
```

## Troubleshooting Installation

### Build Errors

**Error: `rustc` not found**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Error: `link.exe` not found (Windows)**
```
Install Visual Studio Build Tools:
https://visualstudio.microsoft.com/downloads/
Select "Desktop development with C++"
```

**Error: `cc` not found (Linux)**
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf groupinstall "Development Tools"
```

### Permission Errors

**Linux/macOS:**
```bash
# Make binary executable
chmod +x target/release/systrix

# Or use sudo for system-wide install
sudo cp target/release/systrix /usr/local/bin/
```

**Windows:**
```powershell
# Run PowerShell as Administrator
# Right-click PowerShell → "Run as Administrator"
```

### PATH Issues

**Linux/macOS:**
```bash
# Check if directory is in PATH
echo $PATH

# Add to PATH temporarily
export PATH="/path/to/systrix/target/release:$PATH"

# Add to PATH permanently (add to ~/.bashrc or ~/.zshrc)
echo 'export PATH="/path/to/systrix/target/release:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows:**
```powershell
# Check PATH
$env:Path

# Add to PATH temporarily
$env:Path += ";D:\local\systrix\target\release"

# For permanent, use Environment Variables GUI or PowerShell commands above
```

## Platform-Specific Notes

### Linux
- Full feature support
- No special requirements for basic functionality
- GPU monitoring requires NVIDIA drivers with NVML
- SMART data requires `smartmontools` package

### macOS
- Core features fully supported
- Some disk I/O metrics limited
- GPU monitoring not available
- May require Xcode Command Line Tools

### Windows
- Core features fully supported
- Requires Visual Studio Build Tools or MinGW
- Some process metrics limited
- GPU monitoring limited

## Next Steps

After installation:

1. Read [RUNNING.md](RUNNING.md) for usage instructions
2. Read [QUICKSTART.md](QUICKSTART.md) for quick start
3. Read [EXAMPLES.md](EXAMPLES.md) for usage examples
4. Check [README.md](README.md) for full documentation

## Getting Help

- **Repository**: https://github.com/Kazeku-06/systrix
- **Issues**: https://github.com/Kazeku-06/systrix/issues
- **Documentation**: See README.md and other .md files

---

**Happy Monitoring! 🚀**

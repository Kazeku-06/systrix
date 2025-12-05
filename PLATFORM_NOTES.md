# Platform-Specific Notes and Limitations

## Overview

Systrix is designed to be cross-platform, but some features and metrics vary by operating system due to platform-specific APIs and capabilities.

## Linux

### Fully Supported Features
- ✅ CPU monitoring (usage, frequency, per-core stats)
- ✅ Memory monitoring (RAM, swap)
- ✅ Disk monitoring (partitions, usage, I/O)
- ✅ Network monitoring (interfaces, traffic)
- ✅ Process management (list, kill, suspend, resume)
- ✅ Process details (CPU, memory, disk I/O, threads)

### Optional Features
- ⚠️ **SMART disk data**: Requires `smartmontools` package
  ```bash
  # Install on Ubuntu/Debian
  sudo apt install smartmontools
  
  # Install on Fedora/RHEL
  sudo dnf install smartmontools
  ```

- ⚠️ **GPU monitoring**: Requires NVIDIA GPU with drivers
  ```bash
  # Verify NVML is available
  nvidia-smi
  
  # Build with GPU support
  cargo build --release --features gpu
  ```

### Permissions
- Most metrics available to regular users
- Process killing may require elevated privileges for processes owned by other users
- SMART data requires root access

### Known Issues
- None currently

## macOS

### Fully Supported Features
- ✅ CPU monitoring (usage, frequency, cores)
- ✅ Memory monitoring (RAM, swap)
- ✅ Disk monitoring (partitions, usage)
- ✅ Network monitoring (interfaces, traffic)
- ✅ Process listing

### Limited Features
- ⚠️ **Disk I/O metrics**: Limited granularity compared to Linux
- ⚠️ **Process disk I/O**: May not be available for all processes
- ⚠️ **CPU temperature**: Not available through standard APIs

### Unsupported Features
- ❌ **GPU monitoring**: Not implemented (no NVML on macOS)
- ❌ **SMART data**: Not implemented
- ⚠️ **Process suspend/resume**: Limited support

### Permissions
- Most metrics available to regular users
- Process killing requires appropriate permissions
- Some system processes protected by SIP (System Integrity Protection)

### Known Issues
- CPU frequency may not update in real-time on Apple Silicon
- Some process metrics may be unavailable due to macOS security restrictions

## Windows

### Fully Supported Features
- ✅ CPU monitoring (usage, cores)
- ✅ Memory monitoring (RAM)
- ✅ Disk monitoring (partitions, usage)
- ✅ Network monitoring (interfaces, traffic)
- ✅ Process listing

### Limited Features
- ⚠️ **Process details**: Some metrics limited compared to Linux
- ⚠️ **Disk I/O**: Limited granularity
- ⚠️ **CPU frequency**: May not be accurate on all systems
- ⚠️ **Process user**: May show SID instead of username

### Unsupported Features
- ❌ **SMART data**: Not implemented
- ❌ **Process suspend/resume**: Not supported on Windows
- ⚠️ **GPU monitoring**: Limited support

### Permissions
- Run as Administrator for full functionality
- Process killing requires appropriate privileges
- Some system processes cannot be killed

### Known Issues
- Terminal rendering may have issues with some console configurations
- Unicode characters may not display correctly in older terminals
- Process CPU usage calculation may differ from Task Manager

## Cross-Platform Assumptions

### Data Collection
- **Refresh rate**: Minimum 100ms enforced on all platforms
- **Process sampling**: May take 200-500ms for accurate CPU usage
- **Caching**: Expensive operations cached for 5 seconds by default

### Terminal Support
- Requires terminal with ANSI color support
- Minimum terminal size: 80x24 characters
- Unicode support recommended but not required

### Performance
- **Target idle CPU**: 3-5% on modern hardware
- **Memory footprint**: 10-20 MB typical
- **Disk I/O**: Minimal (only for config and reports)

## Feature Availability Matrix

| Feature | Linux | macOS | Windows |
|---------|-------|-------|---------|
| CPU Usage | ✅ | ✅ | ✅ |
| CPU Frequency | ✅ | ⚠️ | ⚠️ |
| CPU Temperature | ⚠️ | ❌ | ❌ |
| Memory Usage | ✅ | ✅ | ✅ |
| Swap Usage | ✅ | ✅ | ⚠️ |
| Disk Usage | ✅ | ✅ | ✅ |
| Disk I/O | ✅ | ⚠️ | ⚠️ |
| SMART Data | ⚠️ | ❌ | ❌ |
| Network Traffic | ✅ | ✅ | ✅ |
| Process List | ✅ | ✅ | ✅ |
| Process CPU | ✅ | ✅ | ⚠️ |
| Process Memory | ✅ | ✅ | ✅ |
| Process Disk I/O | ✅ | ⚠️ | ⚠️ |
| Kill Process | ✅ | ✅ | ✅ |
| Suspend/Resume | ✅ | ⚠️ | ❌ |
| GPU Monitoring | ⚠️ | ❌ | ⚠️ |

Legend:
- ✅ Fully supported
- ⚠️ Partial support or requires additional setup
- ❌ Not supported

## Troubleshooting

### Linux

**Issue**: Permission denied when accessing process information
```bash
# Solution: Run with sudo for full access
sudo systrix tui
```

**Issue**: GPU not detected
```bash
# Verify NVIDIA drivers
nvidia-smi

# Check NVML library
ldconfig -p | grep libnvidia-ml
```

### macOS

**Issue**: Process metrics incomplete
```
This is expected on macOS due to security restrictions.
Some metrics are only available to root or processes with
special entitlements.
```

**Issue**: Terminal rendering issues
```bash
# Try different terminal emulator
# iTerm2 recommended for best compatibility
```

### Windows

**Issue**: Unicode characters not displaying
```
Use Windows Terminal or ConEmu for better Unicode support.
Legacy cmd.exe has limited Unicode support.
```

**Issue**: Permission denied
```
Run PowerShell or Command Prompt as Administrator:
Right-click → "Run as administrator"
```

## Building for Specific Platforms

### Linux Static Binary
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### macOS Universal Binary
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo -create -output systrix \
  target/x86_64-apple-darwin/release/systrix \
  target/aarch64-apple-darwin/release/systrix
```

### Windows
```bash
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

## Testing on Different Platforms

### Automated Testing
CI runs tests on:
- Ubuntu 22.04 (Linux)
- Windows Server 2022
- macOS 12 (Monterey)

### Manual Testing Checklist
- [ ] Basic CLI commands work
- [ ] TUI launches and displays correctly
- [ ] Process list shows current processes
- [ ] Kill command works (on non-critical process)
- [ ] Terminal properly restored on exit
- [ ] No crashes or panics during normal operation

## Future Improvements

### Planned
- Better GPU support on Windows
- Temperature monitoring on more platforms
- Battery status for laptops
- Container/VM detection

### Under Consideration
- FreeBSD support
- Android/Termux support
- Web-based UI (via remote agent)

# Build and Release Guide

## Building from Source

### Standard Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With all features
cargo build --release --all-features
```

### Feature Flags

```bash
# TUI only (default)
cargo build --release --features tui

# With GPU monitoring
cargo build --release --features gpu

# With remote agent
cargo build --release --features remote

# With dynamic plugins
cargo build --release --features dynamic-plugins

# All features
cargo build --release --all-features
```

## Cross-Compilation

### Using `cross`

Install cross:
```bash
cargo install cross
```

Build for different targets:
```bash
# Linux x86_64
cross build --release --target x86_64-unknown-linux-gnu

# Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu

# Windows x86_64
cross build --release --target x86_64-pc-windows-gnu

# macOS x86_64 (requires macOS host)
cargo build --release --target x86_64-apple-darwin

# macOS ARM64 (Apple Silicon)
cargo build --release --target aarch64-apple-darwin
```

### Static Linking (Linux)

For a fully static binary on Linux:

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Build static binary
cargo build --release --target x86_64-unknown-linux-musl
```

## Release Process

### 1. Update Version

Update version in `Cargo.toml`:
```toml
[package]
version = "0.2.0"
```

### 2. Build Release Binaries

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/systrix

# Windows
cargo build --release --target x86_64-pc-windows-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
```

### 3. Create Archives

```bash
# Linux
tar -czf systrix-v0.2.0-linux-x86_64.tar.gz \
  -C target/x86_64-unknown-linux-gnu/release systrix

# Windows
zip systrix-v0.2.0-windows-x86_64.zip \
  target/x86_64-pc-windows-gnu/release/systrix.exe

# macOS
tar -czf systrix-v0.2.0-macos-x86_64.tar.gz \
  -C target/x86_64-apple-darwin/release systrix
```

### 4. Generate Checksums

```bash
sha256sum systrix-v0.2.0-*.tar.gz > checksums.txt
sha256sum systrix-v0.2.0-*.zip >> checksums.txt
```

## Installation

### From Binary

```bash
# Download and extract
tar -xzf systrix-v0.2.0-linux-x86_64.tar.gz

# Move to PATH
sudo mv systrix /usr/local/bin/

# Verify
systrix version
```

### From Source

```bash
cargo install --path .
```

### From crates.io (when published)

```bash
cargo install systrix
```

## Platform-Specific Notes

### Linux

- No special requirements
- For GPU monitoring: Install NVIDIA drivers with NVML support
- For SMART data: Install `smartmontools`

### macOS

- Some metrics may be limited compared to Linux
- GPU monitoring not available

### Windows

- Some process metrics may be limited
- Run as Administrator for full process management capabilities
- GPU monitoring limited

## Optimization

The release profile in `Cargo.toml` is optimized for size and performance:

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
strip = true         # Strip symbols
```

## Troubleshooting

### Build Errors

**Error: `ratatui` not found**
- Solution: Enable the `tui` feature: `cargo build --features tui`

**Error: NVML not found**
- Solution: Install NVIDIA drivers or disable GPU feature

**Error: Cross-compilation fails**
- Solution: Use `cross` instead of `cargo`

### Runtime Issues

**Permission denied when killing processes**
- Solution: Run with elevated privileges (sudo/admin)

**GPU not detected**
- Solution: Install NVIDIA drivers with NVML support

## Performance Profiling

```bash
# Build with debug symbols
cargo build --release --profile release-with-debug

# Profile with perf (Linux)
perf record -g ./target/release/systrix tui
perf report

# Profile with Instruments (macOS)
instruments -t "Time Profiler" ./target/release/systrix tui
```

# Acceptance Criteria

## Build & Compilation

- [x] `cargo build --release` completes without errors
- [x] `cargo build --all-features` completes without errors
- [x] Project compiles on Rust 1.70+
- [x] No clippy warnings with `cargo clippy -- -D warnings`

## Testing

- [x] `cargo test` passes all tests
- [x] Unit tests cover core monitoring logic
- [x] Integration tests for CLI commands
- [x] Mock-based tests for process management

## CLI Commands

### `systrix info`
- [x] Displays CPU information (model, cores, usage)
- [x] Displays memory information (total, used, available)
- [x] Displays disk information (total, used, available)
- [x] Displays system uptime
- [x] Displays OS name and version
- [x] Output is human-readable

### `systrix ps`
- [x] Lists running processes
- [x] Shows PID, user, name, CPU%, memory%, threads
- [x] Supports `--sort` flag (cpu, mem, io)
- [x] Supports `--filter` flag for name filtering
- [x] Supports `--limit` flag for result count
- [x] Lists at least the systrix process itself

### `systrix kill <pid>`
- [x] Attempts to kill specified process
- [x] Supports `--signal` flag (SIGTERM, SIGKILL)
- [x] Requires confirmation by default
- [x] Supports `--force` flag to skip confirmation
- [x] Prevents killing PID 1 without --force
- [x] Shows appropriate error for permission denied
- [x] Returns success/failure message

### `systrix net`
- [x] Lists network interfaces
- [x] Shows received/transmitted bytes
- [x] Shows current transfer rates

### `systrix disk`
- [x] Lists disk partitions
- [x] Shows mount points and filesystem types
- [x] Shows total, used, available space
- [x] Shows usage percentage

### `systrix report`
- [x] Exports system snapshot to JSON
- [x] Supports `--output` flag for file path
- [x] Generated JSON is valid and parseable
- [x] Includes timestamp

### `systrix version`
- [x] Displays version number
- [x] Lists enabled features

## TUI Mode

### Basic Functionality
- [x] `systrix tui` opens full-screen TUI
- [x] TUI displays without crashing
- [x] Terminal is properly restored on exit
- [x] Supports `--refresh-interval` flag

### Overview Panel
- [x] Shows CPU usage gauge
- [x] Shows memory usage gauge
- [x] Shows disk usage gauge
- [x] Shows network statistics
- [x] Shows system information (model, cores, uptime, OS)

### Processes Panel
- [x] Lists running processes in table format
- [x] Shows PID, user, name, CPU%, memory%, threads
- [x] Processes are selectable with arrow keys
- [x] Selected process is highlighted
- [x] Displays total process count

### Navigation
- [x] `q` quits the application
- [x] `1-5` switches between panels
- [x] `Tab` cycles through panels
- [x] `↑↓` navigates process list
- [x] `PageUp/PageDown` scrolls by page
- [x] `Home/End` jumps to top/bottom

### Actions
- [x] `k` shows kill confirmation modal
- [x] `p` pauses/resumes refresh
- [x] `t` toggles theme
- [x] `Esc` cancels modal/action

### Themes
- [x] Dark theme implemented
- [x] Light theme implemented
- [x] Dracula theme implemented
- [x] Theme toggle works

## Performance

- [ ] TUI idle CPU usage <= 5% (manual verification required)
- [x] Refresh interval configurable
- [x] Minimum refresh interval clamped to 100ms
- [x] No blocking operations in UI thread

## Error Handling

- [x] Graceful handling of missing data
- [x] Permission errors show helpful messages
- [x] Invalid PIDs handled appropriately
- [x] Missing features show "N/A" instead of crashing

## Cross-Platform

- [x] Compiles on Linux
- [x] Compiles on Windows
- [x] Compiles on macOS
- [x] Platform-specific features have fallbacks
- [x] Platform limitations documented

## Documentation

- [x] README.md with installation instructions
- [x] README.md with usage examples
- [x] README.md with keyboard shortcuts
- [x] BUILD.md with build instructions
- [x] BUILD.md with cross-compilation guide
- [x] CONTRIBUTING.md with development guide
- [x] Code comments on complex logic
- [x] Cargo.toml with proper metadata

## CI/CD

- [x] GitHub Actions workflow configured
- [x] CI runs on push and PR
- [x] CI tests on Linux, Windows, macOS
- [x] CI runs cargo test
- [x] CI runs cargo clippy
- [x] CI builds release artifacts

## Optional Features

### GPU Monitoring (feature flag)
- [ ] Detects NVIDIA GPU via NVML
- [ ] Shows GPU usage, memory, temperature
- [ ] Falls back gracefully if not available

### Remote Agent (feature flag)
- [ ] HTTP server starts on configured port
- [ ] WebSocket support for real-time updates
- [ ] Token-based authentication
- [ ] JSON API endpoints

### Dynamic Plugins (feature flag)
- [ ] Plugin trait defined
- [ ] Plugin registry implemented
- [ ] Example plugin included
- [ ] Safety warnings documented

## Known Limitations

### Linux
- SMART data requires smartctl (optional)
- Some operations require root privileges

### macOS
- Limited disk I/O metrics
- GPU monitoring not available
- Some process metrics limited

### Windows
- SMART data not available
- Some process metrics limited
- Suspend/resume not supported
- GPU monitoring limited

## Manual Testing Checklist

- [ ] Run `systrix info` and verify output
- [ ] Run `systrix ps` and verify process list
- [ ] Run `systrix tui` and navigate all panels
- [ ] Test killing a non-critical process
- [ ] Test theme switching in TUI
- [ ] Test pause/resume in TUI
- [ ] Verify terminal restoration after crash (Ctrl+C)
- [ ] Test on Linux
- [ ] Test on Windows
- [ ] Test on macOS

# Systrix Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         SYSTRIX                                 │
│                   System Monitor CLI + TUI                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
        ┌─────────────────────────────────────────┐
        │           main.rs (Entry Point)         │
        │  - Parse CLI arguments (clap)           │
        │  - Initialize logging (tracing)         │
        │  - Dispatch to CLI or TUI               │
        └─────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                ▼                           ▼
    ┌───────────────────────┐   ┌───────────────────────┐
    │   CLI Mode (cli.rs)   │   │   TUI Mode (app.rs)   │
    │  - info               │   │  - Full-screen UI     │
    │  - ps                 │   │  - Event loop         │
    │  - kill               │   │  - Panel management   │
    │  - net                │   │  - Keyboard handling  │
    │  - disk               │   └───────────────────────┘
    │  - report             │               │
    │  - version            │               ▼
    └───────────────────────┘   ┌───────────────────────┐
                │               │   TUI Components      │
                │               │  - ui.rs (rendering)  │
                │               │  - event.rs (input)   │
                │               │  - panels/ (views)    │
                │               └───────────────────────┘
                │                           │
                └───────────┬───────────────┘
                            ▼
            ┌───────────────────────────────────┐
            │   MonitorBackend Trait            │
            │  (monitor/mod.rs)                 │
            │  - cpu_snapshot()                 │
            │  - memory_snapshot()              │
            │  - disk_snapshot()                │
            │  - network_snapshot()             │
            │  - process_list()                 │
            └───────────────────────────────────┘
                            │
                            ▼
            ┌───────────────────────────────────┐
            │   SysinfoBackend                  │
            │  (Real Implementation)            │
            └───────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        ▼                   ▼                   ▼
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│  cpu.rs      │   │  memory.rs   │   │  disk.rs     │
│  - Usage     │   │  - RAM       │   │  - Partitions│
│  - Frequency │   │  - Swap      │   │  - Usage     │
│  - Cores     │   │  - Available │   │  - I/O       │
└──────────────┘   └──────────────┘   └──────────────┘
        ▼                   ▼                   ▼
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│  network.rs  │   │  process.rs  │   │  utils.rs    │
│  - Interfaces│   │  - List      │   │  - Formatting│
│  - Traffic   │   │  - Kill      │   │  - Helpers   │
│  - Rates     │   │  - Suspend   │   │              │
└──────────────┘   └──────────────┘   └──────────────┘
```

## Component Layers

### Layer 1: Entry Point
```
main.rs
├── Initialize tracing/logging
├── Parse CLI arguments (clap)
└── Dispatch to CLI or TUI
```

### Layer 2: User Interface
```
CLI (cli.rs)                    TUI (app.rs + tui/)
├── info command                ├── Application state
├── ps command                  ├── Event loop
├── kill command                ├── UI rendering
├── net command                 ├── Panel management
├── disk command                └── Keyboard handling
├── report command
└── version command
```

### Layer 3: Business Logic
```
MonitorBackend Trait
├── cpu_snapshot() -> CpuSnapshot
├── memory_snapshot() -> MemorySnapshot
├── disk_snapshot() -> DiskSnapshot
├── network_snapshot() -> NetworkSnapshot
└── process_list() -> Vec<ProcessInfo>

ProcessManager Trait
├── kill_process(pid, signal)
├── suspend_process(pid)
└── resume_process(pid)
```

### Layer 4: Data Collection
```
SysinfoBackend (monitor/)
├── cpu.rs - CPU metrics
├── memory.rs - Memory metrics
├── disk.rs - Disk metrics
├── network.rs - Network metrics
└── process.rs - Process management
```

### Layer 5: System APIs
```
sysinfo crate
├── System::new_all()
├── System::refresh_*()
└── Platform-specific implementations
```

## Data Flow

### CLI Command Flow
```
User Input
    │
    ▼
main.rs (parse args)
    │
    ▼
cli.rs (execute command)
    │
    ▼
SysinfoBackend (collect data)
    │
    ▼
monitor/* (system APIs)
    │
    ▼
Format & Display
    │
    ▼
stdout
```

### TUI Event Loop
```
┌─────────────────────────────────────┐
│  TUI Event Loop (app.rs)            │
│                                     │
│  1. Poll for events (event.rs)     │
│     ├── Keyboard input              │
│     └── Timeout (refresh interval)  │
│                                     │
│  2. Handle events                   │
│     ├── Navigation (arrows, Tab)    │
│     ├── Actions (k, p, t)           │
│     └── Quit (q, Ctrl+C)            │
│                                     │
│  3. Update data (if not paused)     │
│     └── Call MonitorBackend         │
│                                     │
│  4. Render UI (ui.rs)               │
│     ├── Header (system summary)     │
│     ├── Tabs (panel selection)      │
│     ├── Active panel                │
│     └── Footer (shortcuts)          │
│                                     │
│  5. Repeat                          │
└─────────────────────────────────────┘
```

## Module Dependencies

```
main.rs
├── cli.rs
│   ├── monitor/mod.rs
│   │   ├── cpu.rs
│   │   ├── memory.rs
│   │   ├── disk.rs
│   │   ├── network.rs
│   │   └── process.rs
│   └── utils.rs
│
├── app.rs (feature: tui)
│   ├── tui/ui.rs
│   │   ├── tui/panels/overview.rs
│   │   ├── tui/panels/processes.rs
│   │   ├── tui/panels/network.rs
│   │   ├── tui/panels/disk.rs
│   │   └── tui/panels/gpu.rs
│   ├── tui/event.rs
│   ├── monitor/mod.rs
│   └── utils.rs
│
├── plugins.rs
└── remote_agent.rs (feature: remote)
```

## Async Architecture

```
Tokio Runtime
├── Main Task (TUI event loop or CLI command)
│   ├── Spawns: Data collection tasks
│   │   ├── CPU sampling (200ms delay for accuracy)
│   │   ├── Memory snapshot
│   │   ├── Disk snapshot
│   │   └── Network snapshot
│   │
│   └── Spawns: UI rendering task (TUI only)
│       └── Renders at refresh_interval
│
└── Optional: Remote agent server (feature: remote)
    ├── HTTP endpoints
    └── WebSocket connections
```

## State Management

### TUI State (ui.rs)
```rust
struct Ui {
    // Navigation
    active_panel: usize,
    scroll_offset: usize,
    selected_process_index: usize,
    
    // Configuration
    theme: Theme,
    paused: bool,
    
    // Modal state
    show_modal: bool,
    modal_message: String,
    
    // Data cache
    cpu_data: Option<CpuSnapshot>,
    memory_data: Option<MemorySnapshot>,
    disk_data: Option<DiskSnapshot>,
    network_data: Option<NetworkSnapshot>,
    process_data: Vec<ProcessInfo>,
}
```

### Monitor Backend State
```rust
struct SysinfoBackend {
    system: Arc<Mutex<System>>,
}
```

## Error Handling Strategy

```
┌─────────────────────────────────────┐
│  Error Types                        │
├─────────────────────────────────────┤
│  anyhow::Error                      │
│  - Used for application errors      │
│  - Provides context and backtraces  │
│                                     │
│  Result<T, anyhow::Error>           │
│  - Standard return type             │
│  - Propagated with ?                │
└─────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────┐
│  Error Handling                     │
├─────────────────────────────────────┤
│  1. System API errors               │
│     └── Graceful fallback (N/A)     │
│                                     │
│  2. Permission errors               │
│     └── User-friendly message       │
│                                     │
│  3. Invalid input                   │
│     └── Validation + error message  │
│                                     │
│  4. TUI errors                      │
│     └── Restore terminal + exit     │
└─────────────────────────────────────┘
```

## Performance Considerations

### Caching Strategy
```
Expensive Operations (TTL: 5s)
├── GPU metrics (nvml-wrapper)
├── SMART data (smartctl)
└── Detailed disk I/O

Fast Operations (No cache)
├── CPU usage
├── Memory usage
├── Process list
└── Network stats
```

### Refresh Intervals
```
Default: 500ms
Minimum: 100ms (enforced)
Recommended:
├── High-frequency monitoring: 250-500ms
├── Normal monitoring: 500-1000ms
└── Low-overhead monitoring: 1000-2000ms
```

### Resource Budget
```
Target Performance:
├── CPU usage (idle): 3-5%
├── Memory footprint: 10-20 MB
├── Startup time: < 1 second
└── UI frame rate: 30-60 FPS
```

## Testing Architecture

```
Unit Tests (src/*)
├── monitor/cpu.rs
├── monitor/memory.rs
├── monitor/disk.rs
├── monitor/network.rs
├── monitor/process.rs
├── utils.rs
└── plugins.rs

Integration Tests (tests/*)
├── monitor_tests.rs
│   ├── test_cpu_snapshot
│   ├── test_memory_snapshot
│   ├── test_disk_snapshot
│   ├── test_network_snapshot
│   └── test_process_list
│
└── cli_tests.rs
    ├── test_version_command
    ├── test_info_command
    ├── test_ps_command
    ├── test_kill_command
    └── test_report_command

Mock Strategy:
├── MonitorBackend trait allows mocking
├── Dependency injection for testability
└── No real system operations in tests
```

## Plugin System Architecture

```
┌─────────────────────────────────────┐
│  Plugin Trait                       │
├─────────────────────────────────────┤
│  fn name() -> &str                  │
│  fn version() -> &str               │
│  fn init() -> Result<()>            │
│  fn execute(args) -> Result<String> │
│  fn cleanup() -> Result<()>         │
└─────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────┐
│  PluginRegistry                     │
├─────────────────────────────────────┤
│  plugins: HashMap<String, Plugin>   │
│                                     │
│  fn register(plugin)                │
│  fn get(name) -> Option<&Plugin>    │
│  fn execute(name, args)             │
└─────────────────────────────────────┘
        │
        ├── Compile-time plugins
        │   └── Built into binary
        │
        └── Dynamic plugins (feature: dynamic-plugins)
            └── Loaded via libloading
```

## Security Architecture

```
Process Management
├── Safety checks
│   ├── Prevent killing PID ≤ 1
│   ├── Require confirmation by default
│   └── Check permissions before action
│
├── Error handling
│   ├── Permission denied → helpful message
│   ├── Process not found → clear error
│   └── Invalid signal → validation
│
└── Audit trail
    └── Log all process management actions

Remote Agent (optional)
├── Token-based authentication
├── HTTPS recommended (not enforced)
├── CORS configuration
└── Rate limiting (TODO)
```

## Cross-Platform Abstraction

```
Platform-Specific Code
├── Linux
│   ├── Full feature support
│   ├── SMART data (via smartctl)
│   └── GPU monitoring (NVIDIA)
│
├── macOS
│   ├── Core features
│   ├── Limited disk I/O
│   └── No GPU monitoring
│
└── Windows
    ├── Core features
    ├── Limited process metrics
    └── No suspend/resume

Abstraction Strategy:
├── sysinfo crate handles platform differences
├── Feature detection at runtime
├── Graceful fallbacks for missing features
└── Platform notes in documentation
```

## Future Architecture Enhancements

### Planned
- [ ] Historical data storage (SQLite)
- [ ] Alert system with thresholds
- [ ] Web UI (via remote agent)
- [ ] Container/VM detection
- [ ] Battery monitoring

### Under Consideration
- [ ] Distributed monitoring (multiple agents)
- [ ] Plugin marketplace
- [ ] Custom dashboards
- [ ] Export to Prometheus/Grafana
- [ ] Mobile app (via remote agent)

---

**Architecture Version**: 1.0
**Last Updated**: 2025-12-05

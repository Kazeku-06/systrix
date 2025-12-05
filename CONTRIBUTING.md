# Contributing to Systrix

Thank you for your interest in contributing to Systrix!

## Development Setup

1. Install Rust 1.70 or later:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/systrix.git
   cd systrix
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new features
- Update documentation

## Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Update documentation
6. Submit a pull request

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_cpu_snapshot

# Run with logging
RUST_LOG=debug cargo test
```

## Architecture

See README.md for project structure and design decisions.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

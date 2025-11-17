# ui-test-rs

> A Rust-based CLI tool for UI testing with Playwright MCP integration

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

## Overview

`ui-test-rs` is a command-line tool that simplifies UI testing by integrating with Playwright's Model Context Protocol (MCP). It provides a clean, efficient interface for running web UI tests from the command line, with support for parallel execution, multiple output formats, and AI-assisted test authoring.

## Features

- **CLI-First Design**: Simple, intuitive command-line interface
- **Playwright MCP Integration**: Deterministic browser automation via accessibility tree
- **Multiple Output Formats**: Text, JSON, and JUnit XML reporting
- **Parallel Execution**: Run tests concurrently for faster results
- **Test Discovery**: Automatic test file discovery with configurable patterns
- **Configuration**: Flexible config via files, environment variables, or CLI flags
- **AI-Friendly**: Extended help with AI Coding Agent instructions

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/softwarewrighter/ui-test-rs
cd ui-test-rs

# Build and install
cargo build --release
sw-install -p .
```

### Using Cargo

```bash
cargo install ui-test-rs
```

### Pre-built Binaries

Download pre-built binaries from [GitHub Releases](https://github.com/softwarewrighter/ui-test-rs/releases).

## Prerequisites

- Rust toolchain (2024 edition)
- Node.js v20+ (for Playwright MCP)
- Playwright MCP server installed

### Install Playwright MCP

```bash
# For Claude Code users
claude mcp add playwright -s user -- npx -y @playwright/mcp

# Verify installation
npx playwright install chromium
```

## Quick Start

### Basic Usage

```bash
# Run all tests in current directory
ui-test-rs

# Run tests in specific directory
ui-test-rs tests/

# Run specific test file
ui-test-rs tests/login_test.rs

# Run with verbose output
ui-test-rs -v tests/

# Dry-run (preview without executing)
ui-test-rs --dry-run tests/
```

### Output Formats

```bash
# Text output (default)
ui-test-rs tests/

# JSON output
ui-test-rs --format json tests/ > results.json

# JUnit XML (for CI/CD)
ui-test-rs --format junit tests/ > junit.xml
```

### Parallel Execution

```bash
# Run with 4 parallel workers
ui-test-rs -j 4 tests/

# Run with as many workers as CPU cores
ui-test-rs -j 0 tests/
```

### Test Filtering

```bash
# Run only tests matching pattern
ui-test-rs --filter login tests/
```

## Writing Tests

Create test files with the naming pattern `*_test.rs` or `test_*.rs`:

```rust
// tests/login_test.rs

use ui_test_rs::prelude::*;

#[test_case("admin_login")]
async fn test_admin_login(ctx: &mut TestContext) -> Result<()> {
    // Navigate to login page
    ctx.navigate("https://example.com/login").await?;

    // Fill in credentials
    ctx.fill("username", "admin").await?;
    ctx.fill("password", "secret").await?;

    // Submit form
    ctx.click("button[type=submit]").await?;

    // Assert redirect to dashboard
    ctx.assert_url_contains("/dashboard").await?;

    Ok(())
}

#[test_case("failed_login")]
async fn test_failed_login(ctx: &mut TestContext) -> Result<()> {
    ctx.navigate("https://example.com/login").await?;
    ctx.fill("username", "wrong").await?;
    ctx.fill("password", "wrong").await?;
    ctx.click("button[type=submit]").await?;

    // Assert error message appears
    ctx.assert_text_visible("Invalid credentials").await?;

    Ok(())
}
```

## Configuration

### Config File

Create `ui-test.toml` in your project root:

```toml
# Test discovery patterns
[discovery]
patterns = ["*_test.rs", "test_*.rs"]
exclude = ["target/**", "node_modules/**"]

# Browser configuration
[browser]
type = "chromium"  # chromium, firefox, webkit
headless = true
viewport = { width = 1280, height = 720 }

# Playwright MCP settings
[playwright]
server_url = "npx -y @playwright/mcp"
timeout = 30000  # milliseconds

# Output settings
[output]
format = "text"  # text, json, junit
verbose = false
color = true

# Parallel execution
[execution]
jobs = 4
fail_fast = false
```

### Environment Variables

Override config with environment variables:

```bash
export UI_TEST_VERBOSE=1
export UI_TEST_FORMAT=json
export UI_TEST_BROWSER=firefox
export UI_TEST_HEADLESS=1
export UI_TEST_JOBS=4
```

### CLI Flags (Highest Priority)

```bash
ui-test-rs -v --format json -j 4 tests/
```

## Help and Version

### Short Help

```bash
ui-test-rs -h
```

### Extended Help (with AI Instructions)

```bash
ui-test-rs --help
```

### Version Information

```bash
ui-test-rs -V
ui-test-rs --version
```

## CI/CD Integration

### GitHub Actions

```yaml
name: UI Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install Playwright
        run: npx playwright install chromium
      - name: Install ui-test-rs
        run: cargo install ui-test-rs
      - name: Run tests
        run: ui-test-rs --format junit tests/ > junit.xml
      - name: Publish test results
        uses: EnricoMi/publish-unit-test-result-action@v2
        if: always()
        with:
          files: junit.xml
```

### GitLab CI

```yaml
test:
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y nodejs npm
    - npx playwright install chromium
    - cargo install ui-test-rs
  script:
    - ui-test-rs --format junit tests/ > junit.xml
  artifacts:
    reports:
      junit: junit.xml
```

## Examples

See the [examples/](examples/) directory for more test examples:

- `examples/basic/` - Simple navigation and form testing
- `examples/e2e/` - End-to-end user flows
- `examples/accessibility/` - Accessibility testing patterns
- `examples/visual/` - Screenshot-based testing

## Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/softwarewrighter/ui-test-rs
cd ui-test-rs

# Build
cargo build

# Run tests
cargo test

# Lint and format
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
```

### Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Process

This project follows strict quality standards:

- **TDD**: Red/Green/Refactor cycle for all features
- **Pre-Commit Gates**: Tests, linting, formatting, markdown validation
- **Standards Compliance**: Passes `sw-checklist` validation
- **Tech Debt Limits**: Max 3 TODOs per file, files under 500 lines

See [docs/process.md](docs/process.md) for detailed development workflow.

## Documentation

- [Product Requirements Document](docs/prd.md) - Feature specifications
- [Architecture](docs/architecture.md) - System design and components
- [Design Document](docs/design.md) - Implementation details
- [Implementation Plan](docs/plan.md) - Task breakdown and timeline
- [Project Status](docs/status.md) - Current progress and milestones
- [Development Process](docs/process.md) - Workflow and standards
- [Tools Guide](docs/tools.md) - Development tools reference

## Troubleshooting

### Playwright MCP Connection Failed

```bash
# Ensure Playwright MCP is installed
claude mcp list

# If not installed, add it:
claude mcp add playwright -s user -- npx -y @playwright/mcp

# Install browser
npx playwright install chromium
```

### Element Not Found

```bash
# Use verbose mode to see accessibility tree
ui-test-rs -v tests/failing_test.rs

# Take screenshot to debug
# (Add ctx.screenshot("debug.png") in your test)
```

### Tests Timeout

```bash
# Increase timeout in config
# ui-test.toml:
[playwright]
timeout = 60000  # 60 seconds
```

## License

Copyright (c) 2025 Michael A Wright

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/softwarewrighter/ui-test-rs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/softwarewrighter/ui-test-rs/discussions)
- **Documentation**: [GitHub Wiki](https://github.com/softwarewrighter/ui-test-rs/wiki)

## Acknowledgments

- [Playwright](https://playwright.dev/) - Browser automation framework
- [Playwright MCP](https://github.com/microsoft/playwright-mcp) - Model Context Protocol integration
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [clap](https://docs.rs/clap/) - Command-line argument parser

## Roadmap

### v0.1.0 (Current)
- [x] Basic CLI structure
- [x] Test discovery
- [ ] Playwright MCP integration
- [ ] Text, JSON, JUnit reporters
- [ ] Parallel execution

### v0.2.0 (Future)
- [ ] Visual regression testing
- [ ] Screenshot comparison
- [ ] Performance metrics
- [ ] HTML report generation

### v0.3.0 (Future)
- [ ] Test recorder
- [ ] Interactive test builder
- [ ] Plugin system
- [ ] Custom assertions library

---

**Status**: Development (v0.1.0-dev)

For the latest updates, see [docs/status.md](docs/status.md).

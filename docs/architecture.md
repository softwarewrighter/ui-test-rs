# Architecture Document

## Project: ui-test-rs

**Version**: 0.1.0
**Date**: 2025-11-16
**Author**: Michael A Wright

## 1. System Overview

### 1.1 Purpose
`ui-test-rs` is a Rust-based CLI tool for UI testing that integrates with Playwright MCP for browser automation. It provides a simple, efficient interface for running web UI tests from the command line.

### 1.2 High-Level Architecture

```
+-------------------+
|                   |
|   CLI Interface   |  <-- clap argument parsing
|   (main.rs)       |
|                   |
+---------+---------+
          |
          v
+-------------------+
|                   |
|  Test Runner      |  <-- Orchestrates test execution
|  (runner.rs)      |
|                   |
+---------+---------+
          |
          +------------------+------------------+
          |                  |                  |
          v                  v                  v
    +----------+      +------------+     +-------------+
    |          |      |            |     |             |
    | Test     |      | Playwright |     | Reporter    |
    | Loader   |      | MCP Client |     | (output)    |
    |          |      |            |     |             |
    +----------+      +------------+     +-------------+
          |                  |                  |
          v                  v                  v
    Test Files        Browser Actions      Results
```

## 2. Component Design

### 2.1 CLI Interface (`src/main.rs`)

**Responsibilities**:
- Parse command-line arguments with clap
- Validate input parameters
- Initialize configuration
- Invoke test runner
- Handle exit codes

**Key Types**:
```rust
#[derive(Parser)]
struct Cli {
    /// Path to test file or directory
    #[arg(default_value = ".")]
    test_path: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Dry-run mode (preview without executing)
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Output format: text, json, junit
    #[arg(long, default_value = "text")]
    format: OutputFormat,
}
```

**Dependencies**:
- `clap` for argument parsing
- `anyhow` for error handling

### 2.2 Configuration (`src/config.rs`)

**Responsibilities**:
- Load configuration from file (`ui-test.toml`)
- Merge with environment variables
- Apply command-line overrides
- Provide default values

**Key Types**:
```rust
#[derive(Debug, Deserialize)]
struct Config {
    /// Test discovery patterns
    patterns: Vec<String>,

    /// Browser configuration
    browser: BrowserConfig,

    /// Playwright MCP settings
    playwright: PlaywrightConfig,

    /// Output settings
    output: OutputConfig,
}
```

**Config Precedence** (highest to lowest):
1. Command-line flags
2. Environment variables
3. Config file
4. Default values

### 2.3 Test Loader (`src/loader.rs`)

**Responsibilities**:
- Discover test files by pattern
- Parse test file metadata
- Build test execution plan
- Filter tests by name/tag

**Key Types**:
```rust
struct TestLoader {
    patterns: Vec<String>,
    root_path: PathBuf,
}

struct TestSuite {
    name: String,
    file_path: PathBuf,
    tests: Vec<TestCase>,
}

struct TestCase {
    name: String,
    tags: Vec<String>,
    // Test function reference
}
```

**Test Discovery Algorithm**:
1. Start at root path
2. Apply glob patterns (`*_test.rs`, `test_*.rs`)
3. Parse test files for test functions
4. Build test suite hierarchy
5. Apply filters if specified

### 2.4 Test Runner (`src/runner.rs`)

**Responsibilities**:
- Execute tests in sequence or parallel
- Manage test lifecycle
- Handle timeouts and failures
- Collect results

**Key Types**:
```rust
struct TestRunner {
    config: Config,
    playwright: PlaywrightClient,
    reporter: Reporter,
}

struct TestResult {
    test_name: String,
    status: TestStatus,
    duration: Duration,
    error: Option<String>,
}

enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Error,
}
```

**Execution Flow**:
1. Initialize Playwright MCP connection
2. For each test:
   - Setup test environment
   - Execute test actions
   - Assert expectations
   - Cleanup
3. Collect and report results

### 2.5 Playwright MCP Client (`src/playwright.rs`)

**Responsibilities**:
- Connect to Playwright MCP server
- Send browser automation commands
- Receive accessibility snapshots
- Handle browser lifecycle

**Key Types**:
```rust
struct PlaywrightClient {
    connection: McpConnection,
    browser_config: BrowserConfig,
}

enum BrowserAction {
    Navigate(String),
    Click { element: String, ref_id: String },
    Type { element: String, ref_id: String, text: String },
    Snapshot,
    Screenshot { path: String },
}
```

**MCP Integration**:
- Uses `mcp__playwright__*` tools
- Parses accessibility tree for element selection
- Deterministic element targeting (no screenshots)

### 2.6 Reporter (`src/reporter.rs`)

**Responsibilities**:
- Format test results
- Output to console, file, or JSON
- Generate reports (HTML, JUnit XML)
- Calculate statistics

**Key Types**:
```rust
trait Reporter {
    fn report_start(&mut self, suite: &TestSuite);
    fn report_test(&mut self, result: &TestResult);
    fn report_end(&mut self, stats: &TestStats);
}

struct TextReporter;
struct JsonReporter;
struct JunitReporter;

struct TestStats {
    total: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
    duration: Duration,
}
```

## 3. Data Flow

### 3.1 Test Execution Flow

```
1. CLI parses args
   |
   v
2. Load config (file + env + flags)
   |
   v
3. Discover tests (TestLoader)
   |
   v
4. Initialize Playwright MCP
   |
   v
5. For each test:
   a. Setup browser
   b. Execute test actions
   c. Assert expectations
   d. Collect result
   |
   v
6. Generate report
   |
   v
7. Exit with code
```

### 3.2 Configuration Flow

```
Default Config
      |
      v
File Config (ui-test.toml)
      |
      v
Environment Variables
      |
      v
CLI Flags
      |
      v
Final Config
```

## 4. Error Handling

### 4.1 Error Types

```rust
#[derive(Debug, thiserror::Error)]
enum UiTestError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Test discovery failed: {0}")]
    Discovery(String),

    #[error("Playwright MCP error: {0}")]
    Playwright(String),

    #[error("Test execution failed: {0}")]
    Execution(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### 4.2 Error Recovery

- **Config errors**: Use defaults where possible, fail early if critical
- **Discovery errors**: Warn and skip unreadable files
- **MCP errors**: Retry with backoff, fail test after max attempts
- **Test errors**: Continue with remaining tests, report all failures

### 4.3 Exit Codes

- `0`: All tests passed
- `1`: Some tests failed
- `2`: Error (config, discovery, MCP connection, etc.)

## 5. Testing Strategy

### 5.1 Unit Tests

**Location**: `#[cfg(test)]` modules in each source file

**Coverage**:
- Config parsing and merging
- Test discovery patterns
- Result formatting
- Error handling

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_precedence() {
        let config = Config::from_sources(
            Some(file_config),
            env_vars,
            cli_flags,
        );
        assert_eq!(config.verbose, true); // CLI flag wins
    }
}
```

### 5.2 Integration Tests

**Location**: `tests/` directory

**Coverage**:
- CLI argument parsing
- Config file loading
- Test discovery end-to-end
- Reporter output formats

**Example**:
```rust
#[test]
fn test_cli_help_output() {
    let output = Command::new("ui-test-rs")
        .arg("--help")
        .output()
        .unwrap();

    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("AI CODING AGENT INSTRUCTIONS"));
}
```

### 5.3 Playwright Tests

**Location**: `tests/playwright/`

**Coverage**:
- Browser automation actions
- Element selection
- Screenshot capture
- Error handling

**Approach**: Use Playwright MCP in tests to validate our Playwright integration

## 6. Performance Considerations

### 6.1 Startup Time

**Goal**: < 100ms

**Optimizations**:
- Lazy initialization of MCP connection
- Parallel test discovery
- Minimal dependencies

### 6.2 Test Execution

**Goal**: Maximum throughput

**Optimizations**:
- Parallel test execution (configurable)
- Browser reuse across tests
- Incremental result reporting

### 6.3 Memory Usage

**Goal**: < 50MB base + per-test overhead

**Optimizations**:
- Streaming test results
- Browser cleanup after each test
- Efficient test representation

## 7. Security Considerations

### 7.1 Test File Execution

- Validate file paths (no path traversal)
- Sandbox test execution
- Limit resource usage (timeouts, memory)

### 7.2 Playwright MCP

- Validate MCP server certificate
- Secure communication channel
- Limit browser capabilities

### 7.3 Configuration

- Validate config file schema
- Sanitize user inputs
- No arbitrary code execution

## 8. Extensibility

### 8.1 Plugin System (Future)

**Design**:
- Trait-based plugin interface
- Dynamic loading from config
- Hooks for test lifecycle events

**Use Cases**:
- Custom reporters
- Additional assertion libraries
- Test data generators

### 8.2 Custom Actions (Future)

**Design**:
- User-defined test actions
- Composable action primitives
- Reusable action libraries

## 9. Dependencies

### 9.1 Required Dependencies

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
anyhow = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 9.2 Development Dependencies

```toml
[dev-dependencies]
tempfile = "3.0"
assert_cmd = "2.0"
predicates = "3.0"
```

## 10. Deployment

### 10.1 Installation Methods

**Cargo Install**:
```bash
cargo install ui-test-rs
```

**From Source**:
```bash
git clone https://github.com/softwarewrighter/ui-test-rs
cd ui-test-rs
cargo build --release
sw-install -p .
```

**Binary Distribution**:
- GitHub Releases with pre-built binaries
- Platform-specific packages (macOS, Linux, Windows)

### 10.2 Prerequisites

- Rust toolchain (for source builds)
- Playwright MCP server installed
- Node.js v20+ (for Playwright MCP)

### 10.3 Setup Verification

```bash
# Check installation
ui-test-rs --version

# Verify MCP connection
ui-test-rs --check-setup
```

## 11. Future Enhancements

### 11.1 Visual Regression Testing
- Screenshot baseline management
- Pixel-diff comparison
- Tolerance configuration

### 11.2 Performance Metrics
- Page load time tracking
- Resource usage monitoring
- Trend analysis

### 11.3 CI/CD Integration
- GitHub Actions workflow templates
- GitLab CI examples
- Docker container images

### 11.4 Test Authoring
- Test generator CLI
- Interactive test recorder
- Test template library

## 12. References

- Playwright MCP Documentation: https://github.com/microsoft/playwright-mcp
- Rust Async Book: https://rust-lang.github.io/async-book/
- Clap Documentation: https://docs.rs/clap/
- Software Wrighter Standards: docs/process.md

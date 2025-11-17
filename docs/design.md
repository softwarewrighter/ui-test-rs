# Design Document

## Project: ui-test-rs

**Version**: 0.1.0
**Date**: 2025-11-16
**Author**: Michael A Wright

## 1. Design Principles

### 1.1 Core Principles

1. **Simplicity First**: CLI should be intuitive and require minimal configuration
2. **Convention Over Configuration**: Sensible defaults that work out of the box
3. **Fail Fast**: Validate early, provide clear error messages
4. **Testability**: Every component must be unit testable
5. **Performance**: Startup time < 100ms, efficient resource usage
6. **Standards Compliance**: Follow Software Wrighter LLC standards

### 1.2 Design Constraints

- **Language**: Rust 2024 edition
- **CLI Framework**: clap with derive macros
- **Async Runtime**: tokio
- **Error Handling**: anyhow for applications, thiserror for libraries
- **File Size**: Maximum 500 lines per file
- **Function Size**: Maximum 50 lines per function
- **Tech Debt**: Maximum 3 TODO comments per file

## 2. CLI Design

### 2.1 Argument Structure

**Philosophy**: Follow Unix conventions, be consistent with similar tools

```rust
#[derive(Parser)]
#[command(
    name = "ui-test-rs",
    version,
    about = "CLI tool for UI testing with Playwright MCP integration",
    long_about = None,
    after_help = AI_INSTRUCTIONS,
)]
struct Cli {
    /// Path to test file or directory
    #[arg(
        default_value = ".",
        value_name = "TEST_PATH",
        help = "Path to test file or directory to run"
    )]
    test_path: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Dry-run mode (preview without executing)
    #[arg(short = 'n', long)]
    dry_run: bool,

    /// Output format
    #[arg(long, value_enum, default_value = "text")]
    format: OutputFormat,

    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Filter tests by name pattern
    #[arg(long, value_name = "PATTERN")]
    filter: Option<String>,

    /// Number of parallel test workers
    #[arg(short = 'j', long, default_value = "1")]
    jobs: usize,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Junit,
}
```

### 2.2 Help Output Design

**Requirements** (from sw-checklist):
- Short help (`-h`): Concise, one-screen overview
- Extended help (`--help`): Full documentation with AI Coding Agent Instructions
- Version output (`-V`, `--version`): Include copyright and license

**Implementation**:
```rust
const AI_INSTRUCTIONS: &str = r#"
AI CODING AGENT INSTRUCTIONS:

This tool runs UI tests using Playwright MCP for browser automation.

USAGE FOR AI AGENTS:
  1. Basic test execution:
     $ ui-test-rs tests/
     Exit code 0 = all passed, 1 = failures, 2 = error

  2. Verbose output for debugging:
     $ ui-test-rs -v tests/login_test.rs

  3. JSON output for parsing:
     $ ui-test-rs --format json tests/ > results.json

  4. Dry-run to preview:
     $ ui-test-rs --dry-run tests/

INTEGRATION:
  - Use in CI/CD with --format junit for test reporting
  - Combine with --filter for subset testing
  - Use --jobs for parallel execution

For more information:
https://github.com/softwarewrighter/ui-test-rs
"#;
```

### 2.3 Version Output Design

```rust
fn version_info() -> String {
    format!(
        "{} {}\n\n\
        Copyright (c) 2025 Michael A Wright\n\
        License: MIT (https://opensource.org/licenses/MIT)\n\
        Repository: https://github.com/softwarewrighter/ui-test-rs",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}
```

## 3. Configuration Design

### 3.1 Configuration File Format

**File**: `ui-test.toml` (optional)

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
fail_fast = false  # Stop on first failure
```

### 3.2 Configuration Precedence

**Resolution Order** (highest to lowest):
1. CLI flags (e.g., `--verbose`)
2. Environment variables (e.g., `UI_TEST_VERBOSE=1`)
3. Config file (`ui-test.toml`)
4. Default values (hardcoded)

**Implementation Pattern**:
```rust
impl Config {
    fn resolve(
        file_config: Option<FileConfig>,
        env_vars: EnvVars,
        cli_args: &Cli,
    ) -> Self {
        let verbose = cli_args.verbose
            || env_vars.get_bool("UI_TEST_VERBOSE")
            || file_config.as_ref()
                .and_then(|c| c.output.verbose)
            || false;  // default

        // ... similar for other fields
    }
}
```

### 3.3 Environment Variables

**Supported Variables**:
- `UI_TEST_VERBOSE`: Enable verbose output (1/true/yes)
- `UI_TEST_FORMAT`: Output format (text/json/junit)
- `UI_TEST_BROWSER`: Browser type (chromium/firefox/webkit)
- `UI_TEST_HEADLESS`: Run headless (1/true/yes)
- `UI_TEST_JOBS`: Parallel workers (number)
- `PLAYWRIGHT_MCP_URL`: MCP server URL override

## 4. Test Discovery Design

### 4.1 File Naming Conventions

**Supported Patterns**:
- `*_test.rs` (e.g., `login_test.rs`)
- `test_*.rs` (e.g., `test_login.rs`)
- Files in `tests/` directory

**Discovery Algorithm**:
```rust
fn discover_tests(root: &Path, patterns: &[String]) -> Result<Vec<TestFile>> {
    let mut files = Vec::new();

    for pattern in patterns {
        let glob_pattern = format!("{}/{}", root.display(), pattern);
        for entry in glob::glob(&glob_pattern)? {
            let path = entry?;
            if is_test_file(&path) {
                files.push(parse_test_file(path)?);
            }
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}
```

### 4.2 Test File Format

**Structure**:
```rust
// Example test file: tests/login_test.rs

use ui_test_rs::prelude::*;

#[test_case("admin_login")]
async fn test_admin_login(ctx: &mut TestContext) -> Result<()> {
    ctx.navigate("https://example.com/login").await?;
    ctx.fill("username", "admin").await?;
    ctx.fill("password", "secret").await?;
    ctx.click("button[type=submit]").await?;
    ctx.assert_url_contains("/dashboard").await?;
    Ok(())
}

#[test_case("failed_login")]
async fn test_failed_login(ctx: &mut TestContext) -> Result<()> {
    // ...
}
```

### 4.3 Test Filtering

**Filter by Name**:
```bash
# Run only tests matching "login"
ui-test-rs --filter login tests/
```

**Filter by Tag** (future):
```rust
#[test_case("admin_login", tags = ["smoke", "auth"])]
```

## 5. Test Execution Design

### 5.1 Test Lifecycle

**Phases**:
1. **Setup**: Initialize browser, load config
2. **Before Each**: Setup test-specific state
3. **Execute**: Run test function
4. **After Each**: Cleanup test state
5. **Teardown**: Close browser, cleanup

**Implementation**:
```rust
async fn run_test(test: &TestCase, ctx: &TestContext) -> TestResult {
    let start = Instant::now();

    // Setup
    ctx.before_each().await?;

    // Execute
    let status = match test.run(ctx).await {
        Ok(_) => TestStatus::Passed,
        Err(e) => {
            eprintln!("Test failed: {}", e);
            TestStatus::Failed
        }
    };

    // Cleanup
    ctx.after_each().await?;

    TestResult {
        name: test.name.clone(),
        status,
        duration: start.elapsed(),
        error: None,
    }
}
```

### 5.2 Parallel Execution

**Design**:
- Use tokio tasks for parallelism
- Configurable worker count (`--jobs`)
- Independent browser instances per worker
- Shared result collector

**Implementation**:
```rust
async fn run_tests_parallel(
    tests: Vec<TestCase>,
    jobs: usize,
) -> Vec<TestResult> {
    let (tx, mut rx) = mpsc::channel(tests.len());

    let semaphore = Arc::new(Semaphore::new(jobs));

    for test in tests {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let tx = tx.clone();

        tokio::spawn(async move {
            let result = run_test(&test, &TestContext::new()).await;
            tx.send(result).await.unwrap();
            drop(permit);
        });
    }

    drop(tx);

    let mut results = Vec::new();
    while let Some(result) = rx.recv().await {
        results.push(result);
    }

    results
}
```

### 5.3 Timeout Handling

**Default Timeout**: 30 seconds per test

**Implementation**:
```rust
async fn run_test_with_timeout(
    test: &TestCase,
    timeout: Duration,
) -> TestResult {
    match tokio::time::timeout(timeout, run_test(test, &ctx)).await {
        Ok(result) => result,
        Err(_) => TestResult {
            name: test.name.clone(),
            status: TestStatus::Error,
            duration: timeout,
            error: Some("Test timed out".to_string()),
        },
    }
}
```

## 6. Playwright Integration Design

### 6.1 MCP Connection

**Connection Management**:
```rust
struct PlaywrightClient {
    process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl PlaywrightClient {
    async fn connect(server_url: &str) -> Result<Self> {
        let mut process = Command::new("npx")
            .args(["-y", "@playwright/mcp"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = process.stdin.take().unwrap();
        let stdout = BufReader::new(process.stdout.take().unwrap());

        Ok(Self { process, stdin, stdout })
    }

    async fn send_command(&mut self, cmd: Command) -> Result<Response> {
        let json = serde_json::to_string(&cmd)?;
        self.stdin.write_all(json.as_bytes()).await?;
        self.stdin.write_all(b"\n").await?;

        let mut line = String::new();
        self.stdout.read_line(&mut line).await?;
        let response = serde_json::from_str(&line)?;

        Ok(response)
    }
}
```

### 6.2 Browser Actions

**Action Abstraction**:
```rust
impl TestContext {
    async fn navigate(&self, url: &str) -> Result<()> {
        self.playwright
            .send_command(Command::Navigate { url: url.to_string() })
            .await?;
        Ok(())
    }

    async fn click(&self, selector: &str) -> Result<()> {
        let snapshot = self.playwright.snapshot().await?;
        let element = snapshot.find_element(selector)?;

        self.playwright
            .send_command(Command::Click {
                element: element.description.clone(),
                ref_id: element.ref_id.clone(),
            })
            .await?;
        Ok(())
    }

    async fn fill(&self, selector: &str, text: &str) -> Result<()> {
        let snapshot = self.playwright.snapshot().await?;
        let element = snapshot.find_element(selector)?;

        self.playwright
            .send_command(Command::Type {
                element: element.description.clone(),
                ref_id: element.ref_id.clone(),
                text: text.to_string(),
            })
            .await?;
        Ok(())
    }
}
```

### 6.3 Element Selection

**Strategy**: Use accessibility tree, not pixel coordinates

**Selector Types**:
- Role + Name: `button[name="Submit"]`
- ARIA labels: `[aria-label="Search"]`
- Text content: `text="Login"`
- CSS selectors: `#username`

**Implementation**:
```rust
struct AccessibilitySnapshot {
    elements: Vec<Element>,
}

impl AccessibilitySnapshot {
    fn find_element(&self, selector: &str) -> Result<&Element> {
        // Parse selector
        let query = SelectorQuery::parse(selector)?;

        // Search accessibility tree
        for element in &self.elements {
            if query.matches(element) {
                return Ok(element);
            }
        }

        Err(anyhow!("Element not found: {}", selector))
    }
}
```

## 7. Reporting Design

### 7.1 Text Reporter

**Format**:
```
Running 5 tests from tests/

test tests/login_test.rs::test_admin_login ... ok (1.2s)
test tests/login_test.rs::test_failed_login ... ok (0.8s)
test tests/checkout_test.rs::test_add_to_cart ... FAILED (2.1s)
test tests/checkout_test.rs::test_remove_item ... ok (1.0s)
test tests/search_test.rs::test_search ... ok (0.9s)

Failures:

---- tests/checkout_test.rs::test_add_to_cart ----
Error: Element not found: button[name="Add to Cart"]
  at tests/checkout_test.rs:15

Summary:
  5 tests, 4 passed, 1 failed, 0 skipped
  Duration: 6.0s

Exit code: 1
```

### 7.2 JSON Reporter

**Format**:
```json
{
  "version": "1.0",
  "timestamp": "2025-11-16T12:00:00Z",
  "total": 5,
  "passed": 4,
  "failed": 1,
  "skipped": 0,
  "duration_ms": 6000,
  "tests": [
    {
      "name": "tests/login_test.rs::test_admin_login",
      "status": "passed",
      "duration_ms": 1200
    },
    {
      "name": "tests/checkout_test.rs::test_add_to_cart",
      "status": "failed",
      "duration_ms": 2100,
      "error": "Element not found: button[name=\"Add to Cart\"]",
      "location": "tests/checkout_test.rs:15"
    }
  ]
}
```

### 7.3 JUnit XML Reporter

**Format** (for CI/CD integration):
```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuites tests="5" failures="1" time="6.0">
  <testsuite name="ui-test-rs" tests="5" failures="1" time="6.0">
    <testcase name="test_admin_login" classname="tests.login_test" time="1.2"/>
    <testcase name="test_add_to_cart" classname="tests.checkout_test" time="2.1">
      <failure message="Element not found">
        Error: Element not found: button[name="Add to Cart"]
        at tests/checkout_test.rs:15
      </failure>
    </testcase>
  </testsuite>
</testsuites>
```

## 8. Error Handling Design

### 8.1 Error Types

**Hierarchy**:
```rust
#[derive(Debug, thiserror::Error)]
enum UiTestError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Test discovery failed: {0}")]
    Discovery(String),

    #[error("Playwright MCP connection failed: {0}")]
    PlaywrightConnection(String),

    #[error("Browser action failed: {0}")]
    BrowserAction(String),

    #[error("Assertion failed: {0}")]
    Assertion(String),

    #[error("Test timeout after {0:?}")]
    Timeout(Duration),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### 8.2 Error Messages

**Principles**:
- Clear and actionable
- Include context (file, line, element)
- Suggest fixes when possible

**Examples**:
```
Error: Element not found: button[name="Add to Cart"]
  at tests/checkout_test.rs:15

  Suggestion: Check if the button exists in the page.
  Try: Take a screenshot to debug: ctx.screenshot("debug.png")

Error: Playwright MCP connection failed: Connection refused

  Suggestion: Ensure Playwright MCP server is installed:
    $ claude mcp add playwright -s user -- npx -y @playwright/mcp
```

## 9. Testing the Tool Itself

### 9.1 Unit Tests

**Coverage**:
- Config parsing and merging
- Test discovery patterns
- Reporter output formatting
- Error message generation

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_tests() {
        let temp = tempfile::tempdir().unwrap();
        let test_file = temp.path().join("login_test.rs");
        fs::write(&test_file, "// test file").unwrap();

        let tests = discover_tests(temp.path(), &["*_test.rs"]).unwrap();
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].path, test_file);
    }
}
```

### 9.2 Integration Tests

**Coverage**:
- CLI argument parsing
- End-to-end test execution
- Output format validation

**Example**:
```rust
#[test]
fn test_cli_version() {
    let output = Command::cargo_bin("ui-test-rs")
        .unwrap()
        .arg("--version")
        .output()
        .unwrap();

    let version = String::from_utf8(output.stdout).unwrap();
    assert!(version.contains("Copyright (c) 2025 Michael A Wright"));
}
```

## 10. Performance Optimizations

### 10.1 Startup Time

**Target**: < 100ms

**Optimizations**:
- Lazy dependency loading
- Minimal upfront work
- Fast config parsing

### 10.2 Test Execution

**Target**: Maximum throughput

**Optimizations**:
- Parallel execution
- Browser instance reuse
- Efficient element selection

### 10.3 Memory Usage

**Target**: < 50MB base

**Optimizations**:
- Streaming test results
- Cleanup after each test
- Efficient data structures

## 11. Future Design Considerations

### 11.1 Plugin System

**Design**:
- Trait-based interface
- Dynamic loading from config
- Lifecycle hooks

### 11.2 Test Recorder

**Design**:
- Record browser interactions
- Generate test code
- Replay for verification

### 11.3 Visual Regression

**Design**:
- Screenshot baseline storage
- Pixel diff comparison
- Configurable thresholds

## 12. Design Decisions Log

### Decision 1: Use clap derive macros
**Rationale**: Type-safe, less boilerplate, better documentation
**Alternatives**: structopt (deprecated), manual parsing
**Date**: 2025-11-16

### Decision 2: Tokio for async runtime
**Rationale**: Industry standard, well-maintained, feature-rich
**Alternatives**: async-std, smol
**Date**: 2025-11-16

### Decision 3: TOML for config format
**Rationale**: Readable, Rust ecosystem standard, good error messages
**Alternatives**: YAML, JSON
**Date**: 2025-11-16

### Decision 4: Accessibility tree for element selection
**Rationale**: Deterministic, no vision model needed, faster
**Alternatives**: Screenshot-based, pixel coordinates
**Date**: 2025-11-16

# CLI Interface

The CLI Interface component (`main.rs`) is the entry point of ui-test-rs, responsible for parsing command-line arguments, displaying help/version information, and initiating the test execution process.

## Overview

```mermaid
flowchart TB
    User([User])
    CLI[CLI Interface main.rs]
    Parser[Argument Parser clap]
    Validator[Input Validator]
    ConfigResolver[Config Resolver]
    Runner[Test Runner]

    User -->|command| CLI
    CLI --> Parser
    Parser --> Validator
    Validator --> ConfigResolver
    ConfigResolver --> Runner

    style CLI fill:#e1f5ff
    style Parser fill:#fff4e1
    style Validator fill:#e8f5e9
    style ConfigResolver fill:#f3e5f5
```

## Responsibilities

- Parse command-line arguments using clap
- Validate user input
- Display help and version information
- Resolve final configuration
- Initialize and invoke test runner
- Handle exit codes

## Structure

### Cli Struct

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

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Filter tests by name pattern
    #[arg(long)]
    filter: Option<String>,

    /// Number of parallel test workers
    #[arg(short = 'j', long, default_value = "1")]
    jobs: usize,
}
```

### Component Diagram

```mermaid
classDiagram
    class Cli {
        +PathBuf test_path
        +bool verbose
        +bool dry_run
        +OutputFormat format
        +Option~PathBuf~ config
        +Option~String~ filter
        +usize jobs
        +parse() Cli
    }

    class OutputFormat {
        <<enumeration>>
        Text
        Json
        Junit
    }

    class AI_INSTRUCTIONS {
        <<const>>
        +&str content
    }

    Cli --> OutputFormat
    Cli ..> AI_INSTRUCTIONS
```

## Argument Parsing

### Parsing Flow

```mermaid
sequenceDiagram
    participant User
    participant Main
    participant Clap
    participant Validator

    User->>Main: ui-test-rs [args]
    Main->>Clap: parse arguments
    activate Clap

    alt Valid Arguments
        Clap->>Validator: validate inputs
        activate Validator
        alt Valid Inputs
            Validator-->>Clap: validated
            Clap-->>Main: Cli struct
        else Invalid Inputs
            Validator-->>Clap: validation error
            Clap->>User: show error message
            Clap->>Main: exit(2)
        end
        deactivate Validator
    else Parse Error
        Clap->>User: show help/error
        Clap->>Main: exit(2)
    end

    deactivate Clap
```

### Supported Arguments

| Argument | Type | Default | Required | Description |
|----------|------|---------|----------|-------------|
| `TEST_PATH` | Path | `.` | No | Path to test file or directory |
| `-v, --verbose` | Flag | `false` | No | Enable verbose output |
| `-n, --dry-run` | Flag | `false` | No | Preview without executing |
| `--format` | Enum | `text` | No | Output format (text/json/junit) |
| `-c, --config` | Path | - | No | Custom config file path |
| `--filter` | String | - | No | Filter tests by name pattern |
| `-j, --jobs` | Number | `1` | No | Number of parallel workers |
| `-h, --help` | Flag | - | No | Show help message |
| `-V, --version` | Flag | - | No | Show version information |

## Help Output

### Short Help (-h)

```
CLI tool for UI testing with Playwright MCP integration

Usage: ui-test-rs [OPTIONS] [TEST_PATH]

Arguments:
  [TEST_PATH]  Path to test file or directory to run [default: .]

Options:
  -v, --verbose           Enable verbose output
  -n, --dry-run           Dry-run mode (preview without executing)
      --format <FORMAT>   Output format: text, json, junit [default: text]
  -c, --config <FILE>     Configuration file path
      --filter <PATTERN>  Filter tests by name pattern
  -j, --jobs <JOBS>       Number of parallel test workers [default: 1]
  -h, --help              Print help
  -V, --version           Print version
```

### Extended Help (--help)

Includes the short help plus:
- Detailed descriptions of each option
- Examples of common usage patterns
- AI Coding Agent Instructions section
- Link to repository and documentation

### AI Coding Agent Instructions

```
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

EXIT CODES:
  0 - All tests passed
  1 - Some tests failed
  2 - Error (config, discovery, MCP connection, etc.)

For more information:
https://github.com/softwarewrighter/ui-test-rs
```

## Version Output

### Short Version (-V)

```
ui-test-rs 0.1.0
```

### Extended Version (--version)

```
ui-test-rs 0.1.0

Copyright (c) 2025 Michael A Wright
License: MIT (https://opensource.org/licenses/MIT)
Repository: https://github.com/softwarewrighter/ui-test-rs
```

## Main Function Flow

```mermaid
flowchart TB
    Start([main])
    ParseArgs[Parse CLI Arguments]
    SpecialCmd{Special Command?}
    ShowHelp[Show Help]
    ShowVersion[Show Version]
    Exit0([Exit 0])

    ValidateArgs{Valid Arguments?}
    ShowError[Show Error]
    Exit2([Exit 2])

    ResolveConfig[Resolve Configuration]
    ConfigValid{Config Valid?}

    InitRunner[Initialize Test Runner]
    RunTests[Execute Tests]
    Results{Test Results?}

    AllPassed[All Tests Passed]
    SomeFailed[Some Tests Failed]
    ErrorOccurred[Error Occurred]

    Exit0b([Exit 0])
    Exit1([Exit 1])
    Exit2b([Exit 2])

    Start --> ParseArgs
    ParseArgs --> SpecialCmd

    SpecialCmd -->|Help| ShowHelp
    SpecialCmd -->|Version| ShowVersion
    SpecialCmd -->|None| ValidateArgs

    ShowHelp --> Exit0
    ShowVersion --> Exit0

    ValidateArgs -->|No| ShowError
    ValidateArgs -->|Yes| ResolveConfig
    ShowError --> Exit2

    ResolveConfig --> ConfigValid
    ConfigValid -->|No| Exit2
    ConfigValid -->|Yes| InitRunner

    InitRunner --> RunTests
    RunTests --> Results

    Results -->|All Passed| AllPassed
    Results -->|Failures| SomeFailed
    Results -->|Error| ErrorOccurred

    AllPassed --> Exit0b
    SomeFailed --> Exit1
    ErrorOccurred --> Exit2b

    style Start fill:#c8e6c9
    style Exit0 fill:#c8e6c9
    style Exit0b fill:#c8e6c9
    style Exit1 fill:#ffecb3
    style Exit2 fill:#ffcdd2
    style Exit2b fill:#ffcdd2
```

## Exit Codes

```mermaid
graph TB
    subgraph ExitCodes["Exit Codes"]
        E0[0 - Success All tests passed]
        E1[1 - Test Failures Some tests failed]
        E2[2 - Error Configuration/Discovery/MCP error]
    end

    subgraph Scenarios["Example Scenarios"]
        S1[10 tests, 10 passed]
        S2[10 tests, 8 passed, 2 failed]
        S3[Config file not found]
        S4[MCP connection failed]
        S5[No tests discovered]
    end

    S1 --> E0
    S2 --> E1
    S3 --> E2
    S4 --> E2
    S5 --> E2

    style E0 fill:#c8e6c9
    style E1 fill:#ffecb3
    style E2 fill:#ffcdd2
```

## Input Validation

### Validation Rules

```mermaid
flowchart TB
    Start([Validate Input])
    CheckPath{TEST_PATH exists?}
    PathError[Error: Path not found]
    CheckJobs{jobs > 0?}
    JobsError[Error: Jobs must be > 0]
    CheckConfig{config file specified?}
    ConfigExists{config file exists?}
    ConfigError[Error: Config file not found]
    CheckFormat{format valid?}
    FormatError[Error: Invalid format]
    Valid([Input Valid])

    Start --> CheckPath
    CheckPath -->|No| PathError
    CheckPath -->|Yes| CheckJobs
    CheckJobs -->|No| JobsError
    CheckJobs -->|Yes| CheckConfig
    CheckConfig -->|Yes| ConfigExists
    CheckConfig -->|No| CheckFormat
    ConfigExists -->|No| ConfigError
    ConfigExists -->|Yes| CheckFormat
    CheckFormat -->|No| FormatError
    CheckFormat -->|Yes| Valid

    style Start fill:#c8e6c9
    style Valid fill:#c8e6c9
    style PathError fill:#ffcdd2
    style JobsError fill:#ffcdd2
    style ConfigError fill:#ffcdd2
    style FormatError fill:#ffcdd2
```

## Usage Examples

### Basic Usage

```bash
# Run all tests in current directory
ui-test-rs

# Run tests in specific directory
ui-test-rs tests/

# Run specific test file
ui-test-rs tests/login_test.rs
```

### With Options

```bash
# Verbose output
ui-test-rs -v tests/

# Dry-run mode
ui-test-rs --dry-run tests/

# JSON output
ui-test-rs --format json tests/ > results.json

# Parallel execution with 4 workers
ui-test-rs -j 4 tests/

# Filter tests by name
ui-test-rs --filter login tests/
```

### Combined Options

```bash
# Verbose, parallel, filtered
ui-test-rs -v -j 8 --filter "smoke" tests/

# Custom config, JUnit output
ui-test-rs -c custom.toml --format junit tests/
```

## Error Messages

### Examples

```
Error: Test path not found: tests/nonexistent

Error: Configuration file not found: custom.toml

Error: Number of jobs must be greater than 0

Error: Invalid output format: xml
  Supported formats: text, json, junit
```

## Standards Compliance

The CLI Interface must comply with `sw-checklist` validation:

### Required Standards

1. **Help Output**
   - Short help (`-h`) must be concise and single-screen
   - Extended help (`--help`) must include AI instructions

2. **Version Output**
   - Short version (`-V`) shows version number only
   - Extended version (`--version`) includes copyright and license

3. **Exit Codes**
   - `0` for success
   - `1` for test failures
   - `2` for errors

4. **Error Messages**
   - Clear and actionable
   - Include context
   - Suggest fixes when possible

## Related Documentation

- [Configuration](Configuration) - Configuration system
- [Test Runner](Test-Runner) - Test execution
- [Architecture](Architecture) - System architecture

---

**Last Updated:** 2025-11-18

# Configuration

This page describes the configuration system in ui-test-rs, including configuration sources, precedence rules, and available options.

## Overview

ui-test-rs uses a flexible multi-layered configuration system that allows settings to come from multiple sources with clear precedence rules.

### Configuration Sources

```mermaid
flowchart TB
    subgraph Sources["Configuration Sources (Lowest to Highest Priority)"]
        direction TB
        D[1. Default Values<br/>Hardcoded in application]
        F[2. Config File<br/>ui-test.toml]
        E[3. Environment Variables<br/>UI_TEST_*]
        C[4. CLI Flags<br/>--verbose, etc.]
    end

    Final[Final Configuration]

    D --> F
    F --> E
    E --> C
    C --> Final

    style D fill:#f5f5f5
    style F fill:#e3f2fd
    style E fill:#e8f5e9
    style C fill:#fff9c4
    style Final fill:#c8e6c9
```

## Configuration File

### Location and Format

The configuration file is optional and should be named `ui-test.toml` in the current working directory or specified via `--config` flag.

**Format:** TOML (Tom's Obvious, Minimal Language)

### Full Configuration Example

```toml
# ui-test.toml - UI Test Runner Configuration

# Test Discovery
[discovery]
patterns = ["*_test.rs", "test_*.rs"]
exclude = ["target/**", "node_modules/**", ".git/**"]

# Browser Configuration
[browser]
type = "chromium"  # Options: chromium, firefox, webkit
headless = true
viewport = { width = 1280, height = 720 }

# Playwright MCP Settings
[playwright]
server_url = "npx -y @playwright/mcp"
timeout = 30000  # milliseconds (30 seconds)
retries = 3

# Output Settings
[output]
format = "text"  # Options: text, json, junit
verbose = false
color = true

# Parallel Execution
[execution]
jobs = 4  # Number of parallel workers
fail_fast = false  # Stop on first failure
```

### Configuration Sections

```mermaid
mindmap
  root((ui-test.toml))
    discovery
      patterns
      exclude
    browser
      type
      headless
      viewport
    playwright
      server_url
      timeout
      retries
    output
      format
      verbose
      color
    execution
      jobs
      fail_fast
```

## Environment Variables

All configuration options can be overridden via environment variables using the `UI_TEST_` prefix.

### Supported Variables

| Variable | Type | Example | Description |
|----------|------|---------|-------------|
| `UI_TEST_VERBOSE` | Boolean | `1`, `true`, `yes` | Enable verbose output |
| `UI_TEST_FORMAT` | String | `json`, `junit` | Output format |
| `UI_TEST_BROWSER` | String | `firefox`, `webkit` | Browser type |
| `UI_TEST_HEADLESS` | Boolean | `1`, `true`, `yes` | Run in headless mode |
| `UI_TEST_JOBS` | Integer | `8` | Number of parallel workers |
| `UI_TEST_TIMEOUT` | Integer | `60000` | Test timeout in milliseconds |
| `PLAYWRIGHT_MCP_URL` | String | `npx -y @playwright/mcp` | MCP server command |

### Boolean Values

Environment variables accept multiple formats for boolean values:

**True:** `1`, `true`, `True`, `TRUE`, `yes`, `Yes`, `YES`
**False:** `0`, `false`, `False`, `FALSE`, `no`, `No`, `NO`

### Example Usage

```bash
# Enable verbose output
export UI_TEST_VERBOSE=1

# Set output format to JSON
export UI_TEST_FORMAT=json

# Use 8 parallel workers
export UI_TEST_JOBS=8

# Run tests
ui-test-rs tests/
```

## CLI Flags

Command-line flags have the highest priority and override all other configuration sources.

### Available Flags

```mermaid
mindmap
  root((CLI Flags))
    Positional
      TEST_PATH
    General
      --verbose/-v
      --dry-run/-n
      --help/-h
      --version/-V
    Configuration
      --config/-c
      --format
      --jobs/-j
    Filtering
      --filter
```

### Flag Reference

| Flag | Short | Type | Default | Description |
|------|-------|------|---------|-------------|
| `TEST_PATH` | - | Path | `.` | Path to test file or directory |
| `--verbose` | `-v` | Boolean | `false` | Enable verbose output |
| `--dry-run` | `-n` | Boolean | `false` | Preview without executing |
| `--format` | - | Enum | `text` | Output format (text/json/junit) |
| `--config` | `-c` | Path | - | Config file path |
| `--filter` | - | String | - | Filter tests by name pattern |
| `--jobs` | `-j` | Integer | `1` | Number of parallel workers |
| `--help` | `-h` | Flag | - | Show help message |
| `--version` | `-V` | Flag | - | Show version info |

## Configuration Precedence

### Resolution Algorithm

```mermaid
flowchart TB
    Start([Config Resolution])
    LoadDefaults[Load Default Values]
    CheckFile{Config File<br/>Specified?}
    LoadFile[Load Config File]
    FileExists{File Exists?}
    Error[Fail with Error]
    MergeFile[Merge with Defaults]
    LoadEnv[Load Environment Variables]
    MergeEnv[Merge with Current Config]
    LoadCLI[Load CLI Flags]
    MergeCLI[Merge with Current Config]
    Validate{Valid Config?}
    Done([Final Configuration])

    Start --> LoadDefaults
    LoadDefaults --> CheckFile
    CheckFile -->|Yes| LoadFile
    CheckFile -->|No| LoadEnv
    LoadFile --> FileExists
    FileExists -->|No| Error
    FileExists -->|Yes| MergeFile
    MergeFile --> LoadEnv
    LoadEnv --> MergeEnv
    MergeEnv --> LoadCLI
    LoadCLI --> MergeCLI
    MergeCLI --> Validate
    Validate -->|No| Error
    Validate -->|Yes| Done

    style Start fill:#c8e6c9
    style Error fill:#ffcdd2
    style Done fill:#c8e6c9
    style LoadDefaults fill:#f5f5f5
    style MergeFile fill:#e3f2fd
    style MergeEnv fill:#e8f5e9
    style MergeCLI fill:#fff9c4
```

### Precedence Examples

#### Example 1: Verbose Flag

```
Default:     verbose = false
File:        (not set)
Environment: UI_TEST_VERBOSE=1
CLI:         (not set)
---
Result:      verbose = true  (from environment)
```

#### Example 2: Output Format

```
Default:     format = text
File:        format = json
Environment: (not set)
CLI:         --format junit
---
Result:      format = junit  (from CLI)
```

#### Example 3: Job Count

```
Default:     jobs = 1
File:        jobs = 4
Environment: UI_TEST_JOBS=8
CLI:         (not set)
---
Result:      jobs = 8  (from environment)
```

### Precedence Visualization

```mermaid
graph TB
    subgraph Priority["Configuration Priority"]
        direction TB
        CLI[CLI Flags<br/>HIGHEST]
        Env[Environment Variables]
        File[Config File]
        Default[Default Values<br/>LOWEST]
    end

    CLI -.->|overrides| Env
    Env -.->|overrides| File
    File -.->|overrides| Default

    style CLI fill:#4caf50,color:#fff
    style Env fill:#8bc34a
    style File fill:#cddc39
    style Default fill:#ffeb3b
```

## Default Configuration

### Built-in Defaults

```rust
// Conceptual representation of default configuration
Config {
    discovery: Discovery {
        patterns: vec!["*_test.rs", "test_*.rs"],
        exclude: vec!["target/**", ".git/**"],
    },
    browser: Browser {
        type: BrowserType::Chromium,
        headless: true,
        viewport: Viewport { width: 1280, height: 720 },
    },
    playwright: Playwright {
        server_url: "npx -y @playwright/mcp",
        timeout: 30000,  // 30 seconds
        retries: 3,
    },
    output: Output {
        format: OutputFormat::Text,
        verbose: false,
        color: true,
    },
    execution: Execution {
        jobs: 1,
        fail_fast: false,
    },
}
```

## Configuration Validation

### Validation Rules

```mermaid
flowchart TB
    Start([Validate Config])
    CheckTimeout{Timeout > 0?}
    CheckJobs{Jobs > 0?}
    CheckPatterns{Patterns<br/>non-empty?}
    CheckFormat{Valid format?}
    CheckBrowser{Valid browser?}
    AllValid{All Valid?}
    Error[Configuration Error]
    Success([Valid Configuration])

    Start --> CheckTimeout
    CheckTimeout -->|No| Error
    CheckTimeout -->|Yes| CheckJobs
    CheckJobs -->|No| Error
    CheckJobs -->|Yes| CheckPatterns
    CheckPatterns -->|No| Error
    CheckPatterns -->|Yes| CheckFormat
    CheckFormat -->|No| Error
    CheckFormat -->|Yes| CheckBrowser
    CheckBrowser -->|No| Error
    CheckBrowser -->|Yes| AllValid
    AllValid -->|Yes| Success
    AllValid -->|No| Error

    style Start fill:#c8e6c9
    style Error fill:#ffcdd2
    style Success fill:#c8e6c9
```

### Validation Errors

Common validation errors and their messages:

| Error | Message | Fix |
|-------|---------|-----|
| Invalid timeout | "Timeout must be greater than 0" | Set `playwright.timeout > 0` |
| Invalid jobs | "Jobs must be at least 1" | Set `execution.jobs >= 1` |
| Empty patterns | "At least one discovery pattern required" | Add to `discovery.patterns` |
| Invalid format | "Format must be text, json, or junit" | Use valid format value |
| Invalid browser | "Browser must be chromium, firefox, or webkit" | Use valid browser type |

## Configuration Best Practices

### 1. Use Config File for Project Settings

Store project-specific settings in `ui-test.toml`:

```toml
# Commit this file to version control
[discovery]
patterns = ["*_test.rs"]

[browser]
type = "chromium"
headless = true
```

### 2. Use Environment Variables for CI/CD

```bash
# In CI/CD pipeline
export UI_TEST_VERBOSE=1
export UI_TEST_FORMAT=junit
export UI_TEST_JOBS=4
```

### 3. Use CLI Flags for One-off Overrides

```bash
# Quick debugging with verbose output
ui-test-rs -v tests/specific_test.rs

# Generate JSON for custom processing
ui-test-rs --format json tests/ > results.json
```

### 4. Configuration Organization

```mermaid
flowchart LR
    subgraph VCS["Version Control"]
        ConfigFile[ui-test.toml<br/>Team Settings]
    end

    subgraph CI["CI/CD"]
        EnvVars[Environment Variables<br/>Build Settings]
    end

    subgraph Local["Local Development"]
        LocalEnv[.env file<br/>Personal Settings]
        CLIFlags[CLI Flags<br/>Quick Overrides]
    end

    VCS -.-> CI
    VCS -.-> Local
    CI -.-> EnvVars
    Local -.-> LocalEnv
    Local -.-> CLIFlags

    style VCS fill:#e3f2fd
    style CI fill:#e8f5e9
    style Local fill:#fff9c4
```

## Configuration Examples

### Example 1: Development Setup

```toml
# ui-test.toml - Development configuration
[browser]
headless = false  # See browser for debugging
viewport = { width = 1920, height = 1080 }

[output]
verbose = true
color = true

[execution]
jobs = 1  # Sequential for easier debugging
fail_fast = true  # Stop on first failure
```

### Example 2: CI/CD Setup

```bash
# In CI/CD pipeline script
export UI_TEST_HEADLESS=1
export UI_TEST_FORMAT=junit
export UI_TEST_JOBS=8
export UI_TEST_VERBOSE=1

ui-test-rs tests/ --format junit > test-results.xml
```

### Example 3: Custom Browser

```toml
# ui-test.toml - Using Firefox
[browser]
type = "firefox"
headless = true

[playwright]
timeout = 60000  # Longer timeout for slower browser
```

## Troubleshooting

### Debug Configuration

To see the final resolved configuration, use verbose mode:

```bash
ui-test-rs -v --dry-run tests/
```

This will show:
- Loaded configuration sources
- Final merged configuration
- Validation results

### Common Issues

```mermaid
flowchart TB
    Issue{Configuration<br/>Issue?}

    NotFound[Config file<br/>not found]
    ParseError[Parse error]
    ValidationError[Validation error]
    PrecedenceIssue[Unexpected value]

    CheckPath[Check file path<br/>and --config flag]
    CheckSyntax[Check TOML syntax<br/>validate structure]
    CheckValues[Check validation rules<br/>fix invalid values]
    CheckPrecedence[Review precedence<br/>check all sources]

    Issue -->|File Missing| NotFound
    Issue -->|Invalid TOML| ParseError
    Issue -->|Invalid Values| ValidationError
    Issue -->|Wrong Setting| PrecedenceIssue

    NotFound --> CheckPath
    ParseError --> CheckSyntax
    ValidationError --> CheckValues
    PrecedenceIssue --> CheckPrecedence

    style Issue fill:#fff9c4
    style NotFound fill:#ffecb3
    style ParseError fill:#ffecb3
    style ValidationError fill:#ffecb3
    style PrecedenceIssue fill:#ffecb3
```

## Related Documentation

- [Architecture](Architecture) - System architecture
- [CLI Interface](CLI-Interface) - Command-line interface details
- [Development Guide](Development-Guide) - Building and testing

---

**Last Updated:** 2025-11-18

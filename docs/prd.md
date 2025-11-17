# Product Requirements Document (PRD)

## Project: ui-test-rs

**Version**: 0.1.0
**Date**: 2025-11-16
**Author**: Michael A Wright
**Status**: Draft

## 1. Overview

### 1.1 Purpose
`ui-test-rs` is a command-line tool for UI testing automation built in Rust. It provides a simple, efficient way to run UI tests leveraging Playwright MCP (Model Context Protocol) for browser automation.

### 1.2 Goals
- Provide a CLI interface for UI testing workflows
- Integrate with Playwright MCP for browser automation
- Follow Software Wrighter LLC development standards
- Support test-driven development practices
- Be portable and easy to install

### 1.3 Non-Goals
- GUI interface (CLI only)
- Support for non-browser UI testing (focused on web)
- Test framework implementation (leverages existing tools)

## 2. User Stories

### 2.1 As a Developer
- I want to run UI tests from the command line so I can integrate them into my workflow
- I want clear help documentation so I can learn how to use the tool
- I want verbose output options so I can debug test failures
- I want the tool to validate my test setup so I know everything is configured correctly

### 2.2 As a CI/CD Pipeline
- I want exit codes that indicate success/failure so I can fail builds appropriately
- I want JSON output options so I can parse results programmatically
- I want the tool to be fast and lightweight so builds complete quickly

### 2.3 As an AI Coding Agent
- I want extended help with AI-specific instructions so I know how to use the tool effectively
- I want predictable output formats so I can parse results reliably
- I want the tool to follow conventions so I can use it consistently

## 3. Features

### 3.1 Core Features (MVP)

#### 3.1.1 CLI Interface
- **help** output with `-h` (short) and `--help` (extended with AI instructions)
- **version** output with `-V` and `--version` including metadata
- Verbose mode (`-v, --verbose`)
- Dry-run mode (`-n, --dry-run`)

#### 3.1.2 Test Execution
- Run test files or directories
- Support for glob patterns
- Parallel test execution
- Test filtering by name or tag

#### 3.1.3 Reporting
- Human-readable output (default)
- JSON output format (`--json`)
- JUnit XML output format (`--junit`)
- Exit codes: 0 (success), 1 (test failures), 2 (errors)

#### 3.1.4 Configuration
- Config file support (`ui-test.toml`)
- Environment variable overrides
- Command-line flag precedence

### 3.2 Future Features (Post-MVP)

#### 3.2.1 Advanced Testing
- Screenshot comparison
- Visual regression testing
- Performance metrics collection
- Accessibility testing integration

#### 3.2.2 Integration
- GitHub Actions integration
- GitLab CI templates
- Docker container support

#### 3.2.3 Reporting Enhancements
- HTML report generation
- Test coverage metrics
- Historical trend tracking

## 4. Technical Requirements

### 4.1 Dependencies
- Rust 2024 edition
- clap (CLI argument parsing)
- serde (serialization for JSON/TOML)
- tokio (async runtime)

### 4.2 Playwright MCP Integration
- Connect to Playwright MCP server
- Use accessibility tree for element selection
- Support browser automation actions
- Handle browser lifecycle

### 4.3 Quality Standards
- Zero clippy warnings (`-D warnings`)
- 100% formatted with `cargo fmt`
- Unit tests for core logic
- Integration tests for CLI interface
- Documentation for public APIs

### 4.4 Tool Compliance
- Pass `sw-checklist` validation
- Pass `markdown-checker` for all docs
- Follow TDD Red/Green/Refactor cycle
- Maximum 3 TODO comments per file
- Files under 500 lines
- Functions under 50 lines

## 5. User Interface

### 5.1 Command Structure

```bash
# Basic usage
ui-test-rs [OPTIONS] [TEST_PATH]

# Examples
ui-test-rs                          # Run all tests in current directory
ui-test-rs tests/                   # Run all tests in tests/ directory
ui-test-rs tests/login_test.rs      # Run specific test file
ui-test-rs --verbose tests/         # Run with verbose output
ui-test-rs --dry-run tests/         # Preview what would be tested
ui-test-rs --json tests/            # Output results as JSON
```

### 5.2 Help Output Format

#### Short Help (`-h`)
```
CLI tool for UI testing with Playwright MCP integration

Usage: ui-test-rs [OPTIONS] [TEST_PATH]

Arguments:
  [TEST_PATH]  Path to test file or directory [default: .]

Options:
  -v, --verbose   Show verbose output
  -n, --dry-run   Preview test execution without running
  --json          Output results as JSON
  -h, --help      Print help
  -V, --version   Print version
```

#### Extended Help (`--help`)
Includes AI Coding Agent Instructions section with:
- Usage patterns for automated workflows
- Exit code meanings
- Output format specifications
- Common use cases
- Integration examples

### 5.3 Version Output Format

```
ui-test-rs 0.1.0

Copyright (c) 2025 Michael A Wright
License: MIT (https://opensource.org/licenses/MIT)
Repository: https://github.com/softwarewrighter/ui-test-rs
```

## 6. Success Criteria

### 6.1 Functional
- [ ] CLI accepts test paths and options
- [ ] Tests execute and report results correctly
- [ ] Exit codes reflect test outcomes
- [ ] Help and version outputs meet standards
- [ ] AI instructions included in extended help

### 6.2 Quality
- [ ] All tests pass (`cargo test`)
- [ ] Zero clippy warnings
- [ ] All code formatted
- [ ] README validates with markdown-checker
- [ ] Passes sw-checklist validation

### 6.3 Documentation
- [ ] README with installation and usage
- [ ] Architecture documentation
- [ ] Design documentation
- [ ] Inline code documentation
- [ ] Examples in docs/

### 6.4 Performance
- [ ] Startup time under 100ms
- [ ] Minimal memory footprint
- [ ] Efficient test execution

## 7. Risks and Mitigations

### 7.1 Playwright MCP Dependency
**Risk**: MCP server unavailable or incompatible
**Mitigation**: Graceful error handling, clear setup instructions, version compatibility checks

### 7.2 Browser Compatibility
**Risk**: Tests fail on different browsers
**Mitigation**: Document supported browsers, provide browser installation instructions

### 7.3 Performance at Scale
**Risk**: Slow with large test suites
**Mitigation**: Parallel execution, test filtering, incremental runs

## 8. Open Questions

1. Should we support test discovery auto-magic or require explicit paths?
   - **Decision**: Support both - auto-discover by convention, allow explicit paths

2. What test file naming convention should we enforce?
   - **Decision**: `*_test.rs` or `test_*.rs` patterns

3. Should config file be required or optional?
   - **Decision**: Optional, with sensible defaults

4. How to handle browser installation?
   - **Decision**: Document requirement, provide setup check command

## 9. Timeline

### Phase 1: MVP (Week 1)
- Basic CLI structure with clap
- Help and version outputs
- Simple test execution (single file)
- Basic reporting
- Documentation

### Phase 2: Enhancement (Week 2)
- Directory/glob support
- JSON/JUnit output formats
- Config file support
- Parallel execution

### Phase 3: Integration (Week 3)
- Playwright MCP integration
- Browser automation
- Advanced test features
- CI/CD templates

## 10. Appendix

### 10.1 References
- Playwright MCP: https://github.com/microsoft/playwright-mcp
- Software Wrighter Standards: See docs/process.md
- Rust Best Practices: https://rust-lang.github.io/api-guidelines/

### 10.2 Related Documents
- docs/architecture.md - System design and component structure
- docs/design.md - Detailed design decisions
- docs/plan.md - Implementation plan and task breakdown
- docs/status.md - Current project status

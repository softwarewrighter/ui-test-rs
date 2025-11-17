# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`ui-test-rs` is a Rust CLI tool for UI testing that integrates with Playwright MCP (Model Context Protocol) for browser automation. It uses accessibility trees instead of screenshots for deterministic element selection.

**Current Status**: Phase 1 - Foundation (documentation complete, implementation starting)

## Architecture

The system follows a layered architecture:

1. **CLI Interface** (main.rs) - clap-based argument parsing with AI-specific help sections
2. **Test Runner** (runner.rs) - Orchestrates test execution lifecycle
3. **Test Loader** (loader.rs) - Discovers test files via glob patterns (*_test.rs, test_*.rs)
4. **Playwright MCP Client** (playwright.rs) - Communicates with Playwright MCP server for browser automation
5. **Reporter** (reporter.rs) - Outputs results in text, JSON, or JUnit XML formats

**Key Design Decisions**:
- Configuration precedence: CLI flags > environment variables > config file > defaults
- Async runtime: tokio for all async operations
- Element selection: Accessibility tree (not pixel-based)
- Test discovery: Convention-based (*_test.rs pattern)

## CRITICAL: Always Use Scripts

**ALWAYS use scripts in `./scripts/` for all build and operational commands.**

This ensures:
- Reproducible builds with proper metadata (build host, commit, timestamp)
- Consistent environment setup
- Detection of script regressions through regular use
- Documentation of operational procedures

**If the command you need is not in `./scripts/`, create a script for it before running.**

Available scripts:
- `./scripts/build-all.sh [dev|release]` - Build with metadata (default: release)

## Build and Test Commands

### Building
```bash
# ALWAYS use the build script (captures build metadata)
./scripts/build-all.sh          # Release build (default)
./scripts/build-all.sh release  # Explicit release build
./scripts/build-all.sh dev      # Development build

# Install locally after building
./scripts/build-all.sh && sw-install -p .

# NEVER use cargo build directly - always use ./scripts/build-all.sh
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test cli_tests
```

### Quality Checks
```bash
# Linting (strict - no warnings allowed)
cargo clippy --all-targets --all-features -- -D warnings

# Formatting
cargo fmt --all

# Check formatting without changing
cargo fmt --all -- --check

# Validate markdown documentation
markdown-checker -f "**/*.md"

# Validate project standards
sw-checklist .
```

## Mandatory Pre-Commit Process

**CRITICAL**: Before EVERY commit, run these steps in order. ALL must pass:

```bash
# 0. Build with metadata (ALWAYS use the script)
./scripts/build-all.sh

# 1. Tests
cargo test

# 2. Linting (zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# 3. Formatting
cargo fmt --all

# 4. Markdown validation
markdown-checker -f "**/*.md"

# 5. Project standards validation
sw-checklist .

# 6. Update status
# Edit docs/status.md with current progress

# 7. Commit and push immediately
git add -A
git commit -m "type: description

[AI] Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
git push
```

**No exceptions**: Never disable checks, never skip steps, never defer fixes.

## Development Workflow

This project uses **Test-Driven Development (TDD)** with strict Red/Green/Refactor cycles:

1. **RED**: Write a failing test that defines expected behavior
2. **GREEN**: Write minimal code to make the test pass
3. **REFACTOR**: Improve code while keeping tests green
4. **REPEAT**: Continue for next functionality

### Code Constraints

- **File size**: Maximum 500 lines (prefer 200-300)
- **Function size**: Maximum 50 lines (prefer 10-30)
- **TODO comments**: Maximum 3 per file
- **FIXME comments**: Never commit - resolve immediately
- **Rust edition**: 2021 (using 2024 idioms where available)

### Rust-Specific Standards

- Use `cargo clippy` suggestions - trust its recommendations
- Inline format arguments: `format!("{name}")` not `format!("{}", name)`
- Inner doc comments for modules: `//!` not `///` + empty line
- Module docs: Use `//!` at top of file
- Item docs: Use `///` before items
- Remove unused imports after refactoring

## sw-checklist Requirements

The tool must pass `sw-checklist` validation which checks:

### Help Output Requirements
- **Short help** (`-h`): Concise, single-screen overview
- **Extended help** (`--help`): Full documentation WITH "AI CODING AGENT INSTRUCTIONS" section
- AI section must include usage patterns, exit codes, and integration examples

### Version Output Requirements
- **Short version** (`-V`): Version number only
- **Extended version** (`--version`): Must include:
  - Version number
  - Copyright notice: "Copyright (c) 2025 Michael A Wright"
  - License: "License: MIT (https://opensource.org/licenses/MIT)"
  - Repository URL

## Configuration System

Three-level precedence (highest to lowest):
1. CLI flags (e.g., `--verbose`)
2. Environment variables (e.g., `UI_TEST_VERBOSE=1`)
3. Config file (`ui-test.toml`)
4. Default values

All components must respect this precedence order.

## Playwright MCP Integration

### Connection
- MCP server spawned as subprocess: `npx -y @playwright/mcp`
- Communication via stdin/stdout JSON-RPC
- Browser lifecycle managed per-test or shared based on config

### Element Selection Strategy
Use accessibility tree, NOT screenshot-based:
- Prefer role + name: `button[name="Submit"]`
- Support ARIA labels: `[aria-label="Search"]`
- Support text content: `text="Login"`
- Fallback to CSS selectors: `#username`

### Error Handling
- Timeout default: 30 seconds per test
- Provide actionable error messages with context
- Suggest fixes: "Element not found - try taking screenshot for debugging"

## Testing Strategy

### Unit Tests
- Location: `#[cfg(test)]` modules in source files
- Coverage: Config parsing, test discovery, formatting, error handling
- Use `tempfile` for filesystem operations

### Integration Tests
- Location: `tests/` directory
- Coverage: CLI argument parsing, end-to-end flows, output formats
- Use `assert_cmd` and `predicates` for CLI testing

### Playwright Tests
- Location: `tests/playwright/` (future)
- Use MCP in tests to validate Playwright integration

## Documentation Requirements

When making changes:
- Update `docs/status.md` with current progress
- If you encounter issues, document in `docs/learnings.md` with:
  - What went wrong
  - Why it wasn't caught sooner
  - What process change prevents this
- Update README.md if features added or changed
- Add inline documentation for all public APIs

## Key Files and Their Purpose

- `docs/prd.md` - Product requirements and feature specifications
- `docs/architecture.md` - System design and component interactions
- `docs/design.md` - Implementation details and patterns
- `docs/plan.md` - Task breakdown and timeline
- `docs/status.md` - Current progress (update regularly)
- `docs/next-steps.md` - Immediate next actions to take
- `docs/process.md` - Full development workflow reference
- `docs/tools.md` - Development tools (markdown-checker, sw-checklist, sw-install)

## Common Pitfalls

Based on learnings from similar projects:

1. **Doc comments**: Use `//!` for module docs at top of file, not `///` + empty line
2. **Unused imports**: Remove after refactoring - clippy will catch these
3. **Format arguments**: Use inline `"{name}"` not `"{}", name` (Rust 2021+)
4. **Needless borrows**: Trust clippy on generic arguments
5. **File organization**: Split files before 500 lines, extract functions before 50 lines

## Environment Setup

Required tools in PATH:
- `markdown-checker` - Validates markdown is ASCII-only
- `sw-checklist` - Validates project standards
- `sw-install` - Installs built binaries to ~/.local/softwarewrighter/bin/

These are available in `~/.local/softwarewrighter/bin/` - ensure this is in your PATH.

## Exit Codes

The tool follows this convention:
- `0` - Success (all tests passed)
- `1` - Test failures (some tests failed)
- `2` - Error (config error, discovery error, MCP connection failure, etc.)

## License

Copyright (c) 2025 Michael A Wright. Licensed under MIT License.

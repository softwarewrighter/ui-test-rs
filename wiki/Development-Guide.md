# Development Guide

This guide covers building, testing, and contributing to ui-test-rs.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Building](#building)
- [Testing](#testing)
- [Quality Checks](#quality-checks)
- [Development Workflow](#development-workflow)
- [Project Standards](#project-standards)
- [Common Tasks](#common-tasks)

## Prerequisites

### Required Tools

```mermaid
graph TB
    subgraph Core["Core Requirements"]
        Rust[Rust Toolchain<br/>Edition 2021+]
        Cargo[Cargo<br/>Package Manager]
    end

    subgraph DevTools["Development Tools"]
        Clippy[cargo clippy<br/>Linting]
        Fmt[cargo fmt<br/>Formatting]
    end

    subgraph CustomTools["Custom Tools"]
        MD[markdown-checker<br/>Doc Validation]
        CL[sw-checklist<br/>Standards Validation]
        SI[sw-install<br/>Binary Installation]
    end

    subgraph Runtime["Runtime Dependencies"]
        Node[Node.js v20+<br/>Playwright MCP]
        PW[Playwright MCP<br/>@playwright/mcp]
    end

    Core --> DevTools
    DevTools --> CustomTools
    Core --> Runtime

    style Core fill:#e1f5ff
    style DevTools fill:#e8f5e9
    style CustomTools fill:#fff9c4
    style Runtime fill:#f3e5f5
```

### Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (for Playwright MCP)
# macOS
brew install node

# Linux
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs

# Verify installations
rustc --version
cargo --version
node --version
npm --version
```

### Custom Tools Setup

The custom tools are available in `~/.local/softwarewrighter/bin/`:

```bash
# Add to PATH (add to .bashrc or .zshrc)
export PATH="$HOME/.local/softwarewrighter/bin:$PATH"

# Verify tools are available
markdown-checker --version
sw-checklist --version
sw-install --version
```

## Building

### CRITICAL: Always Use Build Scripts

**NEVER use `cargo build` directly. ALWAYS use the build script.**

```bash
# Release build (default) - ALWAYS USE THIS
./scripts/build-all.sh

# Development build
./scripts/build-all.sh dev

# Explicit release build
./scripts/build-all.sh release
```

### Why Use Scripts?

```mermaid
flowchart TB
    Script[./scripts/build-all.sh]

    Meta[Capture Build Metadata<br/>• Host<br/>• Commit<br/>• Timestamp]
    Env[Set Environment Variables]
    Build[Run Cargo Build]
    Verify[Verify Build Success]

    Script --> Meta
    Meta --> Env
    Env --> Build
    Build --> Verify

    style Script fill:#c8e6c9
    style Meta fill:#fff9c4
    style Verify fill:#e8f5e9
```

### Build Process

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Script
    participant Git
    participant Cargo
    participant Binary

    Dev->>Script: ./scripts/build-all.sh
    activate Script

    Script->>Git: get commit hash
    Git-->>Script: commit SHA

    Script->>Script: get hostname
    Script->>Script: get timestamp

    Script->>Cargo: cargo build --release
    activate Cargo
    Cargo->>Cargo: compile with metadata
    Cargo-->>Binary: binary created
    deactivate Cargo

    Script-->>Dev: build complete
    deactivate Script
```

### Installation

```bash
# Build and install in one step
./scripts/build-all.sh && sw-install -p .

# Verify installation
ui-test-rs --version
```

## Testing

### Test Types

```mermaid
mindmap
  root((Tests))
    Unit Tests
      Config parsing
      Test discovery
      Formatters
      Error handling
    Integration Tests
      CLI arguments
      End-to-end flows
      Output formats
    Future
      Playwright tests
      Browser automation
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test cli_tests

# Run with verbose output
cargo test -- --test-threads=1 --nocapture
```

### Test Organization

```mermaid
graph TB
    subgraph UnitTests["Unit Tests (in source files)"]
        Config[config.rs<br/>#[cfg(test)] mod tests]
        Loader[loader.rs<br/>#[cfg(test)] mod tests]
        Reporter[reporter.rs<br/>#[cfg(test)] mod tests]
    end

    subgraph IntegrationTests["Integration Tests (tests/ directory)"]
        CLI[cli_tests.rs]
        E2E[e2e_tests.rs]
    end

    style UnitTests fill:#e8f5e9
    style IntegrationTests fill:#fff9c4
```

## Quality Checks

### Pre-Commit Checks

**Run these in order before EVERY commit:**

```mermaid
flowchart TB
    Start([Start Pre-Commit])
    Build[./scripts/build-all.sh]
    Test[cargo test]
    Clippy[cargo clippy]
    Fmt[cargo fmt --all]
    MDCheck[markdown-checker]
    SWCheck[sw-checklist]
    Status[Update docs/status.md]
    Done([Ready to Commit])

    Fail([Fix Errors])

    Start --> Build
    Build -->|Success| Test
    Build -->|Fail| Fail
    Test -->|Pass| Clippy
    Test -->|Fail| Fail
    Clippy -->|Pass| Fmt
    Clippy -->|Fail| Fail
    Fmt -->|Success| MDCheck
    Fmt -->|Fail| Fail
    MDCheck -->|Pass| SWCheck
    MDCheck -->|Fail| Fail
    SWCheck -->|Pass| Status
    SWCheck -->|Fail| Fail
    Status --> Done

    style Start fill:#c8e6c9
    style Done fill:#c8e6c9
    style Fail fill:#ffcdd2
```

### Individual Checks

```bash
# 0. Build with metadata (ALWAYS use the script)
./scripts/build-all.sh

# 1. Tests
cargo test

# 2. Linting (zero warnings allowed)
cargo clippy --all-targets --all-features -- -D warnings

# 3. Formatting
cargo fmt --all

# 4. Markdown validation
markdown-checker -f "**/*.md"

# 5. Project standards
sw-checklist .

# 6. Update status
vim docs/status.md  # Update current progress
```

### Linting Rules

```mermaid
graph TB
    Clippy[cargo clippy]

    subgraph Rules["Clippy Rules"]
        Warn[Warnings → Errors<br/>-D warnings]
        Unused[No unused imports]
        Needless[No needless borrows]
        Format[Inline format args]
    end

    Clippy --> Rules

    Pass{All Rules<br/>Pass?}
    Success([Success])
    Fix([Fix Issues])

    Rules --> Pass
    Pass -->|Yes| Success
    Pass -->|No| Fix

    style Clippy fill:#fff9c4
    style Success fill:#c8e6c9
    style Fix fill:#ffcdd2
```

## Development Workflow

### TDD Cycle

ui-test-rs follows strict Test-Driven Development:

```mermaid
flowchart LR
    Red[🔴 RED<br/>Write Failing Test]
    Green[🟢 GREEN<br/>Make Test Pass]
    Refactor[🔵 REFACTOR<br/>Improve Code]

    Red --> Green
    Green --> Refactor
    Refactor --> Red

    style Red fill:#ffcdd2
    style Green fill:#c8e6c9
    style Refactor fill:#e1f5ff
```

### Development Process

```mermaid
sequenceDiagram
    participant Dev as Developer
    participant Test
    participant Code
    participant Quality

    Dev->>Test: Write failing test (RED)
    activate Test
    Test-->>Dev: Test fails ✗
    deactivate Test

    Dev->>Code: Write minimal code (GREEN)
    activate Code
    Code-->>Test: Run test
    activate Test
    Test-->>Dev: Test passes ✓
    deactivate Test
    deactivate Code

    Dev->>Code: Refactor (REFACTOR)
    activate Code
    Code-->>Test: Run tests
    activate Test
    Test-->>Dev: All tests pass ✓
    deactivate Test
    deactivate Code

    Dev->>Quality: Run quality checks
    activate Quality
    Quality-->>Dev: All checks pass ✓
    deactivate Quality

    Dev->>Dev: Commit changes
```

## Project Standards

### Code Constraints

```mermaid
graph TB
    subgraph Constraints["Code Constraints"]
        FileSize[File Size<br/>Max: 500 lines<br/>Prefer: 200-300]
        FuncSize[Function Size<br/>Max: 50 lines<br/>Prefer: 10-30]
        TODO[TODO Comments<br/>Max: 3 per file]
        FIXME[FIXME Comments<br/>Never commit]
    end

    subgraph RustStandards["Rust Standards"]
        Edition[Edition: 2021<br/>Idioms: 2024]
        Inline[Inline format args<br/>format!"{name}"]
        DocComments[Module docs: //!<br/>Item docs: ///]
    end

    style Constraints fill:#ffecb3
    style RustStandards fill:#e8f5e9
```

### File Organization

```mermaid
flowchart TB
    Start([New Feature])
    Check{File > 500<br/>lines?}
    Split[Split into<br/>Multiple Files]
    FuncCheck{Function > 50<br/>lines?}
    Extract[Extract<br/>Helper Functions]
    Done([Implementation Complete])

    Start --> Check
    Check -->|Yes| Split
    Check -->|No| FuncCheck
    Split --> FuncCheck
    FuncCheck -->|Yes| Extract
    FuncCheck -->|No| Done
    Extract --> Done

    style Start fill:#c8e6c9
    style Done fill:#c8e6c9
    style Split fill:#fff9c4
    style Extract fill:#fff9c4
```

### Documentation Standards

```rust
//! Module-level documentation
//! Use //! at the top of files

/// Item-level documentation
/// Use /// before items (functions, structs, etc.)
pub fn example() {
    // Implementation
}
```

## Common Tasks

### Adding a New Feature

```mermaid
flowchart TB
    Start([New Feature Request])
    Plan[Plan Implementation<br/>Update docs/plan.md]
    Test[Write Tests<br/>RED phase]
    Implement[Implement Feature<br/>GREEN phase]
    Refactor[Refactor Code<br/>REFACTOR phase]
    Quality[Run Quality Checks]
    UpdateDocs[Update Documentation<br/>• README.md<br/>• docs/status.md]
    Commit[Commit & Push]
    Done([Feature Complete])

    Start --> Plan
    Plan --> Test
    Test --> Implement
    Implement --> Refactor
    Refactor --> Quality
    Quality --> UpdateDocs
    UpdateDocs --> Commit
    Commit --> Done

    style Start fill:#c8e6c9
    style Done fill:#c8e6c9
```

### Fixing a Bug

```mermaid
flowchart TB
    Start([Bug Reported])
    Reproduce[Write Failing Test<br/>Reproducing Bug]
    Fix[Fix Bug<br/>Make Test Pass]
    Verify[Verify All Tests Pass]
    Quality[Run Quality Checks]
    Document[Document in<br/>docs/learnings.md]
    Commit[Commit & Push]
    Done([Bug Fixed])

    Start --> Reproduce
    Reproduce --> Fix
    Fix --> Verify
    Verify --> Quality
    Quality --> Document
    Document --> Commit
    Commit --> Done

    style Start fill:#ffecb3
    style Done fill:#c8e6c9
```

### Refactoring Code

```mermaid
flowchart TB
    Start([Refactoring Needed])
    EnsureTests[Ensure Tests Exist<br/>Add if missing]
    RunTests[Run Tests<br/>All should pass]
    Refactor[Refactor Code]
    TestAgain[Run Tests Again]
    Pass{Tests<br/>Pass?}
    Quality[Run Quality Checks]
    Commit[Commit Changes]
    Done([Refactoring Complete])
    Fix([Fix Broken Tests<br/>or Code])

    Start --> EnsureTests
    EnsureTests --> RunTests
    RunTests --> Refactor
    Refactor --> TestAgain
    TestAgain --> Pass
    Pass -->|Yes| Quality
    Pass -->|No| Fix
    Fix --> Refactor
    Quality --> Commit
    Commit --> Done

    style Start fill:#e1f5ff
    style Done fill:#c8e6c9
    style Fix fill:#ffcdd2
```

## Commit Process

### Commit Message Format

```
type: brief description

[AI] Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Commit Types

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

### Full Commit Workflow

```bash
# After all quality checks pass

# Stage all changes
git add -A

# Commit with message
git commit -m "feat: Add browser action timeout handling

[AI] Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

# Push to remote
git push
```

## Troubleshooting

### Common Issues

```mermaid
mindmap
  root((Common Issues))
    Build Failures
      Forgot to use script
      Missing dependencies
      Outdated Rust version
    Test Failures
      Flaky tests
      Missing test data
      Environment issues
    Clippy Warnings
      Unused imports
      Needless borrows
      Format arguments
    Markdown Errors
      Non-ASCII characters
      Invalid formatting
```

### Debug Tips

```bash
# Verbose build
RUST_BACKTRACE=1 ./scripts/build-all.sh

# Run single test with output
cargo test test_name -- --nocapture

# Check what clippy would fix
cargo clippy --fix

# Preview formatting changes
cargo fmt --all -- --check
```

## Related Documentation

- [Architecture](Architecture) - System architecture
- [Testing Strategy](Testing-Strategy) - Testing approach
- [CLI Interface](CLI-Interface) - Command-line interface
- [Configuration](Configuration) - Configuration system

---

**Last Updated:** 2025-11-18

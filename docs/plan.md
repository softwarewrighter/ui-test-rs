# Implementation Plan

## Project: ui-test-rs

**Version**: 0.1.0
**Date**: 2025-11-16
**Author**: Michael A Wright

## 1. Project Phases

### Phase 1: Foundation (MVP)
**Goal**: Basic CLI with help/version, passes sw-checklist

**Duration**: 1-2 days

**Deliverables**:
- CLI structure with clap
- Help output (short and extended with AI instructions)
- Version output with copyright
- Basic error handling
- MIT LICENSE
- README.md
- Passes all quality gates

### Phase 2: Core Functionality
**Goal**: Test discovery and basic execution

**Duration**: 2-3 days

**Deliverables**:
- Test file discovery
- Configuration loading (file + env + CLI)
- Test runner skeleton
- Text reporter
- Unit tests

### Phase 3: Playwright Integration
**Goal**: Browser automation via MCP

**Duration**: 3-4 days

**Deliverables**:
- Playwright MCP client
- Browser actions (navigate, click, type)
- Element selection via accessibility tree
- Screenshot support
- Integration tests

### Phase 4: Advanced Features
**Goal**: Parallel execution, multiple formats

**Duration**: 2-3 days

**Deliverables**:
- Parallel test execution
- JSON reporter
- JUnit XML reporter
- Test filtering
- Performance optimizations

### Phase 5: Polish
**Goal**: Production-ready release

**Duration**: 1-2 days

**Deliverables**:
- Complete documentation
- Examples and tutorials
- CI/CD templates
- Release artifacts
- v0.1.0 tag

## 2. Phase 1: Foundation (Current)

### 2.1 Tasks

#### Task 1.1: Project Setup
**Status**: In Progress

**Subtasks**:
- [x] Read documentation (process.md, tools.md, ai_agent_instructions.md)
- [x] Create project planning docs (PRD, architecture, design, plan, status)
- [ ] Create MIT LICENSE file
- [ ] Update .gitignore
- [ ] Update Cargo.toml with metadata

**Acceptance Criteria**:
- All planning docs created
- LICENSE file present
- .gitignore covers build artifacts
- Cargo.toml has all required fields

#### Task 1.2: CLI Structure
**Status**: Not Started

**Subtasks**:
- [ ] Add clap dependency
- [ ] Define CLI struct with derive macros
- [ ] Implement argument parsing
- [ ] Add AI_INSTRUCTIONS constant
- [ ] Implement version_info function

**Acceptance Criteria**:
- `ui-test-rs -h` shows short help
- `ui-test-rs --help` shows extended help with AI section
- `ui-test-rs -V` shows version with copyright
- `ui-test-rs --version` shows full version info

#### Task 1.3: Basic Error Handling
**Status**: Not Started

**Subtasks**:
- [ ] Add anyhow and thiserror dependencies
- [ ] Define UiTestError enum
- [ ] Implement error formatting
- [ ] Add exit code handling

**Acceptance Criteria**:
- Errors display clearly
- Exit codes: 0 (success), 1 (failure), 2 (error)
- Stack traces in verbose mode

#### Task 1.4: Documentation
**Status**: In Progress

**Subtasks**:
- [x] Create docs/prd.md
- [x] Create docs/architecture.md
- [x] Create docs/design.md
- [x] Create docs/plan.md
- [ ] Create docs/status.md
- [ ] Create README.md
- [ ] Add inline code documentation

**Acceptance Criteria**:
- All docs/ files created and complete
- README.md covers installation, usage, examples
- All docs pass markdown-checker
- Code has doc comments on public items

#### Task 1.5: Quality Gates
**Status**: Not Started

**Subtasks**:
- [ ] Run cargo test (all pass)
- [ ] Run cargo clippy (zero warnings)
- [ ] Run cargo fmt
- [ ] Run markdown-checker on all .md files
- [ ] Run sw-checklist (all pass)

**Acceptance Criteria**:
- All tests pass
- Zero clippy warnings
- All code formatted
- All markdown ASCII-only
- sw-checklist reports success

### 2.2 Dependencies for Phase 1

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
```

### 2.3 File Structure After Phase 1

```
ui-test-rs/
+- Cargo.toml
+- LICENSE
+- README.md
+- .gitignore
+- docs/
|  +- prd.md
|  +- architecture.md
|  +- design.md
|  +- plan.md
|  +- status.md
|  +- ai_agent_instructions.md
|  +- process.md
|  +- tools.md
+- src/
|  +- main.rs
|  +- error.rs
+- tests/
   +- cli_tests.rs
```

### 2.4 Success Criteria for Phase 1

- [ ] CLI runs and shows help/version
- [ ] All quality gates pass
- [ ] Documentation complete
- [ ] sw-checklist validation passes
- [ ] Ready to commit and push

## 3. Phase 2: Core Functionality

### 3.1 Tasks

#### Task 2.1: Configuration System
**Subtasks**:
- [ ] Add serde and toml dependencies
- [ ] Define Config struct
- [ ] Implement config file loading
- [ ] Implement env var parsing
- [ ] Implement precedence resolution
- [ ] Add unit tests for config merging

#### Task 2.2: Test Discovery
**Subtasks**:
- [ ] Add glob dependency
- [ ] Implement file pattern matching
- [ ] Parse test file metadata
- [ ] Build test suite hierarchy
- [ ] Add test filtering
- [ ] Unit tests for discovery

#### Task 2.3: Test Runner Skeleton
**Subtasks**:
- [ ] Define TestRunner struct
- [ ] Implement test lifecycle
- [ ] Add timeout handling
- [ ] Collect test results
- [ ] Integration tests

#### Task 2.4: Text Reporter
**Subtasks**:
- [ ] Define Reporter trait
- [ ] Implement TextReporter
- [ ] Format test results
- [ ] Display summary statistics
- [ ] Unit tests for formatting

### 3.2 Dependencies for Phase 2

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
glob = "0.3"
```

## 4. Phase 3: Playwright Integration

### 4.1 Tasks

#### Task 3.1: MCP Client
**Subtasks**:
- [ ] Add tokio dependency
- [ ] Implement MCP connection
- [ ] Define command/response protocol
- [ ] Handle connection errors
- [ ] Unit tests for client

#### Task 3.2: Browser Actions
**Subtasks**:
- [ ] Implement navigate action
- [ ] Implement click action
- [ ] Implement type action
- [ ] Implement snapshot retrieval
- [ ] Implement screenshot capture

#### Task 3.3: Element Selection
**Subtasks**:
- [ ] Parse accessibility snapshots
- [ ] Implement selector queries
- [ ] Match elements by role/name
- [ ] Support CSS selectors
- [ ] Error messages for not found

#### Task 3.4: Test Context API
**Subtasks**:
- [ ] Define TestContext struct
- [ ] Implement convenience methods
- [ ] Add assertion helpers
- [ ] Handle async operations
- [ ] Integration tests with Playwright

### 4.2 Dependencies for Phase 3

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

## 5. Phase 4: Advanced Features

### 5.1 Tasks

#### Task 4.1: Parallel Execution
**Subtasks**:
- [ ] Implement worker pool
- [ ] Distribute tests across workers
- [ ] Collect results concurrently
- [ ] Handle worker failures
- [ ] Performance tests

#### Task 4.2: JSON Reporter
**Subtasks**:
- [ ] Define JSON schema
- [ ] Implement JsonReporter
- [ ] Serialize results
- [ ] Unit tests for output

#### Task 4.3: JUnit Reporter
**Subtasks**:
- [ ] Define JUnit XML schema
- [ ] Implement JunitReporter
- [ ] Generate XML output
- [ ] Validate against schema
- [ ] Unit tests

#### Task 4.4: Test Filtering
**Subtasks**:
- [ ] Implement name pattern matching
- [ ] Implement tag filtering
- [ ] Combine multiple filters
- [ ] Unit tests

## 6. Phase 5: Polish

### 6.1 Tasks

#### Task 5.1: Documentation
**Subtasks**:
- [ ] Complete README with examples
- [ ] Write user guide
- [ ] Create tutorial
- [ ] Document all public APIs
- [ ] Add troubleshooting section

#### Task 5.2: Examples
**Subtasks**:
- [ ] Create example test files
- [ ] Write getting started guide
- [ ] Add CI/CD templates
- [ ] Create best practices doc

#### Task 5.3: Release Preparation
**Subtasks**:
- [ ] Update version to 0.1.0
- [ ] Create CHANGELOG.md
- [ ] Build release binaries
- [ ] Create GitHub release
- [ ] Tag v0.1.0

## 7. Task Breakdown by File

### main.rs
**Lines**: ~200

**Tasks**:
- CLI struct definition (50 lines)
- Argument parsing (30 lines)
- Config loading (30 lines)
- Main execution flow (50 lines)
- Error handling (20 lines)
- Help/version constants (20 lines)

### config.rs
**Lines**: ~250

**Tasks**:
- Config struct definition (80 lines)
- File loading (50 lines)
- Env var parsing (40 lines)
- Precedence resolution (50 lines)
- Tests (30 lines)

### discovery.rs
**Lines**: ~200

**Tasks**:
- TestLoader struct (30 lines)
- Pattern matching (50 lines)
- File parsing (50 lines)
- Filtering (40 lines)
- Tests (30 lines)

### runner.rs
**Lines**: ~300

**Tasks**:
- TestRunner struct (50 lines)
- Test execution (80 lines)
- Lifecycle management (50 lines)
- Timeout handling (40 lines)
- Result collection (50 lines)
- Tests (30 lines)

### playwright.rs
**Lines**: ~350

**Tasks**:
- PlaywrightClient struct (60 lines)
- Connection management (70 lines)
- Command protocol (80 lines)
- Element selection (80 lines)
- Error handling (40 lines)
- Tests (20 lines)

### reporter.rs
**Lines**: ~400

**Tasks**:
- Reporter trait (30 lines)
- TextReporter (100 lines)
- JsonReporter (100 lines)
- JunitReporter (120 lines)
- Tests (50 lines)

### error.rs
**Lines**: ~150

**Tasks**:
- Error enum (50 lines)
- Error formatting (50 lines)
- Suggestions (30 lines)
- Tests (20 lines)

**Total Estimated Lines**: ~1,850 (within budget)

## 8. Testing Strategy

### 8.1 Unit Tests

**Target Coverage**: 80%+

**Focus Areas**:
- Config parsing and merging
- Test discovery patterns
- Reporter formatting
- Element selection
- Error messages

### 8.2 Integration Tests

**Test Cases**:
- CLI argument parsing
- End-to-end test execution
- Config file loading
- Output format validation
- Playwright MCP integration

### 8.3 Playwright Tests

**Test Cases**:
- Browser navigation
- Element interaction
- Screenshot capture
- Accessibility tree parsing
- Error handling

## 9. Risk Management

### Risk 1: Playwright MCP Compatibility
**Probability**: Medium
**Impact**: High
**Mitigation**:
- Test with multiple Playwright versions
- Document minimum required version
- Provide fallback error messages

### Risk 2: Parallel Execution Complexity
**Probability**: High
**Impact**: Medium
**Mitigation**:
- Start with sequential execution
- Add parallelism incrementally
- Extensive testing with various worker counts

### Risk 3: Cross-Platform Support
**Probability**: Medium
**Impact**: Medium
**Mitigation**:
- Test on macOS, Linux, Windows
- Use cross-platform dependencies
- Document platform-specific issues

### Risk 4: Performance at Scale
**Probability**: Low
**Impact**: Medium
**Mitigation**:
- Benchmark with large test suites
- Profile and optimize hot paths
- Document performance characteristics

## 10. Milestones

### Milestone 1: CLI Basics (Phase 1)
**Date**: Day 2
**Criteria**: Help/version working, passes sw-checklist

### Milestone 2: Test Discovery (Phase 2)
**Date**: Day 5
**Criteria**: Can discover and list tests

### Milestone 3: Playwright Working (Phase 3)
**Date**: Day 9
**Criteria**: Can execute simple browser test

### Milestone 4: Full Featured (Phase 4)
**Date**: Day 12
**Criteria**: All reporters, parallel execution

### Milestone 5: Release (Phase 5)
**Date**: Day 14
**Criteria**: v0.1.0 released to GitHub

## 11. Daily Checklist

**Before Starting Work**:
- [ ] Review status.md
- [ ] Check current phase tasks
- [ ] Read any new documentation

**During Development**:
- [ ] Follow TDD Red/Green/Refactor
- [ ] Keep functions under 50 lines
- [ ] Keep files under 500 lines
- [ ] Maximum 3 TODOs per file
- [ ] Run tests frequently

**Before Committing** (Pre-Commit Process):
- [ ] cargo test (all pass)
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [ ] cargo fmt --all
- [ ] markdown-checker -f "**/*.md"
- [ ] Update docs/status.md
- [ ] Update docs/learnings.md if issues found

**After Committing**:
- [ ] git push immediately
- [ ] Verify CI passes (when added)

## 12. Dependencies Summary

### Production Dependencies
```toml
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
anyhow = "1.0"
thiserror = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
glob = "0.3"
```

### Development Dependencies
```toml
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.0"
```

## 13. Next Steps

**Immediate (Today)**:
1. Complete Phase 1 Task 1.1 (Project Setup)
2. Start Phase 1 Task 1.2 (CLI Structure)
3. Write basic unit tests
4. Run pre-commit quality gates
5. Create first commit

**Tomorrow**:
1. Finish Phase 1 tasks
2. Validate with sw-checklist
3. Start Phase 2 (Configuration)

**This Week**:
1. Complete Phases 1-2
2. Begin Playwright integration
3. First working prototype

## 14. Success Metrics

**Code Quality**:
- Zero clippy warnings
- 80%+ test coverage
- All docs validate with markdown-checker
- Passes sw-checklist

**Functionality**:
- Can run basic UI tests
- Clear error messages
- Multiple output formats
- Good performance (< 100ms startup)

**Documentation**:
- Complete README with examples
- All public APIs documented
- Architecture and design docs
- User guide and tutorials

**Process**:
- All commits follow pre-commit process
- TDD for all features
- Regular pushes to remote
- Status.md kept up-to-date

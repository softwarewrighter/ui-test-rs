# Test Loader

The Test Loader component (`loader.rs`) is responsible for discovering test files, parsing test metadata, and building a test execution plan.

## Overview

```mermaid
flowchart TB
    Loader[Test Loader loader.rs]
    FS[File System]
    Parser[Test Parser]
    Filter[Test Filter]
    Suite[Test Suite]

    Loader --> FS
    Loader --> Parser
    Loader --> Filter
    Loader --> Suite

    FS -.->|Files| Loader
    Parser -.->|Test Functions| Loader
    Filter -.->|Filtered Tests| Loader

    style Loader fill:#e8f5e9
    style FS fill:#f5f5f5
    style Parser fill:#fff9c4
    style Filter fill:#e1f5ff
    style Suite fill:#e1bee7
```

## Responsibilities

- Discover test files using glob patterns
- Parse test file metadata
- Extract test functions and their attributes
- Filter tests by name or tags
- Build test execution plan
- Validate test structure

## Structure

### TestLoader Struct

```rust
struct TestLoader {
    patterns: Vec<String>,
    exclude: Vec<String>,
    root_path: PathBuf,
}

impl TestLoader {
    fn new(root: PathBuf, patterns: Vec<String>) -> Self;
    fn discover_tests(&self) -> Result<Vec<TestSuite>>;
    fn apply_filter(&self, suites: Vec<TestSuite>, filter: &str) -> Vec<TestSuite>;
}
```

### Component Diagram

```mermaid
classDiagram
    class TestLoader {
        -Vec~String~ patterns
        -Vec~String~ exclude
        -PathBuf root_path
        +new(root, patterns) Self
        +discover_tests() Result~Vec~TestSuite~~
        +apply_filter(suites, filter) Vec~TestSuite~
    }

    class TestSuite {
        +String name
        +PathBuf file_path
        +Vec~TestCase~ tests
    }

    class TestCase {
        +String name
        +Vec~String~ tags
        +Option~String~ description
    }

    TestLoader --> TestSuite
    TestSuite "1" --> "*" TestCase
```

## Discovery Process

### Discovery Flow

```mermaid
flowchart TB
    Start([Start Discovery])
    GetPatterns[Get Glob Patterns *_test.rs, test_*.rs]
    ApplyGlob[Apply Glob to Root Path]
    MoreFiles{More Files?}

    ReadFile[Read File Content]
    CheckExclude{Matches Exclude?}
    SkipFile[Skip File]

    ValidRust{Valid Rust Syntax?}
    WarnInvalid[Log Warning Skip File]

    ParseFile[Parse File for Tests]
    HasTests{Has Test Functions?}
    SkipEmpty[Skip Empty File]

    CreateSuite[Create Test Suite]
    AddToResults[Add to Results]

    AllDone{All Files Processed?}
    SortSuites[Sort Suites by Path]
    Done([Return Test Suites])

    Start --> GetPatterns
    GetPatterns --> ApplyGlob
    ApplyGlob --> MoreFiles

    MoreFiles -->|Yes| ReadFile
    MoreFiles -->|No| SortSuites

    ReadFile --> CheckExclude
    CheckExclude -->|Yes| SkipFile
    CheckExclude -->|No| ValidRust

    SkipFile --> MoreFiles

    ValidRust -->|No| WarnInvalid
    ValidRust -->|Yes| ParseFile

    WarnInvalid --> MoreFiles

    ParseFile --> HasTests
    HasTests -->|No| SkipEmpty
    HasTests -->|Yes| CreateSuite

    SkipEmpty --> MoreFiles

    CreateSuite --> AddToResults
    AddToResults --> MoreFiles

    SortSuites --> Done

    style Start fill:#c8e6c9
    style Done fill:#e1bee7
    style WarnInvalid fill:#ffecb3
    style SkipFile fill:#f5f5f5
    style SkipEmpty fill:#f5f5f5
```

### Discovery Sequence

```mermaid
sequenceDiagram
    participant Runner
    participant Loader
    participant FS as File System
    participant Parser
    participant Filter

    Runner->>Loader: discover_tests()
    activate Loader

    Loader->>Loader: get patterns

    loop For Each Pattern
        Loader->>FS: glob(pattern)
        FS-->>Loader: matching file paths
    end

    loop For Each File
        Loader->>FS: read_file(path)
        FS-->>Loader: file contents

        alt Valid Rust File
            Loader->>Parser: parse_test_file(contents)
            activate Parser
            Parser->>Parser: extract test functions
            Parser-->>Loader: Vec<TestCase>
            deactivate Parser

            alt Has Tests
                Loader->>Loader: create TestSuite
            else No Tests
                Loader->>Loader: skip file
            end
        else Invalid File
            Loader->>Loader: log warning, skip
        end
    end

    Loader->>Loader: sort suites
    Loader-->>Runner: Vec<TestSuite>
    deactivate Loader
```

## Test File Patterns

### Supported Patterns

```mermaid
graph TB
    subgraph Patterns["Discovery Patterns"]
        P1[*_test.rs Suffix Pattern]
        P2[test_*.rs Prefix Pattern]
        P3[tests/**/*.rs Directory Pattern]
    end

    subgraph Examples["File Examples"]
        E1[login_test.rs ✓]
        E2[test_login.rs ✓]
        E3[tests/auth/login.rs ✓]
        E4[utils.rs ✗]
    end

    P1 -.-> E1
    P2 -.-> E2
    P3 -.-> E3
    P3 -.-> E4

    style P1 fill:#c8e6c9
    style P2 fill:#c8e6c9
    style P3 fill:#c8e6c9
    style E1 fill:#e8f5e9
    style E2 fill:#e8f5e9
    style E3 fill:#e8f5e9
    style E4 fill:#ffcdd2
```

### Exclude Patterns

```mermaid
flowchart LR
    Files[All .rs Files]
    Exclude[Exclude Patterns]

    subgraph Excluded["Excluded Paths"]
        E1[target/**]
        E2[node_modules/**]
        E3[.git/**]
        E4[vendor/**]
    end

    Files --> Exclude
    Exclude -.-> E1
    Exclude -.-> E2
    Exclude -.-> E3
    Exclude -.-> E4

    Exclude --> Filtered[Filtered Files]

    style Files fill:#e1f5ff
    style Exclude fill:#fff9c4
    style Excluded fill:#ffcdd2
    style Filtered fill:#c8e6c9
```

## Test Parsing

### Parsing Process

```mermaid
flowchart TB
    Start([File Content])
    Tokenize[Tokenize Rust Code]
    FindFunctions[Find Function Definitions]
    MoreFuncs{More Functions?}

    CheckAttrs[Check Attributes]
    HasTestAttr{Has #[test] or #[test_case]?}
    SkipFunc[Skip Function]

    ExtractName[Extract Function Name]
    ExtractTags[Extract Tags from Attrs]
    ExtractDesc[Extract Description]

    CreateTestCase[Create TestCase]
    AddToSuite[Add to Test Suite]

    Done([Test Cases Extracted])

    Start --> Tokenize
    Tokenize --> FindFunctions
    FindFunctions --> MoreFuncs

    MoreFuncs -->|Yes| CheckAttrs
    MoreFuncs -->|No| Done

    CheckAttrs --> HasTestAttr
    HasTestAttr -->|No| SkipFunc
    HasTestAttr -->|Yes| ExtractName

    SkipFunc --> MoreFuncs

    ExtractName --> ExtractTags
    ExtractTags --> ExtractDesc
    ExtractDesc --> CreateTestCase
    CreateTestCase --> AddToSuite
    AddToSuite --> MoreFuncs

    style Start fill:#c8e6c9
    style Done fill:#e1bee7
    style SkipFunc fill:#f5f5f5
```

### Test Attributes

```rust
// Example test file with attributes
use ui_test_rs::prelude::*;

/// Tests user login functionality
#[test_case("login_admin")]
#[tags("smoke", "auth")]
async fn test_admin_login(ctx: &mut TestContext) -> Result<()> {
    // Test implementation
    Ok(())
}

#[test_case("login_invalid")]
#[tags("auth", "negative")]
async fn test_invalid_credentials(ctx: &mut TestContext) -> Result<()> {
    // Test implementation
    Ok(())
}
```

### Attribute Diagram

```mermaid
classDiagram
    class TestFunction {
        +String name
        +Vec~Attribute~ attributes
        +Vec~Parameter~ parameters
        +ReturnType return_type
    }

    class Attribute {
        +String name
        +Vec~String~ values
    }

    class TestCase {
        +String name
        +Vec~String~ tags
        +Option~String~ description
    }

    TestFunction "1" --> "*" Attribute
    Attribute ..> TestCase : extracts
```

## Test Filtering

### Filter Types

```mermaid
flowchart TB
    AllTests[All Discovered Tests]

    Filter{Filter Applied?}

    NameFilter[Name Pattern Filter]
    TagFilter[Tag Filter]
    NoFilter[No Filter]

    MatchName[Match Test Name Against Pattern]
    MatchTag[Match Test Tags Against Filter]

    FilteredTests[Filtered Test Set]

    AllTests --> Filter

    Filter -->|Name Pattern| NameFilter
    Filter -->|Tag Filter| TagFilter
    Filter -->|None| NoFilter

    NameFilter --> MatchName
    TagFilter --> MatchTag
    NoFilter --> FilteredTests

    MatchName --> FilteredTests
    MatchTag --> FilteredTests

    style AllTests fill:#e1f5ff
    style FilteredTests fill:#c8e6c9
    style Filter fill:#fff9c4
```

### Filter Examples

```mermaid
graph TB
    subgraph Original["Original Tests (10)"]
        T1[test_login_admin]
        T2[test_login_user]
        T3[test_logout]
        T4[test_signup]
        T5[test_password_reset]
        T6[test_checkout]
        T7[test_cart_add]
        T8[test_cart_remove]
        T9[test_search]
        T10[test_filter]
    end

    subgraph Filtered["Filtered: 'login' (2)"]
        F1[test_login_admin]
        F2[test_login_user]
    end

    T1 -.-> F1
    T2 -.-> F2

    style Filtered fill:#c8e6c9
```

### Filter Algorithm

```mermaid
flowchart LR
    Start([Filter Tests])
    Pattern[Get Filter Pattern]

    Loop{More Tests?}
    GetTest[Get Next Test]
    Match{Name Matches?}

    KeepTest[Keep Test]
    SkipTest[Skip Test]

    Done([Filtered Tests])

    Start --> Pattern
    Pattern --> Loop

    Loop -->|Yes| GetTest
    Loop -->|No| Done

    GetTest --> Match
    Match -->|Yes| KeepTest
    Match -->|No| SkipTest

    KeepTest --> Loop
    SkipTest --> Loop

    style Start fill:#c8e6c9
    style Done fill:#e1bee7
    style KeepTest fill:#c8e6c9
    style SkipTest fill:#f5f5f5
```

## Test Suite Structure

### Suite Hierarchy

```mermaid
graph TB
    Root[Test Root]

    subgraph Suite1["TestSuite: login_test.rs"]
        S1T1[TestCase: test_admin_login]
        S1T2[TestCase: test_user_login]
        S1T3[TestCase: test_failed_login]
    end

    subgraph Suite2["TestSuite: checkout_test.rs"]
        S2T1[TestCase: test_add_to_cart]
        S2T2[TestCase: test_remove_item]
        S2T3[TestCase: test_complete_purchase]
    end

    Root --> Suite1
    Root --> Suite2

    style Root fill:#e1f5ff
    style Suite1 fill:#e8f5e9
    style Suite2 fill:#fff9c4
```

## Error Handling

### Discovery Errors

```mermaid
flowchart TB
    Error([Discovery Error])
    Type{Error Type?}

    PathNotFound[Path Not Found]
    InvalidPattern[Invalid Glob Pattern]
    ParseError[File Parse Error]
    PermissionDenied[Permission Denied]

    Recover{Recoverable?}

    LogWarning[Log Warning Continue]
    FailDiscovery[Fail Discovery Exit Code 2]

    Error --> Type

    Type -->|Path| PathNotFound
    Type -->|Pattern| InvalidPattern
    Type -->|Parse| ParseError
    Type -->|Permission| PermissionDenied

    PathNotFound --> Recover
    InvalidPattern --> FailDiscovery
    ParseError --> Recover
    PermissionDenied --> Recover

    Recover -->|Yes| LogWarning
    Recover -->|No| FailDiscovery

    style Error fill:#ffcdd2
    style FailDiscovery fill:#b71c1c,color:#fff
    style LogWarning fill:#ffecb3
```

## Performance Optimization

### Parallel Discovery

```mermaid
sequenceDiagram
    participant Loader
    participant Worker1
    participant Worker2
    participant WorkerN
    participant Collector

    Loader->>Loader: split file list

    par Worker 1
        Loader->>Worker1: files 1-N
        Worker1->>Worker1: parse files
        Worker1->>Collector: test suites
    and Worker 2
        Loader->>Worker2: files N-M
        Worker2->>Worker2: parse files
        Worker2->>Collector: test suites
    and Worker N
        Loader->>WorkerN: files M-Z
        WorkerN->>WorkerN: parse files
        WorkerN->>Collector: test suites
    end

    Collector->>Loader: aggregated suites
```

## Configuration

### Loader Configuration

```rust
struct LoaderConfig {
    patterns: Vec<String>,
    exclude: Vec<String>,
    follow_symlinks: bool,
    max_depth: Option<usize>,
}
```

## Usage Examples

### Basic Discovery

```rust
let loader = TestLoader::new(
    PathBuf::from("tests"),
    vec!["*_test.rs".to_string(), "test_*.rs".to_string()],
);

let suites = loader.discover_tests()?;
```

### With Filtering

```rust
let loader = TestLoader::new(root, patterns);
let suites = loader.discover_tests()?;
let filtered = loader.apply_filter(suites, "login");
```

## Validation

### Test Suite Validation

```mermaid
flowchart TB
    Start([Validate Suite])
    CheckEmpty{Suite Empty?}
    EmptyError[Error: No tests found]

    CheckDuplicates{Duplicate Names?}
    DuplicateError[Error: Duplicate test names]

    CheckValid{Valid Test Signatures?}
    SignatureError[Error: Invalid test signature]

    Valid([Suite Valid])

    Start --> CheckEmpty
    CheckEmpty -->|Yes| EmptyError
    CheckEmpty -->|No| CheckDuplicates

    CheckDuplicates -->|Yes| DuplicateError
    CheckDuplicates -->|No| CheckValid

    CheckValid -->|No| SignatureError
    CheckValid -->|Yes| Valid

    style Start fill:#c8e6c9
    style Valid fill:#c8e6c9
    style EmptyError fill:#ffcdd2
    style DuplicateError fill:#ffcdd2
    style SignatureError fill:#ffcdd2
```

## Related Documentation

- [Test Runner](Test-Runner) - Test execution
- [CLI Interface](CLI-Interface) - Command-line interface
- [Configuration](Configuration) - Configuration system
- [Architecture](Architecture) - System architecture
- [Sequences](Sequences) - Data flows

---

**Last Updated:** 2025-11-18

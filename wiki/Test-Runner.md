# Test Runner

The Test Runner component (`runner.rs`) orchestrates the entire test execution lifecycle, managing test execution, coordinating with other components, and collecting results.

## Overview

```mermaid
flowchart TB
    Runner[Test Runner runner.rs]
    Loader[Test Loader]
    PW[Playwright Client]
    Reporter[Reporter]
    Collector[Result Collector]

    Runner --> Loader
    Runner --> PW
    Runner --> Reporter
    Runner --> Collector

    Loader -.->|Test Suites| Runner
    PW -.->|Browser Actions| Runner
    Collector -.->|Results| Reporter

    style Runner fill:#fff4e1
    style Loader fill:#e8f5e9
    style PW fill:#f3e5f5
    style Reporter fill:#fce4ec
    style Collector fill:#e1bee7
```

## Responsibilities

- Orchestrate test execution lifecycle
- Manage test execution (sequential or parallel)
- Coordinate between Loader, Playwright, and Reporter
- Handle test timeouts
- Collect and aggregate results
- Determine final exit code

## Structure

### TestRunner Struct

```rust
struct TestRunner {
    config: Config,
    playwright: PlaywrightClient,
    reporter: Box<dyn Reporter>,
}

impl TestRunner {
    fn new(config: Config) -> Result<Self>;
    async fn run(&mut self, test_path: PathBuf) -> Result<TestStats>;
    async fn execute_test(&self, test: &TestCase) -> TestResult;
}
```

### Component Diagram

```mermaid
classDiagram
    class TestRunner {
        -Config config
        -PlaywrightClient playwright
        -Box~Reporter~ reporter
        +new(config: Config) Result~Self~
        +run(test_path: PathBuf) Result~TestStats~
        -execute_test(test: TestCase) TestResult
        -run_parallel(tests: Vec) Vec~TestResult~
        -run_sequential(tests: Vec) Vec~TestResult~
    }

    class TestResult {
        +String test_name
        +TestStatus status
        +Duration duration
        +Option~String~ error
    }

    class TestStatus {
        <<enumeration>>
        Passed
        Failed
        Skipped
        Error
    }

    class TestStats {
        +usize total
        +usize passed
        +usize failed
        +usize skipped
        +Duration duration
    }

    TestRunner --> TestResult
    TestRunner --> TestStats
    TestResult --> TestStatus
```

## Execution Lifecycle

### Full Test Execution Flow

```mermaid
sequenceDiagram
    participant Main
    participant Runner
    participant Loader
    participant PW as Playwright
    participant Reporter

    Main->>Runner: run(test_path)
    activate Runner

    Runner->>Loader: discover_tests(path)
    activate Loader
    Loader-->>Runner: test_suites
    deactivate Loader

    Runner->>Reporter: report_start(suites)
    Reporter-->>Runner: started

    Runner->>PW: connect()
    activate PW
    PW-->>Runner: connected
    deactivate PW

    loop For Each Test
        Runner->>Runner: execute_test(test)
        activate Runner
        Runner->>PW: run test actions
        PW-->>Runner: actions complete
        Runner-->>Runner: test_result
        deactivate Runner
        Runner->>Reporter: report_test(result)
    end

    Runner->>Reporter: report_end(stats)
    Runner-->>Main: TestStats

    deactivate Runner
```

### Execution State Machine

```mermaid
stateDiagram-v2
    [*] --> Initializing: run() called
    Initializing --> Discovering: Config ready
    Discovering --> Connecting: Tests loaded
    Connecting --> Ready: MCP connected

    Ready --> Executing: Start tests
    Executing --> Executing: More tests

    Executing --> Collecting: All tests done
    Collecting --> Reporting: Results aggregated
    Reporting --> Complete: Report generated

    Initializing --> Failed: Init error
    Discovering --> Failed: Discovery error
    Connecting --> Failed: Connection error
    Executing --> Failed: Fatal error

    Complete --> [*]: Success
    Failed --> [*]: Error

    note right of Initializing
        Load configuration
        Create components
    end note

    note right of Executing
        Sequential or
        Parallel execution
    end note
```

## Execution Modes

### Sequential Execution

```mermaid
sequenceDiagram
    participant Runner
    participant Test1
    participant Test2
    participant Test3
    participant Collector

    Runner->>Test1: execute()
    activate Test1
    Test1-->>Runner: Result
    deactivate Test1
    Runner->>Collector: collect(result1)

    Runner->>Test2: execute()
    activate Test2
    Test2-->>Runner: Result
    deactivate Test2
    Runner->>Collector: collect(result2)

    Runner->>Test3: execute()
    activate Test3
    Test3-->>Runner: Result
    deactivate Test3
    Runner->>Collector: collect(result3)

    Collector-->>Runner: all results
```

### Parallel Execution

```mermaid
sequenceDiagram
    participant Runner
    participant Semaphore
    participant Worker1
    participant Worker2
    participant WorkerN
    participant Collector

    Runner->>Semaphore: new(max_jobs)
    Runner->>Collector: create channel

    par Worker 1
        Runner->>Worker1: spawn test1
        Worker1->>Semaphore: acquire()
        activate Worker1
        Worker1->>Worker1: execute()
        Worker1->>Collector: send(result)
        Worker1->>Semaphore: release()
        deactivate Worker1
    and Worker 2
        Runner->>Worker2: spawn test2
        Worker2->>Semaphore: acquire()
        activate Worker2
        Worker2->>Worker2: execute()
        Worker2->>Collector: send(result)
        Worker2->>Semaphore: release()
        deactivate Worker2
    and Worker N
        Runner->>WorkerN: spawn testN
        WorkerN->>Semaphore: acquire()
        activate WorkerN
        WorkerN->>WorkerN: execute()
        WorkerN->>Collector: send(result)
        WorkerN->>Semaphore: release()
        deactivate WorkerN
    end

    Collector->>Runner: aggregate results
```

### Execution Mode Comparison

```mermaid
graph TB
    subgraph Sequential["Sequential Execution (jobs = 1)"]
        S1[Test 1] --> S2[Test 2]
        S2 --> S3[Test 3]
        S3 --> S4[Test 4]
    end

    subgraph Parallel["Parallel Execution (jobs = 4)"]
        P1[Test 1]
        P2[Test 2]
        P3[Test 3]
        P4[Test 4]
    end

    style Sequential fill:#fff9c4
    style Parallel fill:#c8e6c9
```

## Test Lifecycle Management

### Per-Test Lifecycle

```mermaid
flowchart TB
    Start([Test Selected])
    Setup[Setup Phase before_each]
    Execute[Execute Phase Run test function]
    Assert[Assert Phase Check expectations]
    Cleanup[Cleanup Phase after_each]
    Collect[Collect Result]
    Done([Test Complete])

    Error[Handle Error]

    Start --> Setup
    Setup -->|Success| Execute
    Setup -->|Error| Error
    Execute -->|Success| Assert
    Execute -->|Error| Error
    Assert -->|Success| Cleanup
    Assert -->|Failure| Error
    Error --> Cleanup
    Cleanup --> Collect
    Collect --> Done

    style Start fill:#c8e6c9
    style Error fill:#ffcdd2
    style Done fill:#e1bee7
```

### Resource Management

```mermaid
sequenceDiagram
    participant Runner
    participant Browser
    participant Resources

    Runner->>Browser: initialize()
    activate Browser
    Runner->>Resources: allocate()
    activate Resources

    loop Test Execution
        Runner->>Browser: use browser
        Runner->>Resources: use resources
    end

    alt Normal Completion
        Runner->>Resources: cleanup()
        deactivate Resources
        Runner->>Browser: close()
        deactivate Browser
    else Error or Timeout
        Runner->>Resources: force cleanup()
        deactivate Resources
        Runner->>Browser: force close()
        deactivate Browser
    end
```

## Timeout Handling

### Timeout Architecture

```mermaid
flowchart TB
    StartTest[Start Test Execution]
    Timer[Start Timeout Timer]
    Execute[Execute Test]

    Race{Which Completes?}

    TestDone[Test Completes]
    Timeout[Timeout Fires]

    CancelTimer[Cancel Timer]
    CancelTest[Cancel Test]

    Success[Return Success Result]
    TimeoutError[Return Timeout Error]

    StartTest --> Timer
    Timer --> Execute

    Execute --> Race
    Race -->|Test First| TestDone
    Race -->|Timer First| Timeout

    TestDone --> CancelTimer
    Timeout --> CancelTest

    CancelTimer --> Success
    CancelTest --> TimeoutError

    style StartTest fill:#c8e6c9
    style Timeout fill:#ffecb3
    style TimeoutError fill:#ffcdd2
    style Success fill:#c8e6c9
```

### Timeout Implementation

```rust
async fn execute_with_timeout(
    test: &TestCase,
    timeout: Duration,
) -> TestResult {
    match tokio::time::timeout(timeout, execute_test(test)).await {
        Ok(result) => result,
        Err(_) => TestResult {
            test_name: test.name.clone(),
            status: TestStatus::Error,
            duration: timeout,
            error: Some(format!("Test timed out after {:?}", timeout)),
        },
    }
}
```

## Result Collection

### Collection Process

```mermaid
flowchart TB
    Start([Test Results Ready])
    InitStats[Initialize Statistics]
    MoreResults{More Results?}
    GetResult[Get Next Result]
    UpdateStats[Update Statistics]
    CheckStatus{Status?}

    IncrPassed[Increment Passed]
    IncrFailed[Increment Failed]
    IncrSkipped[Increment Skipped]
    IncrError[Increment Error]

    AddDuration[Add Duration]
    Calculate[Calculate Percentages]
    Done([Statistics Ready])

    Start --> InitStats
    InitStats --> MoreResults
    MoreResults -->|Yes| GetResult
    MoreResults -->|No| Calculate
    GetResult --> UpdateStats
    UpdateStats --> CheckStatus

    CheckStatus -->|Passed| IncrPassed
    CheckStatus -->|Failed| IncrFailed
    CheckStatus -->|Skipped| IncrSkipped
    CheckStatus -->|Error| IncrError

    IncrPassed --> AddDuration
    IncrFailed --> AddDuration
    IncrSkipped --> AddDuration
    IncrError --> AddDuration

    AddDuration --> MoreResults
    Calculate --> Done

    style Start fill:#c8e6c9
    style Done fill:#e1bee7
    style IncrPassed fill:#c8e6c9
    style IncrFailed fill:#ffcdd2
    style IncrError fill:#ffcdd2
```

### Statistics Structure

```mermaid
classDiagram
    class TestStats {
        +usize total
        +usize passed
        +usize failed
        +usize skipped
        +usize error
        +Duration total_duration
        +f64 pass_rate
        +success_rate() f64
        +failure_rate() f64
    }

    class TestResult {
        +String test_name
        +TestStatus status
        +Duration duration
        +Option~String~ error
    }

    TestStats "1" --> "*" TestResult
```

## Error Handling

### Error Recovery Strategy

```mermaid
flowchart TB
    Error([Error Occurs])
    Classify{Error Type?}

    TestError[Test Execution Error]
    MCPError[MCP Connection Error]
    TimeoutError[Timeout Error]
    FatalError[Fatal Error]

    RecoverTest{Fail Fast?}
    RecoverMCP{Retry?}

    MarkFailed[Mark Test Failed Continue]
    StopAll[Stop All Tests]
    RetryMCP[Retry Connection]
    Exit[Exit with Error]

    Error --> Classify
    Classify -->|Test| TestError
    Classify -->|MCP| MCPError
    Classify -->|Timeout| TimeoutError
    Classify -->|Fatal| FatalError

    TestError --> RecoverTest
    RecoverTest -->|No| MarkFailed
    RecoverTest -->|Yes| StopAll

    MCPError --> RecoverMCP
    RecoverMCP -->|Yes| RetryMCP
    RecoverMCP -->|No| Exit

    TimeoutError --> MarkFailed
    FatalError --> Exit

    style Error fill:#ffcdd2
    style Exit fill:#b71c1c,color:#fff
    style MarkFailed fill:#ffecb3
    style StopAll fill:#ff9800,color:#fff
```

## Exit Code Determination

### Exit Code Logic

```mermaid
flowchart TB
    Start([Test Execution Complete])
    CheckErrors{Any Fatal Errors?}
    Exit2([Exit Code 2])

    CheckFailed{Any Test Failures?}
    Exit1([Exit Code 1])

    AllPassed{All Tests Passed?}
    Exit0([Exit Code 0])

    Start --> CheckErrors
    CheckErrors -->|Yes| Exit2
    CheckErrors -->|No| CheckFailed
    CheckFailed -->|Yes| Exit1
    CheckFailed -->|No| AllPassed
    AllPassed -->|Yes| Exit0
    AllPassed -->|No| Exit1

    style Start fill:#c8e6c9
    style Exit0 fill:#4caf50,color:#fff
    style Exit1 fill:#ff9800,color:#fff
    style Exit2 fill:#f44336,color:#fff
```

## Performance Optimization

### Parallel Execution Strategy

```mermaid
graph TB
    subgraph WorkerPool["Worker Pool"]
        W1[Worker 1 Browser Instance 1]
        W2[Worker 2 Browser Instance 2]
        W3[Worker 3 Browser Instance 3]
        W4[Worker 4 Browser Instance 4]
    end

    Queue[Test Queue]
    Semaphore[Semaphore Max: 4]

    Queue -.-> Semaphore
    Semaphore --> W1
    Semaphore --> W2
    Semaphore --> W3
    Semaphore --> W4

    W1 -.-> Results[Results Collector]
    W2 -.-> Results
    W3 -.-> Results
    W4 -.-> Results

    style Queue fill:#e1f5ff
    style Semaphore fill:#fff4e1
    style WorkerPool fill:#e8f5e9
    style Results fill:#fce4ec
```

## Configuration

### Runner Configuration

```rust
struct RunnerConfig {
    // Execution mode
    jobs: usize,              // Number of parallel workers
    fail_fast: bool,          // Stop on first failure

    // Timeouts
    test_timeout: Duration,   // Per-test timeout
    total_timeout: Duration,  // Total execution timeout

    // Retry settings
    retry_count: usize,       // Number of retries for failed tests
    retry_delay: Duration,    // Delay between retries
}
```

## Usage Examples

### Basic Usage

```rust
// Create runner with config
let runner = TestRunner::new(config)?;

// Execute tests
let stats = runner.run(PathBuf::from("tests/")).await?;

// Check results
if stats.failed > 0 {
    std::process::exit(1);
}
```

### With Custom Reporter

```rust
let reporter = Box::new(JsonReporter::new());
let mut runner = TestRunner {
    config,
    playwright: PlaywrightClient::connect(&config.playwright)?,
    reporter,
};

let stats = runner.run(test_path).await?;
```

## Related Documentation

- [CLI Interface](CLI-Interface) - Command-line interface
- [Test Loader](Test-Loader) - Test discovery
- [Playwright Client](Playwright-Client) - Browser automation
- [Reporter](Reporter) - Result reporting
- [Architecture](Architecture) - System architecture
- [Sequences](Sequences) - Execution flows

---

**Last Updated:** 2025-11-18

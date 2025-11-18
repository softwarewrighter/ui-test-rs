# Reporter

The Reporter component (`reporter.rs`) is responsible for formatting and outputting test results in various formats including text, JSON, and JUnit XML.

## Overview

```mermaid
flowchart TB
    Reporter[Reporter reporter.rs]
    Text[Text Reporter]
    Json[JSON Reporter]
    Junit[JUnit Reporter]
    Output[Output stdout/files]

    Reporter --> Text
    Reporter --> Json
    Reporter --> Junit

    Text --> Output
    Json --> Output
    Junit --> Output

    style Reporter fill:#fce4ec
    style Text fill:#e8f5e9
    style Json fill:#fff9c4
    style Junit fill:#e1f5ff
    style Output fill:#e1bee7
```

## Responsibilities

- Format test results for different output types
- Generate test statistics
- Output to console or files
- Support colorized output
- Calculate test metrics (pass rate, duration, etc.)
- Provide actionable error messages

## Structure

### Reporter Trait

```rust
trait Reporter {
    fn report_start(&mut self, suite: &TestSuite);
    fn report_test(&mut self, result: &TestResult);
    fn report_end(&mut self, stats: &TestStats);
}

struct TextReporter {
    verbose: bool,
    color: bool,
}

struct JsonReporter {
    output_path: Option<PathBuf>,
}

struct JunitReporter {
    output_path: PathBuf,
}
```

### Component Diagram

```mermaid
classDiagram
    class Reporter {
        <<interface>>
        +report_start(suite)
        +report_test(result)
        +report_end(stats)
    }

    class TextReporter {
        -bool verbose
        -bool color
        +new(verbose, color) Self
    }

    class JsonReporter {
        -Option~PathBuf~ output_path
        +new(path) Self
    }

    class JunitReporter {
        -PathBuf output_path
        +new(path) Self
    }

    class TestStats {
        +usize total
        +usize passed
        +usize failed
        +usize skipped
        +Duration duration
        +pass_rate() f64
    }

    Reporter <|-- TextReporter
    Reporter <|-- JsonReporter
    Reporter <|-- JunitReporter
    Reporter ..> TestStats
```

## Reporting Flow

### Reporting Sequence

```mermaid
sequenceDiagram
    participant Runner
    participant Reporter
    participant Output

    Runner->>Reporter: report_start(suite)
    activate Reporter
    Reporter->>Output: write header
    deactivate Reporter

    loop For Each Test
        Runner->>Reporter: report_test(result)
        activate Reporter
        Reporter->>Reporter: format result
        Reporter->>Output: write result line
        deactivate Reporter
    end

    Runner->>Reporter: report_end(stats)
    activate Reporter
    Reporter->>Reporter: calculate statistics
    Reporter->>Reporter: format summary
    Reporter->>Output: write summary
    deactivate Reporter
```

### Reporting State Machine

```mermaid
stateDiagram-v2
    [*] --> NotStarted
    NotStarted --> Started: report_start()
    Started --> Reporting: report_test()
    Reporting --> Reporting: more tests
    Reporting --> Ending: report_end()
    Ending --> Complete: summary written
    Complete --> [*]

    note right of Started
        Output header
        Show test suite info
    end note

    note right of Reporting
        Format each result
        Track statistics
    end note

    note right of Ending
        Calculate metrics
        Output summary
    end note
```

## Text Reporter

### Text Output Format

```
Running 5 tests from tests/

test tests/login_test.rs::test_admin_login ... ok (1.2s)
test tests/login_test.rs::test_user_login ... ok (0.8s)
test tests/checkout_test.rs::test_add_to_cart ... FAILED (2.1s)
test tests/checkout_test.rs::test_remove_item ... ok (1.0s)
test tests/search_test.rs::test_search ... ok (0.9s)

Failures:

---- tests/checkout_test.rs::test_add_to_cart ----
Error: Element not found: button[name="Add to Cart"]
  at tests/checkout_test.rs:15

  Suggestion: Check if the button exists in the page.
  Try: Take a screenshot to debug: ctx.screenshot("debug.png")

Summary:
  5 tests, 4 passed, 1 failed, 0 skipped
  Duration: 6.0s
  Pass rate: 80.0%

Exit code: 1
```

### Text Formatting Flow

```mermaid
flowchart TB
    Start([Format Text])
    Header[Write Header 'Running N tests from...']

    MoreTests{More Tests?}
    GetResult[Get Test Result]
    CheckStatus{Status?}

    FormatPassed[Format as: ... ok (Xs)]
    FormatFailed[Format as: ... FAILED (Xs)]
    FormatSkipped[Format as: ... skipped]
    FormatError[Format as: ... ERROR (Xs)]

    WriteResult[Write Result Line]

    AllDone{All Tests Reported?}

    FormatFailures[Format Failure Details]
    FormatSummary[Format Summary Stats]
    WriteSummary[Write Summary]

    Done([Output Complete])

    Start --> Header
    Header --> MoreTests

    MoreTests -->|Yes| GetResult
    GetResult --> CheckStatus

    CheckStatus -->|Passed| FormatPassed
    CheckStatus -->|Failed| FormatFailed
    CheckStatus -->|Skipped| FormatSkipped
    CheckStatus -->|Error| FormatError

    FormatPassed --> WriteResult
    FormatFailed --> WriteResult
    FormatSkipped --> WriteResult
    FormatError --> WriteResult

    WriteResult --> AllDone
    AllDone -->|More| MoreTests
    AllDone -->|Done| FormatFailures

    FormatFailures --> FormatSummary
    FormatSummary --> WriteSummary
    WriteSummary --> Done

    style Start fill:#c8e6c9
    style Done fill:#e1bee7
    style FormatPassed fill:#c8e6c9
    style FormatFailed fill:#ffcdd2
    style FormatError fill:#ffcdd2
```

### Colorized Output

```mermaid
graph TB
    subgraph Colors["Color Scheme"]
        Green[Passed: Green]
        Red[Failed: Red]
        Yellow[Skipped: Yellow]
        Cyan[Info: Cyan]
        Gray[Duration: Gray]
    end

    Output[Console Output]
    ColorEnabled{Color Enabled?}
    Apply[Apply Colors]
    Plain[Plain Text]

    Output --> ColorEnabled
    ColorEnabled -->|Yes| Apply
    ColorEnabled -->|No| Plain

    Apply --> Colors

    style Green fill:#c8e6c9
    style Red fill:#ffcdd2
    style Yellow fill:#fff9c4
    style Cyan fill:#e1f5ff
    style Gray fill:#f5f5f5
```

## JSON Reporter

### JSON Output Format

```json
{
  "version": "1.0",
  "timestamp": "2025-11-18T12:00:00Z",
  "suite": "tests/",
  "total": 5,
  "passed": 4,
  "failed": 1,
  "skipped": 0,
  "error": 0,
  "duration_ms": 6000,
  "pass_rate": 0.8,
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
      "error": {
        "message": "Element not found: button[name=\"Add to Cart\"]",
        "location": "tests/checkout_test.rs:15",
        "suggestion": "Check if the button exists in the page."
      }
    }
  ]
}
```

### JSON Structure

```mermaid
classDiagram
    class JsonReport {
        +String version
        +String timestamp
        +String suite
        +usize total
        +usize passed
        +usize failed
        +usize skipped
        +usize error
        +u64 duration_ms
        +f64 pass_rate
        +Vec~JsonTest~ tests
    }

    class JsonTest {
        +String name
        +String status
        +u64 duration_ms
        +Option~JsonError~ error
    }

    class JsonError {
        +String message
        +Option~String~ location
        +Option~String~ suggestion
    }

    JsonReport "1" --> "*" JsonTest
    JsonTest "1" --> "0..1" JsonError
```

## JUnit Reporter

### JUnit XML Format

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuites tests="5" failures="1" errors="0" time="6.0">
  <testsuite name="ui-test-rs" tests="5" failures="1" errors="0" time="6.0" timestamp="2025-11-18T12:00:00Z">
    <testcase name="test_admin_login" classname="tests.login_test" time="1.2"/>
    <testcase name="test_user_login" classname="tests.login_test" time="0.8"/>
    <testcase name="test_add_to_cart" classname="tests.checkout_test" time="2.1">
      <failure message="Element not found">
        Error: Element not found: button[name="Add to Cart"]
        at tests/checkout_test.rs:15

        Suggestion: Check if the button exists in the page.
        Try: Take a screenshot to debug: ctx.screenshot("debug.png")
      </failure>
    </testcase>
    <testcase name="test_remove_item" classname="tests.checkout_test" time="1.0"/>
    <testcase name="test_search" classname="tests.search_test" time="0.9"/>
  </testsuite>
</testsuites>
```

### JUnit Structure

```mermaid
graph TB
    TestSuites[testsuites Root Element]
    TestSuite[testsuite Suite Element]

    TestCase1[testcase test_admin_login]
    TestCase2[testcase test_add_to_cart]
    TestCase3[testcase test_remove_item]

    Failure[failure Error Details]

    TestSuites --> TestSuite
    TestSuite --> TestCase1
    TestSuite --> TestCase2
    TestSuite --> TestCase3

    TestCase2 --> Failure

    style TestSuites fill:#e1f5ff
    style TestSuite fill:#fff9c4
    style TestCase1 fill:#c8e6c9
    style TestCase2 fill:#ffcdd2
    style TestCase3 fill:#c8e6c9
    style Failure fill:#ffcdd2
```

## Statistics Calculation

### Metrics Calculation

```mermaid
flowchart TB
    Start([Calculate Statistics])
    InitCounters[Initialize Counters total, passed, failed, etc.]
    StartTimer[Start Duration Timer]

    MoreResults{More Results?}
    GetResult[Get Next Result]
    IncrementTotal[Increment Total]

    CheckStatus{Status?}
    IncrPassed[Increment Passed]
    IncrFailed[Increment Failed]
    IncrSkipped[Increment Skipped]
    IncrError[Increment Error]

    AddDuration[Add Test Duration]

    CalcPassRate[Calculate Pass Rate passed / total]
    CalcFailRate[Calculate Fail Rate failed / total]
    CalcAvgDuration[Calculate Average Duration total_duration / total]

    Done([Statistics Ready])

    Start --> InitCounters
    InitCounters --> StartTimer
    StartTimer --> MoreResults

    MoreResults -->|Yes| GetResult
    GetResult --> IncrementTotal
    IncrementTotal --> CheckStatus

    CheckStatus -->|Passed| IncrPassed
    CheckStatus -->|Failed| IncrFailed
    CheckStatus -->|Skipped| IncrSkipped
    CheckStatus -->|Error| IncrError

    IncrPassed --> AddDuration
    IncrFailed --> AddDuration
    IncrSkipped --> AddDuration
    IncrError --> AddDuration

    AddDuration --> MoreResults

    MoreResults -->|No| CalcPassRate
    CalcPassRate --> CalcFailRate
    CalcFailRate --> CalcAvgDuration
    CalcAvgDuration --> Done

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
        +pass_rate() f64
        +failure_rate() f64
        +success_rate() f64
        +average_duration() Duration
    }

    class Metrics {
        +f64 pass_rate
        +f64 fail_rate
        +Duration avg_duration
        +Duration min_duration
        +Duration max_duration
    }

    TestStats --> Metrics : calculates
```

## Error Message Formatting

### Error Format

```mermaid
flowchart TB
    Error([Test Error])
    GetDetails[Get Error Details]
    FormatMessage[Format Error Message]
    AddLocation[Add Location Info]
    AddContext[Add Context]
    AddSuggestion[Add Suggestion]
    Output[Formatted Error]

    Error --> GetDetails
    GetDetails --> FormatMessage
    FormatMessage --> AddLocation
    AddLocation --> AddContext
    AddContext --> AddSuggestion
    AddSuggestion --> Output

    style Error fill:#ffcdd2
    style Output fill:#ffecb3
```

### Error Message Example

```
---- tests/checkout_test.rs::test_add_to_cart ----
Error: Element not found: button[name="Add to Cart"]
  at tests/checkout_test.rs:15

  Context: Looking for element with selector: button[name="Add to Cart"]
  Page: https://example.com/checkout

  Suggestion: Check if the button exists in the page.
  Try: Take a screenshot to debug: ctx.screenshot("debug.png")
```

## Verbose Mode

### Verbose Output

```mermaid
graph TB
    VerboseMode{Verbose Enabled?}

    Normal[Normal Output Pass/Fail Status Only]
    Verbose[Verbose Output + Test Details + Actions Taken + Timing Info]

    VerboseMode -->|No| Normal
    VerboseMode -->|Yes| Verbose

    style Normal fill:#e8f5e9
    style Verbose fill:#fff9c4
```

### Verbose Output Example

```
test tests/login_test.rs::test_admin_login ...
  [00:00.123] Navigate to https://example.com/login
  [00:00.456] Fill input[name='username'] with 'admin'
  [00:00.678] Fill input[name='password'] with 'secret'
  [00:00.890] Click button[name='Submit']
  [00:01.123] Assert URL contains '/dashboard'
ok (1.2s)
```

## Output Destination

### Output Flow

```mermaid
flowchart LR
    Reporter[Reporter]
    Format{Output Format?}

    Text[Text]
    Json[JSON]
    Junit[JUnit]

    StdOut[stdout]
    JsonFile[results.json]
    JunitFile[junit.xml]

    Reporter --> Format

    Format -->|Text| Text
    Format -->|JSON| Json
    Format -->|JUnit| Junit

    Text --> StdOut
    Json --> JsonFile
    Json -.->|Optional| StdOut
    Junit --> JunitFile

    style Reporter fill:#fce4ec
    style StdOut fill:#e8f5e9
    style JsonFile fill:#fff9c4
    style JunitFile fill:#e1f5ff
```

## Usage Examples

### Text Reporter

```rust
let reporter = TextReporter::new(verbose: true, color: true);
reporter.report_start(&suite);

for result in results {
    reporter.report_test(&result);
}

reporter.report_end(&stats);
```

### JSON Reporter

```rust
let reporter = JsonReporter::new(Some(PathBuf::from("results.json")));
reporter.report_start(&suite);

for result in results {
    reporter.report_test(&result);
}

reporter.report_end(&stats);
```

### JUnit Reporter

```rust
let reporter = JunitReporter::new(PathBuf::from("junit.xml"));
reporter.report_start(&suite);

for result in results {
    reporter.report_test(&result);
}

reporter.report_end(&stats);
```

## CI/CD Integration

### CI/CD Output Flow

```mermaid
sequenceDiagram
    participant CI as CI Pipeline
    participant UITest as ui-test-rs
    participant Reporter
    participant Artifact as CI Artifacts

    CI->>UITest: run with --format junit
    UITest->>Reporter: generate JUnit XML
    Reporter->>Reporter: format results
    Reporter->>Artifact: save junit.xml

    alt Tests Passed
        Reporter-->>CI: exit code 0
        CI->>CI: mark build success
    else Tests Failed
        Reporter-->>CI: exit code 1
        CI->>CI: mark build failed
        CI->>CI: display test report
    end
```

## Related Documentation

- [Test Runner](Test-Runner) - Test execution
- [CLI Interface](CLI-Interface) - Command-line interface
- [Architecture](Architecture) - System architecture
- [Configuration](Configuration) - Configuration system

---

**Last Updated:** 2025-11-18

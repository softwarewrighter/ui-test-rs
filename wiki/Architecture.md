# Architecture

This page describes the system architecture of ui-test-rs, including component relationships, data flows, and design decisions.

## System Overview

ui-test-rs follows a layered architecture with five main components that work together to provide UI testing capabilities through Playwright MCP integration.

### High-Level Architecture

```mermaid
flowchart TB
    subgraph UserInterface["User Interface Layer"]
        CLI[CLI Interface<br/>Argument Parsing<br/>Help/Version]
    end

    subgraph Orchestration["Orchestration Layer"]
        Runner[Test Runner<br/>Lifecycle Management<br/>Result Collection]
    end

    subgraph Services["Service Layer"]
        Loader[Test Loader<br/>Discovery<br/>Filtering]
        PW[Playwright Client<br/>MCP Integration<br/>Browser Control]
        Reporter[Reporter<br/>Formatting<br/>Output]
    end

    subgraph External["External Systems"]
        MCPServer[Playwright MCP<br/>Server]
        Browser[Browser<br/>Chromium/Firefox/WebKit]
        TestFiles[(Test Files<br/>*_test.rs)]
        Output[(Results<br/>Text/JSON/JUnit)]
    end

    CLI --> Runner
    Runner --> Loader
    Runner --> PW
    Runner --> Reporter

    Loader -.-> TestFiles
    PW <--> MCPServer
    MCPServer <--> Browser
    Reporter -.-> Output

    style UserInterface fill:#e1f5ff
    style Orchestration fill:#fff4e1
    style Services fill:#e8f5e9
    style External fill:#f3e5f5
```

## Component Architecture

```mermaid
graph TB
    subgraph CLI["CLI Interface (main.rs)"]
        ArgParser[Argument Parser<br/>clap derive]
        Config[Config Resolver<br/>CLI + Env + File]
    end

    subgraph Runner["Test Runner (runner.rs)"]
        Lifecycle[Lifecycle Manager]
        Executor[Test Executor]
        Collector[Result Collector]
    end

    subgraph Loader["Test Loader (loader.rs)"]
        Discovery[File Discovery<br/>Glob Patterns]
        Parser[Test Parser]
        Filter[Test Filter]
    end

    subgraph PW["Playwright Client (playwright.rs)"]
        Connection[MCP Connection]
        Actions[Browser Actions]
        Elements[Element Selection<br/>Accessibility Tree]
    end

    subgraph Reporter["Reporter (reporter.rs)"]
        TextRep[Text Reporter]
        JsonRep[JSON Reporter]
        JunitRep[JUnit Reporter]
        Stats[Statistics]
    end

    ArgParser --> Config
    Config --> Lifecycle
    Lifecycle --> Discovery
    Discovery --> Parser
    Parser --> Filter
    Filter --> Executor
    Executor --> Connection
    Connection --> Actions
    Actions --> Elements
    Executor --> Collector
    Collector --> Stats
    Stats --> TextRep
    Stats --> JsonRep
    Stats --> JunitRep

    style CLI fill:#e1f5ff
    style Runner fill:#fff4e1
    style Loader fill:#e8f5e9
    style PW fill:#f3e5f5
    style Reporter fill:#fce4ec
```

## Data Flow Architecture

### Configuration Flow

```mermaid
flowchart LR
    Defaults[Default<br/>Values]
    File[Config File<br/>ui-test.toml]
    Env[Environment<br/>Variables]
    CLI[CLI Flags]
    Final[Final<br/>Configuration]

    Defaults --> File
    File --> Env
    Env --> CLI
    CLI --> Final

    style Defaults fill:#f5f5f5
    style File fill:#e3f2fd
    style Env fill:#e8f5e9
    style CLI fill:#fff9c4
    style Final fill:#c8e6c9
```

### Test Execution Flow

```mermaid
flowchart TB
    Start([Start])
    ParseArgs[Parse CLI Arguments]
    LoadConfig[Load Configuration]
    Discover[Discover Tests]
    InitMCP[Initialize Playwright MCP]

    ForEach{For Each Test}
    Setup[Setup Test Environment]
    Execute[Execute Test Actions]
    Assert[Assert Expectations]
    Cleanup[Cleanup]
    Collect[Collect Result]

    AllDone{All Tests Done?}
    Report[Generate Report]
    Exit([Exit with Code])

    Start --> ParseArgs
    ParseArgs --> LoadConfig
    LoadConfig --> Discover
    Discover --> InitMCP
    InitMCP --> ForEach

    ForEach -->|Next Test| Setup
    Setup --> Execute
    Execute --> Assert
    Assert --> Cleanup
    Cleanup --> Collect
    Collect --> AllDone

    AllDone -->|More Tests| ForEach
    AllDone -->|Done| Report
    Report --> Exit

    style Start fill:#c8e6c9
    style Exit fill:#ffcdd2
    style Execute fill:#fff9c4
    style Report fill:#e1bee7
```

## Component Relationships

### Dependency Graph

```mermaid
graph LR
    Main[main.rs]
    Config[config.rs]
    Runner[runner.rs]
    Loader[loader.rs]
    PW[playwright.rs]
    Reporter[reporter.rs]
    Errors[errors.rs]

    Main --> Config
    Main --> Runner
    Runner --> Config
    Runner --> Loader
    Runner --> PW
    Runner --> Reporter
    Loader --> Config
    Loader --> Errors
    PW --> Config
    PW --> Errors
    Reporter --> Config

    style Main fill:#e1f5ff
    style Runner fill:#fff4e1
    style Config fill:#e8f5e9
    style Errors fill:#ffcdd2
```

## Design Principles

### Layered Architecture Benefits

1. **Separation of Concerns**
   - Each layer has a specific responsibility
   - Changes in one layer minimally impact others
   - Easy to test components in isolation

2. **Dependency Flow**
   - Dependencies flow downward (user interface -> services -> external)
   - No circular dependencies
   - Clean interfaces between layers

3. **Extensibility**
   - New reporters can be added without changing core logic
   - Browser actions can be extended independently
   - Test discovery patterns can evolve separately

### Component Isolation

```mermaid
flowchart TB
    subgraph Isolated["Isolated Components"]
        CLI2[CLI Interface<br/>✓ No external deps]
        Loader2[Test Loader<br/>✓ Filesystem only]
        Reporter2[Reporter<br/>✓ Pure formatting]
    end

    subgraph Integrated["Integrated Components"]
        Runner2[Test Runner<br/>⊕ Orchestrates all]
        PW2[Playwright Client<br/>⊕ External MCP]
    end

    Runner2 -.-> CLI2
    Runner2 -.-> Loader2
    Runner2 -.-> Reporter2
    Runner2 --> PW2

    style Isolated fill:#e8f5e9
    style Integrated fill:#fff9c4
```

## Error Handling Architecture

### Error Propagation

```mermaid
flowchart TB
    ConfigErr[Configuration Error]
    DiscoveryErr[Discovery Error]
    MCPErr[MCP Connection Error]
    TestErr[Test Execution Error]

    Handler[Error Handler]

    Recover{Recoverable?}
    Log[Log Warning]
    Continue[Continue Execution]
    Fail[Fail with Exit Code 2]

    ConfigErr --> Handler
    DiscoveryErr --> Handler
    MCPErr --> Handler
    TestErr --> Handler

    Handler --> Recover
    Recover -->|Yes| Log
    Log --> Continue
    Recover -->|No| Fail

    style ConfigErr fill:#ffcdd2
    style DiscoveryErr fill:#ffcdd2
    style MCPErr fill:#ffcdd2
    style TestErr fill:#ffcdd2
    style Fail fill:#b71c1c,color:#fff
```

### Error Types Hierarchy

```mermaid
classDiagram
    class UiTestError {
        <<enumeration>>
        +Config(String)
        +Discovery(String)
        +PlaywrightConnection(String)
        +BrowserAction(String)
        +Assertion(String)
        +Timeout(Duration)
        +Io(IoError)
    }

    class Result~T~ {
        Ok(T)
        Err(UiTestError)
    }

    UiTestError --> Result
```

## Performance Architecture

### Parallel Execution Model

```mermaid
flowchart TB
    Queue[Test Queue]

    subgraph Workers["Worker Pool (Configurable Size)"]
        W1[Worker 1<br/>Browser Instance]
        W2[Worker 2<br/>Browser Instance]
        W3[Worker N<br/>Browser Instance]
    end

    Collector[Result Collector<br/>Thread-Safe]
    Reporter[Reporter]

    Queue --> W1
    Queue --> W2
    Queue --> W3

    W1 --> Collector
    W2 --> Collector
    W3 --> Collector

    Collector --> Reporter

    style Queue fill:#e1f5ff
    style Workers fill:#fff4e1
    style Collector fill:#e8f5e9
    style Reporter fill:#fce4ec
```

### Resource Management

```mermaid
sequenceDiagram
    participant Runner
    participant Worker
    participant Browser
    participant MCP

    Runner->>Worker: Initialize (Semaphore)
    Worker->>MCP: Connect
    MCP->>Browser: Launch

    loop For Each Test
        Runner->>Worker: Execute Test
        Worker->>Browser: Actions
        Browser-->>Worker: Results
        Worker-->>Runner: Test Result
    end

    Worker->>Browser: Cleanup
    Worker->>MCP: Disconnect
    Worker-->>Runner: Release Semaphore
```

## Security Architecture

### Sandboxing Model

```mermaid
flowchart TB
    subgraph Trusted["Trusted Zone"]
        CLI3[CLI Interface]
        Config3[Config Parser]
    end

    subgraph Limited["Limited Trust Zone"]
        Loader3[Test Loader<br/>Path Validation]
        TestFiles3[Test Files<br/>Resource Limits]
    end

    subgraph Sandboxed["Sandboxed Zone"]
        Browser3[Browser<br/>Isolated Process]
        MCP3[MCP Server<br/>Subprocess]
    end

    CLI3 --> Config3
    Config3 --> Loader3
    Loader3 --> TestFiles3
    TestFiles3 -.-> Browser3
    Browser3 <--> MCP3

    style Trusted fill:#c8e6c9
    style Limited fill:#fff9c4
    style Sandboxed fill:#ffecb3
```

## Technology Stack

```mermaid
mindmap
  root((ui-test-rs))
    Language
      Rust 2021
      Edition 2024 Idioms
    CLI Framework
      clap v4.5
      Derive Macros
    Async Runtime
      tokio v1.0
      Full Features
    Serialization
      serde v1.0
      toml v0.8
    Error Handling
      anyhow v1.0
      thiserror v1.0
    Testing
      tempfile v3.0
      assert_cmd v2.0
      predicates v3.0
    Browser Automation
      Playwright MCP
      JSON-RPC
```

## Deployment Architecture

```mermaid
flowchart TB
    subgraph Build["Build Artifacts"]
        Binary[ui-test-rs Binary]
        Metadata[Build Metadata<br/>Host/Commit/Timestamp]
    end

    subgraph Distribution["Distribution Methods"]
        CargoInstall[cargo install]
        Source[Build from Source]
        Releases[GitHub Releases]
    end

    subgraph Runtime["Runtime Requirements"]
        Rust[Rust Toolchain<br/>Build Only]
        Node[Node.js v20+<br/>Playwright MCP]
        PW_MCP[Playwright MCP<br/>@playwright/mcp]
    end

    Binary --> CargoInstall
    Binary --> Source
    Binary --> Releases

    CargoInstall -.-> Node
    Source -.-> Rust
    Source -.-> Node
    Releases -.-> Node

    Node --> PW_MCP

    style Build fill:#e1f5ff
    style Distribution fill:#e8f5e9
    style Runtime fill:#fff9c4
```

## Future Architecture Enhancements

### Plugin System (Planned)

```mermaid
flowchart TB
    Core[Core System]

    subgraph Plugins["Plugin System"]
        Interface[Plugin Interface<br/>Trait]
        Loader4[Plugin Loader<br/>Dynamic]
        Registry[Plugin Registry]
    end

    subgraph PluginTypes["Plugin Types"]
        Reporter3[Custom Reporters]
        Actions2[Custom Actions]
        Assertions[Custom Assertions]
    end

    Core --> Interface
    Interface --> Loader4
    Loader4 --> Registry

    Registry -.-> Reporter3
    Registry -.-> Actions2
    Registry -.-> Assertions

    style Core fill:#e1f5ff
    style Plugins fill:#fff4e1
    style PluginTypes fill:#e8f5e9
```

## Related Documentation

- [Sequences](Sequences) - Detailed flow diagrams and sequence diagrams
- [Configuration](Configuration) - Configuration system details
- [CLI Interface](CLI-Interface) - CLI component details
- [Test Runner](Test-Runner) - Runner component details
- [Playwright Client](Playwright-Client) - Browser automation details

---

**Last Updated:** 2025-11-18

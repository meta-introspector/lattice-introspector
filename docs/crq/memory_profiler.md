## Proposed Plan: Introspector Profiler Macros

### 1. New Crate for Profiling Macros

Create a new, dedicated crate named `introspector_profiler_macros`. This will ensure separation of concerns and avoid direct modification of vendorized code.

### 2. Macro Design

*   `#[profile_fn]`: An attribute macro for functions.
*   `#[profile_type]`: An attribute macro for structs and enums (likely wrapping `impl` blocks or specific methods).
*   `#[profile_block]`: An attribute macro for arbitrary code blocks. This will require careful parsing and injection.

### 3. Profiling Data Collection

*   **Time:** Capture timestamps at the entry and exit points of the profiled code.
*   **Memory:** Leverage the `ragit-memory-monitor` crate to take memory snapshots before and after profiled code execution. This will require understanding and integrating its API.
*   **CPU:** Initially, focus on basic time measurements. Explore more advanced CPU profiling later, potentially using external crates or system calls, as this is often OS-specific.

### 4. Data Reporting

Implement a mechanism to collect and report profiling data. Options include:
*   Logging the results.
*   A global collector to aggregate data, which can be queried or dumped at the end of execution.

This project will involve significant Rust procedural macro development.
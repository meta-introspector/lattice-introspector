# Change Request (CRQ) - rust-smallvec Vendorization

*   **CRQ ID:** TBD-025-RUST-SMALLVEC-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `rust-smallvec` repository as a Git submodule. This repository likely provides a `SmallVec` implementation in Rust, which is a vector that can store a small number of elements directly on the stack, avoiding heap allocation for small collections. The repository, specifically `https://github.com/meta-introspector/rust-smallvec`, will be integrated into this project under the `vendor/rust-smallvec` path, tracking the `main` branch. This allows the project to directly utilize its functionalities for performance optimization.
*   **Reason for Change (Compliance Link):**
    To incorporate a `SmallVec` implementation as a direct dependency, enabling performance optimizations by reducing heap allocations for small collections and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `rust-smallvec`.
    *   **Users:** Developers working with `rust-smallvec` components or needing performance optimization capabilities.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `rust-smallvec` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/rust-smallvec` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `rust-smallvec`.
    *   Thorough testing of functionalities relying on `rust-smallvec`.
*   **Rollback Plan:**
    *   Remove the `vendor/rust-smallvec` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/rust-smallvec` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/rust-smallvec` submodule initializes correctly.
    *   Build the project and run tests to ensure `rust-smallvec` integration is successful.

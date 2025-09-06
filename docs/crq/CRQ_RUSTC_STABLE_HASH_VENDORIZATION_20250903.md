# Change Request (CRQ) - rustc-stable-hash Vendorization

*   **CRQ ID:** TBD-024-RUSTC-STABLE-HASH-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `rustc-stable-hash` repository as a Git submodule. This repository likely provides a stable hashing mechanism for Rust compiler artifacts or related data. The repository, specifically `https://github.com/meta-introspector/rustc-stable-hash`, will be integrated into this project under the `vendor/rustc-stable-hash` path, tracking the `main` branch. This allows the project to directly utilize its functionalities.
*   **Reason for Change (Compliance Link):**
    To incorporate a stable hashing mechanism for Rust compiler artifacts as a direct dependency, enabling its functionalities and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `rustc-stable-hash`.
    *   **Users:** Developers working with `rustc-stable-hash` components or needing stable hashing capabilities.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `rustc-stable-hash` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/rustc-stable-hash` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `rustc-stable-hash`.
    *   Thorough testing of functionalities relying on `rustc-stable-hash`.
*   **Rollback Plan:**
    *   Remove the `vendor/rustc-stable-hash` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/rustc-stable-hash` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/rustc-stable-hash` submodule initializes correctly.
    *   Build the project and run tests to ensure `rustc-stable-hash` integration is successful.

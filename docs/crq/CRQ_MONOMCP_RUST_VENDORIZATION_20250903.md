# Change Request (CRQ) - monomcp-rust Vendorization

*   **CRQ ID:** TBD-022-MONOMCP-RUST-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `monomcp-rust` repository as a Git submodule. This repository likely provides a monolithic multi-core processor (MCP) component implemented in Rust. The repository, specifically `https://github.com/meta-introspector/monomcp-rust`, will be integrated into this project under the `vendor/monomcp-rust` path, tracking the `main` branch. This allows the project to directly utilize its functionalities.
*   **Reason for Change (Compliance Link):**
    To incorporate a monolithic MCP component as a direct dependency, enabling its functionalities and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `monomcp-rust`.
    *   **Users:** Developers working with `monomcp-rust` components or needing monolithic MCP capabilities.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `monomcp-rust` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/monomcp-rust` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `monomcp-rust`.
    *   Thorough testing of functionalities relying on `monomcp-rust`.
*   **Rollback Plan:**
    *   Remove the `vendor/monomcp-rust` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/monomcp-rust` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/monomcp-rust` submodule initializes correctly.
    *   Build the project and run tests to ensure `monomcp-rust` integration is successful.

# Change Request (CRQ) - copper Vendorization

*   **CRQ ID:** TBD-016-COPPER-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `copper` repository as a Git submodule. `Copper` is a constraint programming solver written in Rust, designed for combinatorial problems with complex, non-linear constraints. The repository, specifically `https://github.com/meta-introspector/copper`, will be integrated into this project under the `vendor/copper` path, tracking the `main` branch. This allows the project to directly utilize `Copper` for solving constraint-based problems.
*   **Reason for Change (Compliance Link):**
    To incorporate a constraint programming solver as a direct dependency, enabling the use of its functionalities for solving complex combinatorial problems and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `copper`.
    *   **Users:** Developers working with `copper` components or needing constraint programming capabilities.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `copper` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/copper` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `copper`.
    *   Thorough testing of functionalities relying on `copper`.
*   **Rollback Plan:**
    *   Remove the `vendor/copper` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/copper` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/copper` submodule initializes correctly.
    *   Build the project and run tests to ensure `copper` integration is successful.

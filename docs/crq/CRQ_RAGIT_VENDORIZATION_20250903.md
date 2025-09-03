# Change Request (CRQ) - ragit Vendorization

*   **CRQ ID:** TBD-012-RAGIT-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `ragit` repository as a Git submodule. The `ragit` repository, specifically `https://github.com/meta-introspector/ragit.git`, will be integrated into this project under the `vendor/ragit` path, tracking the `main` branch. This allows the project to directly utilize components from `ragit`.
*   **Reason for Change (Compliance Link):**
    To incorporate the `ragit` library as a direct dependency, enabling the use of its functionalities and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `ragit`.
    *   **Users:** Developers working with `ragit` components.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `ragit` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/ragit` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `ragit`.
    *   Thorough testing of functionalities relying on `ragit`.
*   **Rollback Plan:**
    *   Remove the `vendor/ragit` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/ragit` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/ragit` submodule initializes correctly.
    *   Build the project and run tests to ensure `ragit` integration is successful.

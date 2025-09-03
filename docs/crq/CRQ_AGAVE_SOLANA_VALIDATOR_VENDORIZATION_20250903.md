# Change Request (CRQ) - agave-solana-validator Vendorization

*   **CRQ ID:** TBD-032-AGAVE-SOLANA-VALIDATOR-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `agave-solana-validator` repository as a Git submodule. This repository likely provides components for a Solana validator, possibly related to the "Agave" project. The repository, specifically `https://github.com/meta-introspector/agave-solana-validator`, will be integrated into this project under the `vendor/agave-solana-validator` path, tracking the `main` branch. This allows the project to directly utilize its functionalities for Solana blockchain validation.
*   **Reason for Change (Compliance Link):**
    To incorporate Solana validator components as a direct dependency, enabling its functionalities and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `agave-solana-validator`.
    *   **Users:** Developers working with `agave-solana-validator` components or needing Solana validation capabilities.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `agave-solana-validator` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/agave-solana-validator` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `agave-solana-validator`.
    *   Thorough testing of functionalities relying on `agave-solana-validator`.
*   **Rollback Plan:**
    *   Remove the `vendor/agave-solana-validator` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/agave-solana-validator` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/agave-solana-validator` submodule initializes correctly.
    *   Build the project and run tests to ensure `agave-solana-validator` integration is successful.

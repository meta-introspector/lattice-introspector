# Change Request (CRQ) - hugging-face-dataset-validator-rust Vendorization

*   **CRQ ID:** TBD-018-HUGGING-FACE-DATASET-VALIDATOR-RUST-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `hugging-face-dataset-validator-rust` repository as a Git submodule. This repository likely provides Rust-based tools for validating datasets on Hugging Face. The repository, specifically `https://github.com/meta-introspector/hugging-face-dataset-validator-rust`, will be integrated into this project under the `vendor/hugging-face-dataset-validator-rust` path, tracking the `main` branch. This allows the project to directly utilize its functionalities.
*   **Reason for Change (Compliance Link):**
    To incorporate a Hugging Face dataset validation tool as a direct dependency, enabling its functionalities and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `hugging-face-dataset-validator-rust`.
    *   **Users:** Developers working with `hugging-face-dataset-validator-rust` components or needing dataset validation capabilities.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `hugging-face-dataset-validator-rust` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/hugging-face-dataset-validator-rust` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `hugging-face-dataset-validator-rust`.
    *   Thorough testing of functionalities relying on `hugging-face-dataset-validator-rust`.
*   **Rollback Plan:**
    *   Remove the `vendor/hugging-face-dataset-validator-rust` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/hugging-face-dataset-validator-rust` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/hugging-face-dataset-validator-rust` submodule initializes correctly.
    *   Build the project and run tests to ensure `hugging-face-dataset-validator-rust` integration is successful.

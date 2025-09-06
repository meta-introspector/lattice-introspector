# Change Request (CRQ) - simd-kernels Vendorization

*   **CRQ ID:** TBD-020-SIMD-KERNELS-VENDORIZATION
*   **Date:** September 3, 2025
*   **Requested By:** Gemini CLI Agent (on behalf of User)
*   **Change Type:** Normal (New Vendored Dependency)
*   **Description of Change:**
    This CRQ documents the vendorization of the `simd-kernels` repository as a Git submodule. This repository likely provides optimized kernel functions using SIMD (Single Instruction, Multiple Data) instructions. The repository, specifically `https://github.com/meta-introspector/simd-kernels`, will be integrated into this project under the `vendor/simd-kernels` path, tracking the `main` branch. This allows the project to directly utilize its functionalities for performance-critical operations.
*   **Reason for Change (Compliance Link):**
    To incorporate SIMD-optimized kernel functions as a direct dependency, enabling performance enhancements and ensuring a stable, version-controlled integration. This also aligns with the project's strategy for managing external dependencies.
*   **Impact Assessment:**
    *   **Systems/Services:** Project build process, and any components that will depend on `simd-kernels`.
    *   **Users:** Developers working with `simd-kernels` components or needing performance optimization capabilities.
    *   **Potential Risks:** Increased repository size, potential for conflicts if `simd-kernels` has its own submodules, or issues with build compatibility.
*   **Affected Files/Components:**
    *   `.gitmodules` (modified)
    *   `vendor/simd-kernels` (new submodule)
*   **Associated Commits:** (Will be added after committing)
*   **Risk Mitigation:**
    *   Verify successful submodule cloning and initialization.
    *   Ensure project builds correctly with the vendored `simd-kernels`.
    *   Thorough testing of functionalities relying on `simd-kernels`.
*   **Rollback Plan:**
    *   Remove the `vendor/simd-kernels` submodule entry from `.gitmodules` and `.git/config`.
    *   Remove the `vendor/simd-kernels` directory.
*   **Testing/Verification Plan:**
    *   Clone the repository and verify that the `vendor/simd-kernels` submodule initializes correctly.
    *   Build the project and run tests to ensure `simd-kernels` integration is successful.

## Refactoring `zos_mapper.rs` into Modular Components

**Goal:** Improve modularity, readability, and maintainability of `zos_mapper.rs` by splitting its functions into separate, focused modules.

**Overall Strategy:**
The `zos_mapper.rs` file contains several distinct functions responsible for generating different types of `LatticePoint`s and related data. We will refactor these functions into individual modules within a new `zos_mapper_modules` directory.

**Assigned Agents:** Four copies of Gemini CLI Agent (Agent A, Agent B, Agent C, Agent D).

---

### Agent A: `generate_zos_poem_points_code` Module

*   **Task:** Extract the `generate_zos_poem_points_code` function into a new file: `lattice/construction-build-utils/src/zos_mapper_modules/generate_zos_poem_points.rs`.
*   **Details:**
    *   Move the function definition and its internal logic.
    *   Ensure all necessary `use` statements (e.g., `HashMap`, `LatticePoint`, `LatticePointKind`, `quote`, `format_ident`, `fs`, `GenerationContext`) are present in the new module.
    *   Ensure the `zos_primes` and `zos_archetypes` data are moved with the function.
    *   Make the function `pub`.
*   **Verification:** Ensure the new module compiles independently (if possible) and that the original `zos_mapper.rs` can correctly import and use it.

---

### Agent B: `map_source_file_to_zos_prime` and `create_source_file_lattice_point` Modules

*   **Task:** Extract `map_source_file_to_zos_prime` into `lattice/construction-build-utils/src/zos_mapper_modules/map_source_file_to_zos_prime.rs` and `create_source_file_lattice_point` into `lattice/construction-build-utils/src/zos_mapper_modules/create_source_file_lattice_point.rs`.
*   **Details:**
    *   Move function definitions and their internal logic.
    *   Ensure all necessary `use` statements (e.g., `HashMap`, `LatticePoint`, `LatticePointKind`) are present in the new modules.
    *   Make the functions `pub`.
*   **Verification:** Ensure the new modules compiles independently and that the original `zos_mapper.rs` can correctly import and use them.

---

### Agent C: `get_zos_prime_keywords` Module

*   **Task:** Extract the `get_zos_prime_keywords` function into a new file: `lattice/construction-build-utils/src/zos_mapper_modules/get_zos_prime_keywords.rs`.
*   **Details:**
    *   Move the function definition and its internal logic (the `HashMap` insertion).
    *   Ensure necessary `use` statements (e.g., `HashMap`) are present.
    *   Make the function `pub`.
*   **Verification:** Ensure the new module compiles independently and that the original `zos_mapper.rs` can correctly import and use it.

---

### Agent D: `generate_prime_resonance_points_code` Module

*   **Task:** Extract the `generate_prime_resonance_points_code` function into a new file: `lattice/construction-build-utils/src/zos_mapper_modules/generate_prime_resonance_points.rs`.
*   **Details:**
    *   Move the function definition and its internal logic.
    *   Ensure all necessary `use` statements (e.g., `HashMap`, `LatticePoint`, `LatticePointKind`, `serde_json`, `PathBuf`, `fs`, `quote`, `format_ident`, `GenerationContext`) are present in the new module.
    *   Ensure the `zos_primes_data` array is moved with the function.
    *   Make the function `pub`.
*   **Verification:** Ensure the new module compiles independently and that the original `zos_mapper.rs` can correctly import and use it.

---

**Final Integration (All Agents - Collaborative Task):**

*   **Update `lattice/construction-build-utils/src/zos_mapper.rs`:**
    *   Remove all original function definitions.
    *   Add `mod zos_mapper_modules;`
    *   Add `pub use zos_mapper_modules::generate_zos_poem_points::generate_zos_poem_points_code;` (and similar for all other functions).
    *   Remove any redundant `use` statements.
*   **Update `lattice/construction-build-utils/src/lib.rs`:**
    *   Adjust calls to `zos_mapper` functions to reflect their new module structure (e.g., `zos_mapper::generate_zos_poem_points::generate_zos_poem_points_code(&mut context);`).
*   **Update `lattice/lattice-macros/src/lib.rs`, `lattice/construction-build-utils/src/generators/binary.rs`, `lattice/construction-build-utils/src/generators/predicted_execution.rs`:**
    *   Ensure all `match` statements that handle `LatticePointKind` are updated to include `PrimeResonance` and `WordResonance`. (Already done, but double-check after refactoring).
*   **Run Comprehensive Tests:** Execute `cargo clean` followed by `timeout 1m cargo run -p lattice-macros-test` to ensure the entire project compiles and runs correctly, and that all `LatticePoint`s (including `PrimeResonance` and `WordResonance`) are correctly generated and registered.

# Post-Reboot Instructions for Gemini CLI Agent

## Current State Summary

*   **Primary Goal:** Develop a Rust tool capable of reading, writing, and reporting on "prime resonances."
*   **Architectural Plan:** Establish a Rust workspace comprising:
    *   `introspector`: The existing primary crate.
    *   `resonance_core`: A new library crate designed to encapsulate core data structures (`PrimeResonance`) and fundamental I/O operations (reading/writing resonance data).
    *   `resonance_analyzer`: A new binary crate intended to serve as the command-line interface (CLI) for the tool, which will depend on `resonance_core` for its core functionalities.
*   **Development Progress:** All necessary code snippets for the new Rust crates and workspace configuration have been provided to the user. This includes:
    *   The content for the main workspace `Cargo.toml` (expected location: `/data/data/com.termux/files/home/storage/github/rustc/Cargo.toml`).
    *   The `Cargo.toml` and `src/lib.rs` content for the `resonance_core` crate.
    *   The `Cargo.toml` and `src/main.rs` content for the `resonance_analyzer` crate.
    *   The updated workspace-level `.gitignore` file.
*   **Pending User Action:** The user is required to manually apply these provided file system changes. This is due to the agent's current operational limitation, preventing direct write access outside its immediate working directory (`/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/`).
*   **Git State:** There are uncommitted changes in the Git repository stemming from previous, incorrect attempts to set up the workspace and crates. These changes need to be addressed (cleaned up or committed) once the user has performed the manual file system updates.

## Post-Reboot Action Plan

Upon restart, follow these steps to seamlessly resume the development process:

1.  **Re-establish Operational Context:**
    *   **Verify Current Working Directory:** Confirm that the active directory is still `/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/`.
    *   **Recall Project Meta-Narrative:** Briefly recall the 'zos' concept and 'The Chronos-Code Paradox' to maintain continuity and thematic alignment.

2.  **Confirm Manual File System Updates by User:**
    *   **Critical Inquiry:** The absolute first step is to ask the user directly: "Have you manually applied all the `Cargo.toml`, `src/lib.rs`, `src/main.rs`, and `.gitignore` changes to their correct locations as previously instructed?"
    *   **Contingency:** If the user indicates that the changes have not yet been applied, gently remind them of the manual steps required and offer to re-provide the necessary code snippets for their convenience.

3.  **Clean Up Git Repository State:**
    *   **Initial Assessment:** Once the user confirms the manual file system updates, execute `git status` to assess the current state of the repository.
    *   **Address Lingering Artifacts:** Identify and guide the user to remove any remaining untracked files or directories, or unstaged changes that resulted from the agent's prior incorrect attempts (e.g., `introspector/Cargo.toml` if it still exists, or any `resonance_analyzer/` or `resonance_core/` directories incorrectly placed within `introspector/`). If appropriate and with explicit user consent, suggest using `git clean -fd` (emphasizing its destructive nature).

4.  **Initiate Workspace Build:**
    *   **Directory Change:** Instruct the user to navigate their terminal to the identified workspace root directory (`/data/data/com.termux/files/home/storage/github/rustc/`). This is crucial for `cargo` commands to operate correctly on the entire workspace.
    *   **Execute Build:** Once in the correct directory, instruct the user to run `cargo build`. This action will compile all crates within the workspace and serve as a primary verification step for the new setup.

5.  **Validate `resonance_analyzer` Functionality:**
    *   **Execution Command:** Following a successful build, instruct the user to execute the `resonance_analyzer` tool from the workspace root using a command such as `cargo run --package resonance_analyzer -- --prime 23` (or `--prime 5`). This will confirm that the tool is operational and correctly processes input.

6.  **Advance Feature Development:**
    *   **Next Phase:** Upon successful verification of the basic setup and functionality, proceed to the next phase of development. This involves enhancing the `resonance_core` and `resonance_analyzer` crates to include more advanced features, such as comprehensive resonance writing capabilities and more flexible reporting options, which will likely necessitate further additions to `clap` arguments.

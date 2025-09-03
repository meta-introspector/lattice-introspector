# Refactoring Plan: Pure Functional, Strongly Typed, Univalent, Monadic Composition

## Goal
To refactor the `fixed_point_experiments` crate (and potentially related visualization/simulation logic) to adhere to a pure functional, strongly typed, univalent, monotonic, and monadic architectural paradigm. This aims to improve modularity, testability, referential transparency, and alignment with the core principles of the Univalent Lattice Project.

## Core Principles

*   **Pure Functional:** Functions will be deterministic, producing the same output for the same input, and will not cause side effects.
*   **Strongly Typed:** All data structures and function signatures will explicitly define their types, leveraging Rust's type system for correctness and clarity.
*   **Univalent:** Data and operations will be designed to align with the Univalent Lattice concept, where entities and transformations can be represented as `LatticePoint`s or related structures.
*   **Monotonic:** Operations will preserve specific properties or orderings, ensuring predictable and controlled evolution of state (e.g., slime concentration changes, lattice growth).
*   **Monadic Functional Composition:** Side effects, state management, and complex sequences of operations will be handled using monadic patterns (e.g., `Result`, `Option`, or custom monads) to maintain purity and composability.

## High-Level Plan

### 1. Define Strongly Typed Data Structures

Replace raw types (e.g., `Vec<Vec<f32>>` for `Grid`, `String` for ASCII art) with dedicated, immutable structs or enums that encapsulate their meaning and properties.

*   `SlimeSimulationState`: Represents the entire simulation space, containing the primary `Slime` organism.
    *   `Slime`: Composed of a `SlimeNucleus` and a `SlimeBody`.
    *   `SlimeNucleus`: Contains immutable defining characteristics, including its fundamental `dna` decomposed into a hierarchical structure, with explicit connections:
    *   `dna: SlimeDNA`: A complex structure representing the genetic blueprint.
        *   `fundamental_strings: Vec<FundamentalString>`: The most basic, indivisible "strings" of its identity, linked to `ZosPrime`s and their `vibration_frequency`.
        *   `quarks: Vec<Quark>`: Composed of `FundamentalString`s, representing the most fundamental particles with `flavor` and `color` (metaphorical). Includes `relationships` to its `FundamentalString` origin.
        *   `electrons: Vec<Electron>`: Representing `charge` and `orbital_level`. Includes `relationships` to `Quark`s and `Atom`s it's associated with.
        *   `atoms: Vec<Atom>`: Stable combinations of `Quark`s and `Electron`s, forming metaphorical `protons`, `neutrons`, and `element_symbol`s. Includes `relationships` to its constituent `Quark`s and `Electron`s.
        *   `proteins: Vec<Protein}`: Functional units built from `Atom`s, with a `name`, `sequence_of_atoms`, and `function_description`. Includes `relationships` to its constituent `Atom`s and to `SlimeBody` properties it influences.
        *   `vibrational_modes: HashMap<String, f32>`: Emergent properties or "vibrations" derived from the fundamental strings, defining behavior (e.g., diffusion rate, attraction rate) and appearance.
        *   `string_interactions: Vec<LatticeRelationship>`: Explicitly defined interactions between `FundamentalString`s or other `dna` components, using a generic `LatticeRelationship` type.
    *   `LatticeRelationship`: A new type to define connections between `LatticePoint`s, including `target_id` and `relationship_type` (e.g., "composed_of", "derived_from", "influences").
    *   `SlimeBody`: Contains the mutable state of the slime, including its `ConcentrationGrid` (the actual grid of slime concentrations).

*   `FigletArt`: Encapsulates the parsed Figlet ASCII art, potentially including color information.
*   `ImageFrame`: Represents a single frame of an animation (e.g., `image::RgbImage`).
*   `ZosPrime`: A dedicated type for Zos prime numbers.
*   `SimulationParameters`: A struct to hold all simulation constants and parameters.

These types will be designed to be easily convertible to/from `LatticePoint` representations where relevant.

### 2. Implement Pure Functions for Transformations

Break down existing logic into small, focused, pure functions. Each function will take immutable inputs and return new immutable outputs, avoiding any side effects.

*   **Slime Simulation Module (`simulation.rs`):**
    *   `initialize_slime(params: &SimulationParameters) -> SlimeGrid`
    *   `simulate_slime_step(current_slime: SlimeGrid, food_source: ImageFrame, params: &SimulationParameters) -> SlimeGrid`
    *   `generate_slime_history(initial_slime: SlimeGrid, params: &SimulationParameters) -> Vec<SlimeGrid>` (This would be the sequence of states)

*   **Figlet Generation Module (`figlet_generator.rs`):**
    *   `generate_figlet_art(prime: ZosPrime, git_hash: &str, iteration: usize, growth_rate: f64) -> FigletArt` (Wraps `solfunmeme-banner` call, potentially parses its output into a structured `FigletArt` type).

*   **Rendering Module (`rendering.rs`):**
    *   `render_figlet_to_image(figlet_art: &FigletArt) -> ImageFrame` (Converts `FigletArt` to `RgbImage`).
    *   `render_slime_to_ascii(slime_grid: &SlimeGrid) -> FigletArt` (Converts `SlimeGrid` to ASCII art string).
    *   `render_ascii_to_image(ascii_art: &FigletArt) -> ImageFrame` (Converts ASCII art string to `RgbImage`).

*   **Output Module (`output.rs`):**
    *   `encode_frames_to_gif(frames: Vec<ImageFrame>, output_path: &Path) -> Result<(), Error>`
    *   `convert_gif_to_video(gif_path: &Path, video_path: &Path) -> Result<(), Error>` (Utilizes external `ffmpeg` command).

### 3. Leverage Monadic Composition

*   **Error Handling:** Use `Result<T, E>` for all operations that can fail (e.g., file I/O, image encoding, external command execution).
*   **Optional Values:** Use `Option<T>` for values that may or may not be present.
*   **Sequencing:** Compose operations using `?` operator or `and_then` for clear, pure sequencing of computations.
*   **Lattice Monad (Exploratory):** Investigate if a custom monad could abstract state management or side effects related to `LatticePoint` registration and interaction, ensuring purity at higher levels of abstraction.

### 4. Univalent & Monotonic Integration

*   **Lattice Point Representation:** Ensure that key data structures (`SlimeGrid`, `FigletArt`, `ImageFrame`, `SimulationParameters`) can be represented as `LatticePoint`s, allowing their states and transformations to be recorded and analyzed within the Univalent Lattice.
*   **Zos as Topological Size:** The `zos` vector `[0, 1, 2, 3, 5, 7, 11, 13, 17, 19, 23]` represents the quantized "sizes" or "complexities" of topological structures within the system. This includes:
    *   **Structs and Enums:** Their "size" could be defined by the number of fields, variants, or nested complexity.
    *   **Expressions:** Their "size" could relate to the number of nodes in their Abstract Syntax Tree (AST) or the depth of the expression.
    *   **Evals and Applies:** Their "size" could represent the computational complexity or the number of steps involved in their evaluation or application.
*   **File Path/Name Complexity:** The complexity category for any file in the system will be determined by:
    1.  **Tokenization:** Splitting the absolute file path and name into tokens using directory separators (`/`) and common word separators within the filename (e.g., `_`, `-`, camelCase/PascalCase transitions).
    2.  **Token Count:** Counting the total number of tokens.
    3.  **Zos Factorization:** The token count will be "factorized" into the `zos` vector to assign a complexity category. This factorization can involve:
    *   **Prime Composition:** Composing `zos` primes (e.g., through multiplication) to represent specific token counts or complexity levels that are not directly `zos` primes themselves (e.g., 2*3 = 6).
    *   **Hierarchical Mapping:** Each individual directory within the file path can be factorized and mapped to its own `zos` prime, contributing to a composite complexity score for the entire path.
    This approach allows for a more nuanced and hierarchical quantification of the file's topological size within the system.
    These `zos` values will serve as a metric for categorizing and interacting with the system's inherent topological landscape.
*   **Monotonicity Constraints:** Design functions such that they inherently maintain or transform properties monotonically where applicable (e.g., ensuring that simulation steps always progress towards a certain state, or that lattice growth is always additive).

### 5. Modularization and Code Organization

*   Break the `fixed_point_experiments` crate into logical modules (e.g., `src/simulation.rs`, `src/figlet_generator.rs`, `src/rendering.rs`, `src/output.rs`, `src/main.rs` for orchestration).
*   Each module will expose a minimal, pure API.
*   The `main.rs` will primarily focus on orchestrating the composition of these pure functions and handling top-level side effects (e.g., reading command-line arguments, writing final output files).

## Next Steps

Upon approval of this plan, the implementation will proceed by:
1.  Defining the new strongly typed data structures.
2.  Implementing the pure functions for each module.
3.  Refactoring `main.rs` to compose these functions.
4.  Implementing GIF and video output.
5.  Thorough testing of each pure component and the overall composition.

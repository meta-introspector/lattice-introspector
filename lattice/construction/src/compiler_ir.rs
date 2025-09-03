//! Data structures for modeling the compiler's internal representation of code elements.

use lattice_macros::LatticePointDerive; // Add this line

/// Represents a conceptual memory location within the rustc compiler's process.
/// This location is associated with a specific source code element (e.g., a struct or enum).
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct CompilerMemoryLocation {
    pub address_id: String, // A unique identifier for this conceptual memory location
    pub rustc_invocation_id: String, // The ID of the rustc invocation that produced this state
    pub rustc_version: String, // The version of rustc used
    pub build_environment_hash: String, // A hash representing the build environment (for reproducibility)
    // Add more fields to pinpoint the exact location within the compiler's internal data structures
}

/// Represents a source code element (like a struct or enum) as it exists within the compiler's memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct CompilerInternalRepresentation {
    pub element_name: String, // The name of the struct or enum
    pub source_file_path: String, // Path to the source file where it's defined
    pub line_number: u32, // Line number in the source file
    pub memory_location: CompilerMemoryLocation, // The conceptual memory location within the compiler
    // Add more fields to represent the parsed AST/HIR/MIR of the element
}
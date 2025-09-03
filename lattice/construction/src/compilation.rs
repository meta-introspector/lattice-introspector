//! Data structures for modeling rustc compilation invocations.

use lattice_macros::LatticePointDerive; // Add this line

/// Represents a single invocation of the rustc compiler.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct RustcInvocation {
    pub invocation_id: String, // Unique ID for this compilation
    pub input_crates: Vec<String>, // Names of input crates
    pub output_artifacts: Vec<String>, // Paths to generated artifacts (e.g., .rlib, .so, executable)
    pub compiler_flags: Vec<String>, // Command-line flags used
    pub timestamp: u64, // Unix timestamp of the compilation
    // Add more details like rustc version, target triple, etc.
}
use lattice_macros::LatticePointDerive;

/// Represents a Rust source file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct RustFile {
    pub path: String, // Absolute path to the file
    pub content_hash: String, // A hash of the file's content for quick comparison
    // Add more fields like AST representation, function definitions, etc.
}

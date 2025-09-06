use lattice_macros::LatticePointDerive;

/// Represents an FFI (Foreign Function Interface) binding.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct FfiBinding {
    pub name: String, // Name of the FFI function/symbol
    pub signature: String, // The function signature (e.g., "fn(i32) -> *mut c_char")
    pub target_language: String, // The language it binds to (e.g., "C", "Python")
    pub source_file: String, // Path to the Rust file where it's declared
    // Add more fields like linked library, etc.
}

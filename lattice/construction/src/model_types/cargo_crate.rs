use lattice_macros::LatticePointDerive;

/// Represents a Cargo crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct CargoCrate {
    pub name: String,
    pub version: String,
    pub path: String, // Absolute path to the crate directory
    // Add more fields like dependencies, features, etc.
}

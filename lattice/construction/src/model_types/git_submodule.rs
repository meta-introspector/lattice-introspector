use lattice_macros::LatticePointDerive;

/// Represents a Git submodule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct GitSubmodule {
    pub name: String,
    pub path: String, // Path relative to the parent repository
    pub url: String,
    pub commit_id: String, // The specific commit the submodule is pinned to
    // Add a reference to the parent repository if needed, but for now, assume context
}

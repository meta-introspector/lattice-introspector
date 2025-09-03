use lattice_macros::LatticePointDerive;

/// Represents an asset that is derived from a Git repository.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GitDerivedAsset {
    pub git_repo_url: String,
    pub derivation_method: String, // e.g., "cloned", "sampled_from_execution"
    pub timestamp: String,
    pub description: Option<String>,
}

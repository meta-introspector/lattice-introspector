use lattice_macros::LatticePointDerive;

/// Represents a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct Repository {
    pub name: String,
    pub url: String,
    pub git_repo_url: String, // URL of the Git repo
    // Add more fields as needed, e.g., organization, main branch
}

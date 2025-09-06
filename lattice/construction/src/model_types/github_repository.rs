use lattice_macros::LatticePointDerive;

/// Represents a GitHub repository.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GitHubRepository {
    pub name: String,
    pub owner: String, // User or organization name
    pub url: String,
    pub description: Option<String>,
    pub stars: u32,
    pub forks: u32,
    pub last_commit_sha: Option<String>,
    pub git_repo_url: String, // Self-referential, but useful for consistency
}

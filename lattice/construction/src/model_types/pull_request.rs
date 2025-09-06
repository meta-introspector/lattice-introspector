use lattice_macros::LatticePointDerive;

/// Represents a GitHub Pull Request.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct PullRequest {
    pub pr_id: u64,
    pub title: String,
    pub author_username: String,
    pub state: String, // e.g., "open", "closed", "merged"
    pub created_at: String,
    pub updated_at: String,
    pub merged_at: Option<String>,
    pub repository_url: String,
    pub head_ref: String,
    pub base_ref: String,
}

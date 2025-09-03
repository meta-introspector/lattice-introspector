use lattice_macros::LatticePointDerive;

/// Represents a GitHub Actions workflow run.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GitHubActionRun {
    pub run_id: u64,
    pub workflow_name: String,
    pub status: String, // e.g., "success", "failure", "running"
    pub event: String, // e.g., "push", "pull_request"
    pub created_at: String,
    pub updated_at: String,
    pub repository_url: String,
    pub commit_sha: String,
}

use lattice_macros::LatticePointDerive;

/// Represents a Git commit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GitCommit {
    pub sha: String,
    pub author_name: String,
    pub author_email: String,
    pub commit_date: String,
    pub message: String,
    pub repository_url: String,
}

use lattice_macros::LatticePointDerive;

/// Represents a GitHub account or organization.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GitHubAccount {
    pub username: String,
    pub account_type: String, // "User" or "Organization"
    pub profile_url: String,
    pub public_repos: u32,
    pub followers: Option<u32>,
}

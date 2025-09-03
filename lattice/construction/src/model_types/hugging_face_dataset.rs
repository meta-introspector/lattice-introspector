use lattice_macros::LatticePointDerive;

/// Represents a Hugging Face dataset containing execution samples.
#[derive(Debug, Clone, PartialEq, LatticePointDerive)]
pub struct HuggingFaceDataset {
    pub name: String,
    pub huggingface_repo: String,
    pub sampled_from_models: Vec<String>,
    pub sampling_method: String,
    pub size_gb: f64,
    pub num_samples: u64,
    pub description: Option<String>,
    pub git_repo_url: String, // URL of the Git repo it was derived from
}

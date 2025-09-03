use lattice_macros::LatticePointDerive;

/// Represents a GGUF model from Hugging Face within the lattice.
#[derive(Debug, Clone, PartialEq, LatticePointDerive)]
pub struct GGUFModel {
    pub name: String,
    pub huggingface_repo: String,
    pub file_name: String,
    pub quant_method: String,
    pub size_gb: f64,
    pub description: Option<String>,
}

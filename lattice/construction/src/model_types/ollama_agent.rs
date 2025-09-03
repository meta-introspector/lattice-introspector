use lattice_macros::LatticePointDerive;

/// Represents an Ollama instance within the lattice.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct OllamaAgent {
    pub version: String,
    pub host: String,
    pub models_served: Vec<String>,
    pub status: String,
}

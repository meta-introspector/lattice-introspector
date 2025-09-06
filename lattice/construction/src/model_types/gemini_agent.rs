use lattice_macros::LatticePointDerive;

/// Represents the Gemini agent within the lattice.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GeminiAgent {
    pub version: String,
    pub role: String,
    pub capabilities: Vec<String>,
    pub current_task: Option<String>,
}

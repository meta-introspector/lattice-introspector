use lattice_macros::LatticePointDerive;

/// Represents a self-proving statement within the lattice.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct SelfProvingStatement {
    pub statement_text: String,
    pub current_step: u32,
    pub target_steps: u32,
    pub is_proven: bool,
}

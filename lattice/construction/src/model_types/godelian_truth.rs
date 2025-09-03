use lattice_macros::LatticePointDerive;

/// Represents a foundational, unprovable truth or axiom within the lattice's framework.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct GodelianTruth {
    pub truth_id: String,
    pub statement: String,
    pub implications: Vec<String>,
    pub source_of_truth: String, // e.g., "axiom", "emergent_property", "poetic_rhyme"
    pub timestamp: String,
}

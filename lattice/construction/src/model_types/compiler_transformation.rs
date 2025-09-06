use lattice_macros::LatticePointDerive;

/// Represents the transformation performed by a compiler, turning source code into binaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct CompilerTransformation {
    pub transformation_id: String,
    pub compiler_name: String,
    pub compiler_version: String,
    pub input_source_ids: Vec<String>, // IDs of source code points
    pub output_binary_id: String,      // ID of the binary point
    pub description: Option<String>,
    pub timestamp: String,
}

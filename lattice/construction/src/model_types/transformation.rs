use lattice_macros::LatticePointDerive;

/// Represents a transformation or operation within the lattice, such as a matrix multiplication.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)]
pub struct Transformation {
    pub transformation_id: String,
    pub input_point_ids: Vec<String>,
    pub output_point_ids: Vec<String>,
    pub operation_type: String, // e.g., "matrix_multiplication", "vector_tweaking"
    pub description: Option<String>,
    pub timestamp: String,
}

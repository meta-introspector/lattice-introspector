use crate::GenerationContext;

pub trait LatticeInput {
    fn from_lattice(context: &GenerationContext) -> Self;
}

pub trait LatticeOutput {
    fn to_lattice(&self, context: &mut GenerationContext);
}

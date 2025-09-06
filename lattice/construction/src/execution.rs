//! Data structures for modeling program execution and memory topology.

use lattice_macros::LatticePointDerive; // Add this line

/// Represents a single instruction in the compiled program.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct Instruction {
    pub instruction_id: String, // Unique ID for the instruction
    pub module_name: String, // The module/crate this instruction belongs to
    pub address: u64, // Memory address of the instruction
    // Add more details like opcode, operands, etc.
}

/// Represents a region of memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct MemoryRegion {
    pub region_id: String, // Unique ID for the memory region
    pub start_address: u64,
    pub end_address: u64,
    pub purpose: String, // e.g., "stack", "heap", "static data", "code"
}

/// Represents an access (read or write) to a memory location by an instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct MemoryAccess {
    pub access_id: String, // Unique ID for the memory access event
    pub instruction_id: String, // The instruction performing the access
    pub memory_address: u64, // The specific memory address accessed
    pub access_type: AccessType, // Read or Write
    pub timestamp: u64, // When the access occurred during execution
}

/// Type of memory access.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub enum AccessType {
    Read,
    Write,
}
//! Data structures for modeling virtual machine (QEMU/VM) execution context.

use lattice_macros::LatticePointDerive; // Changed to derive macro import

/// Represents the state of CPU registers.
#[derive(Debug, Clone, PartialEq, Eq, LatticePointDerive)] // Apply the derive macro
pub struct Registers {
    pub register_id: String, // Unique ID for this set of registers
    pub values: std::collections::HashMap<String, u64>, // Register name to its 64-bit value
    // This HashMap is expected to contain the Instruction Pointer (e.g., "rip" or "pc").
    // Add more specific register fields as needed (e.g., stack pointer)
}

/// Represents a simplified view of memory as a vector of bytes.
#[derive(Debug, Clone, PartialEq, Eq, LatticePointDerive)] // Apply the derive macro
pub struct MemoryVector {
    pub memory_id: String, // Unique ID for this memory state
    pub data: Vec<u8>, // The raw bytes of memory
    // Add metadata like base address, size, etc.
}

/// Represents a specific point in time during VM execution, capturing its state.
#[derive(Debug, Clone, PartialEq, Eq, LatticePointDerive)] // Apply the derive macro
pub struct VmExecutionSnapshot {
    pub snapshot_id: String, // Unique ID for this snapshot
    pub registers: Registers, // The state of the CPU registers
    pub memory_index: u64, // An index into a larger conceptual memory vector
    pub instruction_pointer: u64, // The current instruction pointer, typically derived from `registers`
    // Add a timestamp or sequence number for ordering snapshots
}

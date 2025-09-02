use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Represents the kind of entity a LatticePoint describes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LatticePointKind {
    Struct,
    Enum,
    Function,
    MemoryRegion,
    Instruction,
    CompileTimeEvent,
    RunTimeEvent,
    TraceEvent,
    LatticeMeta,
    MarkdownDocument,
    PredictedExecution,
    ActualExecution,
    LogEvent, // Added for structured log events
    // Add more kinds as needed for the "universe of universes"
}

/// Represents a single point in the univalent lattice.
/// This point can describe a Rust struct, a function, a memory region,
/// a compile-time event, a runtime event, or even the lattice itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatticePoint {
    pub id: String, // Unique identifier for this point
    pub kind: LatticePointKind,
    pub metadata: HashMap<String, String>, // Flexible storage for point-specific data
    pub relationships: Vec<String>, // IDs of other LatticePoints this point is related to
}
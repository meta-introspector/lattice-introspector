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
    LogEvent,
    GeminiAgent,
    OllamaAgent, // Added for Ollama instances
    GGUFModel,   // Added for GGUF models
    HuggingFaceDataset, // Added for Hugging Face datasets
    GitHubRepository,   // Added for GitHub repositories
    GitHubAccount,      // Added for GitHub accounts/organizations
    GitCommit,          // Added for Git commits
    PullRequest,        // Added for Pull Requests
    GitHubActionRun,    // Added for GitHub Action runs
    GitDerivedAsset,    // Added for assets derived from Git repos
    UserIntent,         // Added for user intent/vibe as a multivector
    Transformation,     // Added for representing transformations/matrix operations
    CompilerTransformation, // Added for representing compiler transformations
    GodelianTruth,          // Added for representing unprovable truths/foundational axioms
    AcademicPaper,          // Added for academic papers and publications
    AcademicAuthor,         // Added for academic authors and researchers
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
    pub hero_status: Option<String>, // Optional field to indicate heroification status/stage
}

/// An opaque handle to a LatticePoint, identified by its unique ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LatticeHandle(pub String);

impl From<String> for LatticeHandle {
    fn from(id: String) -> Self {
        LatticeHandle(id)
    }
}

impl From<&str> for LatticeHandle {
    fn from(id: &str) -> Self {
        LatticeHandle(id.to_string())
    }
}

impl std::fmt::Display for LatticeHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A trait for accessing LatticePoints from a Lattice-like structure.
pub trait LatticeAccess {
    /// Retrieves a reference to a LatticePoint by its handle.
    fn get_point(&self, handle: &LatticeHandle) -> Option<&LatticePoint>;

    /// Adds a LatticePoint to the lattice, returning its handle.
    fn add_point(&mut self, point: LatticePoint) -> LatticeHandle;
}
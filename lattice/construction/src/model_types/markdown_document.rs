use lattice_macros::LatticePointDerive;

/// Represents a Markdown document.
#[derive(Debug, Clone, PartialEq, Eq, Hash, LatticePointDerive)] // Apply the macro
pub struct MarkdownDocument {
    pub path: String, // Absolute path to the Markdown file
    pub content_hash: String, // A hash of the file's content
    pub title: Option<String>, // Extracted title from the Markdown content
    pub summary: Option<String>, // Extracted summary from the Markdown content
}

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use sha2::{Sha256, Digest};

use lattice_types::{LatticePoint, LatticePointKind};

/// Introspects a Markdown document and converts it into a LatticePoint.
pub fn introspect_markdown_document(file_path: &Path) -> LatticePoint {
    let path_str = file_path.to_string_lossy().to_string();
    let content = fs::read_to_string(file_path).unwrap_or_default();

    let mut hasher = Sha256::new();
    hasher.update(&content);
    let content_hash = format!("{:x}", hasher.finalize());

    let title = content.lines().next().and_then(|line| {
        if line.starts_with("# ") {
            Some(line[2..].trim().to_string())
        } else {
            None
        }
    });

    let summary = content.lines().skip(1).find(|line| !line.trim().is_empty()).map(|line| {
        line.trim().to_string()
    });

    let mut metadata = HashMap::new();
    metadata.insert("path".to_string(), path_str.clone());
    metadata.insert("content_hash".to_string(), content_hash);
    if let Some(t) = title {
        metadata.insert("title".to_string(), t);
    }
    if let Some(s) = summary {
        metadata.insert("summary".to_string(), s);
    }

    LatticePoint {
        id: format!("markdown_document_{}", path_str.replace("/", "_").replace(".", "_")),
        kind: LatticePointKind::MarkdownDocument,
        metadata,
        relationships: Vec::new(), // Relationships can be added later (e.g., links to other docs)
    }
}

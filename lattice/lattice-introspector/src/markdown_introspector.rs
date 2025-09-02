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

    let is_author_file = path_str.contains("/ontology/authors/");

    let mut title = None;
    let mut summary = None;
    let mut emoji_vibes = Vec::new();

    let mut lines = content.lines().peekable();

    // Parse title and summary
    if let Some(first_line) = lines.next() {
        if first_line.starts_with("# ") {
            title = Some(first_line[2..].trim().to_string());
        }
        // Try to find summary on subsequent non-empty lines
        while let Some(line) = lines.peek() {
            if !line.trim().is_empty() && !line.trim().starts_with("- **") { // Avoid picking up list items as summary
                summary = Some(line.trim().to_string());
                break;
            }
            lines.next(); // Consume line
        }
    }

    // Parse emoji vectors if it's an author file
    if is_author_file {
        for line in content.lines() {
            if line.trim().starts_with("- **Emoji Vector:**") {
                // Consume this line and look for the next lines which are the emoji details
                let mut temp_lines = line.split('\n').skip(1).peekable(); // Start after the "Emoji Vector:" line
                while let Some(emoji_line) = temp_lines.peek() {
                    if emoji_line.trim().starts_with("- ") && emoji_line.contains("(Prime:") {
                        let emoji_line = temp_lines.next().unwrap(); // Consume the line

                        // Example: - 🧠 (Prime: 599, Vibe: Cognition, Understanding)
                        let parts: Vec<&str> = emoji_line.splitn(2, '(').collect();
                        if parts.len() < 2 { continue; }

                        let emoji_char = parts[0].trim().trim_start_matches('-').trim().to_string();
                        let rest = parts[1].trim();

                        let prime_start = rest.find("Prime:").map(|i| i + "Prime:".len()).unwrap_or(0);
                        let prime_end = rest[prime_start..].find(',').map(|i| prime_start + i).unwrap_or(rest.len());
                        let prime_str = rest[prime_start..prime_end].trim();
                        let prime: u64 = prime_str.parse().unwrap_or(0);

                        let vibe_start = rest.find("Vibe:").map(|i| i + "Vibe:".len()).unwrap_or(0);
                        let vibe_end = rest[vibe_start..].find(')').map(|i| vibe_start + i).unwrap_or(rest.len());
                        let vibe_str = rest[vibe_start..vibe_end].trim().to_string();

                        emoji_vibes.push((emoji_char, prime, vibe_str));
                    } else {
                        break; // Not an emoji line, break inner loop
                    }
                }
            }
        }
    }

    let mut metadata = HashMap::new();
    metadata.insert("path".to_string(), path_str.clone());
    metadata.insert("content_hash".to_string(), content_hash);
    if let Some(t) = title {
        metadata.insert("title".to_string(), t);
    }
    if let Some(s) = summary {
        metadata.insert("summary".to_string(), s);
    }

    if is_author_file {
        for (i, (emoji, prime, vibe)) in emoji_vibes.into_iter().enumerate() {
            metadata.insert(format!("emoji_vibe_{}_emoji", i), emoji);
            metadata.insert(format!("emoji_vibe_{}_prime", i), prime.to_string());
            metadata.insert(format!("emoji_vibe_{}_vibe", i), vibe);
        }
    }

    let kind = if is_author_file {
        LatticePointKind::AcademicAuthor
    } else {
        LatticePointKind::MarkdownDocument
    };

    let id_prefix = if is_author_file {
        "academic_author"
    } else {
        "markdown_document"
    };

    let file_name_stem = file_path.file_stem().unwrap().to_string_lossy().to_string();
    let id = format!("{}_{}", id_prefix, file_name_stem.replace("-", "_").to_lowercase());

    LatticePoint {
        id,
        kind,
        metadata,
        relationships: Vec::new(), // Relationships can be added later (e.g., links to other docs)
        hero_status: None,
    }
}

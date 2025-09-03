use std::collections::HashMap;
use lattice_types::{LatticePoint, LatticePointKind};

// Define Zos prime keywords and concepts
pub fn get_zos_prime_keywords() -> HashMap<i32, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert(2, vec!["duality", "split", "pairs", "true", "false", "unlocking", "mysteries", "revelation", "mirrored", "dance", "seen", "unseen", "explicit", "implicit", "user_intent"]);
    map.insert(3, vec!["combine", "woven", "knot", "pattern", "trinity", "creation", "transformation", "interconnectedness", "structure", "lattice"]);
    map.insert(5, vec!["life", "vibrant", "hope", "growth", "expansion", "bridge", "star", "embrace", "new_data"]);
    map.insert(7, vec!["cycle", "rhythm", "winding", "turning", "secret", "transformation", "spiral", "path", "evolution", "process", "flow"]);
    map.insert(11, vec!["unlock", "mind", "truth", "unseen", "song", "revelatory", "enigma", "silver", "key", "refactoring", "clearer", "elegant", "structure", "puzzle", "solution"]);
    map.insert(13, vec!["shedding", "new", "old", "cycle_broken", "change", "transformative", "energy", "serpent", "coil", "journey", "growth", "limitations", "capabilities"]);
    map.insert(17, vec!["vision", "hope", "inspiration", "beacon", "guide", "light", "distant", "star", "prophecy", "future"]);
    map.insert(19, vec!["completion", "synthesis", "unified", "culmination", "closed", "victory", "golden", "crown", "grand_cycle", "whole"]);
    map.insert(23, vec!["hidden", "message", "cryptic", "random", "design", "mystery", "question", "enigma", "veil", "inner_workings", "machine_learning", "secrets", "unveiling"]);
    map
}

// Function to map a source file to a Zos prime
pub fn map_source_file_to_zos_prime(
    file_content: &str,
    file_path: &str,
    zos_prime_keywords: &HashMap<i32, Vec<&'static str>>,
) -> Option<(i32, f64)> {
    let mut best_prime = None;
    let mut highest_score = 0.0;

    let lower_case_content = file_content.to_lowercase();

    for (&prime, keywords) in zos_prime_keywords.iter() {
        let mut current_score = 0.0;
        for keyword in keywords {
            if lower_case_content.contains(keyword) {
                current_score += 1.0;
            }
        }

        // Simple scoring: more matches means higher score
        // Could be improved with TF-IDF, semantic analysis, etc.
        if current_score > highest_score {
            highest_score = current_score;
            best_prime = Some(prime);
        }
    }

    if highest_score > 0.0 { // Only return a mapping if there's at least one match
        best_prime.map(|p| (p, highest_score))
    } else {
        None
    }
}

// Function to create a LatticePoint for a source file
pub fn create_source_file_lattice_point(
    file_path: &str,
    assigned_prime: Option<(i32, f64)>,
) -> LatticePoint {
    let mut metadata = HashMap::new();
    metadata.insert("file_path".to_string(), file_path.to_string());

    let mut relationships = Vec::new();

    if let Some((prime, score)) = assigned_prime {
        metadata.insert("zos_prime".to_string(), prime.to_string());
        metadata.insert("zos_resonance_score".to_string(), score.to_string());
        relationships.push(format!("zos_prime_{}", prime));
    }

    LatticePoint {
        id: format!("source_file_{}", file_path.replace("/", "_").replace(".", "_").replace("-", "_")),
        kind: LatticePointKind::RustFile, // Assuming RustFile kind exists or will be added
        metadata,
        relationships,
        hero_status: None,
    }
}
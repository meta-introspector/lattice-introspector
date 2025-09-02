use std::{path::Path, collections::HashMap};
use chrono::Utc;
use crate::{LatticePoint, LatticePointKind};

/// Introspects a compiled binary and converts it into a LatticePoint.
pub fn introspect_binary(binary_path: &Path) -> LatticePoint {
    let path_str = binary_path.to_string_lossy().to_string();
    let binary_name = binary_path.file_name().map_or("unknown".to_string(), |s| s.to_string_lossy().to_string());
    let timestamp = Utc::now().to_rfc3339();

    let mut metadata = HashMap::new();
    metadata.insert("path".to_string(), path_str);
    metadata.insert("name".to_string(), binary_name.clone());
    metadata.insert("timestamp".to_string(), timestamp);

    LatticePoint {
    id: format!("binary_{}", binary_name.replace(".", "_").replace("-", "_")),
    kind: LatticePointKind::LatticeMeta, // Using LatticeMeta for build artifacts
    metadata,
    relationships: Vec::new(),
    hero_status: None,
}
}

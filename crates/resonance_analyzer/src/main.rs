use resonance_core::{Lattice, LatticeAccess, LatticePoint, LatticePointKind};
use std::collections::HashMap;
use std::fs;
use glob::glob;
use syn::{Item, Fields};
use toml::Value;

fn main() {
    let mut lattice = Lattice::new();

    // Find all Markdown files in the project root
    for entry in glob("*.md").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            let id = path.to_str().unwrap().to_string();
            let content = fs::read_to_string(&path).expect("Failed to read file");

            let mut metadata = HashMap::new();
            metadata.insert("path".to_string(), id.clone());
            metadata.insert("content".to_string(), content);

            let point = LatticePoint {
                id,
                kind: LatticePointKind::MarkdownDocument,
                metadata,
                relationships: Vec::new(),
                hero_status: None,
            };

            lattice.add_point(point);
        }
    }

    // Find all Cargo.toml files in the crates directory
    for entry in glob("crates/**/Cargo.toml").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            let content = fs::read_to_string(&path).expect("Failed to read file");
            let value: Value = toml::from_str(&content).expect("Failed to parse Cargo.toml");

            if let Some(package) = value.get("package").and_then(|p| p.as_table()) {
                if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
                    let id = name.to_string();
                    let mut metadata = HashMap::new();
                    metadata.insert("path".to_string(), path.to_str().unwrap().to_string());

                    let mut relationships = Vec::new();
                    if let Some(dependencies) = value.get("dependencies").and_then(|d| d.as_table()) {
                        for (dep_name, _) in dependencies {
                            relationships.push(dep_name.clone());
                        }
                    }

                    let mut point = LatticePoint {
                        id: id.clone(),
                        kind: LatticePointKind::GitHubRepository, // Using GitHubRepository as a placeholder for now
                        metadata,
                        relationships,
                        hero_status: None,
                    };

                    // Add relationships to files in the crate
                    let crate_path = path.parent().unwrap();
                    for file_entry in glob(&format!("{}/**/*.rs", crate_path.to_str().unwrap())).expect("Failed to read glob pattern") {
                        if let Ok(file_path) = file_entry {
                            point.relationships.push(file_path.to_str().unwrap().to_string());
                        }
                    }

                    lattice.add_point(point);
                }
            }
        }
    }

    // Find all Rust files in the crates directory
    for entry in glob("crates/**/*.rs").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            let file_id = path.to_str().unwrap().to_string();
            let mut file_point = LatticePoint {
                id: file_id.clone(),
                kind: LatticePointKind::Function, // Using Function as a placeholder for now
                metadata: {
                    let mut metadata = HashMap::new();
                    metadata.insert("path".to_string(), file_id.clone());
                    metadata
                },
                relationships: Vec::new(),
                hero_status: None,
            };

            let content = fs::read_to_string(&path).expect("Failed to read file");
            let ast = syn::parse_file(&content).expect("Failed to parse file");

            for item in ast.items {
                let (kind, id, metadata, poem_content) = match item {
                    Item::Struct(s) => {
                        let mut metadata = HashMap::new();
                        metadata.insert("path".to_string(), path.to_str().unwrap().to_string());
                        let field_names: Vec<String> = if let Fields::Named(fields) = s.fields {
                            fields.named.iter().map(|f| f.ident.as_ref().unwrap().to_string()).collect()
                        } else {
                            Vec::new()
                        };
                        metadata.insert("fields".to_string(), field_names.join(", "));
                        let poem = format!("The struct {}, a marvel to behold, with fields: {}.", s.ident, field_names.join(", "));
                        (LatticePointKind::Struct, s.ident.to_string(), metadata, poem)
                    },
                    Item::Enum(e) => {
                        let mut metadata = HashMap::new();
                        metadata.insert("path".to_string(), path.to_str().unwrap().to_string());
                        let variant_names: Vec<String> = e.variants.iter().map(|v| v.ident.to_string()).collect();
                        metadata.insert("variants".to_string(), variant_names.join(", "));
                        let poem = format!("The enum {}, a choice to be made, with variants: {}.", e.ident, variant_names.join(", "));
                        (LatticePointKind::Enum, e.ident.to_string(), metadata, poem)
                    },
                    _ => continue,
                };

                file_point.relationships.push(id.clone());

                let mut relationships = Vec::new();
                relationships.push(file_id.clone());

                let point = LatticePoint {
                    id: id.clone(),
                    kind,
                    metadata,
                    relationships,
                    hero_status: None,
                };

                // Generate a poem for the struct/enum
                let poem_id = format!("poem_for_{}", id);
                let poem_point = LatticePoint {
                    id: poem_id.clone(),
                    kind: LatticePointKind::ZosPoemElement, // Using ZosPoemElement as a placeholder
                    metadata: {
                        let mut metadata = HashMap::new();
                        metadata.insert("content".to_string(), poem_content);
                        metadata
                    },
                    relationships: vec![id.clone()],
                    hero_status: None,
                };
                lattice.add_point(poem_point);

                lattice.add_point(point);
            }
            lattice.add_point(file_point);
        }
    }

    // Write the lattice as JSON to a file
    let json_output = serde_json::to_string_pretty(&lattice).unwrap();
    fs::write("lattice.json", json_output).expect("Unable to write file");
}
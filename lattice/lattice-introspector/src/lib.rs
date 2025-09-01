use std::collections::HashMap;
use syn::{self, Type, Fields, DeriveInput, Data, DataStruct, DataEnum, Variant};
use quote::quote;
use std::path::Path;
use std::fs;
use sha2::{Sha256, Digest};

// Import LatticePoint and LatticePointKind from the lattice-types crate
use lattice_types::{LatticePoint, LatticePointKind};

/// Introspects a Rust item (struct, enum) and converts it into a LatticePoint.
pub fn introspect_item(input: &DeriveInput) -> LatticePoint {
    let item_name = input.ident.to_string();
    let mut metadata = HashMap::new();
    let relationships = Vec::new();
    let mut kind = LatticePointKind::Struct; // Default to Struct, will be updated

    metadata.insert("name".to_string(), item_name.clone());

    match &input.data {
        Data::Struct(data_struct) => {
            kind = LatticePointKind::Struct;
            metadata.insert("type".to_string(), "struct".to_string());
            match &data_struct.fields {
                Fields::Named(fields) => {
                    for field in &fields.named {
                        let field_name = field.ident.as_ref().map_or("".to_string(), |id| id.to_string());
                        let field_type = type_to_string(&field.ty);
                        metadata.insert(format!("field_{}_name", field_name), field_name.clone());
                        metadata.insert(format!("field_{}_type", field_name), field_type.clone());
                        // Add relationships if needed, e.g., to other structs if field_type is a struct
                    }
                }
                Fields::Unnamed(fields) => {
                    for (i, field) in fields.unnamed.iter().enumerate() {
                        let field_name = format!("{}", i); // Use index as name for tuple structs
                        let field_type = type_to_string(&field.ty);
                        metadata.insert(format!("field_{}_name", field_name), field_name.clone());
                        metadata.insert(format!("field_{}_type", field_name), field_type.clone());
                    }
                }
                Fields::Unit => {
                    // Unit struct, no fields
                }
            }
        }
        Data::Enum(data_enum) => {
            kind = LatticePointKind::Enum;
            metadata.insert("type".to_string(), "enum".to_string());
            for (i, variant) in data_enum.variants.iter().enumerate() {
                let variant_name = variant.ident.to_string();
                metadata.insert(format!("variant_{}_name", i), variant_name.clone());
                // Add fields for enum variants if they have them
                match &variant.fields {
                    Fields::Named(fields) => {
                        for field in &fields.named {
                            let field_name = field.ident.as_ref().map_or("".to_string(), |id| id.to_string());
                            let field_type = type_to_string(&field.ty);
                            metadata.insert(format!("variant_{}_{}_field_name", i, field_name), field_name.clone());
                            metadata.insert(format!("variant_{}_{}_field_type", i, field_name), field_type.clone());
                        }
                    }
                    Fields::Unnamed(fields) => {
                        for (j, field) in fields.unnamed.iter().enumerate() {
                            let field_name = format!("{}", j);
                            let field_type = type_to_string(&field.ty);
                            metadata.insert(format!("variant_{}_{}_field_name", i, field_name), field_name.clone());
                            metadata.insert(format!("variant_{}_{}_field_type", i, field_name), field_type.clone());
                        }
                    }
                    Fields::Unit => {
                        // Unit variant
                    }
                }
            }
        }
        Data::Union(_) => {
            // Unions are not yet supported for detailed introspection
            kind = LatticePointKind::Struct; // Fallback, could be a specific Union kind later
            metadata.insert("type".to_string(), "union_unsupported".to_string());
        }
    }

    LatticePoint {
        id: format!("{}_{}", metadata.get("type").unwrap_or(&"unknown".to_string()), item_name),
        kind,
        metadata,
        relationships,
    }
}

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


// Helper function to convert a syn::Type to a String representation
fn type_to_string(ty: &Type) -> String {
    quote! { #ty }.to_string()
}
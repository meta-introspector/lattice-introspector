use std::{fs, path::Path, io::{Read, Write}};
use syn::{Item, Ident};
use quote::{quote, format_ident, TokenStreamExt};
use proc_macro2::TokenStream;

// Re-export necessary types from dependencies
pub use lattice_types::{LatticePoint, LatticePointKind};
pub use once_cell::sync::Lazy;

/// Manages the step count for the self-proving statement.
/// Reads the current step from a file, increments it, and writes it back.
/// Returns the incremented step count.
pub fn get_and_increment_step_count() -> u32 {
    let project_root = Path::new("/data/data/com.termux/files/home/storage/github/rustc/"); // Hardcoded project root
    let gemini_dir = project_root.join(".gemini");
    let step_file_path = gemini_dir.join("self_proving_step.txt");

    // Ensure .gemini directory exists
    fs::create_dir_all(&gemini_dir).unwrap();

    let mut current_step = 0;

    if step_file_path.exists() {
        let mut file = fs::File::open(&step_file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        current_step = content.trim().parse().unwrap_or(0);
    }

    current_step += 1;

    let mut file = fs::File::create(&step_file_path).unwrap();
    file.write_all(current_step.to_string().as_bytes()).unwrap();

    current_step
}

/// Generates the Rust code for registering all lattice points.
/// This includes static definitions for markdown documents and a function
/// to register all points (derived types and markdown documents).
pub fn generate_lattice_registration_code(
    src_dir: &Path,
    markdown_paths: &[&Path],
) -> String {
    let mut getter_function_definitions = Vec::new();
    let mut add_point_calls = Vec::new();

    // Get and increment the step count
    let current_step = get_and_increment_step_count();
    let target_steps = 42;
    let is_proven = current_step >= target_steps;

    // Create the SelfProvingStatement LatticePoint
    let statement_text = "This statement will prove itself in 42 steps as a fixed point in the lattice.".to_string();
    let self_proving_point_id = "self_proving_statement".to_string();

    // Add the SelfProvingStatement to getter_function_definitions and add_point_calls
    getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        static SELF_PROVING_STATEMENT_LATTICE_POINT: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind};

            let mut metadata = HashMap::new();
            metadata.insert("statement_text".to_string(), #statement_text.to_string());
            metadata.insert("current_step".to_string(), #current_step.to_string());
            metadata.insert("target_steps".to_string(), #target_steps.to_string());
            metadata.insert("is_proven".to_string(), #is_proven.to_string());

            lattice_types::LatticePoint {
                id: #self_proving_point_id.to_string(),
                kind: lattice_types::LatticePointKind::Struct, // Assuming it's a struct
                metadata,
                relationships: Vec::new(),
            }
        });

        #[allow(dead_code)]
        pub fn get_self_proving_statement_lattice_point() -> &'static lattice_types::LatticePoint {
            &SELF_PROVING_STATEMENT_LATTICE_POINT
        }
    });
    add_point_calls.push(quote! {
        lattice.add_point(crate::model::get_selfprovingstatement_lattice_point().clone());
    });

    // --- Update PLAN.md with Self-Proving Statement Status ---
    let plan_md_path = Path::new("PLAN.md"); // Assuming PLAN.md is in the construction crate root
    if plan_md_path.exists() {
        let mut plan_content = fs::read_to_string(plan_md_path).unwrap();
        let current_step_str = current_step.to_string();
        let is_proven_str = is_proven.to_string();

        // Replace placeholders
        plan_content = plan_content.replace("<CURRENT_STEP>", &current_step_str);
        plan_content = plan_content.replace("<IS_PROVEN>", &is_proven_str);

        // Write back to PLAN.md
        fs::write(plan_md_path, plan_content).unwrap();
    }

    // --- Process Rust files ---
    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(&path).unwrap();
            let syntax = syn::parse_file(&content).unwrap();

            for item in syntax.items {
                let (item_ident, item_attrs) = match &item {
                    Item::Struct(s) => (Some(&s.ident), &s.attrs),
                    Item::Enum(e) => (Some(&e.ident), &e.attrs),
                    _ => (None, &vec![]),
                };

                if let Some(ident) = item_ident {
                    let has_derive_lattice_point = item_attrs.iter().any(|attr| {
                        if attr.path().is_ident("derive") {
                            let mut found = false;
                            attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("LatticePointDerive") {
                                    found = true;
                                }
                                Ok(())
                            }).ok();
                            found
                        } else {
                            false
                        }
                    });

                    if has_derive_lattice_point {
                        let get_fn_name = format_ident!("get_{}_lattice_point", ident.to_string().to_lowercase());
                        let module_prefix = get_module_prefix_for_ident(ident);
                        add_point_calls.push(quote! {
                            lattice.add_point(#module_prefix #get_fn_name().clone());
                        });
                    }
                }
            }
        }
    }

    // --- Process Markdown files ---
    for md_path in markdown_paths {
        if md_path.exists() && md_path.is_file() {
            let file_stem = md_path.file_stem().unwrap().to_string_lossy().to_string();
            let get_fn_name = format_ident!("get_{}_markdown_document_lattice_point", file_stem.to_lowercase());

            // Call the introspection function directly in build.rs
            let md_point = lattice_introspector::introspect_markdown_document(md_path);
            let md_id = md_point.id;
            let md_kind = match md_point.kind {
                lattice_types::LatticePointKind::MarkdownDocument => quote! { lattice_types::LatticePointKind::MarkdownDocument },
                _ => unreachable!(), // Should always be MarkdownDocument
            };
            let md_metadata_inserts = md_point.metadata.iter().map(|(k, v)| {
                quote! { metadata.insert(#k.to_string(), #v.to_string()); }
            });
            let md_relationships = md_point.relationships.iter().map(|r| {
                quote! { #r.to_string() }
            });

            let static_md_name = format_ident!("{}_MARKDOWN_DOCUMENT_LATTICE_POINT", file_stem.to_uppercase());

            getter_function_definitions.push(quote! {
                #[allow(dead_code)]
                static #static_md_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
                    use std::collections::HashMap;
                    use lattice_types::{LatticePoint, LatticePointKind};

                    let mut metadata = HashMap::new();
                    #(#md_metadata_inserts)*

                    let relationships = vec![#(#md_relationships),*];

                    lattice_types::LatticePoint {
                        id: #md_id.to_string(),
                        kind: #md_kind,
                        metadata,
                        relationships,
                    }
                });

                #[allow(dead_code)]
                pub fn #get_fn_name() -> &'static lattice_types::LatticePoint {
                    &#static_md_name
                }
            });
            add_point_calls.push(quote! {
                lattice.add_point(#get_fn_name().clone());
            });
        }
    }

    let generated_code = quote! {
        #(#getter_function_definitions)*

        pub fn register_all_lattice_points(lattice: &mut crate::lattice::Lattice) {
            #(#add_point_calls)*
        }
    };

    generated_code.to_string()
}

// Helper function to determine the module prefix for a given Ident
fn get_module_prefix_for_ident(ident: &Ident) -> TokenStream {
    match ident.to_string().as_str() {
        "Repository" | "GitSubmodule" | "CargoCrate" | "RustFile" | "FfiBinding" | "MarkdownDocument" | "SelfProvingStatement" => quote! { crate::model:: },
        "RustcInvocation" => quote! { crate::compilation:: },
        "CompilerMemoryLocation" | "CompilerInternalRepresentation" => quote! { crate::compiler_ir:: },
        "Instruction" | "MemoryRegion" | "MemoryAccess" | "AccessType" => quote! { crate::execution:: },
        "Lattice" => quote! { crate::lattice:: },
        "Registers" | "MemoryVector" | "VmExecutionSnapshot" => quote! { crate::vm_context:: },
        _ => quote! { crate:: }, // Fallback, though all should be covered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::Ident;

    #[test]
    fn test_get_module_prefix_for_ident() {
        // Test cases for crate::model
        assert_eq!(get_module_prefix_for_ident(&Ident::new("Repository", proc_macro2::Span::call_site())).to_string(), "crate :: model ::");
        assert_eq!(get_module_prefix_for_ident(&Ident::new("MarkdownDocument", proc_macro2::Span::call_site())).to_string(), "crate :: model ::");

        // Test cases for other modules
        assert_eq!(get_module_prefix_for_ident(&Ident::new("RustcInvocation", proc_macro2::Span::call_site())).to_string(), "crate :: compilation ::");
        assert_eq!(get_module_prefix_for_ident(&Ident::new("CompilerMemoryLocation", proc_macro2::Span::call_site())).to_string(), "crate :: compiler_ir ::");
        assert_eq!(get_module_prefix_for_ident(&Ident::new("Instruction", proc_macro2::Span::call_site())).to_string(), "crate :: execution ::");
        assert_eq!(get_module_prefix_for_ident(&Ident::new("Lattice", proc_macro2::Span::call_site())).to_string(), "crate :: lattice ::");
        assert_eq!(get_module_prefix_for_ident(&Ident::new("Registers", proc_macro2::Span::call_site())).to_string(), "crate :: vm_context ::");

        // Test fallback
        assert_eq!(get_module_prefix_for_ident(&Ident::new("UnknownType", proc_macro2::Span::call_site())).to_string(), "crate ::");
    }
}
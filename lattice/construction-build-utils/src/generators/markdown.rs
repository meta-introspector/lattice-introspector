use quote::{quote, format_ident};

use crate::GenerationContext;
use lattice_introspector::markdown_introspector::introspect_markdown_document;

pub fn generate_markdown_document_code(context: &mut GenerationContext) {
    // --- Process Markdown files ---
    for md_path in context.markdown_paths {
        if md_path.exists() && md_path.is_file() {
            let file_stem = md_path.file_stem().unwrap().to_string_lossy().to_string();
            let get_fn_name = format_ident!("get_{}_markdown_document_lattice_point", file_stem.to_lowercase());

            // Call the introspection function directly in build.rs
            let md_point = introspect_markdown_document(md_path);
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

            context.getter_function_definitions.push(quote! {
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
                        hero_status: None,
                    }
                });

                #[allow(dead_code)]
                pub fn #get_fn_name() -> &'static lattice_types::LatticePoint {
                    &#static_md_name
                }
            });
            context.add_point_calls.push(quote! {
                lattice.add_point(#get_fn_name().clone());
            });
        }
    }
}

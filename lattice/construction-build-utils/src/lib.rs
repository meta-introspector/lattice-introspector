use std::path::Path;
use quote::quote;
use proc_macro2::TokenStream;

// Re-export necessary types from dependencies
pub use lattice_types::{LatticePoint, LatticePointKind};
pub use once_cell::sync::Lazy;

// Declare the new modules
pub mod generators;
pub mod introspectors;
pub mod utils;
pub mod zos_mapper;

pub struct GenerationContext<'a> {
    pub getter_function_definitions: &'a mut Vec<TokenStream>,
    pub add_point_calls: &'a mut Vec<TokenStream>,
    pub src_dir: &'a Path,
    pub markdown_paths: &'a [&'a Path],
    pub binary_point: &'a Option<LatticePoint>,
    pub predicted_execution_point: &'a Option<LatticePoint>,
    pub lattice_poem_mapping_path: &'a Path,
    pub zos_poem_resonance_map_path: &'a Path,
    pub project_root: &'a Path,
}

/// Generates the Rust code for registering all lattice points.
/// This includes static definitions for markdown documents and a function
/// to register all points (derived types and markdown documents).
pub fn generate_lattice_registration_code(
    src_dir: &Path,
    markdown_paths: &[&Path],
    binary_point: Option<LatticePoint>,
    predicted_execution_point: Option<LatticePoint>,
    project_root: &Path,
) -> String {
    let mut getter_function_definitions = Vec::new();
    let mut add_point_calls = Vec::new();
    let lattice_poem_mapping_path = Path::new("/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/LATTICE_POEM_MAPPING.md");
    let zos_poem_resonance_map_path = Path::new("/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/ZOS_POEM_RESONANCE_MAP.md");

    let mut context = GenerationContext {
        getter_function_definitions: &mut getter_function_definitions,
        add_point_calls: &mut add_point_calls,
        src_dir,
        markdown_paths,
        binary_point: &binary_point,
        predicted_execution_point: &predicted_execution_point,
        lattice_poem_mapping_path,
        zos_poem_resonance_map_path,
        project_root,
    };

    // Call generator functions
    generators::binary::generate_binary_point_code(&mut context);
    generators::predicted_execution::generate_predicted_execution_point_code(&mut context);
    generators::self_proving_statement::generate_self_proving_statement_code(&mut context);
    generators::markdown::generate_markdown_document_code(&mut context);
    generators::model_types::generate_model_types_code(&mut context);
    generators::git_related::generate_git_related_points_code(&mut context);
    generators::user_intent::generate_user_intent_code(&mut context);
    generators::transformation::generate_transformation_code(&mut context);
    generators::compiler_transformation::generate_compiler_transformation_code(
        context.getter_function_definitions,
        context.add_point_calls,
    );
    generators::godelian_truth::generate_godelian_truth_code(&mut context);
    generators::meta_attributes::generate_meta_attributes_code(&mut context);
    generators::zos_poems::generate_zos_poem_points_code(&mut context);
    zos_mapper::generate_prime_resonance_points_code(&mut context);
    println!("DEBUG: add_point_calls after zos_poems: {}", add_point_calls.len());

    let generated_code = quote! {
        use std::collections::HashMap; // Ensure HashMap is available
        use lattice_types::{LatticePoint, LatticePointKind}; // Ensure LatticePoint types are available
        use chrono::Utc; // Ensure Utc is available for timestamps
        use serde::{Serialize, Deserialize}; // Ensure serde is available for meta_attributes

        #(#getter_function_definitions)*

        pub fn register_all_lattice_points(lattice: &mut impl LatticeAccess) {
            #(#add_point_calls)*
        }
    };

    generated_code.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::Ident;
    use crate::utils::get_module_prefix_for_ident; // Import from the new location

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

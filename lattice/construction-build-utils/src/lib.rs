use std::{path::Path};
use syn::{Item, Ident};
use quote::{quote, format_ident, TokenStreamExt};
use proc_macro2::TokenStream;

// Re-export necessary types from dependencies
pub use lattice_types::{LatticePoint, LatticePointKind};
pub use once_cell::sync::Lazy;

// Declare the new modules
pub mod generators;
pub mod introspectors;
pub mod utils;

/// Generates the Rust code for registering all lattice points.
/// This includes static definitions for markdown documents and a function
/// to register all points (derived types and markdown documents).
pub fn generate_lattice_registration_code(
    src_dir: &Path,
    markdown_paths: &[&Path],
    binary_point: Option<LatticePoint>,
    predicted_execution_point: Option<LatticePoint>,
) -> String {
    let mut getter_function_definitions = Vec::new();
    let mut add_point_calls = Vec::new();

    // Call generator functions
    generators::binary::generate_binary_point_code(binary_point, &mut getter_function_definitions, &mut add_point_calls);
    generators::predicted_execution::generate_predicted_execution_point_code(predicted_execution_point, &mut getter_function_definitions, &mut add_point_calls);
    generators::self_proving_statement::generate_self_proving_statement_code(&mut getter_function_definitions, &mut add_point_calls);
    generators::markdown::generate_markdown_document_code(markdown_paths, &mut getter_function_definitions, &mut add_point_calls);
    generators::model_types::generate_model_types_code(&mut getter_function_definitions, &mut add_point_calls);
    generators::git_related::generate_git_related_points_code(&mut getter_function_definitions, &mut add_point_calls);
    generators::user_intent::generate_user_intent_code(&mut getter_function_definitions, &mut add_point_calls);
    generators::transformation::generate_transformation_code(&mut getter_function_definitions, &mut add_point_calls);
    generators::compiler_transformation::generate_compiler_transformation_code(&mut getter_function_definitions, &mut add_point_calls);
    generators::godelian_truth::generate_godelian_truth_code(&mut getter_function_definitions, &mut add_point_calls);
    generators::meta_attributes::generate_meta_attributes_code(&mut getter_function_definitions, &mut add_point_calls, Path::new("/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/LATTICE_POEM_MAPPING.md")); // Add this line

    let generated_code = quote! {
        use std::collections::HashMap; // Ensure HashMap is available
        use lattice_types::{LatticePoint, LatticePointKind}; // Ensure LatticePoint types are available
        use chrono::Utc; // Ensure Utc is available for timestamps
        use serde::{Serialize, Deserialize}; // Ensure serde is available for meta_attributes

        #(#getter_function_definitions)*

        pub fn register_all_lattice_points(lattice: &mut crate::lattice::Lattice) {
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

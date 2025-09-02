use std::{fs, path::Path, io::{Read, Write}};
use syn::{Item, Ident};
use quote::{quote, format_ident, TokenStreamExt};
use proc_macro2::TokenStream;
use std::collections::HashMap;
use chrono::Utc;

// Re-export necessary types from dependencies
pub use lattice_types::{LatticePoint, LatticePointKind};
pub use once_cell::sync::Lazy;

// Declare the new generators module
pub mod generators;

/// Introspects a compiled binary and converts it into a LatticePoint.
pub fn introspect_binary(binary_path: &Path) -> LatticePoint {
    let path_str = binary_path.to_string_lossy().to_string();
    let binary_name = binary_path.file_name().map_or("unknown".to_string(), |s| s.to_string_lossy().to_string());
    let timestamp = Utc::now().to_rfc3339();

    let mut metadata = HashMap::new();
    metadata.insert("path".to_string(), path_str);
    metadata.insert("name".to_string(), binary_name);
    metadata.insert("timestamp".to_string(), timestamp);

    LatticePoint {
        id: format!("binary_{}", binary_name.replace(".", "_")),
        kind: LatticePointKind::LatticeMeta, // Using LatticeMeta for build artifacts
        metadata,
        relationships: Vec::new(),
    }
}

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

    let generated_code = quote! {
        #(#getter_function_definitions)*

        pub fn register_all_lattice_points(lattice: &mut crate::lattice::Lattice) {
            #(#add_point_calls)*
        }
    };

    generated_code.to_string()
}

// Helper function to determine the module prefix for a given Ident
pub fn get_module_prefix_for_ident(ident: &Ident) -> TokenStream {
    match ident.to_string().as_str() {
        "Repository" | "GitSubmodule" | "CargoCrate" | "RustFile" | "FfiBinding" | "MarkdownDocument" | "SelfProvingStatement" | "GeminiAgent" | "OllamaAgent" | "GGUFModel" | "HuggingFaceDataset" | "GitHubRepository" | "GitHubAccount" | "GitCommit" | "PullRequest" | "GitHubActionRun" | "GitDerivedAsset" | "UserIntent" => quote! { crate::model:: },
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

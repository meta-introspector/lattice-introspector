use std::{fs, path::Path, io::{Read, Write}};
use syn::Ident;
use quote::{quote, TokenStreamExt};
use proc_macro2::TokenStream;

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

// Helper function to determine the module prefix for a given Ident
pub fn get_module_prefix_for_ident(ident: &Ident) -> TokenStream {
    match ident.to_string().as_str() {
        "Repository" | "GitSubmodule" | "CargoCrate" | "RustFile" | "FfiBinding" | "MarkdownDocument" | "SelfProvingStatement" | "GeminiAgent" | "OllamaAgent" | "GGUFModel" | "HuggingFaceDataset" | "GitHubRepository" | "GitHubAccount" | "GitCommit" | "PullRequest" | "GitHubActionRun" | "GitDerivedAsset" | "UserIntent" | "Transformation" | "CompilerTransformation" | "GodelianTruth" => quote! { crate::model_types:: },
        "RustcInvocation" => quote! { crate::compilation:: },
        "CompilerMemoryLocation" | "CompilerInternalRepresentation" => quote! { crate::compiler_ir:: },
        "Instruction" | "MemoryRegion" | "MemoryAccess" | "AccessType" => quote! { crate::execution:: },
        "Lattice" => quote! { crate::lattice:: },
        "Registers" | "MemoryVector" | "VmExecutionSnapshot" => quote! { crate::vm_context:: },
        _ => quote! { crate:: }, // Fallback, though all should be covered
    }
}

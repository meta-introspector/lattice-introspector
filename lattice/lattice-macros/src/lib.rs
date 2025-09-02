extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use quote::format_ident;

// Import the introspection function from lattice-introspector
use lattice_introspector::rust_introspector::introspect_item;
// Removed LatticePoint and LatticePointKind import from here, as they are used in generated code
// use lattice_types::{LatticePoint, LatticePointKind};

#[proc_macro_derive(LatticePointDerive)]
pub fn lattice_point_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident.clone();
    let introspected_point = introspect_item(&input);

    let id = introspected_point.id;
    let kind = match introspected_point.kind {
        lattice_types::LatticePointKind::Struct => quote! { lattice_types::LatticePointKind::Struct },
        lattice_types::LatticePointKind::Enum => quote! { lattice_types::LatticePointKind::Enum },
        lattice_types::LatticePointKind::Function => quote! { lattice_types::LatticePointKind::Function },
        lattice_types::LatticePointKind::MemoryRegion => quote! { lattice_types::LatticePointKind::MemoryRegion },
        lattice_types::LatticePointKind::Instruction => quote! { lattice_types::LatticePointKind::Instruction },
        lattice_types::LatticePointKind::CompileTimeEvent => quote! { lattice_types::LatticePointKind::CompileTimeEvent },
        lattice_types::LatticePointKind::RunTimeEvent => quote! { lattice_types::LatticePointKind::RunTimeEvent },
        lattice_types::LatticePointKind::TraceEvent => quote! { lattice_types::LatticePointKind::TraceEvent },
        lattice_types::LatticePointKind::LatticeMeta => quote! { lattice_types::LatticePointKind::LatticeMeta },
        lattice_types::LatticePointKind::MarkdownDocument => quote! { lattice_types::LatticePointKind::MarkdownDocument },
        lattice_types::LatticePointKind::PredictedExecution => quote! { lattice_types::LatticePointKind::PredictedExecution },
        lattice_types::LatticePointKind::ActualExecution => quote! { lattice_types::LatticePointKind::ActualExecution },
        lattice_types::LatticePointKind::LogEvent => quote! { lattice_types::LatticePointKind::LogEvent },
        lattice_types::LatticePointKind::GeminiAgent => quote! { lattice_types::LatticePointKind::GeminiAgent },
        lattice_types::LatticePointKind::OllamaAgent => quote! { lattice_types::LatticePointKind::OllamaAgent },
        lattice_types::LatticePointKind::GGUFModel => quote! { lattice_types::LatticePointKind::GGUFModel },
        lattice_types::LatticePointKind::HuggingFaceDataset => quote! { lattice_types::LatticePointKind::HuggingFaceDataset },
        lattice_types::LatticePointKind::GitHubRepository => quote! { lattice_types::LatticePointKind::GitHubRepository },
        lattice_types::LatticePointKind::GitHubAccount => quote! { lattice_types::LatticePointKind::GitHubAccount },
        lattice_types::LatticePointKind::GitCommit => quote! { lattice_types::LatticePointKind::GitCommit },
        lattice_types::LatticePointKind::PullRequest => quote! { lattice_types::LatticePointKind::PullRequest },
        lattice_types::LatticePointKind::GitHubActionRun => quote! { lattice_types::LatticePointKind::GitHubActionRun },
        lattice_types::LatticePointKind::GitDerivedAsset => quote! { lattice_types::LatticePointKind::GitDerivedAsset },
        lattice_types::LatticePointKind::Transformation => quote! { lattice_types::LatticePointKind::Transformation },
        lattice_types::LatticePointKind::CompilerTransformation => quote! { lattice_types::LatticePointKind::CompilerTransformation },
        lattice_types::LatticePointKind::GodelianTruth => quote! { lattice_types::LatticePointKind::GodelianTruth },
    };

    let metadata_inserts = introspected_point.metadata.iter().map(|(k, v)| {
        quote! { metadata.insert(#k.to_string(), #v.to_string()); }
    });

    let relationships = introspected_point.relationships.iter().map(|r| {
        quote! { #r.to_string() }
    });

    // Generate a static variable to hold the LatticePoint
    // And a public function to get a reference to it
    let static_name = format_ident!("{}_LATTICE_POINT", struct_name.to_string().to_uppercase());
    let get_fn_name = format_ident!("get_{}_lattice_point", struct_name.to_string().to_lowercase());

    let generated_code = quote! {
        #[allow(dead_code)] // Allow dead code for the generated static LatticePoint
        static #static_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind}; // Imported here in the generated code

            let mut metadata = HashMap::new();
            #(#metadata_inserts)*

            let relationships = vec![#(#relationships),*];

            lattice_types::LatticePoint {
                id: #id.to_string(),
                kind: #kind,
                metadata,
                relationships,
            }
        });

        #[allow(dead_code)] // Allow dead code for the getter function
        pub fn #get_fn_name() -> &'static lattice_types::LatticePoint {
            &#static_name
        }
    };

    // Return only the generated code
    TokenStream::from(generated_code)
}
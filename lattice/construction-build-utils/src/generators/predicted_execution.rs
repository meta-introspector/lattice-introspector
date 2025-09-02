use std::path::Path;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use std::collections::HashMap;
use chrono::Utc;

use crate::LatticePoint;

pub fn generate_predicted_execution_point_code(
    predicted_execution_point: Option<LatticePoint>,
    getter_function_definitions: &mut Vec<TokenStream>,
    add_point_calls: &mut Vec<TokenStream>,
) {
    if let Some(pep) = predicted_execution_point {
        let pep_id = pep.id;
        let pep_kind = match pep.kind {
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
            lattice_types::LatticePointKind::UserIntent => quote! { lattice_types::LatticePointKind::UserIntent },
        };
        let pep_metadata_inserts = pep.metadata.iter().map(|(k, v)| {
            quote! { metadata.insert(#k.to_string(), #v.to_string()); }
        });
        let pep_relationships = pep.relationships.iter().map(|r| {
            quote! { #r.to_string() }
        });

        let static_pep_name = format_ident!("{}_LATTICE_POINT", pep_id.to_uppercase());
        let get_pep_fn_name = format_ident!("get_{}_lattice_point", pep_id.to_lowercase());

        getter_function_definitions.push(quote! {
            #[allow(dead_code)]
            static #static_pep_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
                use std::collections::HashMap;
                use lattice_types::{LatticePoint, LatticePointKind};

                let mut metadata = HashMap::new();
                #(#pep_metadata_inserts)*

                let relationships = vec![#(#pep_relationships),*];

                lattice_types::LatticePoint {
                    id: #pep_id.to_string(),
                    kind: #pep_kind,
                    metadata,
                    relationships,
                }
            });

            #[allow(dead_code)]
            pub fn #get_pep_fn_name() -> &'static lattice_types::LatticePoint {
                &#static_pep_name
            }
        });
        add_point_calls.push(quote! {
            lattice.add_point(#get_pep_fn_name().clone());
        });
    }
}

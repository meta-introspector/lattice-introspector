use quote::{quote, format_ident};

use crate::GenerationContext;

pub fn generate_predicted_execution_point_code(context: &mut GenerationContext) {
    if let Some(pep) = context.predicted_execution_point.as_ref() {
        let pep_id = pep.id.clone();
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
            lattice_types::LatticePointKind::Transformation => quote! { lattice_types::LatticePointKind::Transformation },
            lattice_types::LatticePointKind::CompilerTransformation => quote! { lattice_types::LatticePointKind::CompilerTransformation },
            lattice_types::LatticePointKind::GodelianTruth => quote! { lattice_types::LatticePointKind::GodelianTruth },
            lattice_types::LatticePointKind::AcademicPaper => quote! { lattice_types::LatticePointKind::AcademicPaper },
            lattice_types::LatticePointKind::AcademicAuthor => quote! { lattice_types::LatticePointKind::AcademicAuthor },
            lattice_types::LatticePointKind::ZosPoemElement => quote! { lattice_types::LatticePointKind::ZosPoemElement },
            lattice_types::LatticePointKind::ExecutionTrace => quote! { lattice_types::LatticePointKind::ExecutionTrace },
            lattice_types::LatticePointKind::PrimeResonance => quote! { lattice_types::LatticePointKind::PrimeResonance },
            lattice_types::LatticePointKind::WordResonance => quote! { lattice_types::LatticePointKind::WordResonance },
        };
        let pep_metadata_inserts = pep.metadata.iter().map(|(k, v)| {
            quote! { metadata.insert(#k.to_string(), #v.to_string()); }
        });
        let pep_relationships = pep.relationships.iter().map(|r| {
            quote! { #r.to_string() }
        });

        let static_pep_name = format_ident!("{}_LATTICE_POINT", pep_id.to_uppercase());
        let get_pep_fn_name = format_ident!("get_{}_lattice_point", pep_id.to_lowercase());

        context.getter_function_definitions.push(quote! {
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
                    hero_status: None,
                }
            });

            #[allow(dead_code)]
            pub fn #get_pep_fn_name() -> &'static lattice_types::LatticePoint {
                &#static_pep_name
            }
        });
        context.add_point_calls.push(quote! {
            lattice.add_point(#get_pep_fn_name().clone());
        });
    }
}

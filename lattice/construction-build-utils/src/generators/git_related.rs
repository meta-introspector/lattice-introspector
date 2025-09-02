use std::path::Path;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use std::collections::HashMap;
use chrono::Utc;

use crate::LatticePoint;

pub fn generate_git_related_points_code(
    getter_function_definitions: &mut Vec<TokenStream>,
    add_point_calls: &mut Vec<TokenStream>,
) {
    // Add a GitDerivedAsset point for the project's root Git repository
    let project_git_repo_url = "https://github.com/rust-lang/rust".to_string(); // Example URL
    let git_derived_asset_id = format!("git_derived_asset_{}", project_git_repo_url.replace(".", "_").replace("/", "_").replace(":", "_"));
    let git_derived_asset_point = LatticePoint {
        id: git_derived_asset_id.clone(),
        kind: lattice_types::LatticePointKind::GitDerivedAsset,
        metadata: {
            let mut map = HashMap::new();
            map.insert("git_repo_url".to_string(), project_git_repo_url.clone());
            map.insert("derivation_method".to_string(), "project_root".to_string());
            map.insert("timestamp".to_string(), Utc::now().to_rfc3339());
            map
        },
        relationships: Vec::new(),
    };
    let static_gda_name = format_ident!("{}_LATTICE_POINT", git_derived_asset_id.to_uppercase());
    let get_gda_fn_name = format_ident!("get_{}_lattice_point", git_derived_asset_id.to_lowercase());
    getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        static #static_gda_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind};
            let mut metadata = HashMap::new();
            metadata.insert("git_repo_url".to_string(), #project_git_repo_url.to_string());
            metadata.insert("derivation_method".to_string(), "project_root".to_string());
            metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
            lattice_types::LatticePoint {
                id: #git_derived_asset_id.to_string(),
                kind: lattice_types::LatticePointKind::GitDerivedAsset,
                metadata,
                relationships: Vec::new(),
            }
        });
        #[allow(dead_code)]
        pub fn #get_gda_fn_name() -> &'static lattice_types::LatticePoint {
            &#static_gda_name
        }
    });
    add_point_calls.push(quote! {
        lattice.add_point(#get_gda_fn_name().clone());
    });

    // Add a GitHubRepository point for the current project and relate it to GitDerivedAsset
    let project_repo_id = "github_repository_rust_lang_rust".to_string();
    let project_repo_point = LatticePoint {
        id: project_repo_id.clone(),
        kind: lattice_types::LatticePointKind::GitHubRepository,
        metadata: {
            let mut map = HashMap::new();
            map.insert("name".to_string(), "rust".to_string());
            map.insert("owner".to_string(), "rust-lang".to_string());
            map.insert("url".to_string(), project_git_repo_url.clone());
            map.insert("description".to_string(), "The Rust Programming Language".to_string());
            map.insert("stars".to_string(), "100000".to_string()); // Placeholder
            map.insert("forks".to_string(), "20000".to_string()); // Placeholder
            map.insert("last_commit_sha".to_string(), "abcdef1234567890".to_string()); // Placeholder
            map.insert("git_repo_url".to_string(), project_git_repo_url.clone());
            map
        },
        relationships: vec![git_derived_asset_id.clone()], // Relate to the GitDerivedAsset
    };
    let static_ghr_name = format_ident!("{}_LATTICE_POINT", project_repo_id.to_uppercase());
    let get_ghr_fn_name = format_ident!("get_{}_lattice_point", project_repo_id.to_lowercase());
    getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        static #static_ghr_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind};
            let mut metadata = HashMap::new();
            metadata.insert("name".to_string(), "rust".to_string());
            metadata.insert("owner".to_string(), "rust-lang".to_string());
            metadata.insert("url".to_string(), #project_git_repo_url.to_string());
            metadata.insert("description".to_string(), "The Rust Programming Language".to_string());
            metadata.insert("stars".to_string(), "100000".to_string());
            metadata.insert("forks".to_string(), "20000".to_string());
            metadata.insert("last_commit_sha".to_string(), "abcdef1234567890".to_string());
            metadata.insert("git_repo_url".to_string(), #project_git_repo_url.to_string());
            lattice_types::LatticePoint {
                id: #project_repo_id.to_string(),
                kind: lattice_types::LatticePointKind::GitHubRepository,
                metadata,
                relationships: vec![#git_derived_asset_id.to_string()],
            }
        });
        #[allow(dead_code)]
        pub fn #get_ghr_fn_name() -> &'static lattice_types::LatticePoint {
            &#static_ghr_name
        }
    });
    add_point_calls.push(quote! {
        lattice.add_point(#get_ghr_fn_name().clone());
    });
}

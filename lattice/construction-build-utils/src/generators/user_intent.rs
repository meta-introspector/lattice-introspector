use std::path::Path;
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use std::collections::HashMap;
use chrono::Utc;

use crate::LatticePoint;

pub fn generate_user_intent_code(
    getter_function_definitions: &mut Vec<TokenStream>,
    add_point_calls: &mut Vec<TokenStream>,
) {
    // Add a UserIntent point for the project's vibe/user intent
    let user_intent_id = "user_intent_project_vibe".to_string();
    let user_intent_point = LatticePoint {
        id: user_intent_id.clone(),
        kind: lattice_types::LatticePointKind::UserIntent,
        metadata: {
            let mut map = HashMap::new();
            map.insert("keywords".to_string(), "modularity, self-awareness, verifiable truth, meta-reflection, AI, human-AI collaboration".to_string());
            map.insert("emotional_tone".to_string(), "ambitious, philosophical, iterative, evolving".to_string());
            map.insert("goals".to_string(), "context reduction, fixed point convergence, computational meaning".to_string());
            map
        },
        relationships: vec![
            "lattice_meta".to_string(), // Relate to the Lattice itself
            "self_proving_statement".to_string(), // Relate to the self-proving statement
            "gemini_agent_v2_5_flash".to_string(), // Relate to Gemini 2.5 Flash
        ],
    };
    let static_ui_name = format_ident!("{}_LATTICE_POINT", user_intent_id.to_uppercase());
    let get_ui_fn_name = format_ident!("get_{}_lattice_point", user_intent_id.to_lowercase());
    getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        static #static_ui_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind};
            let mut metadata = HashMap::new();
            metadata.insert("keywords".to_string(), "modularity, self-awareness, verifiable truth, meta-reflection, AI, human-AI collaboration".to_string());
            metadata.insert("emotional_tone".to_string(), "ambitious, philosophical, iterative, evolving".to_string());
            metadata.insert("goals".to_string(), "context reduction, fixed point convergence, computational meaning".to_string());
            lattice_types::LatticePoint {
                id: #user_intent_id.to_string(),
                kind: lattice_types::LatticePointKind::UserIntent,
                metadata,
                relationships: vec![
                    "lattice_meta".to_string(),
                    "self_proving_statement".to_string(),
                    "gemini_agent_v2_5_flash".to_string(),
                ],
            }
        });
        #[allow(dead_code)]
        pub fn #get_ui_fn_name() -> &'static lattice_types::LatticePoint {
            &#static_ui_name
        }
    });
    add_point_calls.push(quote! {
        lattice.add_point(#get_ui_fn_name().clone());
    });
}

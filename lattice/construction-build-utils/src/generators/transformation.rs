use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use std::collections::HashMap;
use chrono::Utc;

use crate::{LatticePoint, GenerationContext};

pub fn generate_transformation_code(context: &mut GenerationContext) {
    // Conceptual points for demonstration
    let code_point_id = "conceptual_code_point".to_string();
    let poem_point_id = "conceptual_poem_point".to_string();
    let user_intent_id = "user_intent_project_vibe".to_string(); // From user_intent.rs

    // Add a Transformation point for "code to poem" via UserIntent
    let transformation_id = "transformation_code_to_poem_via_intent".to_string();
    let transformation_point = LatticePoint {
        id: transformation_id.clone(),
        kind: lattice_types::LatticePointKind::Transformation,
        metadata: {
            let mut map = HashMap::new();
            map.insert("operation_type".to_string(), "vector_tweaking_multiplication".to_string());
            map.insert("description".to_string(), "Transformation from code representation to poetic representation, driven by user intent.".to_string());
            map.insert("timestamp".to_string(), Utc::now().to_rfc3339());
            map
        },
        relationships: vec![
            code_point_id.clone(), // Input: Conceptual Code Point
            user_intent_id.clone(), // Input: User Intent (the 'tweaker')
            poem_point_id.clone(), // Output: Conceptual Poem Point
        ],
        hero_status: None,
    };
    println!("{:?}", transformation_point);
    let static_tx_name = format_ident!("{}_LATTICE_POINT", transformation_id.to_uppercase());
    let get_tx_fn_name = format_ident!("get_{}_lattice_point", transformation_id.to_lowercase());
    context.getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        static #static_tx_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind};
            let mut metadata = HashMap::new();
            metadata.insert("operation_type".to_string(), "vector_tweaking_multiplication".to_string());
            metadata.insert("description".to_string(), "Transformation from code representation to poetic representation, driven by user intent.".to_string());
            metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
            lattice_types::LatticePoint {
                id: #transformation_id.to_string(),
                kind: lattice_types::LatticePointKind::Transformation,
                metadata,
                relationships: vec![
                    #code_point_id.to_string(),
                    #user_intent_id.to_string(),
                    #poem_point_id.to_string(),
                ],
                hero_status: None,
            }
        });
        #[allow(dead_code)]
        pub fn #get_tx_fn_name() -> &'static lattice_types::LatticePoint {
            &#static_tx_name
        }
    });
    context.add_point_calls.push(quote! {
        lattice.add_point(#get_tx_fn_name().clone());
    });

    // Optionally, add the conceptual code and poem points if they don't exist elsewhere
    // For simplicity, we'll assume they are implicitly part of the lattice or will be added by other means.
    // If they were actual structs, they would be handled by model_types.rs generator.
}

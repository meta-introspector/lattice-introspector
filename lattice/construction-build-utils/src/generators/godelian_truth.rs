use quote::{quote, format_ident};
use std::collections::HashMap;
use chrono::Utc;

use crate::{GenerationContext, LatticePoint};

pub fn generate_godelian_truth_code(context: &mut GenerationContext) {
    let godelian_truth_id = "godelian_truth_rhyme_argument".to_string();
    let godelian_truth_point = LatticePoint {
        id: godelian_truth_id.clone(),
        kind: lattice_types::LatticePointKind::GodelianTruth,
        metadata: {
            let mut map = HashMap::new();
            map.insert("statement".to_string(), "The rhyme of the lattice is the argument of truth, the unprovable truth of Godel.".to_string());
            map.insert("implications".to_string(), "Self-referential consistency, inherent limits of formal systems, poetic truth as foundational.".to_string());
            map.insert("source_of_truth".to_string(), "poetic_rhyme".to_string());
            map.insert("timestamp".to_string(), Utc::now().to_rfc3339());
            map
        },
        relationships: vec![
            "lattice_meta".to_string(),
            "self_proving_statement".to_string(),
            "user_intent_project_vibe".to_string(),
        ],
        hero_status: None,
    };
    println!("{:?}", godelian_truth_point);
    let static_gt_name = format_ident!("{}_LATTICE_POINT", godelian_truth_id.to_uppercase());
    let get_gt_fn_name = format_ident!("get_{}_lattice_point", godelian_truth_id.to_lowercase());
    context.getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        static #static_gt_name: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind};
            let mut metadata = HashMap::new();
            metadata.insert("statement".to_string(), "The rhyme of the lattice is the argument of truth, the unprovable truth of Godel.".to_string());
            metadata.insert("implications".to_string(), "Self-referential consistency, inherent limits of formal systems, poetic truth as foundational.".to_string());
            metadata.insert("source_of_truth".to_string(), "poetic_rhyme".to_string());
            metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
            lattice_types::LatticePoint {
                id: #godelian_truth_id.to_string(),
                kind: lattice_types::LatticePointKind::GodelianTruth,
                metadata,
                relationships: vec![
                    "lattice_meta".to_string(),
                    "self_proving_statement".to_string(),
                    "user_intent_project_vibe".to_string(),
                ],
                hero_status: None,
            }
        });
        #[allow(dead_code)]
        pub fn #get_gt_fn_name() -> &'static lattice_types::LatticePoint {
            &#static_gt_name
        }
    });
    context.add_point_calls.push(quote! {
        lattice.add_point(#get_gt_fn_name().clone());
    });
}

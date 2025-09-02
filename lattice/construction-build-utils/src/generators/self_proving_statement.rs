use std::{fs, path::Path};
use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use std::collections::HashMap;
use chrono::Utc;

use crate::LatticePoint;
use crate::get_and_increment_step_count;

pub fn generate_self_proving_statement_code(
    getter_function_definitions: &mut Vec<TokenStream>,
    add_point_calls: &mut Vec<TokenStream>,
) {
    // Get and increment the step count
    let current_step = get_and_increment_step_count();
    let target_steps = 42;
    let is_proven = current_step >= target_steps;

    // Create the SelfProvingStatement LatticePoint
    let statement_text = "This statement will prove itself in 42 steps as a fixed point in the lattice.".to_string();
    let self_proving_point_id = "self_proving_statement".to_string();

    // Add the SelfProvingStatement to getter_function_definitions and add_point_calls
    getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        static SELF_PROVING_STATEMENT_LATTICE_POINT: once_cell::sync::Lazy<lattice_types::LatticePoint> = once_cell::sync::Lazy::new(|| {
            use std::collections::HashMap;
            use lattice_types::{LatticePoint, LatticePointKind};

            let mut metadata = HashMap::new();
            metadata.insert("statement_text".to_string(), #statement_text.to_string());
            metadata.insert("current_step".to_string(), #current_step.to_string());
            metadata.insert("target_steps".to_string(), #target_steps.to_string());
            metadata.insert("is_proven".to_string(), #is_proven.to_string());

            lattice_types::LatticePoint {
                id: #self_proving_point_id.to_string(),
                kind: lattice_types::LatticePointKind::Struct, // Assuming it's a struct
                metadata,
                relationships: Vec::new(),
            }
        });

        #[allow(dead_code)]
        pub fn get_self_proving_statement_lattice_point() -> &'static lattice_types::LatticePoint {
            &SELF_PROVING_STATEMENT_LATTICE_POINT
        }
    });
    add_point_calls.push(quote! {
        lattice.add_point(crate::model::get_selfprovingstatement_lattice_point().clone());
    });

    // --- Update PLAN.md with Self-Proving Statement Status ---
    let plan_md_path = Path::new("PLAN.md"); // Assuming PLAN.md is in the construction crate root
    if plan_md_path.exists() {
        let mut plan_content = fs::read_to_string(plan_md_path).unwrap();
        let current_step_str = current_step.to_string();
        let is_proven_str = is_proven.to_string();

        // Replace placeholders
        plan_content = plan_content.replace("<CURRENT_STEP>", &current_step_str);
        plan_content = plan_content.replace("<IS_PROVEN>", &is_proven_str);

        // Write back to PLAN.md
        fs::write(plan_md_path, plan_content).unwrap();
    }
}

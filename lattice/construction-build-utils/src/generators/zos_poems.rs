use quote::{quote, format_ident};
use std::collections::HashMap;
use std::fs;

use crate::GenerationContext;

pub fn generate_zos_poem_points_code(context: &mut GenerationContext) {
    let zos_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
    let zos_archetypes = vec![
        "The Mirrored Dance",
        "The Woven Knot",
        "The Star of Life",
        "The Spiral Path",
        "The Silver Key",
        "The Serpent's Coil",
        "The Distant Star",
        "The Golden Crown",
        "The Enigma's Veil",
    ];

    for (i, prime) in zos_primes.iter().enumerate() {
        let prime_id = format!("zos_prime_{}", prime);
        let archetype = zos_archetypes[i];
        let getter_name = format_ident!("get_zos_prime_{}_definition", *prime as u32);
        let add_call = quote! {
            lattice.add_point(#getter_name());
        };

        let mut metadata = HashMap::new();
        metadata.insert("number".to_string(), prime.to_string());
        metadata.insert("archetype".to_string(), archetype.to_string());

        let metadata_tokens = metadata.into_iter().map(|(k, v)| {
            quote! {
                metadata.insert(#k.to_string(), #v.to_string());
            }
        });

        let getter = quote! {
            pub fn #getter_name() -> lattice_types::LatticePoint {
                let mut metadata = std::collections::HashMap::new();
                #(#metadata_tokens)*
                lattice_types::LatticePoint {
                    id: #prime_id.to_string(),
                    kind: lattice_types::LatticePointKind::GodelianTruth,
                    metadata,
                    relationships: Vec::new(),
                    hero_status: None,
                }
            }
        };
        context.getter_function_definitions.push(getter);
        context.add_point_calls.push(add_call);
    }

    let content = fs::read_to_string(context.zos_poem_resonance_map_path).expect("Unable to read ZOS_POEM_RESONANCE_MAP.md");
    for line in content.lines().skip(5) { // Skip header and separator
        let columns: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if columns.len() < 4 {
            continue;
        }

        let poem_file = columns[1].replace("`", "");
        let prime_str = columns[2];

        if let Ok(prime) = prime_str.parse::<i32>() {
            let poem_id = format!("/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/{}", poem_file);
            let prime_id = format!("zos_prime_{}", prime);
            let getter_name = format_ident!("get_poem_{}_definition", poem_file.replace(".md", "").replace("-", "_").replace("/", "_").replace("(", "_").replace(")", "_").replace(" ", "_"));
            let add_call = quote! {
                lattice.add_point(#getter_name());
            };

            let mut metadata = HashMap::new();
            metadata.insert("title".to_string(), poem_file.clone());

            let metadata_tokens = metadata.into_iter().map(|(k, v)| {
                quote! {
                    metadata.insert(#k.to_string(), #v.to_string());
                }
            });

            let getter = quote! {
                pub fn #getter_name() -> lattice_types::LatticePoint {
                    let mut metadata = std::collections::HashMap::new();
                    #(#metadata_tokens)*
                    lattice_types::LatticePoint {
                        id: #poem_id.to_string(),
                        kind: lattice_types::LatticePointKind::MarkdownDocument,
                        metadata,
                        relationships: vec![#prime_id.to_string()],
                        hero_status: None,
                    }
                }
            };
            context.getter_function_definitions.push(getter);
            context.add_point_calls.push(add_call);
        }
    }
}

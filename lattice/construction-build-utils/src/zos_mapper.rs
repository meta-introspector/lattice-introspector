use std::collections::HashMap;
use lattice_types::{LatticePoint, LatticePointKind};
use serde_json;
use std::path::PathBuf;

// Define Zos prime keywords and concepts
use quote::{quote, format_ident};
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

    // Generate ZosPoemElement points
    let zos_poem_elements = vec![
        (0, "The Seed of Naught: A circle drawn, a breath held deep, Where all that is, lies fast asleep."),
        (1, "The Lonely Pillar: A single line, against the grey, The first to stand, to light the way."),
        (2, "The Mirrored Dance: Two points ignite, a vibrant spark, A mirrored dance, within the dark."),
        (3, "The Woven Knot: Three threads combine, a woven knot, The pattern set, the future caught."),
        (5, "The Star of Life: Five fingers spread, a human hand, A five-pointed star, across the land."),
        (7, "The Spiral Path: Seven notes ascend, a winding stair, A spiral path, beyond compare."),
        (11, "The Silver Key: Eleven gates, a whispered word, A truth unseen, a song unheard."),
        (13, "The Serpent's Coil: Thirteen moons, a serpent's coil, A shedding skin, on sacred soil."),
        (17, "The Distant Star: Seventeen lights, a distant fire, A burning hope, a heart's desire."),
        (19, "The Golden Crown: Nineteen years, the sun and moon, A cosmic dance, a silent tune."),
        (23, "The Enigma's Veil: Twenty-three, the number's art, A hidden message, set apart."),
    ];

    for (number, vibe_description) in zos_poem_elements {
        let element_id = format!("zos_element_{}", number);

        let mut metadata_map = HashMap::new(); // Use a different name to avoid conflict
        metadata_map.insert("number".to_string(), number.to_string());
        metadata_map.insert("vibe_description".to_string(), vibe_description.to_string());

        let metadata_tokens = metadata_map.into_iter().map(|(k, v)| {
            quote! {
                metadata.insert(#k.to_string(), #v.to_string());
            }
        }).collect::<Vec<_>>(); // Collect into a Vec<TokenStream>

        let relationships_vec = vec![format!("zos_prime_{}", number)]; // Use a different name
        let relationships_tokens = relationships_vec.iter().map(|r| {
            quote! { #r.to_string() }
        }).collect::<Vec<_>>(); // Collect into a Vec<TokenStream>

        let add_call = quote! {
            lattice.add_point(lattice_types::LatticePoint {
                id: #element_id.to_string(),
                kind: lattice_types::LatticePointKind::ZosPoemElement,
                metadata: {
                    let mut metadata = std::collections::HashMap::new();
                    #(#metadata_tokens)*
                    metadata
                },
                relationships: vec![#(#relationships_tokens),*],
                hero_status: None,
            });
        };
        context.add_point_calls.push(add_call);
    }

    println!("DEBUG: zos_poems generated {} add_point_calls", context.add_point_calls.len());

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

// Function to map a source file to a Zos prime
pub fn map_source_file_to_zos_prime(
    file_content: &str,
    file_path: &str,
    zos_prime_keywords: &HashMap<i32, Vec<&'static str>>,
) -> Option<(i32, f64)> {
    let mut best_prime = None;
    let mut highest_score = 0.0;

    let lower_case_content = file_content.to_lowercase();

    for (&prime, keywords) in zos_prime_keywords.iter() {
        let mut current_score = 0.0;
        for keyword in keywords {
            if lower_case_content.contains(keyword) {
                current_score += 1.0;
            }
        }

        // Simple scoring: more matches means higher score
        // Could be improved with TF-IDF, semantic analysis, etc.
        if current_score > highest_score {
            highest_score = current_score;
            best_prime = Some(prime);
        }
    }

    if highest_score > 0.0 { // Only return a mapping if there's at least one match
        best_prime.map(|p| (p, highest_score))
    } else {
        None
    }
}

// Function to create a LatticePoint for a source file
pub fn create_source_file_lattice_point(
    file_path: &str,
    assigned_prime: Option<(i32, f64)>,
) -> (String, String, HashMap<String, String>, Vec<String>) {
    let mut metadata = HashMap::new();
    metadata.insert("file_path".to_string(), file_path.to_string());

    let mut relationships = Vec::new();

    if let Some((prime, score)) = assigned_prime {
        metadata.insert("zos_prime".to_string(), prime.to_string());
        metadata.insert("zos_resonance_score".to_string(), score.to_string());
        relationships.push(format!("zos_prime_{}", prime));
    }

    let id = format!("source_file_{}", file_path.replace("/", "_").replace(".", "_").replace("-", "_"));
    let kind_str = "RustFile".to_string();

    (id, kind_str, metadata, relationships)
}
pub fn get_zos_prime_keywords() -> HashMap<i32, Vec<&'static str>> {
    let mut keywords = HashMap::new();
    keywords.insert(0, vec!["seed", "naught", "circle", "breath"]);
    keywords.insert(1, vec!["lonely", "pillar", "single", "line"]);
    keywords.insert(2, vec!["mirrored", "dance", "two", "spark"]);
    keywords.insert(3, vec!["woven", "knot", "three", "threads"]);
    keywords.insert(5, vec!["star", "life", "five", "fingers"]);
    keywords.insert(7, vec!["spiral", "path", "seven", "notes"]);
    keywords.insert(11, vec!["silver", "key", "eleven", "gates"]);
    keywords.insert(13, vec!["serpent", "coil", "thirteen", "moons"]);
    keywords.insert(17, vec!["distant", "star", "seventeen", "lights"]);
    keywords.insert(19, vec!["golden", "crown", "nineteen", "sun", "moon"]);
    keywords.insert(23, vec!["enigma", "veil", "twenty-three", "art"]);
    keywords
}

pub fn generate_prime_resonance_points_code(context: &mut GenerationContext) {
    let zos_primes_data = [
        (0, vec!["seed", "naught", "circle", "breath"], vec!["zero"]),
        (1, vec!["lonely", "pillar", "single", "line"], vec!["one"]),
        (2, vec!["mirrored", "dance", "two", "spark"], vec!["two"]),
        (3, vec!["woven", "knot", "three", "threads"], vec!["three"]),
        (5, vec!["star", "life", "five", "fingers"], vec!["five"]),
        (7, vec!["spiral", "path", "seven", "notes"], vec!["seven"]),
        (11, vec!["silver", "key", "eleven", "gates"], vec!["eleven"]),
        (13, vec!["serpent", "coil", "thirteen", "moons"], vec!["thirteen"]),
        (17, vec!["distant", "star", "seventeen", "lights"], vec!["seventeen"]),
        (19, vec!["golden", "crown", "nineteen", "sun", "moon"], vec!["nineteen"]),
        (23, vec!["enigma", "veil", "twenty-three", "art"], vec!["twenty-three"]),
    ];

    for (prime, keywords, text_forms) in zos_primes_data.iter() {
        let prime_id = format!("zos_prime_{}", prime);
        let prime_resonance_id = format!("prime_resonance_{}", prime);
        let cached_results_path = format!(".gemini/prime_resonances/prime_resonance_{}.json", prime);

        // Create PrimeResonance LatticePoint
                let prime_metadata_tokens = prime_metadata.clone().into_iter().map(|(k, v)| {
            quote! {
                metadata.insert(#k.to_string(), #v.to_string());
            }
        }).collect::<Vec<_>>(); // Collect into a Vec<TokenStream>

        let prime_resonance_add_call = quote! {
            lattice.add_point(lattice_types::LatticePoint {
                id: #prime_resonance_id.to_string(),
                kind: lattice_types::LatticePointKind::PrimeResonance,
                metadata: {
                    let mut metadata = std::collections::HashMap::new();
                    #(#prime_metadata_tokens)* // Use the collected TokenStream
                    metadata
                },
                relationships: vec![#prime_id.to_string()],
                hero_status: None,
            });
        };
        context.add_point_calls.push(prime_resonance_add_call);

        // Read and parse cached results for WordResonance points
        let full_cached_path = PathBuf::from(context.project_root).join(&cached_results_path);
        if let Ok(content) = fs::read_to_string(&full_cached_path) {
            if let Ok(results) = serde_json::from_str::<Vec<HashMap<String, String>>>(&content) {
                for result in results {
                    if let (Some(file_path), Some(line_number), Some(matched_content)) = (
                        result.get("file_path"),
                        result.get("line_number"),
                        result.get("matched_content"),
                    ) {
                        let word_id = format!("word_resonance_{}_{}_{}", prime, file_path.replace("/", "_").replace(".", "_").replace("-", "_"), line_number);
                        let mut word_metadata = HashMap::new();
                        word_metadata.insert("prime_number".to_string(), prime.to_string());
                        word_metadata.insert("file_path".to_string(), file_path.clone());
                        word_metadata.insert("line_number".to_string(), line_number.clone());
                        word_metadata.insert("matched_content".to_string(), matched_content.clone());

                        let word_metadata_tokens = word_metadata.clone().into_iter().map(|(k, v)| { // Clone here
                            quote! {
                                metadata.insert(#k.to_string(), #v.to_string());
                            }
                        }).collect::<Vec<_>>(); // Collect into a Vec<TokenStream>

                        let word_resonance_add_call = quote! {
                            lattice.add_point(lattice_types::LatticePoint {
                                id: #word_id.to_string(),
                                kind: lattice_types::LatticePointKind::WordResonance,
                                metadata: {
                                    let mut metadata = std::collections::HashMap::new();
                                    #(#word_metadata_tokens)* // Use the collected TokenStream
                                    metadata
                                },
                                relationships: vec![#prime_resonance_id.to_string()],
                                hero_status: None,
                            });
                        };
                        context.add_point_calls.push(word_resonance_add_call);
                    }
                }
            }
        }
    }
}
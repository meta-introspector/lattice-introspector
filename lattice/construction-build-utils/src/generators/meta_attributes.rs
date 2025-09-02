use quote::{quote, format_ident};
use proc_macro2::TokenStream;
use std::fs;
use std::path::Path;

pub fn generate_meta_attributes_code(
    getter_function_definitions: &mut Vec<TokenStream>,
    add_point_calls: &mut Vec<TokenStream>,
    lattice_poem_mapping_path: &Path,
) {
    let mut emoji_vibes_tokens = Vec::new();

    if let Ok(content) = fs::read_to_string(lattice_poem_mapping_path) {
        let mut lines = content.lines().peekable();

        while let Some(line) = lines.next() {
            if line.trim().starts_with("- **Emoji Vector:**") {
                // Consume this line and look for the next lines which are the emoji details
                while let Some(emoji_line) = lines.peek() {
                    if emoji_line.trim().starts_with("- ") && emoji_line.contains("(Prime:") {
                        let emoji_line = lines.next().unwrap(); // Consume the line

                        // Example: - 🧠 (Prime: 599, Vibe: Cognition, Understanding)
                        let parts: Vec<&str> = emoji_line.splitn(2, '(').collect();
                        if parts.len() < 2 { continue; }

                        let emoji_char = parts[0].trim().trim_start_matches('-').trim().to_string();
                        let rest = parts[1].trim();

                        let prime_start = rest.find("Prime:").map(|i| i + "Prime:".len()).unwrap_or(0);
                        let prime_end = rest[prime_start..].find(',').map(|i| prime_start + i).unwrap_or(rest.len());
                        let prime_str = rest[prime_start..prime_end].trim();
                        let prime: u64 = prime_str.parse().unwrap_or(0);

                        let vibe_start = rest.find("Vibe:").map(|i| i + "Vibe:".len()).unwrap_or(0);
                        let vibe_end = rest[vibe_start..].find(')').map(|i| vibe_start + i).unwrap_or(rest.len());
                        let vibe_str = rest[vibe_start..vibe_end].trim().to_string();

                        emoji_vibes_tokens.push(quote! {
                            crate::meta_attributes::EmojiVibe {
                                emoji: #emoji_char.to_string(),
                                prime: #prime,
                                vibe: #vibe_str.to_string(),
                            }
                        });
                    } else {
                        break; // Not an emoji line, break inner loop
                    }
                }
            }
        }
    }

    let static_name = format_ident!("ALL_EMOJI_VIBES");
    let get_fn_name = format_ident!("get_all_emoji_vibes");

    getter_function_definitions.push(quote! {
        #[allow(dead_code)]
        pub static #static_name: once_cell::sync::Lazy<Vec<crate::meta_attributes::EmojiVibe>> = once_cell::sync::Lazy::new(|| {
            vec![
                #(#emoji_vibes_tokens),*
            ]
        });

        #[allow(dead_code)]
        pub fn #get_fn_name() -> &'static Vec<crate::meta_attributes::EmojiVibe> {
            &#static_name
        }
    });

    // No add_point_calls for this generator, as it's generating static data, not LatticePoints directly
}

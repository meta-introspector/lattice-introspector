use std::{fs, path::Path};
use syn::{Item, Ident};
use quote::{quote, format_ident};

use crate::utils::get_module_prefix_for_ident;
use crate::GenerationContext;

pub fn generate_model_types_code(context: &mut GenerationContext) {
    let model_types_dir = Path::new("src/model_types");
    for entry in fs::read_dir(model_types_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(&path).unwrap();
            let syntax = syn::parse_file(&content).unwrap();

            for item in syntax.items {
                let (item_ident, item_attrs) = match &item {
                    syn::Item::Struct(s) => (Some(&s.ident), &s.attrs),
                    syn::Item::Enum(e) => (Some(&e.ident), &e.attrs),
                    _ => (None, &vec![]),
                };

                if let Some(ident) = item_ident {
                    let has_derive_lattice_point = item_attrs.iter().any(|attr| {
                        if attr.path().is_ident("derive") {
                            let mut found = false;
                            attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("LatticePointDerive") {
                                    found = true;
                                }
                                Ok(())
                            }).ok();
                            found
                        } else {
                            false
                        }
                    });

                    if has_derive_lattice_point {
                        let get_fn_name = format_ident!("get_{}_lattice_point", ident.to_string().to_lowercase());
                        let module_prefix = get_module_prefix_for_ident(ident);
                        context.add_point_calls.push(quote! {
                            lattice.add_point(#module_prefix #get_fn_name().clone());
                        });
                    }
                }
            }
        }
    }
}

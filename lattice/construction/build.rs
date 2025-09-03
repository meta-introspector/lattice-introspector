use std::{env, fs, io::Write, path::Path};
use construction_build_utils::{generate_lattice_registration_code, introspectors, zos_mapper};
use lattice_types::LatticePoint;
use chrono::Utc;
use quote::quote;

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=PLAN.md");
    println!("cargo:rerun-if-changed=README.md");
    println!("cargo:rerun-if-changed=src/model_types"); // Rerun if model_types changes
    println!("cargo:rerun-if-changed=../ZOS_POEM_RESONANCE_MAP.md");
    println!("cargo:rerun-if-changed=../../../ZOS_AXIOM_POEM.md"); // Rerun if Zos Axiom poem changes
    println!("cargo:rerun-if-changed=../../../ZOS_ELEMENTS_POEMS.md"); // Rerun if Zos Elements poem changes

    let out_dir = env::var("OUT_DIR").unwrap();

    // Generate generated_lattice_registration.rs
    let destination_path = Path::new(&out_dir).join("generated_lattice_registration.rs");
    let mut f = fs::File::create(&destination_path).unwrap();

    let src_dir = Path::new("src"); // This still points to the main src directory
    let markdown_paths = vec![
        Path::new("PLAN.md"),
        Path::new("../README.md"), // This is the project root README.md
        Path::new("../../../OODA_LOOP_SOP.md"),
        Path::new("../../../ZOS_AXIOM_POEM.md"),
    ];

    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap(); // Go up from OUT_DIR to target/debug or target/release
    let binary_path = target_dir.join(format!("lattice-macros-test{}", env::consts::EXE_SUFFIX));
    let binary_point = introspectors::introspect_binary(&binary_path);

    // Create a predicted execution point
    let mut predicted_execution_metadata = std::collections::HashMap::new();
    predicted_execution_metadata.insert("binary_path".to_string(), binary_path.to_string_lossy().to_string());
    predicted_execution_metadata.insert("expected_args".to_string(), "".to_string()); // No specific args expected for now
    predicted_execution_metadata.insert("prediction_timestamp".to_string(), Utc::now().to_rfc3339());

    let predicted_execution_point = LatticePoint {
        id: "predicted_lattice_macros_test_execution".to_string(),
        kind: lattice_types::LatticePointKind::PredictedExecution,
        metadata: predicted_execution_metadata,
        relationships: vec![binary_point.id.clone()], // Relate to the binary point
        hero_status: None,
    };

    // Collect all .rs files in the lattice directory
    let mut rust_source_files = Vec::new();
    for entry in walkdir::WalkDir::new("..")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        rust_source_files.push(entry.path().to_path_buf());
    }

    let zos_prime_keywords = zos_mapper::get_zos_prime_keywords();
    let mut source_file_points = Vec::new();

    for file_path_buf in rust_source_files {
        let file_path_str = file_path_buf.to_string_lossy().to_string();
        let file_content = fs::read_to_string(&file_path_buf).unwrap_or_default();

        let assigned_prime = zos_mapper::map_source_file_to_zos_prime(
            &file_content,
            &file_path_str,
            &zos_prime_keywords,
        );

        let source_file_point = zos_mapper::create_source_file_lattice_point(
            &file_path_str,
            assigned_prime,
        );
        source_file_points.push(source_file_point);
    }

    let project_root = env::current_dir().unwrap();
    let generated_code = generate_lattice_registration_code(src_dir, &markdown_paths, Some(binary_point), Some(predicted_execution_point), &project_root);

    // Append source file points to the generated code
    let mut source_file_point_add_calls = Vec::new();
    for (id, kind_str, metadata_map, relationships_vec) in source_file_points {
        let kind = match kind_str.as_str() {
            "Struct" => quote! { lattice_types::LatticePointKind::Struct },
            "Enum" => quote! { lattice_types::LatticePointKind::Enum },
            "Function" => quote! { lattice_types::LatticePointKind::Function },
            "MemoryRegion" => quote! { lattice_types::LatticePointKind::MemoryRegion },
            "Instruction" => quote! { lattice_types::LatticePointKind::Instruction },
            "CompileTimeEvent" => quote! { lattice_types::LatticePointKind::CompileTimeEvent },
            "RunTimeEvent" => quote! { lattice_types::LatticePointKind::RunTimeEvent },
            "TraceEvent" => quote! { lattice_types::LatticePointKind::TraceEvent },
            "LatticeMeta" => quote! { lattice_types::LatticePointKind::LatticeMeta },
            "MarkdownDocument" => quote! { lattice_types::LatticePointKind::MarkdownDocument },
            "PredictedExecution" => quote! { lattice_types::LatticePointKind::PredictedExecution },
            "ActualExecution" => quote! { lattice_types::LatticePointKind::ActualExecution },
            "LogEvent" => quote! { lattice_types::LatticePointKind::LogEvent },
            "GeminiAgent" => quote! { lattice_types::LatticePointKind::GeminiAgent },
            "OllamaAgent" => quote! { lattice_types::LatticePointKind::OllamaAgent },
            "GGUFModel" => quote! { lattice_types::LatticePointKind::GGUFModel },
            "HuggingFaceDataset" => quote! { lattice_types::LatticePointKind::HuggingFaceDataset },
            "GitHubRepository" => quote! { lattice_types::LatticePointKind::GitHubRepository },
            "GitHubAccount" => quote! { lattice_types::LatticePointKind::GitHubAccount },
            "GitCommit" => quote! { lattice_types::LatticePointKind::GitCommit },
            "PullRequest" => quote! { lattice_types::LatticePointKind::PullRequest },
            "GitHubActionRun" => quote! { lattice_types::LatticePointKind::GitHubActionRun },
            "GitDerivedAsset" => quote! { lattice_types::LatticePointKind::GitDerivedAsset },
            "UserIntent" => quote! { lattice_types::LatticePointKind::UserIntent },
            "Transformation" => quote! { lattice_types::LatticePointKind::Transformation },
            "CompilerTransformation" => quote! { lattice_types::LatticePointKind::CompilerTransformation },
            "GodelianTruth" => quote! { lattice_types::LatticePointKind::GodelianTruth },
            "AcademicPaper" => quote! { lattice_types::LatticePointKind::AcademicPaper },
            "AcademicAuthor" => quote! { lattice_types::LatticePointKind::AcademicAuthor },
            "RustFile" => quote! { lattice_types::LatticePointKind::RustFile },
            _ => panic!("Unknown LatticePointKind: {}", kind_str),
        };

        let metadata = metadata_map.iter().map(|(k, v)| {
            quote! { metadata.insert(#k.to_string(), #v.to_string()); }
        });
        let relationships = relationships_vec.iter().map(|r| {
            quote! { #r.to_string() }
        });

        source_file_point_add_calls.push(quote! {
            {
                let mut metadata = std::collections::HashMap::new();
                #(#metadata)*
                let relationships = vec![#(#relationships),*];
                lattice.add_point(lattice_types::LatticePoint {
                    id: #id.to_string(),
                    kind: #kind,
                    metadata,
                    relationships,
                    hero_status: None,
                });
            }
        });
    }

    println!("Generated registration file: {}", destination_path.display());
    f.write_all(generated_code.to_string().as_bytes()).unwrap();

    // Copy the generated file to a location accessible by the agent
    let agent_accessible_path = Path::new("../../.gemini/generated_lattice_registration.rs");
    fs::create_dir_all(&agent_accessible_path.parent().unwrap()).expect("Failed to create .gemini directory");
    fs::copy(&destination_path, &agent_accessible_path).expect("Failed to copy generated_lattice_registration.rs");
    println!("Copied generated registration file to: {}", agent_accessible_path.display());
}

    


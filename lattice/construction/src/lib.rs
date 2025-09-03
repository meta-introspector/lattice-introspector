use chrono;
use serde;
use lattice_types::LatticeAccess;

pub mod vibe;
pub mod model_types; // New module for model types
pub mod compilation;
pub mod execution;
pub mod compiler_ir;
pub mod vm_context;
pub mod meta_attributes;
pub mod lattice;
pub mod zos_poem;

// Include the generated registration file
include!(concat!(env!("OUT_DIR"), "/generated_lattice_registration.rs"));

// Public function to register all lattice points
pub fn register_all_points(lattice: &mut impl lattice_types::LatticeAccess) {
    register_all_lattice_points(lattice);
}

// Import the generated getter functions for LatticePoints
use crate::model_types::cargo_crate::get_cargocrate_lattice_point;
use crate::model_types::compiler_transformation::get_compilertransformation_lattice_point;
use crate::model_types::ffi_binding::get_ffibinding_lattice_point;
use crate::model_types::gemini_agent::get_geminiagent_lattice_point;
use crate::model_types::gguf_model::get_ggufmodel_lattice_point;
use crate::model_types::git_commit::get_gitcommit_lattice_point;
use crate::model_types::git_derived_asset::get_gitderivedasset_lattice_point;
use crate::model_types::git_submodule::get_gitsubmodule_lattice_point;
use crate::model_types::github_account::get_githubaccount_lattice_point;
use crate::model_types::github_action_run::get_githubactionrun_lattice_point;
use crate::model_types::github_repository::get_githubrepository_lattice_point;
use crate::model_types::godelian_truth::get_godeliantruth_lattice_point;
use crate::model_types::hugging_face_dataset::get_huggingfacedataset_lattice_point;
use crate::model_types::markdown_document::get_markdowndocument_lattice_point;
use crate::model_types::ollama_agent::get_ollamaagent_lattice_point;
use crate::model_types::pull_request::get_pullrequest_lattice_point;
use crate::model_types::repository::get_repository_lattice_point;
use crate::model_types::rust_file::get_rustfile_lattice_point;
use crate::model_types::self_proving_statement::get_selfprovingstatement_lattice_point;
use crate::model_types::transformation::get_transformation_lattice_point;
use crate::model_types::user_intent::get_user_intent_lattice_point;

pub fn run_poem_conversion() {
    zos_poem::print_zos_poem();
}

use lattice_types::{LatticePoint, LatticePointKind};
use std::collections::HashMap;

pub fn get_user_intent_lattice_point() -> LatticePoint {
    let mut metadata = HashMap::new();
    metadata.insert("keywords".to_string(), "modularity, self-awareness, verifiable truth, meta-reflection, AI, human-AI collaboration".to_string());
    metadata.insert("emotional_tone".to_string(), "ambitious, philosophical, iterative, evolving".to_string());
    metadata.insert("goals".to_string(), "context reduction, fixed point convergence, computational meaning".to_string());

    LatticePoint {
        id: "user_intent_project_vibe".to_string(),
        kind: LatticePointKind::UserIntent,
        metadata,
        relationships: vec![
            "lattice_meta".to_string(),
            "self_proving_statement".to_string(),
            "gemini_agent_v2_5_flash".to_string(),
        ],
        hero_status: None,
    }
}

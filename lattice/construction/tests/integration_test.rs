use construction::register_all_points;
use construction::lattice::Lattice;

#[test]
fn test_all_points_registered() {
    let mut lattice = Lattice {
        points: std::collections::HashMap::new(),
        meta_point_id: String::new(), // Placeholder, as it's set by the global lattice
    };

    register_all_points(&mut lattice);

    // Assert that some expected points are registered
    assert!(lattice.get_point("struct_Repository").is_some());
    assert!(lattice.get_point("markdown_document_PLAN_md").is_some());
    assert!(lattice.get_point("markdown_document_README_md").is_some());
    assert!(lattice.get_point("struct_RustcInvocation").is_some());
    assert!(lattice.get_point("struct_CompilerMemoryLocation").is_some());
    assert!(lattice.get_point("struct_Instruction").is_some());
    assert!(lattice.get_point("enum_AccessType").is_some());
    assert!(lattice.get_point("struct_Lattice").is_some());
    assert!(lattice.get_point("struct_Registers").is_some());

    // You can add more specific assertions here, e.g., checking metadata
    let repo_point = lattice.get_point("struct_Repository").unwrap();
    assert_eq!(repo_point.metadata.get("name").unwrap(), "Repository");
}
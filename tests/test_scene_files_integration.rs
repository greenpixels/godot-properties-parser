use godot_properties_parser::{parse_property_file, parse_scene_file};

/// Test that simple_header.tscn parses successfully
#[test]
fn test_parse_simple_header_tscn() {
    let content = include_str!("scenes/simple_header.tscn");
    let result = parse_property_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse simple_header.tscn: {:?}",
        result.err()
    );

    let (remaining, file) = result.unwrap();
    assert_eq!(remaining, "", "Parser did not consume entire file");
    assert_eq!(file.sections.len(), 1, "Expected 1 section");
    assert_eq!(file.sections[0].header_type, "gd_scene");
}

/// Test that simple_header.tscn parses as a scene file
#[test]
fn test_parse_simple_header_scene() {
    let content = include_str!("scenes/simple_header.tscn");
    let result = parse_scene_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse simple_header.tscn as scene: {:?}",
        result.err()
    );

    let (remaining, scene) = result.unwrap();
    assert_eq!(remaining, "", "Parser did not consume entire file");
    assert!(scene.header.is_some(), "Scene should have a header");
    assert_eq!(scene.all_sections.len(), 1);
}

/// Test that shop_sidebar.tscn parses successfully
#[test]
fn test_parse_shop_sidebar_tscn() {
    let content = include_str!("scenes/shop_sidebar.tscn");
    let result = parse_property_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse shop_sidebar.tscn: {:?}",
        result.err()
    );

    let (remaining, file) = result.unwrap();
    assert_eq!(remaining, "", "Parser did not consume entire file");
    assert!(!file.sections.is_empty(), "File should have sections");
}

/// Test that shop_sidebar.tscn has expected structure
#[test]
fn test_shop_sidebar_structure() {
    let content = include_str!("scenes/shop_sidebar.tscn");
    let result = parse_scene_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse shop_sidebar.tscn as scene: {:?}",
        result.err()
    );

    let (_, scene) = result.unwrap();
    assert!(scene.header.is_some(), "Scene should have a header");

    // Shop sidebar should have external resources and nodes
    assert!(
        !scene.ext_resources.is_empty() || !scene.nodes.is_empty(),
        "Scene should have external resources or nodes"
    );
}

/// Test that table_selection_entry.tscn parses successfully
#[test]
fn test_parse_table_selection_entry_tscn() {
    let content = include_str!("scenes/table_selection_entry.tscn");
    let result = parse_property_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse table_selection_entry.tscn: {:?}",
        result.err()
    );

    let (remaining, file) = result.unwrap();
    assert_eq!(remaining, "", "Parser did not consume entire file");
    assert!(!file.sections.is_empty(), "File should have sections");
}

/// Test that table_selection_entry.tscn has expected structure
#[test]
fn test_table_selection_entry_structure() {
    let content = include_str!("scenes/table_selection_entry.tscn");
    let result = parse_scene_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse table_selection_entry.tscn as scene: {:?}",
        result.err()
    );

    let (_, scene) = result.unwrap();
    assert!(scene.header.is_some(), "Scene should have a header");
}

/// Test that play_room.tscn parses successfully (complex file)
#[test]
fn test_parse_play_room_tscn() {
    let content = include_str!("scenes/play_room.tscn");
    let result = parse_property_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse play_room.tscn: {:?}",
        result.err()
    );

    let (remaining, file) = result.unwrap();
    assert_eq!(remaining, "", "Parser did not consume entire file");
    assert!(
        file.sections.len() > 50,
        "play_room.tscn should have many sections"
    );
}

/// Test that play_room.tscn has expected structure (complex file)
#[test]
fn test_play_room_structure() {
    let content = include_str!("scenes/play_room.tscn");
    let result = parse_scene_file(content);

    assert!(
        result.is_ok(),
        "Failed to parse play_room.tscn as scene: {:?}",
        result.err()
    );

    let (_, scene) = result.unwrap();

    // Verify header
    assert!(scene.header.is_some(), "Scene should have a header");
    let header = scene.header.as_ref().unwrap();
    assert_eq!(header.header_type, "gd_scene");

    // Verify categorized sections
    assert!(
        !scene.ext_resources.is_empty(),
        "Should have external resources"
    );
    assert!(!scene.sub_resources.is_empty(), "Should have sub-resources");
    assert!(!scene.nodes.is_empty(), "Should have nodes");
    assert!(!scene.connections.is_empty(), "Should have connections");

    // Verify totals
    assert_eq!(scene.all_sections.len(), 99, "Expected 99 total sections");
    assert_eq!(
        scene.ext_resources.len(),
        31,
        "Expected 31 external resources"
    );
    assert_eq!(scene.sub_resources.len(), 14, "Expected 14 sub-resources");
    assert_eq!(scene.nodes.len(), 41, "Expected 41 nodes");
    assert_eq!(scene.connections.len(), 12, "Expected 12 connections");
}

/// Test all scene files can be parsed without errors
#[test]
fn test_all_scene_files_parse() {
    let scene_files = [
        (
            "simple_header.tscn",
            include_str!("scenes/simple_header.tscn"),
        ),
        (
            "shop_sidebar.tscn",
            include_str!("scenes/shop_sidebar.tscn"),
        ),
        (
            "table_selection_entry.tscn",
            include_str!("scenes/table_selection_entry.tscn"),
        ),
        ("play_room.tscn", include_str!("scenes/play_room.tscn")),
    ];

    for (name, content) in &scene_files {
        let result = parse_property_file(content);
        assert!(
            result.is_ok(),
            "Failed to parse {}: {:?}",
            name,
            result.err()
        );

        let (remaining, _) = result.unwrap();
        assert_eq!(
            remaining, "",
            "Parser did not consume entire file for {}",
            name
        );
    }
}

/// Verify that all parsed sections have valid header types
#[test]
fn test_all_sections_have_valid_headers() {
    let content = include_str!("scenes/play_room.tscn");
    let (_, file) = parse_property_file(content).unwrap();

    for section in &file.sections {
        assert!(
            !section.header_type.is_empty(),
            "Section header_type should not be empty"
        );
        assert!(
            section
                .header_type
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_'),
            "Section header_type '{}' contains invalid characters",
            section.header_type
        );
    }
}

/// Verify that nodes have required properties
#[test]
fn test_nodes_have_name_property() {
    let content = include_str!("scenes/play_room.tscn");
    let (_, scene) = parse_scene_file(content).unwrap();

    for node in &scene.nodes {
        let has_name = node.properties.iter().any(|p| p.key == "name");
        assert!(has_name, "Node section should have 'name' property");
    }
}

/// Verify that external resources have required properties
#[test]
fn test_ext_resources_have_type_property() {
    let content = include_str!("scenes/play_room.tscn");
    let (_, scene) = parse_scene_file(content).unwrap();

    for ext_res in &scene.ext_resources {
        let has_type = ext_res.properties.iter().any(|p| p.key == "type");
        assert!(has_type, "External resource should have 'type' property");
    }
}

/// Verify that sub-resources have id property
#[test]
fn test_sub_resources_have_id_property() {
    let content = include_str!("scenes/play_room.tscn");
    let (_, scene) = parse_scene_file(content).unwrap();

    for sub_res in &scene.sub_resources {
        let has_id = sub_res.properties.iter().any(|p| p.key == "id");
        assert!(has_id, "Sub-resource should have 'id' property");
    }
}

/// Verify that connections have required signal properties
#[test]
fn test_connections_have_signal_property() {
    let content = include_str!("scenes/play_room.tscn");
    let (_, scene) = parse_scene_file(content).unwrap();

    for connection in &scene.connections {
        let has_signal = connection.properties.iter().any(|p| p.key == "signal");
        assert!(has_signal, "Connection should have 'signal' property");
    }
}

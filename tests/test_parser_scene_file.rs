use godot_properties_parser::parse_scene_file;
use godot_properties_parser::parsers::parser_property_file::{PropertyFile, Section};

#[test]
fn test_parse_scene_file_basic() {
    let input = r#"[gd_scene load_steps=46 format=3 uid="uid://test"]

[ext_resource type="Script" path="res://test.gd"]
[node name="Node" type="Node2D"]
[connection signal="pressed" from="Button" to="." method="_on_button_pressed"]
"#;
    let (remaining, scene) = parse_scene_file(input).unwrap();
    assert_eq!(remaining, "");

    // Check header
    assert!(scene.header.is_some());
    assert_eq!(scene.header.as_ref().unwrap().header_type, "gd_scene");

    // Check categorized sections
    assert_eq!(scene.ext_resources.len(), 1);
    assert_eq!(scene.nodes.len(), 1);
    assert_eq!(scene.connections.len(), 1);

    // Check all_sections contains everything
    assert_eq!(scene.all_sections.len(), 4);
}

#[test]
fn test_scene_file_multiple_resources() {
    let input = r#"[gd_scene load_steps=3 format=3]

[ext_resource type="Script" path="res://script1.gd"]
[ext_resource type="Texture2D" path="res://icon.png"]
[sub_resource type="AtlasTexture" id="AtlasTexture_123"]
[sub_resource type="Gradient" id="Gradient_456"]
[node name="Root" type="Node2D"]
[node name="Sprite" type="Sprite2D" parent="."]
"#;
    let (_, scene) = parse_scene_file(input).unwrap();

    assert_eq!(scene.ext_resources.len(), 2);
    assert_eq!(scene.sub_resources.len(), 2);
    assert_eq!(scene.nodes.len(), 2);
    assert_eq!(scene.all_sections.len(), 7);
}

#[test]
fn test_scene_file_from_property_file() {
    use godot_properties_parser::parsers::parser_scene_file::SceneFile;

    let property_file = PropertyFile {
        preamble_properties: vec![],
        sections: vec![
            Section {
                header_type: "gd_scene".to_string(),
                properties: vec![],
            },
            Section {
                header_type: "ext_resource".to_string(),
                properties: vec![],
            },
            Section {
                header_type: "node".to_string(),
                properties: vec![],
            },
            Section {
                header_type: "custom_section".to_string(),
                properties: vec![],
            },
        ],
    };

    let scene = SceneFile::from_property_file(property_file);

    assert!(scene.header.is_some());
    assert_eq!(scene.ext_resources.len(), 1);
    assert_eq!(scene.nodes.len(), 1);
    assert_eq!(scene.all_sections.len(), 4);

    // Custom section should only be in all_sections
    let custom_in_all = scene
        .all_sections
        .iter()
        .any(|s| s.header_type == "custom_section");
    assert!(custom_in_all);
}

#[test]
fn test_scene_file_no_header() {
    let input = r#"[ext_resource type="Script" path="res://test.gd"]
[node name="Node" type="Node2D"]
"#;
    let (_, scene) = parse_scene_file(input).unwrap();

    assert!(scene.header.is_none());
    assert_eq!(scene.ext_resources.len(), 1);
    assert_eq!(scene.nodes.len(), 1);
    assert_eq!(scene.all_sections.len(), 2);
}

#[test]
fn test_scene_file_gd_resource() {
    let input = "[gd_resource type=\"Resource\" format=3]\n";
    let (_, scene) = parse_scene_file(input).unwrap();

    assert!(scene.header.is_some());
    assert_eq!(scene.header.as_ref().unwrap().header_type, "gd_resource");
}

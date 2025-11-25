use godot_properties_parser::parse_property_file;

#[test]
fn test_parse_simple_section() {
    let input = "[gd_scene load_steps=46 format=3]\n";
    let (remaining, file) = parse_property_file(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(file.sections.len(), 1);
    assert_eq!(file.sections[0].header_type, "gd_scene");
    assert_eq!(file.sections[0].properties.len(), 2);
    assert_eq!(file.sections[0].properties[0].key, "load_steps");
    assert_eq!(file.sections[0].properties[0].value, "46");
    assert_eq!(file.sections[0].properties[1].key, "format");
    assert_eq!(file.sections[0].properties[1].value, "3");
}

#[test]
fn test_parse_multiple_sections() {
    let input = r#"[gd_scene load_steps=46 format=3]

[ext_resource type="Script" path="res://test.gd"]
[node name="Node" type="Node2D"]
"#;
    let (remaining, file) = parse_property_file(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(file.sections.len(), 3);

    assert_eq!(file.sections[0].header_type, "gd_scene");
    assert_eq!(file.sections[0].properties.len(), 2);

    assert_eq!(file.sections[1].header_type, "ext_resource");
    assert_eq!(file.sections[1].properties.len(), 2);
    assert_eq!(file.sections[1].properties[0].key, "type");
    assert_eq!(file.sections[1].properties[0].value, "Script");

    assert_eq!(file.sections[2].header_type, "node");
    assert_eq!(file.sections[2].properties.len(), 2);
    assert_eq!(file.sections[2].properties[0].key, "name");
    assert_eq!(file.sections[2].properties[0].value, "Node");
}

#[test]
fn test_parse_section_with_nested_brackets() {
    let input = r#"[gd_scene load_steps=46 format=3 uid="uid://test"]
"#;
    let (remaining, file) = parse_property_file(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(file.sections.len(), 1);
    assert_eq!(file.sections[0].header_type, "gd_scene");
    assert_eq!(file.sections[0].properties.len(), 3);
}

#[test]
fn test_parse_with_spaces_around_equals() {
    let input = "[ext_resource type = \"Script\" uid = \"uid://test\"]\n";
    let (remaining, file) = parse_property_file(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(file.sections.len(), 1);
    assert_eq!(file.sections[0].header_type, "ext_resource");
    assert_eq!(file.sections[0].properties[0].key, "type");
    assert_eq!(file.sections[0].properties[0].value, "Script");
}

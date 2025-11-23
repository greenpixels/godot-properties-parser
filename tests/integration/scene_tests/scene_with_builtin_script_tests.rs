use crate::integration::test_helpers::{SceneExpectations, SceneTestSuite};

const SCENE_CONTENT: &str = include_str!("../../scenes/scene_with_builtin_script.tscn");

fn get_expectations() -> SceneExpectations {
    SceneExpectations::new("scene_with_builtin_script")
        .with_sections(8)
        .with_ext_resources(1)
        .with_sub_resources(1)
        .with_nodes(5)
        .with_connections(0)
        .with_editables(0)
        .with_header_property("load_steps", "3")
        .with_header_property("format", "3")
        .with_header_property("uid", "\"uid://dbu7cb0h10jdt\"")
}

fn get_test_suite() -> SceneTestSuite<'static> {
    SceneTestSuite::new(SCENE_CONTENT, get_expectations())
}

#[test]
fn test_parses_successfully() {
    get_test_suite().test_parses_successfully();
}

#[test]
fn test_consumes_entire_file() {
    get_test_suite().test_consumes_entire_file();
}

#[test]
fn test_parses_as_scene_file() {
    get_test_suite().test_parses_as_scene_file();
}

#[test]
fn test_section_count() {
    get_test_suite().test_section_count();
}

#[test]
fn test_scene_has_header() {
    get_test_suite().test_scene_has_header();
}

#[test]
fn test_first_section_is_gd_scene() {
    get_test_suite().test_first_section_is_gd_scene();
}

#[test]
fn test_header_properties() {
    get_test_suite().test_header_properties();
}

#[test]
fn test_ext_resources_count() {
    get_test_suite().test_ext_resources_count();
}

#[test]
fn test_sub_resources_count() {
    get_test_suite().test_sub_resources_count();
}

#[test]
fn test_nodes_count() {
    get_test_suite().test_nodes_count();
}

#[test]
fn test_connections_count() {
    get_test_suite().test_connections_count();
}

#[test]
fn test_editables_count() {
    get_test_suite().test_editables_count();
}

#[test]
fn test_categorization_is_complete() {
    get_test_suite().test_categorization_is_complete();
}

#[test]
fn test_all_sections_have_valid_header_types() {
    get_test_suite().test_all_sections_have_valid_header_types();
}

#[test]
fn test_all_ext_resources_have_required_properties() {
    get_test_suite().test_all_ext_resources_have_required_properties();
}

#[test]
fn test_all_sub_resources_have_required_properties() {
    get_test_suite().test_all_sub_resources_have_required_properties();
}

#[test]
fn test_all_nodes_have_name() {
    get_test_suite().test_all_nodes_have_name();
}

#[test]
fn test_ext_resource_paths_are_quoted() {
    get_test_suite().test_ext_resource_paths_are_quoted();
}

#[test]
fn test_node_names_are_quoted() {
    get_test_suite().test_node_names_are_quoted();
}

#[test]
fn test_nodes_have_type_or_instance() {
    get_test_suite().test_nodes_have_type_or_instance();
}

#[test]
fn test_sub_resource_ids_are_unique() {
    get_test_suite().test_sub_resource_ids_are_unique();
}

#[test]
fn test_sub_resource_has_gdscript_type() {
    use godot_properties_parser::parse_scene_file;

    let (_, scene) = parse_scene_file(SCENE_CONTENT).unwrap();

    assert_eq!(scene.sub_resources.len(), 1, "Should have 1 sub_resource");

    let sub_resource = &scene.sub_resources[0];
    let resource_name = sub_resource
        .properties
        .iter()
        .find(|p| p.key == "resource_name")
        .map(|p| p.value.as_str());

    assert_eq!(
        resource_name,
        Some("\"TooltipOcerlay\""),
        "Sub resource should have resource_name property"
    );
}

#[test]
fn test_builtin_script_has_script_source_property() {
    use godot_properties_parser::parse_scene_file;

    let (_, scene) = parse_scene_file(SCENE_CONTENT).unwrap();

    let sub_resource = &scene.sub_resources[0];

    // Verify basic sub_resource properties
    assert_eq!(sub_resource.header_type, "sub_resource");

    let has_type = sub_resource.properties.iter().any(|p| p.key == "type");
    let has_id = sub_resource.properties.iter().any(|p| p.key == "id");
    let has_resource_name = sub_resource
        .properties
        .iter()
        .any(|p| p.key == "resource_name");

    assert!(has_type, "Sub resource should have 'type' property");
    assert!(has_id, "Sub resource should have 'id' property");
    assert!(
        has_resource_name,
        "Sub resource should have 'resource_name' property"
    );

    // Find the script/source property
    let script_source = sub_resource
        .properties
        .iter()
        .find(|p| p.key == "script/source");

    assert!(
        script_source.is_some(),
        "Builtin script sub_resource should have 'script/source' property"
    );

    let script_value = &script_source.unwrap().value;

    // Verify it's a string (starts with quote)
    assert!(
        script_value.starts_with('"'),
        "script/source value should be a quoted string"
    );

    // Verify it contains actual GDScript code
    assert!(
        script_value.contains("extends CanvasLayer"),
        "script/source should contain the GDScript code"
    );

    assert!(
        script_value.contains("func _ready()"),
        "script/source should contain function definitions"
    );

    // The script should be parsed as a single multi-line property
    assert!(
        sub_resource.properties.len() < 10,
        "Sub resource should have a reasonable number of properties (not split script lines)"
    );
}

#[test]
fn test_builtin_script_contains_expected_code() {
    use godot_properties_parser::parse_scene_file;

    let (_, scene) = parse_scene_file(SCENE_CONTENT).unwrap();
    let sub_resource = &scene.sub_resources[0];

    let script_source = sub_resource
        .properties
        .iter()
        .find(|p| p.key == "script/source")
        .expect("Sub resource should have 'script/source' property");

    let script_value = &script_source.value;

    // Verify class extension
    assert!(
        script_value.contains("extends CanvasLayer"),
        "Script should extend CanvasLayer"
    );

    // Verify @onready variables
    assert!(
        script_value.contains("@onready var layout_wrapper"),
        "Script should have @onready var layout_wrapper"
    );
    assert!(
        script_value.contains("@onready var tooltip_container"),
        "Script should have @onready var tooltip_container"
    );
    assert!(
        script_value.contains("@onready var main_tooltip"),
        "Script should have @onready var main_tooltip"
    );

    // Verify @export variables
    assert!(
        script_value.contains("@export var highlighted_keys : Array[TooltipHighlightedKey]"),
        "Script should have @export var highlighted_keys"
    );
    assert!(
        script_value.contains("@export var explanations_delay : float = 1.25"),
        "Script should have @export var explanations_delay"
    );

    // Verify regular variables
    assert!(
        script_value.contains("var current_tooltip_element: Control = null"),
        "Script should have var current_tooltip_element"
    );
    assert!(
        script_value.contains("var explanations : Array[Node] = []"),
        "Script should have var explanations"
    );

    // Verify function definitions
    assert!(
        script_value.contains("func _ready()"),
        "Script should have _ready() function"
    );
    assert!(
        script_value.contains("func conceal()"),
        "Script should have conceal() function"
    );
    assert!(
        script_value.contains("func describe(origin_node: Control, content: String, show_extra_explanation: bool = false)"),
        "Script should have describe() function with correct signature"
    );
    assert!(
        script_value.contains(
            "func apply_bbcode_for_common_keys(content: String, show_explanations: bool)"
        ),
        "Script should have apply_bbcode_for_common_keys() function"
    );
    assert!(
        script_value.contains("func add_explanation"),
        "Script should have add_explanation() function"
    );
    assert!(
        script_value.contains("func __cleanup()"),
        "Script should have __cleanup() function"
    );

    // Verify some key logic patterns
    assert!(
        script_value.contains("layout_wrapper.scale = Vector2(0, 1)"),
        "Script should initialize layout_wrapper scale"
    );
    assert!(
        script_value.contains("if OS.get_name() == \\\"Android\\\""),
        "Script should have Android-specific logic"
    );
    assert!(
        script_value.contains("TweenHelper.tween"),
        "Script should use TweenHelper"
    );
    assert!(
        script_value.contains("BBCodeHelper.build"),
        "Script should use BBCodeHelper"
    );

    // Verify the script is substantial (over 100 lines when unescaped)
    // In the parsed value, newlines appear as literal \n in the string
    let actual_newline_count = script_value.matches('\n').count();
    assert!(
        actual_newline_count > 100,
        "Script should contain over 100 lines of code, found {} actual newlines",
        actual_newline_count
    );
}

use crate::integration::test_helpers::{SceneExpectations, SceneTestSuite};

const TABLE_SELECTION_CONTENT: &str = include_str!("../../src/scenes/table_selection_entry.tscn");

fn get_expectations() -> SceneExpectations {
    SceneExpectations::new("table_selection_entry")
        .with_sections(22)
        .with_ext_resources(5)
        .with_sub_resources(3)
        .with_nodes(11)
        .with_connections(2)
        .with_editables(0)
        .with_header_property("format", "3")
}

fn get_test_suite() -> SceneTestSuite<'static> {
    SceneTestSuite::new(TABLE_SELECTION_CONTENT, get_expectations())
}

// Basic tests
#[test]
fn test_parses_successfully() {
    get_test_suite().test_parses_successfully();
}

#[test]
fn test_consumes_entire_file() {
    get_test_suite().test_consumes_entire_file();
}

#[test]
fn test_section_count() {
    get_test_suite().test_section_count();
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
fn test_parses_as_scene_file() {
    get_test_suite().test_parses_as_scene_file();
}

#[test]
fn test_scene_has_header() {
    get_test_suite().test_scene_has_header();
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

// Validation tests
#[test]
fn test_all_ext_resources_have_required_properties() {
    get_test_suite().test_all_ext_resources_have_required_properties();
}

#[test]
fn test_ext_resource_paths_are_quoted() {
    get_test_suite().test_ext_resource_paths_are_quoted();
}

#[test]
fn test_all_sub_resources_have_required_properties() {
    get_test_suite().test_all_sub_resources_have_required_properties();
}

#[test]
fn test_sub_resource_ids_are_unique() {
    get_test_suite().test_sub_resource_ids_are_unique();
}

#[test]
fn test_all_nodes_have_name() {
    get_test_suite().test_all_nodes_have_name();
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
fn test_all_connections_have_required_properties() {
    get_test_suite().test_all_connections_have_required_properties();
}

#[test]
fn test_all_sections_have_valid_header_types() {
    get_test_suite().test_all_sections_have_valid_header_types();
}

#[test]
fn test_categorization_is_complete() {
    get_test_suite().test_categorization_is_complete();
}

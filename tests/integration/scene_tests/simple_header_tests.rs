use crate::integration::test_helpers::{SceneExpectations, SceneTestSuite};

const SIMPLE_HEADER_CONTENT: &str = include_str!("../../scenes/simple_header.tscn");

fn get_expectations() -> SceneExpectations {
    SceneExpectations::new("simple_header")
        .with_sections(1)
        .with_ext_resources(0)
        .with_sub_resources(0)
        .with_nodes(0)
        .with_connections(0)
        .with_editables(0)
        .with_header_property("load_steps", "46")
        .with_header_property("format", "3")
        .with_header_property("uid", "uid://dji6tdg6ku1xb")
}

fn get_test_suite() -> SceneTestSuite<'static> {
    SceneTestSuite::new(SIMPLE_HEADER_CONTENT, get_expectations())
}

// Tests using generic framework
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

#[test]
fn test_all_sections_have_valid_header_types() {
    get_test_suite().test_all_sections_have_valid_header_types();
}

#[test]
fn test_categorization_is_complete() {
    get_test_suite().test_categorization_is_complete();
}

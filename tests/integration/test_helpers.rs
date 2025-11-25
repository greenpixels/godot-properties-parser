use godot_properties_parser::{parse_property_file, parse_scene_file};
use std::collections::HashMap;

/// Expected counts for a scene file
#[derive(Debug, Clone)]
pub struct SceneExpectations {
    pub name: &'static str,
    pub total_sections: usize,
    pub ext_resources: usize,
    pub sub_resources: usize,
    pub nodes: usize,
    pub connections: usize,
    pub editables: usize,
    pub header_properties: HashMap<&'static str, &'static str>,
}

impl SceneExpectations {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            total_sections: 0,
            ext_resources: 0,
            sub_resources: 0,
            nodes: 0,
            connections: 0,
            editables: 0,
            header_properties: HashMap::new(),
        }
    }

    pub fn with_sections(mut self, count: usize) -> Self {
        self.total_sections = count;
        self
    }

    pub fn with_ext_resources(mut self, count: usize) -> Self {
        self.ext_resources = count;
        self
    }

    pub fn with_sub_resources(mut self, count: usize) -> Self {
        self.sub_resources = count;
        self
    }

    pub fn with_nodes(mut self, count: usize) -> Self {
        self.nodes = count;
        self
    }

    pub fn with_connections(mut self, count: usize) -> Self {
        self.connections = count;
        self
    }

    pub fn with_editables(mut self, count: usize) -> Self {
        self.editables = count;
        self
    }

    pub fn with_header_property(mut self, key: &'static str, value: &'static str) -> Self {
        self.header_properties.insert(key, value);
        self
    }
}

/// Generic test suite for scene files
pub struct SceneTestSuite<'a> {
    content: &'a str,
    expectations: SceneExpectations,
}

impl<'a> SceneTestSuite<'a> {
    pub fn new(content: &'a str, expectations: SceneExpectations) -> Self {
        Self {
            content,
            expectations,
        }
    }

    // Basic parsing tests
    pub fn test_parses_successfully(&self) {
        let result = parse_property_file(self.content);
        assert!(
            result.is_ok(),
            "[{}] Failed to parse: {:?}",
            self.expectations.name,
            result.err()
        );
    }

    pub fn test_consumes_entire_file(&self) {
        let (remaining, _) = parse_property_file(self.content).unwrap();
        assert_eq!(
            remaining, "",
            "[{}] Parser did not consume entire file",
            self.expectations.name
        );
    }

    pub fn test_section_count(&self) {
        if self.expectations.total_sections > 0 {
            let (_, file) = parse_property_file(self.content).unwrap();
            assert_eq!(
                file.sections.len(),
                self.expectations.total_sections,
                "[{}] Expected {} sections",
                self.expectations.name,
                self.expectations.total_sections
            );
        }
    }

    pub fn test_first_section_is_gd_scene(&self) {
        let (_, file) = parse_property_file(self.content).unwrap();
        assert_eq!(
            file.sections[0].header_type, "gd_scene",
            "[{}] First section should be gd_scene",
            self.expectations.name
        );
    }

    pub fn test_header_properties(&self) {
        let (_, file) = parse_property_file(self.content).unwrap();
        let props = &file.sections[0].properties;

        for (key, expected_value) in &self.expectations.header_properties {
            let prop = props.iter().find(|p| p.key == *key);
            assert!(
                prop.is_some(),
                "[{}] Should have '{}' property",
                self.expectations.name,
                key
            );
            assert_eq!(
                prop.unwrap().value,
                *expected_value,
                "[{}] Property '{}' should be '{}'",
                self.expectations.name,
                key,
                expected_value
            );
        }
    }

    // Scene file tests
    pub fn test_parses_as_scene_file(&self) {
        let result = parse_scene_file(self.content);
        assert!(
            result.is_ok(),
            "[{}] Failed to parse as scene file: {:?}",
            self.expectations.name,
            result.err()
        );
    }

    pub fn test_scene_has_header(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();
        assert!(
            scene.header.is_some(),
            "[{}] Scene should have a header",
            self.expectations.name
        );
    }

    pub fn test_ext_resources_count(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();
        assert_eq!(
            scene.ext_resources.len(),
            self.expectations.ext_resources,
            "[{}] Expected {} external resources",
            self.expectations.name,
            self.expectations.ext_resources
        );
    }

    pub fn test_sub_resources_count(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();
        assert_eq!(
            scene.sub_resources.len(),
            self.expectations.sub_resources,
            "[{}] Expected {} sub-resources",
            self.expectations.name,
            self.expectations.sub_resources
        );
    }

    pub fn test_nodes_count(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();
        assert_eq!(
            scene.nodes.len(),
            self.expectations.nodes,
            "[{}] Expected {} nodes",
            self.expectations.name,
            self.expectations.nodes
        );
    }

    pub fn test_connections_count(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();
        assert_eq!(
            scene.connections.len(),
            self.expectations.connections,
            "[{}] Expected {} connections",
            self.expectations.name,
            self.expectations.connections
        );
    }

    pub fn test_editables_count(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();
        assert_eq!(
            scene.editables.len(),
            self.expectations.editables,
            "[{}] Expected {} editables",
            self.expectations.name,
            self.expectations.editables
        );
    }

    // Validation tests
    pub fn test_all_ext_resources_have_required_properties(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        for (i, ext_res) in scene.ext_resources.iter().enumerate() {
            let has_type = ext_res.properties.iter().any(|p| p.key == "type");
            let has_path = ext_res.properties.iter().any(|p| p.key == "path");

            assert!(
                has_type,
                "[{}] External resource {} should have 'type' property",
                self.expectations.name, i
            );
            assert!(
                has_path,
                "[{}] External resource {} should have 'path' property",
                self.expectations.name, i
            );
        }
    }

    pub fn test_ext_resource_paths_are_quoted(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        for ext_res in &scene.ext_resources {
            if let Some(path) = ext_res.properties.iter().find(|p| p.key == "path") {
                // Values are now parsed without quotes
                assert!(
                    !path.value.is_empty(),
                    "[{}] Path should not be empty: {}",
                    self.expectations.name,
                    path.value
                );
            }
        }
    }

    pub fn test_all_sub_resources_have_required_properties(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        for (i, sub_res) in scene.sub_resources.iter().enumerate() {
            let has_type = sub_res.properties.iter().any(|p| p.key == "type");
            let has_id = sub_res.properties.iter().any(|p| p.key == "id");

            assert!(
                has_type,
                "[{}] Sub-resource {} should have 'type' property",
                self.expectations.name, i
            );
            assert!(
                has_id,
                "[{}] Sub-resource {} should have 'id' property",
                self.expectations.name, i
            );
        }
    }

    pub fn test_sub_resource_ids_are_unique(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        let mut ids = std::collections::HashSet::new();

        for sub_res in &scene.sub_resources {
            if let Some(id_prop) = sub_res.properties.iter().find(|p| p.key == "id") {
                assert!(
                    ids.insert(id_prop.value.clone()),
                    "[{}] Duplicate sub-resource id found: {}",
                    self.expectations.name,
                    id_prop.value
                );
            }
        }
    }

    pub fn test_all_nodes_have_name(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        for (i, node) in scene.nodes.iter().enumerate() {
            let has_name = node.properties.iter().any(|p| p.key == "name");
            assert!(
                has_name,
                "[{}] Node {} should have 'name' property",
                self.expectations.name, i
            );
        }
    }

    pub fn test_node_names_are_quoted(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        for node in &scene.nodes {
            if let Some(name) = node.properties.iter().find(|p| p.key == "name") {
                // Values are now parsed without quotes
                assert!(
                    !name.value.is_empty(),
                    "[{}] Node name should not be empty: {}",
                    self.expectations.name,
                    name.value
                );
            }
        }
    }

    pub fn test_nodes_have_type_or_instance(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        for (i, node) in scene.nodes.iter().enumerate() {
            let has_type = node.properties.iter().any(|p| p.key == "type");
            let has_instance = node.properties.iter().any(|p| p.key == "instance");

            assert!(
                has_type || has_instance,
                "[{}] Node {} should have either 'type' or 'instance' property",
                self.expectations.name,
                i
            );
        }
    }

    pub fn test_all_connections_have_required_properties(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        for (i, conn) in scene.connections.iter().enumerate() {
            let has_signal = conn.properties.iter().any(|p| p.key == "signal");
            let has_from = conn.properties.iter().any(|p| p.key == "from");
            let has_to = conn.properties.iter().any(|p| p.key == "to");
            let has_method = conn.properties.iter().any(|p| p.key == "method");

            assert!(
                has_signal,
                "[{}] Connection {} should have 'signal' property",
                self.expectations.name, i
            );
            assert!(
                has_from,
                "[{}] Connection {} should have 'from' property",
                self.expectations.name, i
            );
            assert!(
                has_to,
                "[{}] Connection {} should have 'to' property",
                self.expectations.name, i
            );
            assert!(
                has_method,
                "[{}] Connection {} should have 'method' property",
                self.expectations.name, i
            );
        }
    }

    pub fn test_all_sections_have_valid_header_types(&self) {
        let (_, file) = parse_property_file(self.content).unwrap();

        for section in &file.sections {
            assert!(
                !section.header_type.is_empty(),
                "[{}] Header type should not be empty",
                self.expectations.name
            );
            assert!(
                section
                    .header_type
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_'),
                "[{}] Header type '{}' contains invalid characters",
                self.expectations.name,
                section.header_type
            );
        }
    }

    pub fn test_categorization_is_complete(&self) {
        let (_, scene) = parse_scene_file(self.content).unwrap();

        let categorized_count = (if scene.header.is_some() { 1 } else { 0 })
            + scene.ext_resources.len()
            + scene.sub_resources.len()
            + scene.nodes.len()
            + scene.connections.len()
            + scene.editables.len();

        assert_eq!(
            categorized_count,
            scene.all_sections.len(),
            "[{}] All sections should be categorized",
            self.expectations.name
        );
    }
}

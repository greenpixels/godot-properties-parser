use super::parser_property_file::{PropertyFile, Section, parse_property_file};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct SceneFile {
    /// The main scene header (usually gd_scene or gd_resource)
    pub header: Option<Section>,
    /// External resource sections
    pub ext_resources: Vec<Section>,
    /// Sub-resource sections
    pub sub_resources: Vec<Section>,
    /// Node sections
    pub nodes: Vec<Section>,
    /// Connection sections (signals)
    pub connections: Vec<Section>,
    /// Editable sections
    pub editables: Vec<Section>,
    /// All sections in order, including those not categorized above
    pub all_sections: Vec<Section>,
}

impl SceneFile {
    /// Create a new empty SceneFile
    pub fn new() -> Self {
        Self {
            header: None,
            ext_resources: Vec::new(),
            sub_resources: Vec::new(),
            nodes: Vec::new(),
            connections: Vec::new(),
            editables: Vec::new(),
            all_sections: Vec::new(),
        }
    }

    /// Create a SceneFile from a PropertyFile by categorizing sections
    pub fn from_property_file(property_file: PropertyFile) -> Self {
        let mut scene_file = SceneFile::new();

        for section in property_file.sections {
            // Store in all_sections
            scene_file.all_sections.push(section.clone());

            // Categorize by header type
            match section.header_type.as_str() {
                "gd_scene" | "gd_resource" => {
                    scene_file.header = Some(section);
                }
                "ext_resource" => {
                    scene_file.ext_resources.push(section);
                }
                "sub_resource" => {
                    scene_file.sub_resources.push(section);
                }
                "node" => {
                    scene_file.nodes.push(section);
                }
                "connection" => {
                    scene_file.connections.push(section);
                }
                "editable" => {
                    scene_file.editables.push(section);
                }
                _ => {
                    // Other types stay only in all_sections
                }
            }
        }

        scene_file
    }
}

impl Default for SceneFile {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a Godot scene file (`.tscn`) into a structured `SceneFile`.
///
/// Scene files define game objects, their components, and relationships. This parser
/// categorizes sections into `ext_resources`, `sub_resources`, `nodes`, `connections`,
/// and `editables` for easy access, while preserving all sections in `all_sections`.
///
/// # Arguments
///
/// * `input` - The complete `.tscn` file content as a string
///
/// # Returns
///
/// * `Ok((remaining, SceneFile))` - Successfully parsed scene with any unconsumed input
/// * `Err(nom::Err)` - Parse error if the file format is invalid
///
/// # Example
///
/// ```no_run
/// use godot_properties_parser::parse_scene_file;
/// use std::fs;
///
/// let content = fs::read_to_string("scene.tscn").unwrap();
/// let (remaining, scene) = parse_scene_file(&content).unwrap();
///
/// println!("Nodes: {}", scene.nodes.len());
/// println!("Resources: {}", scene.ext_resources.len());
/// ```
pub fn parse_scene_file(input: &str) -> IResult<&str, SceneFile> {
    let (remaining, property_file) = parse_property_file(input)?;
    let scene_file = SceneFile::from_property_file(property_file);
    Ok((remaining, scene_file))
}

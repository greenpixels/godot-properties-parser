use super::parser_property::UntypedProperty;
use super::parser_property_file::{PropertyFile, Section, parse_property_file};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct ProjectFile {
    /// Properties that appear before the first section (e.g., config_version=5)
    pub preamble_properties: Vec<UntypedProperty>,
    /// Application configuration section
    pub application: Option<Section>,
    /// Audio configuration section
    pub audio: Option<Section>,
    /// Autoload section (singleton nodes)
    pub autoload: Option<Section>,
    /// Debug settings section
    pub debug: Option<Section>,
    /// Display settings section
    pub display: Option<Section>,
    /// Editor plugins section
    pub editor_plugins: Option<Section>,
    /// Input mappings section
    pub input: Option<Section>,
    /// Input devices settings section
    pub input_devices: Option<Section>,
    /// Internationalization settings section
    pub internationalization: Option<Section>,
    /// Layer names section
    pub layer_names: Option<Section>,
    /// Physics settings section
    pub physics: Option<Section>,
    /// Rendering settings section
    pub rendering: Option<Section>,
    /// All sections in order, including those not categorized above
    pub all_sections: Vec<Section>,
}

impl ProjectFile {
    /// Create a new empty ProjectFile
    pub fn new() -> Self {
        Self {
            preamble_properties: Vec::new(),
            application: None,
            audio: None,
            autoload: None,
            debug: None,
            display: None,
            editor_plugins: None,
            input: None,
            input_devices: None,
            internationalization: None,
            layer_names: None,
            physics: None,
            rendering: None,
            all_sections: Vec::new(),
        }
    }

    /// Create a ProjectFile from a PropertyFile by categorizing sections
    pub fn from_property_file(property_file: PropertyFile) -> Self {
        let mut project_file = ProjectFile::new();
        project_file.preamble_properties = property_file.preamble_properties;

        for section in property_file.sections {
            // Store in all_sections
            project_file.all_sections.push(section.clone());

            // Categorize by header type
            match section.header_type.as_str() {
                "application" => {
                    project_file.application = Some(section);
                }
                "audio" => {
                    project_file.audio = Some(section);
                }
                "autoload" => {
                    project_file.autoload = Some(section);
                }
                "debug" => {
                    project_file.debug = Some(section);
                }
                "display" => {
                    project_file.display = Some(section);
                }
                "editor_plugins" => {
                    project_file.editor_plugins = Some(section);
                }
                "input" => {
                    project_file.input = Some(section);
                }
                "input_devices" => {
                    project_file.input_devices = Some(section);
                }
                "internationalization" => {
                    project_file.internationalization = Some(section);
                }
                "layer_names" => {
                    project_file.layer_names = Some(section);
                }
                "physics" => {
                    project_file.physics = Some(section);
                }
                "rendering" => {
                    project_file.rendering = Some(section);
                }
                _ => {
                    // Other types stay only in all_sections
                }
            }
        }

        project_file
    }
}

impl Default for ProjectFile {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses a Godot project file (`.godot`) into a structured `ProjectFile`.
///
/// Project files contain global configuration for a Godot project, including application
/// settings, autoloads, input mappings, and rendering options. This parser categorizes
/// known sections (`application`, `audio`, `autoload`, etc.) while preserving all sections
/// including custom ones in `all_sections`.
///
/// # Arguments
///
/// * `input` - The complete `.godot` file content as a string
///
/// # Returns
///
/// * `Ok((remaining, ProjectFile))` - Successfully parsed project with any unconsumed input
/// * `Err(nom::Err)` - Parse error if the file format is invalid
///
/// # Example
///
/// ```no_run
/// use godot_properties_parser::parse_project_file;
/// use std::fs;
///
/// let content = fs::read_to_string("project.godot").unwrap();
/// let (remaining, project) = parse_project_file(&content).unwrap();
///
/// if let Some(app) = project.application {
///     for prop in &app.properties {
///         println!("{}: {}", prop.key, prop.value);
///     }
/// }
/// ```
pub fn parse_project_file(input: &str) -> IResult<&str, ProjectFile> {
    let (remaining, property_file) = parse_property_file(input)?;
    let project_file = ProjectFile::from_property_file(property_file);
    Ok((remaining, project_file))
}

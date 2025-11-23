use super::parser_property_file::{parse_property_file, PropertyFile, Section};
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct ProjectFile {
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

/// Parse a Godot project file (.godot) and return a structured ProjectFile
pub fn parse_project_file(input: &str) -> IResult<&str, ProjectFile> {
    let (remaining, property_file) = parse_property_file(input)?;
    let project_file = ProjectFile::from_property_file(property_file);
    Ok((remaining, project_file))
}

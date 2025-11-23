# Godot Properties Parser

A Rust parser for Godot Engine files.

## Installation

```bash
cargo add godot-properties-parser
```

## Usage

### parse_scene_file

```rust
use godot_properties_parser::parse_scene_file;
use std::fs;

let scene_content = fs::read_to_string("path/to/scene.tscn")
    .expect("Failed to read scene file");

match parse_scene_file(&scene_content) {
    Ok((remaining, scene)) => {
        println!("Header: {:?}", scene.header);
        println!("Number of ext_resources: {}", scene.ext_resources.len());
        println!("Number of sub_resources: {}", scene.sub_resources.len());
        println!("Number of nodes: {}", scene.nodes.len());
        println!("Number of connections: {}", scene.connections.len());
        
        // Access specific sections
        for node in &scene.nodes {
            println!("Node type: {}", node.header_type);
            for prop in &node.properties {
                println!("  {}: {}", prop.key, prop.value);
            }
        }
    }
    Err(e) => eprintln!("Parse error: {:?}", e),
}
```

Returns `SceneFile` with:

- `header`, `ext_resources`, `sub_resources`, `nodes`, `connections`, `editables` - Categorized sections
- `all_sections` - All sections in original order, including any unrecognized types

### parse_project_file

```rust
use godot_properties_parser::parse_project_file;
use std::fs;

let project_content = fs::read_to_string("project.godot")
    .expect("Failed to read project file");

match parse_project_file(&project_content) {
    Ok((remaining, project)) => {
        // Access categorized sections
        if let Some(app) = project.application {
            println!("Application section found");
            for prop in &app.properties {
                println!("  {}: {}", prop.key, prop.value);
            }
        }
        
        if let Some(autoload) = project.autoload {
            println!("Autoloads:");
            for prop in &autoload.properties {
                println!("  {}: {}", prop.key, prop.value);
            }
        }
        
        println!("Total sections: {}", project.all_sections.len());
    }
    Err(e) => eprintln!("Parse error: {:?}", e),
}
```

Returns `ProjectFile` with categorized sections:

- `application`, `audio`, `autoload`, `debug`, `display`, `editor_plugins`, `input`, `input_devices`, `internationalization`, `layer_names`, `physics`, `rendering` - Known section types
- `all_sections` - All sections in original order, including any custom or unrecognized sections

### parse_property_file

Low-level parser that returns generic key-value pairs instead of a specific struct. Useful for any Godot property file format.

```rust
use godot_properties_parser::parse_property_file;
use std::fs;

let content = fs::read_to_string("path/to/file.tscn")
    .expect("Failed to read file");

match parse_property_file(&content) {
    Ok((remaining, property_file)) => {
        for section in &property_file.sections {
            println!("Section type: {}", section.header_type);
            for prop in &section.properties {
                println!("  {}: {}", prop.key, prop.value);
            }
        }
    }
    Err(e) => eprintln!("Parse error: {:?}", e),
}
```

Returns `PropertyFile` with `sections: Vec<Section>`.

Each `Section` has `header_type: String` and `properties: Vec<UntypedProperty>`.

Each `UntypedProperty` has `key: String` and `value: String`.

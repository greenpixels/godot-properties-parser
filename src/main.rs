use godot_properties_parser::{
    parse_header_type_and_consume_enclosure, parse_property_file, properties0,
};

fn main() {
    println!("=== Header Parser Examples ===");
    for input in [
        "[gdscene something=[1, 2, 3]]",
        r#"[gdscene something="]\"]\"]"]"#,
        r#"[gdscene\nsomething="hello"\n]"#,
    ] {
        let (_input, header_type) = match parse_header_type_and_consume_enclosure(input) {
            Ok(result) => result,
            Err(error) => panic!("{}", error),
        };
        println!("Header type: {}", header_type);
    }

    println!("\n=== Properties Parser Examples ===");
    for input in [
        "value = 2300",
        "value=2300 something=test",
        r#"something=["i love","cheese"]"#,
        r#" value=2300 something=["i love",
    "cheese"]"#,
        r#"load_steps=46 format=3 uid="uid://dji6tdg6ku1xb""#,
    ] {
        match properties0(input) {
            Ok((_remaining, properties)) => {
                println!("Input: {}", input.replace('\n', "\\n"));
                for prop in properties {
                    println!("  {} = {}", prop.key, prop.value);
                }
            }
            Err(error) => println!("Error parsing '{}': {}", input, error),
        }
    }

    println!("\n=== Property File Parser Examples ===");

    let simple_file = r#"[gd_scene load_steps=46 format=3 uid="uid://test"]

[ext_resource type="Script" path="res://test.gd"]
[node name="Node" type="Node2D"]
"#;

    match parse_property_file(simple_file) {
        Ok((_, file)) => {
            println!("Parsed {} sections:", file.sections.len());
            for (i, section) in file.sections.iter().enumerate() {
                println!("\n  Section {}: [{}]", i + 1, section.header_type);
                for prop in &section.properties {
                    println!("    {} = {}", prop.key, prop.value);
                }
            }
        }
        Err(error) => println!("Error parsing file: {}", error),
    }

    // Parse a real scene file
    println!("\n=== Parsing simple_header.tscn ===");
    let scene_content = include_str!("scenes/simple_header.tscn");
    match parse_property_file(scene_content) {
        Ok((_, file)) => {
            println!("Successfully parsed {} section(s)", file.sections.len());
            for section in &file.sections {
                println!(
                    "  [{}] with {} properties",
                    section.header_type,
                    section.properties.len()
                );
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    // Parse a complex scene file
    println!("\n=== Parsing play_room.tscn (first 10 sections) ===");
    let complex_scene = include_str!("scenes/play_room.tscn");
    match parse_property_file(complex_scene) {
        Ok((_, file)) => {
            println!("Successfully parsed {} total sections", file.sections.len());
            for (i, section) in file.sections.iter().take(10).enumerate() {
                println!(
                    "  {}. [{}] with {} properties",
                    i + 1,
                    section.header_type,
                    section.properties.len()
                );
                if section.properties.len() <= 3 {
                    for prop in &section.properties {
                        println!(
                            "     {} = {}",
                            prop.key,
                            prop.value.chars().take(40).collect::<String>()
                        );
                    }
                }
            }
        }
        Err(error) => println!("Error: {}", error),
    }

    // Use the scene file parser
    use godot_properties_parser::parse_scene_file;

    println!("\n=== Scene File Parser (play_room.tscn) ===");
    match parse_scene_file(complex_scene) {
        Ok((_, scene)) => {
            println!("Scene structure:");
            if let Some(header) = &scene.header {
                println!(
                    "  Header: [{}] with {} properties",
                    header.header_type,
                    header.properties.len()
                );
            }
            println!("  External resources: {}", scene.ext_resources.len());
            println!("  Sub-resources: {}", scene.sub_resources.len());
            println!("  Nodes: {}", scene.nodes.len());
            println!("  Connections: {}", scene.connections.len());
            println!("  Editables: {}", scene.editables.len());
            println!("  Total sections: {}", scene.all_sections.len());

            println!("\n  First 3 nodes:");
            for (i, node) in scene.nodes.iter().take(3).enumerate() {
                if let Some(name_prop) = node.properties.iter().find(|p| p.key == "name") {
                    println!(
                        "    {}. {} (type: {})",
                        i + 1,
                        name_prop.value,
                        node.properties
                            .iter()
                            .find(|p| p.key == "type")
                            .map(|p| p.value.as_str())
                            .unwrap_or("unknown")
                    );
                }
            }
        }
        Err(error) => println!("Error: {}", error),
    }
}

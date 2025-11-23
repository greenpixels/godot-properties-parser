use nom::{
    IResult, Parser,
    character::complete::{line_ending, multispace0, not_line_ending},
    combinator::opt,
    multi::many0,
    sequence::terminated,
};

use super::parser_property::{UntypedProperty, properties0};

#[derive(Debug, PartialEq, Clone)]
pub struct Section {
    pub header_type: String,
    pub properties: Vec<UntypedProperty>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertyFile {
    pub sections: Vec<Section>,
}

/// Parses any Godot property file into generic sections with key-value pairs.
///
/// This is a low-level parser that returns untyped sections and properties without
/// categorization. Use `parse_scene_file` or `parse_project_file` for structured
/// access to specific file types.
///
/// Each section starts with `[header_type ...]` and contains key-value properties.
/// This parser handles `.tscn`, `.godot`, `.tres`, and similar Godot file formats.
///
/// # Arguments
///
/// * `input` - The complete file content as a string
///
/// # Returns
///
/// * `Ok((remaining, PropertyFile))` - Successfully parsed file with sections
/// * `Err(nom::Err)` - Parse error if the file format is invalid
///
/// # Example
///
/// ```no_run
/// use godot_properties_parser::parse_property_file;
/// use std::fs;
///
/// let content = fs::read_to_string("file.tscn").unwrap();
/// let (remaining, property_file) = parse_property_file(&content).unwrap();
///
/// for section in &property_file.sections {
///     println!("Section: {}", section.header_type);
///     for prop in &section.properties {
///         println!("  {}: {}", prop.key, prop.value);
///     }
/// }
/// ```
pub fn parse_property_file(input: &str) -> IResult<&str, PropertyFile> {
    let (input, _) = multispace0(input)?;
    let (input, sections) = many0(parse_section).parse(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, PropertyFile { sections }))
}

fn parse_section(input: &str) -> IResult<&str, Section> {
    // Skip any empty lines or comments before the section
    let (input, _) = many0(terminated(opt(comment_line), line_ending)).parse(input)?;

    // Parse the header line: [header_type key=value key=value ...]
    let (input, header_line) = not_line_ending(input)?;
    let (input, _) = opt(line_ending).parse(input)?;

    // Parse using a custom approach to extract header_type and properties from header
    let (header_type, mut properties) = parse_header_line(header_line)?;

    // Parse additional property lines that follow the header (until next [ or EOF)
    let (input, body_properties) = parse_section_body(input)?;
    properties.extend(body_properties);

    Ok((
        input,
        Section {
            header_type,
            properties,
        },
    ))
}

fn parse_section_body(input: &str) -> IResult<&str, Vec<UntypedProperty>> {
    let mut remaining = input;
    let mut all_properties = Vec::new();

    loop {
        // Skip empty lines
        let (next_input, _) = multispace0(remaining)?;

        // Check if we hit another section or EOF
        if next_input.is_empty() || next_input.starts_with('[') {
            return Ok((next_input, all_properties));
        }

        // Try to parse a property line (potentially multi-line)
        let (next_input, line) = parse_property_line(next_input)?;

        // Skip empty or comment lines
        if line.trim().is_empty() || line.trim().starts_with(';') {
            remaining = next_input;
            continue;
        }

        // Try to parse properties from this line
        match properties0(&line) {
            Ok((_, props)) => {
                all_properties.extend(props);
            }
            Err(_) => {
                // Not a valid property line, might be end of section
                return Ok((remaining, all_properties));
            }
        }

        remaining = next_input;
    }
}

/// Parses a property line, which may span multiple lines if it contains:
/// - Multi-line quoted strings
/// - Multi-line brace literals {...}
/// - Multi-line bracket literals [...]
fn parse_property_line(input: &str) -> IResult<&str, String> {
    let mut accumulated = String::new();
    let mut remaining = input;
    let mut in_string = false;
    let mut escape_next = false;
    let mut brace_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut line_count = 0;

    loop {
        // Parse one line
        let (next_input, line) = not_line_ending(remaining)?;
        let (next_input, _) = opt(line_ending).parse(next_input)?;

        line_count += 1;
        if line_count > 1 {
            accumulated.push('\n');
        }
        accumulated.push_str(line);

        // Track string state, brace depth, and bracket depth in this line
        for ch in line.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            if in_string {
                match ch {
                    '\\' => escape_next = true,
                    '"' => in_string = false,
                    _ => {}
                }
            } else {
                match ch {
                    '"' => in_string = true,
                    '{' => brace_depth += 1,
                    '}' => brace_depth = brace_depth.saturating_sub(1),
                    '[' => bracket_depth += 1,
                    ']' => bracket_depth = bracket_depth.saturating_sub(1),
                    _ => {}
                }
            }
        }

        // If we're not in a string and all braces/brackets are closed, we're done
        if !in_string && brace_depth == 0 && bracket_depth == 0 {
            return Ok((next_input, accumulated));
        }

        // Continue to next line if we're still inside a structure
        remaining = next_input;

        // Safety check: don't consume entire file
        if remaining.is_empty() {
            // Unclosed structure - return what we have
            return Ok((remaining, accumulated));
        }
    }
}

fn parse_header_line(
    line: &str,
) -> Result<(String, Vec<UntypedProperty>), nom::Err<nom::error::Error<&str>>> {
    use nom::character::complete::char;

    // Start parsing: expect '['
    let (line, _) = char('[')(line)?;

    // Parse the header type (alphanumeric + underscore)
    let (line, header_type) =
        nom::bytes::complete::take_while1(|c: char| c.is_alphanumeric() || c == '_')(line)?;

    // Consume optional whitespace
    let (line, _) = multispace0(line)?;

    // Parse properties until we hit ']'
    let properties = if line.starts_with(']') {
        Vec::new()
    } else {
        // Find the closing bracket, accounting for nested brackets and strings
        let content = extract_until_closing_bracket(line)?;
        match properties0(content) {
            Ok((_, props)) => props,
            Err(_) => Vec::new(),
        }
    };

    Ok((header_type.to_string(), properties))
}

fn extract_until_closing_bracket(input: &str) -> Result<&str, nom::Err<nom::error::Error<&str>>> {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape_next = false;

    for (idx, ch) in input.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '[' if !in_string => depth += 1,
            ']' if !in_string => {
                if depth == 0 {
                    return Ok(&input[..idx]);
                }
                depth -= 1;
            }
            _ => {}
        }
    }

    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Eof,
    )))
}

fn comment_line(input: &str) -> IResult<&str, ()> {
    let (input, line) = not_line_ending(input)?;
    if line.trim().is_empty() || line.trim().starts_with(';') {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

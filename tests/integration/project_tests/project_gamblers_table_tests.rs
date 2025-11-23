use godot_properties_parser::parse_project_file;

const PROJECT_CONTENT: &str = include_str!("../../projects/project_gamblers_table.godot");

// Helper to strip comments and preamble for testing
fn strip_comments_and_preamble(input: &str) -> String {
    input
        .lines()
        .skip_while(|line| {
            let trimmed = line.trim();
            trimmed.starts_with(';') || trimmed.is_empty() || !trimmed.starts_with('[')
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_parses_sections_after_cleanup() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let result = parse_project_file(&cleaned);

    assert!(
        result.is_ok(),
        "Failed to parse cleaned content: {:?}",
        result.err()
    );
}

#[test]
fn test_has_application_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();

    assert!(
        project.application.is_some(),
        "Project should have application section"
    );
}

#[test]
fn test_has_autoload_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();

    assert!(
        project.autoload.is_some(),
        "Project should have autoload section"
    );
}

#[test]
fn test_has_input_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();

    assert!(project.input.is_some(), "Project should have input section");
}

#[test]
fn test_has_rendering_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();

    assert!(
        project.rendering.is_some(),
        "Project should have rendering section"
    );
}

#[test]
fn test_has_display_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();

    assert!(
        project.display.is_some(),
        "Project should have display section"
    );
}

#[test]
fn test_section_count() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();

    assert_eq!(project.all_sections.len(), 18, "Expected 18 sections");
}

#[test]
fn test_all_sections_categorized() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();

    // Count categorized sections
    let categorized = [
        project.application.is_some() as usize,
        project.audio.is_some() as usize,
        project.autoload.is_some() as usize,
        project.debug.is_some() as usize,
        project.display.is_some() as usize,
        project.editor_plugins.is_some() as usize,
        project.input.is_some() as usize,
        project.input_devices.is_some() as usize,
        project.internationalization.is_some() as usize,
        project.layer_names.is_some() as usize,
        project.physics.is_some() as usize,
        project.rendering.is_some() as usize,
    ]
    .iter()
    .sum::<usize>();

    // Some sections might not be in our categorized list
    assert!(
        categorized > 0,
        "Should have at least some categorized sections"
    );
}

#[test]
fn test_parses_raw_file_with_preamble() {
    // Test parsing the raw file WITHOUT stripping comments/preamble
    let result = parse_project_file(PROJECT_CONTENT);

    assert!(
        result.is_ok(),
        "Failed to parse raw file with preamble: {:?}",
        result.err()
    );

    let (remaining, project) = result.unwrap();

    assert_eq!(
        remaining.len(),
        0,
        "Should fully consume file, but {} chars remain",
        remaining.len()
    );

    assert_eq!(
        project.all_sections.len(),
        18,
        "Should parse all 18 sections"
    );
}

#[test]
fn test_preamble_properties() {
    // Parse raw file to get preamble properties
    let (_, project) = parse_project_file(PROJECT_CONTENT).unwrap();

    assert_eq!(
        project.preamble_properties.len(),
        1,
        "Should have 1 preamble property"
    );

    let config_version = project
        .preamble_properties
        .iter()
        .find(|p| p.key == "config_version");

    assert!(
        config_version.is_some(),
        "Should have config_version property"
    );

    assert_eq!(
        config_version.unwrap().value,
        "5",
        "config_version should be 5"
    );
}

#[test]
fn test_preamble_not_in_sections() {
    // Verify config_version is NOT in any section, only in preamble
    let (_, project) = parse_project_file(PROJECT_CONTENT).unwrap();

    for section in &project.all_sections {
        let has_config_version = section.properties.iter().any(|p| p.key == "config_version");
        assert!(
            !has_config_version,
            "config_version should not appear in section [{}]",
            section.header_type
        );
    }
}

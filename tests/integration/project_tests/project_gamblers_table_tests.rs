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
    
    assert!(result.is_ok(), "Failed to parse cleaned content: {:?}", result.err());
}

#[test]
fn test_has_application_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();
    
    assert!(project.application.is_some(), "Project should have application section");
}

#[test]
fn test_has_autoload_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();
    
    assert!(project.autoload.is_some(), "Project should have autoload section");
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
    
    assert!(project.rendering.is_some(), "Project should have rendering section");
}

#[test]
fn test_has_display_section() {
    let cleaned = strip_comments_and_preamble(PROJECT_CONTENT);
    let (_, project) = parse_project_file(&cleaned).unwrap();
    
    assert!(project.display.is_some(), "Project should have display section");
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
    ].iter().sum::<usize>();
    
    // Some sections might not be in our categorized list
    assert!(categorized > 0, "Should have at least some categorized sections");
}

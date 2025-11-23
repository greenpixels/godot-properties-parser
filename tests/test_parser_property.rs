use godot_properties_parser::properties0;

#[test]
fn test_simple_property() {
    let input = "value=2300";
    let (remaining, props) = properties0(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(props.len(), 1);
    assert_eq!(props[0].key, "value");
    assert_eq!(props[0].value, "2300");
}

#[test]
fn test_multiple_properties() {
    let input = "value=2300 something=test";
    let (remaining, props) = properties0(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(props.len(), 2);
    assert_eq!(props[0].key, "value");
    assert_eq!(props[0].value, "2300");
    assert_eq!(props[1].key, "something");
    assert_eq!(props[1].value, "test");
}

#[test]
fn test_array_property() {
    let input = r#"something=["i love","cheese"]"#;
    let (remaining, props) = properties0(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(props.len(), 1);
    assert_eq!(props[0].key, "something");
    assert_eq!(props[0].value, r#"["i love","cheese"]"#);
}

#[test]
fn test_multiline_array() {
    let input = r#" value=2300 something=["i love",
    "cheese"]"#;
    let (remaining, props) = properties0(input).unwrap();
    assert_eq!(remaining, "");
    assert_eq!(props.len(), 2);
    assert_eq!(props[0].key, "value");
    assert_eq!(props[0].value, "2300");
    assert_eq!(props[1].key, "something");
}

use nom::{IResult, bytes::complete::take_while1, character::complete::char};

/// Parses a header like "[gdscene something=[1, 2, 3]]" and returns "gdscene"
/// along with the remaining input after the closing bracket.
#[doc(hidden)]
pub fn parse_header_type_and_consume_enclosure(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('[')(input)?;
    let (input, header_type) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
    let (input, _) = consume_until_matching_bracket(input)?;
    let (input, _) = char(']')(input)?;
    Ok((input, header_type))
}

/// Consumes input until finding the matching closing bracket, accounting for
/// nested brackets and quoted strings.
fn consume_until_matching_bracket(input: &str) -> IResult<&str, ()> {
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
                    return Ok((&input[idx..], ()));
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

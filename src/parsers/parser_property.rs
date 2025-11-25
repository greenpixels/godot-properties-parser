use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, multispace0, multispace1},
    multi::separated_list0,
};

#[derive(Debug, PartialEq, Clone)]
pub struct UntypedProperty {
    pub key: String,
    pub value: String,
}

#[doc(hidden)]
pub fn properties0(input: &str) -> IResult<&str, Vec<UntypedProperty>> {
    let (input, _) = multispace0(input)?;

    let mut parser = separated_list0(multispace1, parse_property);
    parser.parse(input)
}

fn parse_property(input: &str) -> IResult<&str, UntypedProperty> {
    let (input, key) = parse_key(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_value(input)?;

    Ok((
        input,
        UntypedProperty {
            key: key.to_string(),
            value: value.to_string(),
        },
    ))
}

fn parse_key(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '/')(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    let mut parser = alt((
        parse_dict_value,
        parse_array_value,
        parse_quoted_string,
        parse_unquoted_value,
    ));
    parser.parse(input)
}

fn parse_dict_value(input: &str) -> IResult<&str, &str> {
    let start = input;
    let (input, _) = char('{')(input)?;
    let (input, _) = parse_dict_content(input)?;
    let (input, _) = char('}')(input)?;
    let consumed_len = start.len() - input.len();
    Ok((input, &start[..consumed_len]))
}

fn parse_dict_content(input: &str) -> IResult<&str, ()> {
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
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                if depth == 0 {
                    return Ok((&input[idx..], ()));
                }
                depth -= 1;
            }
            _ => {}
        }
    }

    Ok((input, ()))
}

fn parse_array_value(input: &str) -> IResult<&str, &str> {
    let start = input;
    let (input, _) = char('[')(input)?;
    let (input, _) = parse_array_content(input)?;
    let (input, _) = char(']')(input)?;
    let consumed_len = start.len() - input.len();
    Ok((input, &start[..consumed_len]))
}

fn parse_array_content(input: &str) -> IResult<&str, ()> {
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

    Ok((input, ()))
}

fn parse_quoted_string(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('"')(input)?;
    let content_start = input;
    let (input, _) = parse_string_content(input)?;
    let content_len = content_start.len() - input.len();
    let (input, _) = char('"')(input)?;
    Ok((input, &content_start[..content_len]))
}

fn parse_string_content(input: &str) -> IResult<&str, ()> {
    let mut escape_next = false;

    for (idx, ch) in input.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => escape_next = true,
            '"' => return Ok((&input[idx..], ())),
            _ => {}
        }
    }

    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Char,
    )))
}

fn parse_unquoted_value(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && c != ',' && c != ']')(input)
}

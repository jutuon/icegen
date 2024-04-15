
use nom::{
    bytes::complete::{tag, take}, combinator::{not, opt}, multi::many0, sequence::{preceded, tuple}, IResult
};

use crate::parser::whitespace::wsc;

pub fn comma_separated0<T>(
    parser: impl Fn(&str) -> IResult<&str, T>,
    input: &str,
) -> IResult<&str, Vec<T>> {
    comma_separated(parser, false, input)
}

pub fn comma_separated1<T>(
    parser: impl Fn(&str) -> IResult<&str, T>,
    input: &str,
) -> IResult<&str, Vec<T>> {
    comma_separated(parser, true, input)
}

fn comma_separated<T>(
    parser: impl Fn(&str) -> IResult<&str, T>,
    require_first: bool,
    input: &str,
) -> IResult<&str, Vec<T>> {
    let (input, _) = wsc(input)?;
    let (_, _) = not(tag(","))(input)?;

    let (input, first) = if require_first {
        parser(input).map(|(input, v)| (input, Some(v)))
    } else {
        opt(&parser)(input)
    }?;

    let mut parameters = if let Some(first) = first {
        vec![first]
    } else {
        return Ok((input, vec![]));
    };

    let (input, more_parameters) = many0(
        preceded(tuple((wsc, tag(","), wsc)), parser)
    )(input)?;

    parameters.extend(more_parameters);

    Ok((input, parameters))
}


pub fn item_parser<'a, T>(
    end_check: impl Fn(&'a str) -> bool,
    mut item_parser: impl FnMut(&'a str) -> IResult<&'a str, T>,
    input: &'a str,
) -> IResult<&'a str, Vec<T>> {
    let mut all_items = vec![];
    let mut current_input = input;

    loop {
        let mut try_count = 0;
        loop {
            let (input, _) = wsc(current_input)?;
            let (input, items) = many0(
                &mut item_parser
            )(input)?;
            try_count += 1;
            let items_is_empty = items.is_empty();
            all_items.extend(items);
            current_input = input;

            if items_is_empty && try_count > 1 {
                break;
            }
            // Try second time as there can be whitespace in current_input
            // currently.
        }

        if end_check(current_input) {
            break;
        } else {
            let (input, _) = take(1usize)(current_input)?;
            current_input = input;
        }
    }

    Ok((
        current_input,
        all_items,
    ))
}

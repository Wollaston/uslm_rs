use std::str::FromStr;

use winnow::{
    combinator::{delimited, dispatch, fail, peek, preceded},
    stream::AsChar,
    token::{any, take_while},
    PResult, Parser,
};

use crate::{
    common::{parse_attribute_kvs, parse_content},
    header::Attribute,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Tag<'s> {
    tag_type: TagType,
    attributes: Vec<Attribute>,
    content: Option<&'s str>,
    // children: Vec<Tag<'s>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum TagType {
    Property,
    Meta,
    Img,
}

impl FromStr for TagType {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "property" => Ok(TagType::Property),
            "meta" => Ok(TagType::Meta),
            "img" => Ok(TagType::Img),
            _ => panic!("Unknown TagType"),
        }
    }
}

fn parse_tag<'s>(input: &mut &'s str) -> PResult<Tag<'s>> {
    let tag = parse_opening_tag(input)?;
    let attributes = dispatch!(peek(any);
        '>' => parse_close,
        ' ' => parse_open,
        '/' => parse_self_closing,
    _ => fail
    )
    .parse_next(input)?
    .into_attributes();

    let content = parse_content(input).ok();
    parse_closing_tag(input)?;

    Ok(Tag {
        tag_type: tag,
        attributes,
        content,
    })
}

/// Parses the '>' from a tag and returns the empty array
/// required for the Tag's Vec<Attribute>.
fn parse_close<'s>(input: &mut &'s str) -> PResult<Vec<(&'s str, &'s str)>> {
    ">".value(Vec::new()).parse_next(input)
}

fn parse_open<'s>(input: &mut &'s str) -> PResult<Vec<(&'s str, &'s str)>> {
    let output = preceded(" ", parse_attribute_kvs).parse_next(input)?;
    ">".parse_next(input)?;
    Ok(output)
}

fn parse_self_closing<'s>(input: &mut &'s str) -> PResult<Vec<(&'s str, &'s str)>> {
    "/".parse_next(input)?;
    ">".value(Vec::new()).parse_next(input)
}

fn parse_opening_tag<'s>(input: &mut &'s str) -> PResult<TagType> {
    let output = preceded('<', take_while(0.., AsChar::is_alphanum)).parse_next(input)?;
    TagType::from_str(output)
}

fn parse_closing_tag<'s>(input: &mut &'s str) -> PResult<TagType> {
    let output = delimited("</", take_while(0.., AsChar::is_alphanum), '>').parse_next(input)?;
    TagType::from_str(output)
}

trait VecExt {
    fn into_attributes(self) -> Vec<Attribute>;
}

impl VecExt for Vec<(&str, &str)> {
    fn into_attributes(self) -> Vec<Attribute> {
        self.into_iter()
            .map(|(k, v)| match k {
                "name" => Attribute::Name(String::from(v)),
                _ => panic!("Unrecognized attribute"),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag() {
        let mut input = "<property name=&quot;docTitle&quot;>CONTENT</property>";

        let output = parse_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Tag {
                tag_type: TagType::Property,
                attributes: vec![Attribute::Name(String::from("&quot;docTitle&quot;"))],
                content: Some("CONTENT")
            }
        )
    }

    #[test]
    fn test_parse_meta_tag() {
        let mut input = "<meta></meta>";

        let output = parse_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Tag {
                tag_type: TagType::Meta,
                attributes: vec![],
                content: None
            }
        )
    }

    #[test]
    fn test_parse_self_closing() {
        let mut input = "/>";

        let output = parse_self_closing(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![],)
    }

    #[test]
    fn test_parse_open_tag() {
        let mut input = "<property";

        let output = parse_opening_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Property);
    }

    #[test]
    fn test_parse_close_tag() {
        let mut input = "</property>";

        let output = parse_closing_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Property);
    }

    #[test]
    fn test_parse_open() {
        let mut input = " name=&quot;docTitle&quot;>";

        let output = parse_open(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("name", "&quot;docTitle&quot;")]);
    }

    #[test]
    fn test_parse_open_multi() {
        let mut input = " name=&quot;docTitle&quot; second=tag>";

        let output = parse_open(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![("name", "&quot;docTitle&quot;"), ("second", "tag")]
        );
    }

    #[test]
    fn test_parse_close() {
        let mut input = "> CONTENT";

        let output = parse_close(&mut input).unwrap();

        assert_eq!(input, " CONTENT");
        assert_eq!(output, Vec::new());
    }
}

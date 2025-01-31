use std::str::FromStr;

use winnow::{
    combinator::{delimited, dispatch, fail, opt, peek, preceded},
    stream::AsChar,
    token::{any, take_while},
    ModalResult, Parser,
};

use crate::{
    attributes::Attribute,
    common::{parse_attribute_kvs, parse_content, ws},
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Tag<'s> {
    tag_type: TagType,
    attributes: Vec<Attribute<'s>>,
    content: Option<&'s str>,
    children: Vec<Tag<'s>>,
}

fn tag<'s>(input: &mut &'s str) -> ModalResult<Vec<Tag<'s>>> {
    let tag = parse_opening_tag(input)?;

    let attributes = dispatch!(peek(any);
        '>' => parse_close,
        ' ' => parse_open,
        '/' => parse_self_closing,
    _ => fail
    )
    .parse_next(input)?
    .into_attributes();

    opt(ws).parse_next(input)?;

    let children = dispatch!(peek(any);
    '<' => tag,
    _ => Vec::<Tag<'s>>::new(),
    )
    .parse_next(input)?;

    let content = parse_content(input).ok();
    parse_closing_tag(input)?;

    Ok(vec![Tag {
        tag_type: tag,
        attributes,
        content,
        children,
    }])
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TagType {
    Doc(DocTag),
    Meta(MetaTag),
    Standard(StandardTag),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum DocTag {
    LawDoc,
    Bill,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum MetaTag {
    Meta,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum StandardTag {
    Property,
    Img,
}

impl FromStr for TagType {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = match s {
            "lawDoc" | "bill" => TagType::Doc(DocTag::from_str(s)?),
            "meta" => TagType::Meta(MetaTag::from_str(s)?),
            "property" | "img" => TagType::Standard(StandardTag::from_str(s)?),
            _ => panic!("Unknown TagType: {:#?}", s),
        };
        Ok(tag)
    }
}

impl FromStr for DocTag {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lawDoc" => Ok(DocTag::LawDoc),
            "bill" => Ok(DocTag::Bill),
            _ => panic!("Unkown DocTag: {:#?}", s),
        }
    }
}

impl FromStr for MetaTag {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "meta" => Ok(MetaTag::Meta),
            _ => panic!("Unkown MetaTag: {:#?}", s),
        }
    }
}

impl FromStr for StandardTag {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "property" => Ok(StandardTag::Property),
            "img" => Ok(StandardTag::Img),
            _ => panic!("Unkown StandardTag: {:#?}", s),
        }
    }
}

/// Parses the '>' from a tag and returns the empty array
/// required for the Tag's Vec<Attribute<'s>>.
fn parse_close<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    ">".value(Vec::new()).parse_next(input)
}

fn parse_open<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    let output = preceded(" ", parse_attribute_kvs).parse_next(input)?;
    ">".parse_next(input)?;
    Ok(output)
}

fn parse_self_closing<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    "/".parse_next(input)?;
    ">".value(Vec::new()).parse_next(input)
}

fn parse_opening_tag<'s>(input: &mut &'s str) -> ModalResult<TagType> {
    let output = preceded('<', take_while(0.., AsChar::is_alphanum)).parse_next(input)?;
    TagType::from_str(output)
}

fn parse_closing_tag<'s>(input: &mut &'s str) -> ModalResult<TagType> {
    let output = delimited("</", take_while(0.., AsChar::is_alphanum), '>').parse_next(input)?;
    TagType::from_str(output)
}

trait VecExt<'s> {
    fn into_attributes(self) -> Vec<Attribute<'s>>;
}

impl<'s> VecExt<'s> for Vec<(&'s str, &'s str)> {
    fn into_attributes(self) -> Vec<Attribute<'s>> {
        self.into_iter()
            .map(|(k, v)| match k {
                "name" => Attribute::Name(v),
                _ => panic!("Unrecognized attribute"),
            })
            .collect::<Vec<Attribute<'s>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag() {
        let mut input = "<property name=&quot;docTitle&quot;>CONTENT</property>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Standard(StandardTag::Property),
                attributes: vec![Attribute::Name("&quot;docTitle&quot;")],
                content: Some("CONTENT"),
                children: vec![],
            }]
        )
    }

    #[test]
    fn test_parse_meta_tag() {
        let mut input = "<meta></meta>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Meta(MetaTag::Meta),
                attributes: vec![],
                content: None,
                children: vec![],
            }]
        )
    }

    #[test]
    fn test_parse_meta_tag_content() {
        let mut input = "<meta>CONTENT</meta>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Meta(MetaTag::Meta),
                attributes: vec![],
                content: Some("CONTENT"),
                children: vec![],
            }]
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
        assert_eq!(output, TagType::Standard(StandardTag::Property));
    }

    #[test]
    fn test_parse_close_tag() {
        let mut input = "</property>";

        let output = parse_closing_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Standard(StandardTag::Property));
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

use std::{fmt::Debug, str::FromStr};

use winnow::{
    combinator::{alt, delimited, dispatch, fail, opt, peek, preceded, repeat},
    stream::AsChar,
    token::{any, take_while},
    ModalResult, Parser,
};

use crate::{
    attributes::{Attribute, VecExt},
    common::{content, kvs, ws},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'s> {
    pub tag_type: TagType,
    pub attributes: Vec<Attribute<'s>>,
    pub content: Option<&'s str>,
    pub children: Vec<Tag<'s>>,
}

pub fn parse<'s>(input: &mut &'s str) -> ModalResult<Vec<Tag<'s>>> {
    let tags: Vec<Tag<'s>> = repeat(0.., delimited(ws, tag, ws)).parse_next(input)?;
    Ok(tags)
}

fn tag<'s>(input: &mut &'s str) -> ModalResult<Tag<'s>> {
    let tag_type = alt((closing_tag, opening_tag)).parse_next(input)?;

    let attributes = dispatch!(peek(any);
        '>' => tag_close,
        ' ' => tag_open,
        '/' => self_closing_tag,
    _ => fail
    )
    .parse_next(input)?
    .into_attributes();

    let res = if let Some(children) = opt(parse).parse_next(input)? {
        Tag {
            tag_type,
            attributes,
            content: content(input).ok(),
            children,
        }
    } else {
        Tag {
            tag_type,
            attributes,
            content: content(input).ok(),
            children: Vec::new(),
        }
    };

    closing_tag.parse_next(input)?;

    Ok(res)
}

#[derive(Debug, PartialEq, Eq)]
pub enum TagType {
    Doc(DocTag),
    Meta(MetaTag),
    Standard(StandardTag),
}

impl Default for TagType {
    fn default() -> Self {
        Self::Doc(DocTag::default())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DocTag {
    LawDoc,
    Bill,
}

impl Default for DocTag {
    fn default() -> Self {
        Self::LawDoc
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MetaTag {
    Meta,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StandardTag {
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
fn tag_close<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    ">".value(Vec::new()).parse_next(input)
}

fn tag_open<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    let output = preceded(" ", kvs).parse_next(input)?;
    ">".parse_next(input)?;
    Ok(output)
}

fn self_closing_tag<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    "/".parse_next(input)?;
    ">".value(Vec::new()).parse_next(input)
}

fn opening_tag(input: &mut &str) -> ModalResult<TagType> {
    let output = preceded('<', take_while(0.., AsChar::is_alphanum)).parse_next(input)?;
    TagType::from_str(output)
}

fn closing_tag(input: &mut &str) -> ModalResult<TagType> {
    let output = delimited("</", take_while(0.., AsChar::is_alphanum), '>').parse_next(input)?;
    TagType::from_str(output)
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
            Tag {
                tag_type: TagType::Standard(StandardTag::Property),
                attributes: vec![Attribute::Name("&quot;docTitle&quot;")],
                content: Some("CONTENT"),
                children: vec![],
            }
        )
    }

    #[test]
    fn test_parse_meta_tag() {
        let mut input = "<meta></meta>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Tag {
                tag_type: TagType::Meta(MetaTag::Meta),
                attributes: vec![],
                content: None,
                children: vec![],
            }
        )
    }

    #[test]
    fn test_parse_meta_tag_content() {
        let mut input = "<meta>CONTENT</meta>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Tag {
                tag_type: TagType::Meta(MetaTag::Meta),
                attributes: vec![],
                content: Some("CONTENT"),
                children: vec![],
            }
        )
    }

    #[test]
    fn test_parse_self_closing() {
        let mut input = "/>";

        let output = self_closing_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![],)
    }

    #[test]
    fn test_parse_open_tag() {
        let mut input = "<property";

        let output = opening_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Standard(StandardTag::Property));
    }

    #[test]
    fn test_parse_close_tag() {
        let mut input = "</property>";

        let output = closing_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Standard(StandardTag::Property));
    }

    #[test]
    fn test_parse_open() {
        let mut input = " name=&quot;docTitle&quot;>";

        let output = tag_open(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("name", "&quot;docTitle&quot;")]);
    }

    #[test]
    fn test_parse_open_multi() {
        let mut input = " name=&quot;docTitle&quot; second=tag>";

        let output = tag_open(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![("name", "&quot;docTitle&quot;"), ("second", "tag")]
        );
    }

    #[test]
    fn test_parse_close() {
        let mut input = "> CONTENT";

        let output = tag_close(&mut input).unwrap();

        assert_eq!(input, " CONTENT");
        assert_eq!(output, Vec::new());
    }
}

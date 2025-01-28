use std::str::FromStr;

use winnow::{PResult, Parser};

use crate::{
    attributes::Attribute,
    common::{inner, parse_attribute_kvs},
    tags::TagType,
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Doc<'s> {
    tag_type: TagType,
    attributes: Vec<Attribute<'s>>,
    content: Option<&'s str>,
    // children: Vec<Tag<'s>>,
}

impl<'s> Doc<'s> {
    fn parse(input: &mut &'s str) -> PResult<Self> {
        let tag_type = TagType::from_str(inner.parse_next(input)?).unwrap();
        let attributes = parse_attribute_kvs(input)?.into_attributes();
        Ok(Self {
            tag_type,
            attributes,
            content: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::tags::DocTag;

    use super::*;

    #[test]
    fn test_parse_doc_tag() {
        let mut input = r#"bill"#;

        let output = TagType::from_str(inner.parse_next(&mut input).unwrap()).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Doc(DocTag::Bill));
    }

    #[test]
    fn test_parse_attribute_kvs_single() {
        let mut input = r#"xmlns="http://schemas.gpo.gov/xml/uslm""#;

        let output = parse_attribute_kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("xmlns", "http://schemas.gpo.gov/xml/uslm")]);
    }

    #[test]
    fn test_parse_attribute_single() {
        let mut input = r#"xmlns="http://schemas.gpo.gov/xml/uslm""#;

        let output = parse_attribute_kvs(&mut input).unwrap().into_attributes();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Attribute::Xmlns(
                Url::from_str("http://schemas.gpo.gov/xml/uslm").unwrap()
            )]
        );
    }

    #[test]
    fn test_parse_attribute_multiple() {
        let mut input = r#"xmlns="http://schemas.gpo.gov/xml/uslm" xmlns:dc="http://purl.org/dc/elements/1.1/""#;

        let output = parse_attribute_kvs(&mut input).unwrap().into_attributes();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![
                Attribute::Xmlns(Url::from_str("http://schemas.gpo.gov/xml/uslm").unwrap()),
                Attribute::XmlnsDc(Url::from_str("http://purl.org/dc/elements/1.1/").unwrap())
            ]
        );
    }
}

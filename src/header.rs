use std::{error::Error, str::FromStr};
use winnow::{
    combinator::{delimited, repeat},
    ModalResult, Parser,
};

use crate::{
    attributes::{Attribute, VecExt},
    common::{inner, kvs},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Header<'s> {
    pub tags: Vec<HeaderTag<'s>>,
}

impl<'s> Header<'s> {
    pub(super) fn parse(input: &mut &'s str) -> ModalResult<Self> {
        let tags = repeat(0.., delimited("<?", header_tag, "?>")).parse_next(input)?;
        Ok(Header { tags })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeaderTag<'s> {
    pub tag_type: HeaderTagType,
    pub attributes: Vec<Attribute<'s>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HeaderTagType {
    Xml,
    XmlStyleSheet,
}

impl FromStr for HeaderTagType {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "xml" => HeaderTagType::Xml,
            "xml-stylesheet" => HeaderTagType::XmlStyleSheet,
            _ => panic!("Unknown HeaderTagType: {:#?}", s),
        };
        Ok(item)
    }
}

fn header_tag<'s>(input: &mut &'s str) -> ModalResult<HeaderTag<'s>> {
    let tag_type = HeaderTagType::from_str(inner.parse_next(input)?).unwrap();
    let attributes = kvs(input);
    Ok(HeaderTag {
        tag_type,
        attributes: attributes?.into_attributes(),
    })
}

#[cfg(test)]
mod tests {
    use mime::TEXT_CSS;
    use winnow::{ascii::alpha1, error::ContextError};

    use super::*;

    use crate::attributes::{Encoding, Version};

    #[test]
    fn test_xml_tag() {
        let input = r#"xml"#;

        let output = HeaderTagType::from_str(input).unwrap();

        assert_eq!(output, HeaderTagType::Xml);
    }

    #[test]
    fn test_xml_stylesheet_tag() {
        let mut input = r#"xml-stylesheet"#;

        let output = HeaderTagType::from_str(inner.parse_next(&mut input).unwrap()).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, HeaderTagType::XmlStyleSheet);
    }

    #[test]
    fn test_parse_header() {
        let mut input = r#"<?xml?>"#;

        let output = Header::parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Header {
                tags: vec![HeaderTag {
                    tag_type: HeaderTagType::Xml,
                    attributes: vec![],
                }]
            }
        );
    }

    #[test]
    fn test_parse_header_struct_xml() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

        let output = Header::parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Header {
                tags: vec![HeaderTag {
                    tag_type: HeaderTagType::Xml,
                    attributes: vec![
                        Attribute::Version(Version::One),
                        Attribute::Encoding(Encoding::Utf8)
                    ],
                }]
            }
        );
    }

    #[test]
    fn test_parse_header_struct_xml_stylesheet() {
        let mut input = r#"<?xml-stylesheet type="text/css" href="uslm.css"?>"#;

        let output = Header::parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Header {
                tags: vec![HeaderTag {
                    tag_type: HeaderTagType::XmlStyleSheet,
                    attributes: vec![Attribute::StyleType(TEXT_CSS), Attribute::Href("uslm.css"),]
                }]
            }
        );
    }

    #[test]
    fn test_parse_multi_header() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?><?xml-stylesheet type="text/css" href="uslm.css"?>"#;

        let output = Header::parse(&mut input).unwrap();

        assert_eq!(
            output,
            Header {
                tags: vec![
                    HeaderTag {
                        tag_type: HeaderTagType::Xml,
                        attributes: vec![
                            Attribute::Version(Version::One),
                            Attribute::Encoding(Encoding::Utf8)
                        ],
                    },
                    HeaderTag {
                        tag_type: HeaderTagType::XmlStyleSheet,
                        attributes: vec![
                            Attribute::StyleType(TEXT_CSS),
                            Attribute::Href("uslm.css"),
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_delimited() {
        let mut input = r#"<?abc?><?abc?> TEST"#;

        let output: Vec<&str> = repeat(0.., delimited("<?", alpha1::<&str, ContextError>, "?>"))
            .parse_next(&mut input)
            .unwrap();

        assert_eq!(input, " TEST");
        assert_eq!(output, vec!["abc", "abc"]);
    }
}

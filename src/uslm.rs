use header::Header;
use winnow::ModalResult;

use crate::{
    header,
    tags::{self, Tag},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Uslm<'s> {
    pub header: Header<'s>,
    pub content: Vec<Tag<'s>>,
}

impl<'s> Uslm<'s> {
    pub fn parse(input: &mut &'s str) -> ModalResult<Self> {
        let header = Header::parse(input)?;
        let content = tags::parse(input).unwrap_or_default();

        Ok(Uslm { header, content })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use mime::TEXT_CSS;
    use url::Url;

    use crate::{
        attributes::{Attribute, Encoding, Version},
        header::{HeaderTag, HeaderTagType},
        tags::{DocTag, MetaTag, TagType},
    };

    use super::*;

    #[test]
    fn test_parse_uslm() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

        let output = Uslm::parse(&mut input).unwrap();

        assert_eq!(
            output,
            Uslm {
                header: Header {
                    tags: vec![HeaderTag {
                        tag_type: HeaderTagType::Xml,
                        attributes: vec![
                            Attribute::Version(Version::One),
                            Attribute::Encoding(Encoding::Utf8)
                        ],
                    }]
                },
                content: vec![],
            }
        );
    }

    #[test]
    fn test_parse_multi_header_uslm() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?><?xml-stylesheet type="text/css" href="uslm.css"?>"#;

        let output = Uslm::parse(&mut input).unwrap();

        assert_eq!(
            output,
            Uslm {
                header: Header {
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
                },
                content: vec![],
            }
        );
    }

    #[test]
    fn test_example_bill_start() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?><?xml-stylesheet type="text/css" href="uslm.css"?><bill xmlns="http://schemas.gpo.gov/xml/uslm" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:html="http://www.w3.org/1999/xhtml" xmlns:uslm="http://schemas.gpo.gov/xml/uslm" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://schemas.gpo.gov/xml/uslm-2.1.0.xsd" xml:lang="en" id="A1"><meta>CONTENT</meta></bill>"#;

        let output = Uslm::parse(&mut input).unwrap();

        assert_eq!(
            output,
            Uslm {
                header: Header {
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
                },
                content: vec![Tag {
                    tag_type: TagType::Doc(DocTag::Bill),
                    attributes: vec![
                        Attribute::Xmlns(Url::from_str("http://schemas.gpo.gov/xml/uslm").unwrap()),
                        Attribute::XmlnsDc(
                            Url::from_str("http://purl.org/dc/elements/1.1/").unwrap()
                        ),
                        Attribute::XmlnsHtml(
                            Url::from_str("http://www.w3.org/1999/xhtml").unwrap()
                        ),
                        Attribute::XmlnsiUslm(
                            Url::from_str("http://schemas.gpo.gov/xml/uslm").unwrap()
                        ),
                        Attribute::XmlnsiXsi(
                            Url::from_str("http://www.w3.org/2001/XMLSchema-instance").unwrap()
                        ),
                        Attribute::XsiSchemaLocation(
                            Url::from_str("http://schemas.gpo.gov/xml/uslm-2.1.0.xsd").unwrap()
                        ),
                        Attribute::XmlLang("en"),
                        Attribute::Id("A1")
                    ],
                    content: None,
                    children: vec![Tag {
                        tag_type: TagType::Meta(MetaTag::Meta),
                        attributes: vec![],
                        content: Some("CONTENT"),
                        children: vec![]
                    }],
                }],
            }
        );
    }
}

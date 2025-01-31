use header::Header;
use winnow::ModalResult;

use crate::{doc::Doc, header};

#[derive(Debug, PartialEq, Eq)]
pub struct Uslm<'s> {
    pub header: Header<'s>,
    pub doc: Doc<'s>,
}

impl<'s> Uslm<'s> {
    pub fn parse(input: &mut &'s str) -> ModalResult<Self> {
        let header = Header::parse(input)?;
        let doc = Doc::parse(input).unwrap_or_default();

        Ok(Uslm { header, doc })
    }
}

#[cfg(test)]
mod tests {
    use mime::TEXT_CSS;

    use crate::{
        attributes::{Attribute, Encoding, Version},
        header::{HeaderTag, HeaderTagType},
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
                doc: Doc::default(),
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
                doc: Doc::default(),
            }
        );
    }
}

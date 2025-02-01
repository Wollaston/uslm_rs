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
        tags::{Core, Dc, Doc, Meta, Property, TagType},
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
                                Attribute::Type(TEXT_CSS),
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
                                Attribute::Type(TEXT_CSS),
                                Attribute::Href("uslm.css"),
                            ]
                        }
                    ]
                },
                content: vec![Tag {
                    tag_type: TagType::Doc(Doc::Bill),
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
                        tag_type: TagType::Core(Core::Meta),
                        attributes: vec![],
                        content: Some("CONTENT"),
                        children: vec![]
                    }],
                }],
            }
        );
    }

    #[test]
    fn test_example_bill_with_metadata() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?><?xml-stylesheet type="text/css" href="uslm.css"?><bill xmlns="http://schemas.gpo.gov/xml/uslm" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:html="http://www.w3.org/1999/xhtml" xmlns:uslm="http://schemas.gpo.gov/xml/uslm" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://schemas.gpo.gov/xml/uslm" xml:lang="en" id="A1"><meta>
<dc:title>110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes.</dc:title>
<dc:type>Senate Bill</dc:type>
<docNumber>2062</docNumber>
<citableAs>110 S 2062 RIS</citableAs>
<citableAs>110s2062ris</citableAs>
<citableAs>110 S. 2062 RIS</citableAs>
<docStage>Referral Instructions Senate</docStage>
<currentChamber>SENATE</currentChamber>
<dc:creator>United States Senate</dc:creator>
<processedBy>GPO XPub Bill to USLM Generator, version 0.5 + manual changes</processedBy>
<processedDate>2024-09-09</processedDate>
<dc:publisher>United States Government Publishing Office</dc:publisher>
<dc:format>text/xml</dc:format>
<dc:language>EN</dc:language>
<dc:rights>Pursuant to Title 17 Section 105 of the United States Code, this file is not subject to copyright protection and is in the public domain.</dc:rights>
<congress>110</congress>
<session>1</session>
<relatedDocument role="report" href="/us/srpt/110/238" value="CRPT-110srpt238">[Report No. 110–238]</relatedDocument>
<publicPrivate>public</publicPrivate></meta>
</bill>"#;

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
                                Attribute::Type(TEXT_CSS),
                                Attribute::Href("uslm.css"),
                            ]
                        }
                    ]
                },
                content: vec![Tag {
                    tag_type: TagType::Doc(Doc::Bill),
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
                            Url::from_str("http://schemas.gpo.gov/xml/uslm").unwrap()
                        ),
                        Attribute::XmlLang("en"),
                        Attribute::Id("A1")
                    ],
                    content: None,
                    children: vec![Tag {
                        tag_type: TagType::Core(Core::Meta),
                        attributes: vec![],
                        content: None,
                        children: vec![
                            Tag { tag_type: TagType::Meta(Meta::Dc(Dc::Title)), attributes: vec![], content: Some("110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes."), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Dc(Dc::Type)), attributes: vec![], content: Some("Senate Bill"), children: vec![] },
                            Tag { tag_type: TagType::Property(Property::DocNumber), attributes: vec![], content: Some("2062"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::CitableAs), attributes: vec![], content: Some("110 S 2062 RIS"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::CitableAs), attributes: vec![], content: Some("110s2062ris"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::CitableAs), attributes: vec![], content: Some("110 S. 2062 RIS"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::DocStage), attributes: vec![], content: Some("Referral Instructions Senate"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::CurrentChamber), attributes: vec![], content: Some("SENATE"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Dc(Dc::Creator)), attributes: vec![], content: Some("United States Senate"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::ProcessedBy), attributes: vec![], content: Some("GPO XPub Bill to USLM Generator, version 0.5 + manual changes"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::ProcessedDate), attributes: vec![], content: Some("2024-09-09"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Dc(Dc::Publisher)), attributes: vec![], content: Some("United States Government Publishing Office"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Dc(Dc::Format)), attributes: vec![], content: Some("text/xml"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Dc(Dc::Language)), attributes: vec![], content: Some("EN"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Dc(Dc::Rights)), attributes: vec![], content: Some("Pursuant to Title 17 Section 105 of the United States Code, this file is not subject to copyright protection and is in the public domain."), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Congress), attributes: vec![], content: Some("110"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::Session), attributes: vec![], content: Some("1"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::RelatedDocument), attributes: vec![Attribute::Role("report"), Attribute::Href("/us/srpt/110/238"), Attribute::Value("CRPT-110srpt238")], content: Some("[Report No. 110–238]"), children: vec![] },
                            Tag { tag_type: TagType::Meta(Meta::PublicPrivate), attributes: vec![], content: Some("public"), children: vec![] },
                        ]
                    }],
                }],
            }
        );
    }

    #[test]
    fn test_concept_example_header() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

        let output = Uslm::parse(&mut input).unwrap();
        assert_eq!(input, "");
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
                    },]
                },
                content: vec![],
            }
        );
    }

    #[test]
    fn test_concept_example_toc() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?>

        <header>Table of Contents</header>

         <toc>
            <tocItem title="Chapter 1">
               <column>1.</column>
               <column leaders=".">General Provisions</column>
               <column>101</column>
            </tocItem>
         </toc>
"#;

        let output = Uslm::parse(&mut input).unwrap();
        assert_eq!(input, "");
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
                    },]
                },
                content: vec![
                    Tag {
                        tag_type: TagType::Generic(tags::Generic::Header),
                        attributes: vec![],
                        content: Some("Table of Contents"),
                        children: vec![]
                    },
                    Tag {
                        tag_type: TagType::Core(Core::Toc),
                        attributes: vec![],
                        content: None,
                        children: vec![Tag {
                            tag_type: TagType::Core(Core::TocItem),
                            attributes: vec![Attribute::Title("Chapter 1")],
                            content: None,
                            children: vec![
                                Tag {
                                    tag_type: TagType::Generic(tags::Generic::Column),
                                    attributes: vec![],
                                    content: Some("1."),
                                    children: vec![]
                                },
                                Tag {
                                    tag_type: TagType::Generic(tags::Generic::Column),
                                    attributes: vec![Attribute::Leaders(".")],
                                    content: Some("General Provisions"),
                                    children: vec![]
                                },
                                Tag {
                                    tag_type: TagType::Generic(tags::Generic::Column),
                                    attributes: vec![],
                                    content: Some("101"),
                                    children: vec![]
                                },
                            ]
                        }]
                    }
                ],
            }
        )
    }

    #[test]
    fn test_concept_example() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?>

<lawDoc xmlns="http://xml.house.gov/schemas/uslm/1.0" xsi:schemaLocation="http://xml.house.gov/schemas/uslm/1.0" xml:base="http://resolver.mydomain.com" identifier="/us/usc/t5">
   <meta>
      <property name=&quot;docTitle&quot;></property>
   </meta>

   <main>
      <layout>
         <header>Table of Contents</header>
         <toc>
            <tocItem title="Chapter 1">
               <column>1.</column>
               <column leaders=".">General Provisions</column>
               <column>101</column>
            </tocItem>
         </toc>
      </layout>

      <level role=&quot;Chapter&quot;>
         <num value=&quot;1&quot;>CHAPTER 1.</num>
         <heading>General Provisions</heading>
         <content>
         </content>
      </level>
   </main>
</lawDoc>"#;

        let _output = Uslm::parse(&mut input).unwrap();
        assert_eq!(input, "");
    }
}

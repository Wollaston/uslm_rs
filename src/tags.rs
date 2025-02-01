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
    dbg!(&input);
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
    // Dublin Core Metadata Elements
    Dc(Dc),
    Set,
    DocNumber,
    CitableAs,
    DocStage,
    CurrentChamber,
    ProcessedBy,
    ProcessedDate,
    Congress,
    Session,
    RelatedDocument,
    PublicPrivate,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StandardTag {
    Property,
    Img,
}

/// Dublin Core Metadata Elements
#[derive(Debug, PartialEq, Eq)]
pub enum Dc {
    Contributor,
    Coverage,
    Creator,
    Date,
    Description,
    Format,
    Identifier,
    Language,
    Publisher,
    Relation,
    Rights,
    Source,
    Subject,
    Title,
    Type,
}

impl FromStr for Dc {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dc = match s.to_lowercase().as_str() {
            "contributor" => Dc::Contributor,
            "coverage" => Dc::Coverage,
            "creator" => Dc::Creator,
            "date" => Dc::Date,
            "description" => Dc::Description,
            "format" => Dc::Format,
            "identifier" => Dc::Identifier,
            "language" => Dc::Language,
            "publisher" => Dc::Publisher,
            "relation" => Dc::Relation,
            "rights" => Dc::Rights,
            "source" => Dc::Source,
            "subject" => Dc::Subject,
            "title" => Dc::Title,
            "type" => Dc::Type,
            _ => panic!("Unknown Dublin Core variant: {}", s),
        };
        Ok(dc)
    }
}

impl FromStr for TagType {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = match s {
            "lawDoc" | "bill" => TagType::Doc(DocTag::from_str(s)?),
            "meta" | "dc" | "docNumber" | "citableAs" | "docStage" | "currentChamber"
            | "processedBy" | "processedDate" | "congress" | "session" | "relatedDocument"
            | "publicPrivate" => TagType::Meta(MetaTag::from_str(s)?),
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
            "dc" => Ok(MetaTag::Dc(Dc::from_str(s)?)),
            "docNumber" => Ok(MetaTag::DocNumber),
            "citableAs" => Ok(MetaTag::CitableAs),
            "docStage" => Ok(MetaTag::DocStage),
            "currentChamber" => Ok(MetaTag::CurrentChamber),
            "processedBy" => Ok(MetaTag::ProcessedBy),
            "processedDate" => Ok(MetaTag::ProcessedDate),
            "congress" => Ok(MetaTag::Congress),
            "session" => Ok(MetaTag::Session),
            "relatedDocument" => Ok(MetaTag::RelatedDocument),
            "publicPrivate" => Ok(MetaTag::PublicPrivate),
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

fn dc(input: &mut &str) -> ModalResult<TagType> {
    ':'.parse_next(input)?;
    let s = take_while(0.., AsChar::is_alphanum).parse_next(input)?;
    Ok(TagType::Meta(MetaTag::Dc(Dc::from_str(s)?)))
}

fn opening_tag(input: &mut &str) -> ModalResult<TagType> {
    let output = preceded('<', take_while(0.., AsChar::is_alphanum)).parse_next(input)?;
    match output {
        "dc" => dc.parse_next(input),
        _ => TagType::from_str(output),
    }
}

fn closing_tag(input: &mut &str) -> ModalResult<TagType> {
    let output = preceded("</", take_while(0.., AsChar::is_alphanum)).parse_next(input)?;
    match output {
        "dc" => {
            let tag = dc.parse_next(input);
            '>'.parse_next(input)?;
            tag
        }
        _ => {
            '>'.parse_next(input)?;
            TagType::from_str(output)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_bill_block() {
        let mut input = r#"<bill>

        <meta><dc:title>110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes.</dc:title>
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
<publicPrivate>public</publicPrivate>
</meta>

</bill>"#;

        let output = parse.parse_next(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Doc(DocTag::Bill),
                attributes: vec![],
                content: None,
                children: vec![
                 Tag {
                     tag_type: TagType::Meta(MetaTag::Meta),
                     attributes: vec![],
                     content: None,
                     children: vec![
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Title)), attributes: vec![], content: Some("110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes."), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Type)), attributes: vec![], content: Some("Senate Bill"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::DocNumber), attributes: vec![], content: Some("2062"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110 S 2062 RIS"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110s2062ris"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110 S. 2062 RIS"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::DocStage), attributes: vec![], content: Some("Referral Instructions Senate"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CurrentChamber), attributes: vec![], content: Some("SENATE"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Creator)), attributes: vec![], content: Some("United States Senate"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::ProcessedBy), attributes: vec![], content: Some("GPO XPub Bill to USLM Generator, version 0.5 + manual changes"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::ProcessedDate), attributes: vec![], content: Some("2024-09-09"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Publisher)), attributes: vec![], content: Some("United States Government Publishing Office"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Format)), attributes: vec![], content: Some("text/xml"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Language)), attributes: vec![], content: Some("EN"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Rights)), attributes: vec![], content: Some("Pursuant to Title 17 Section 105 of the United States Code, this file is not subject to copyright protection and is in the public domain."), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Congress), attributes: vec![], content: Some("110"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Session), attributes: vec![], content: Some("1"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::RelatedDocument), attributes: vec![Attribute::Role("report"), Attribute::Href("/us/srpt/110/238"), Attribute::Value("CRPT-110srpt238")], content: Some("[Report No. 110–238]"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::PublicPrivate), attributes: vec![], content: Some("public"), children: vec![] },
                     ]
                 }
            ]
            }
            ]
        )
    }

    #[test]
    fn test_parse_meta_block() {
        let mut input = r#"<meta><dc:title>110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes.</dc:title>
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
<publicPrivate>public</publicPrivate>
</meta>"#;

        let output = parse.parse_next(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![
                 Tag {
                     tag_type: TagType::Meta(MetaTag::Meta),
                     attributes: vec![],
                     content: None,
                     children: vec![
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Title)), attributes: vec![], content: Some("110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes."), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Type)), attributes: vec![], content: Some("Senate Bill"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::DocNumber), attributes: vec![], content: Some("2062"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110 S 2062 RIS"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110s2062ris"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110 S. 2062 RIS"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::DocStage), attributes: vec![], content: Some("Referral Instructions Senate"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::CurrentChamber), attributes: vec![], content: Some("SENATE"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Creator)), attributes: vec![], content: Some("United States Senate"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::ProcessedBy), attributes: vec![], content: Some("GPO XPub Bill to USLM Generator, version 0.5 + manual changes"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::ProcessedDate), attributes: vec![], content: Some("2024-09-09"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Publisher)), attributes: vec![], content: Some("United States Government Publishing Office"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Format)), attributes: vec![], content: Some("text/xml"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Language)), attributes: vec![], content: Some("EN"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Rights)), attributes: vec![], content: Some("Pursuant to Title 17 Section 105 of the United States Code, this file is not subject to copyright protection and is in the public domain."), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Congress), attributes: vec![], content: Some("110"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::Session), attributes: vec![], content: Some("1"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::RelatedDocument), attributes: vec![Attribute::Role("report"), Attribute::Href("/us/srpt/110/238"), Attribute::Value("CRPT-110srpt238")], content: Some("[Report No. 110–238]"), children: vec![] },
                        Tag { tag_type: TagType::Meta(MetaTag::PublicPrivate), attributes: vec![], content: Some("public"), children: vec![] },
                     ]
                 }
            ]
        )
    }

    #[test]
    fn test_parse_meta_tags() {
        let mut input = r#"<dc:title>110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes.</dc:title>
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
<publicPrivate>public</publicPrivate>"#;

        let output = parse.parse_next(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
             vec![
                Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Title)), attributes: vec![], content: Some("110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes."), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Type)), attributes: vec![], content: Some("Senate Bill"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::DocNumber), attributes: vec![], content: Some("2062"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110 S 2062 RIS"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110s2062ris"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::CitableAs), attributes: vec![], content: Some("110 S. 2062 RIS"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::DocStage), attributes: vec![], content: Some("Referral Instructions Senate"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::CurrentChamber), attributes: vec![], content: Some("SENATE"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Creator)), attributes: vec![], content: Some("United States Senate"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::ProcessedBy), attributes: vec![], content: Some("GPO XPub Bill to USLM Generator, version 0.5 + manual changes"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::ProcessedDate), attributes: vec![], content: Some("2024-09-09"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Publisher)), attributes: vec![], content: Some("United States Government Publishing Office"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Format)), attributes: vec![], content: Some("text/xml"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Language)), attributes: vec![], content: Some("EN"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Dc(Dc::Rights)), attributes: vec![], content: Some("Pursuant to Title 17 Section 105 of the United States Code, this file is not subject to copyright protection and is in the public domain."), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Congress), attributes: vec![], content: Some("110"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::Session), attributes: vec![], content: Some("1"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::RelatedDocument), attributes: vec![Attribute::Role("report"), Attribute::Href("/us/srpt/110/238"), Attribute::Value("CRPT-110srpt238")], content: Some("[Report No. 110–238]"), children: vec![] },
                Tag { tag_type: TagType::Meta(MetaTag::PublicPrivate), attributes: vec![], content: Some("public"), children: vec![] },
            ]
        )
    }

    #[test]
    fn test_parse_meta_tag_w_kvs_content() {
        let mut input = r#"<relatedDocument role="report" href="/us/srpt/110/238" value="CRPT-110srpt238">[Report No. 110–238]</relatedDocument>"#;

        let output = parse.parse_next(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Meta(MetaTag::RelatedDocument),
                attributes: vec![
                    Attribute::Role("report"),
                    Attribute::Href("/us/srpt/110/238"),
                    Attribute::Value("CRPT-110srpt238")
                ],
                content: Some("[Report No. 110–238]"),
                children: vec![]
            },]
        )
    }

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
    fn test_dc() {
        let mut input = ":title";

        let output = dc(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Meta(MetaTag::Dc(Dc::Title)),);
    }

    #[test]
    fn test_dc_tag() {
        let mut input = "<dc:title></dc:title>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Tag {
                tag_type: TagType::Meta(MetaTag::Dc(Dc::Title)),
                attributes: vec![],
                content: None,
                children: vec![]
            }
        );
    }

    #[test]
    fn test_dc_tag_example() {
        let mut input = "<dc:title>110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes.</dc:title>";

        let output = parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Meta(MetaTag::Dc(Dc::Title)),
                attributes: vec![],
                content: Some("110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes."),
                children: vec![]
            }]
        );
    }

    #[test]
    fn test_dc_tag_content() {
        let mut input = "<dc:title>CONTENT</dc:title>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Tag {
                tag_type: TagType::Meta(MetaTag::Dc(Dc::Title)),
                attributes: vec![],
                content: Some("CONTENT"),
                children: vec![]
            }
        );
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

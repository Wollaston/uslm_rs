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
    Primitive(Primitive),
    Core(Core),
    Generic(Generic),
    Doc(Doc),
    Property(Property),
    Title(Title),
    Level(Level),
    Note(Note),
    Signature(Signature),
    Appendix(Appendix),
    Other(Other),
    Meta(Meta),
    Table(Table),
}

impl FromStr for TagType {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = match s {
            "marker" | "inline" | "block" | "content" => {
                TagType::Primitive(Primitive::from_str(s)?)
            }
            "lawDoc" | "document" | "meta" | "property" | "set" | "toc" | "tocItem" | "main"
            | "statement" | "preamble" | "recital" | "enactingFormula" | "level" | "num"
            | "text" | "heading" | "subheading" | "crossheading" | "instruction" | "action"
            | "notes" | "note" | "appendix" | "signatures" | "signature" | "ref" | "date"
            | "quotedText" | "quotedContent" => TagType::Core(Core::from_str(s)?),
            "layout" | "header" | "row" | "column" | "b" | "i" => {
                TagType::Generic(Generic::from_str(s)?)
            }
            "bill" | "statute" | "resolution" | "amendment" | "uscDoc" => {
                TagType::Doc(Doc::from_str(s)?)
            }
            "docNumber" | "docPublicationName" | "docReleasePoint" => {
                TagType::Property(Property::from_str(s)?)
            }
            "docTitle" | "longTitle" | "shortTitle" => TagType::Title(Title::from_str(s)?),
            "preliminary"
            | "title"
            | "subtitle"
            | "chapter"
            | "subchapter"
            | "part"
            | "subpart"
            | "division"
            | "subdivision"
            | "article"
            | "subarticle"
            | "section"
            | "subsection"
            | "paragraph"
            | "subparagraph"
            | "clause"
            | "subclause"
            | "item"
            | "subitem"
            | "subsubitem"
            | "compiledAct"
            | "courtRules"
            | "courtRule"
            | "reorganizationPlans"
            | "reorganizationPlan" => TagType::Level(Level::from_str(s)?),
            "sourceCredit" | "statutoryNote" | "editorialNote" | "changeNote" => {
                TagType::Note(Note::from_str(s)?)
            }
            "made" | "approved" => TagType::Signature(Signature::from_str(s)?),
            "schedule" => TagType::Appendix(Appendix::from_str(s)?),
            "def" | "term" | "chapeau" | "continuation" | "proviso" => {
                TagType::Other(Other::from_str(s)?)
            }
            "dc" | "citableAs" | "docStage" | "currentChamber" | "processedBy"
            | "processedDate" | "congress" | "session" | "relatedDocument" | "publicPrivate"
            | "img" => TagType::Meta(Meta::from_str(s)?),
            _ => panic!("Unknown TagType: {:#?}", s),
        };
        Ok(tag)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Primitive {
    Marker,
    Inline,
    Block,
    Content,
}

impl FromStr for Primitive {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "marker" => Primitive::Marker,
            "inline" => Primitive::Inline,
            "block" => Primitive::Block,
            "content" => Primitive::Content,
            _ => panic!("Unknown Primitive variant: {}", s),
        };
        Ok(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Core {
    LawDoc,
    Document,
    Meta,
    Property,
    Set,
    Toc,
    TocItem,
    Main,
    Statement,
    Preamble,
    Recital,
    EnactingFormula,
    Level,
    Num,
    Text,
    Heading,
    Subheading,
    Crossheading,
    Instruction,
    Action,
    Notes,
    Note,
    Appendix,
    Signatures,
    Signature,
    Ref,
    Date,
    QuotedText,
    QuotedContent,
}

impl FromStr for Core {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "lawDoc" => Core::LawDoc,
            "document" => Core::Document,
            "meta" => Core::Meta,
            "property" => Core::Property,
            "set" => Core::Set,
            "toc" => Core::Toc,
            "tocItem" => Core::TocItem,
            "main" => Core::Main,
            "statement" => Core::Statement,
            "preamble" => Core::Preamble,
            "recital" => Core::Recital,
            "enactingFormula" => Core::EnactingFormula,
            "level" => Core::Level,
            "num" => Core::Num,
            "text" => Core::Text,
            "heading" => Core::Heading,
            "subheading" => Core::Subheading,
            "crossheading" => Core::Crossheading,
            "instruction" => Core::Instruction,
            "action" => Core::Action,
            "notes" => Core::Notes,
            "note" => Core::Note,
            "appendix" => Core::Appendix,
            "signatures" => Core::Signatures,
            "signature" => Core::Signature,
            "ref" => Core::Ref,
            "date" => Core::Date,
            "quotedText" => Core::QuotedText,
            "quotedContent" => Core::QuotedContent,
            _ => panic!("Unknown Core: {}", s),
        };
        Ok(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Generic {
    Layout,
    Header,
    Row,
    Column,
    B,
    I,
}

impl FromStr for Generic {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "layout" => Generic::Layout,
            "header" => Generic::Header,
            "row" => Generic::Row,
            "column" => Generic::Column,
            "b" => Generic::B,
            "i" => Generic::I,
            _ => panic!("Unknown Generic: {}", s),
        };
        Ok(item)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
pub enum Doc {
    Bill,
    Statute,
    Resolution,
    Amendment,
    UscDoc,
}

impl FromStr for Doc {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "bill" => Doc::Bill,
            "statute" => Doc::Statute,
            "resolution" => Doc::Resolution,
            "amendment" => Doc::Amendment,
            "uscDoc" => Doc::UscDoc,
            _ => panic!("Unkown Doc: {:#?}", s),
        };
        Ok(item)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
pub enum Property {
    DocNumber,
    DocPublicationName,
    DocReleasePoint,
}

impl FromStr for Property {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "docNumber" => Property::DocNumber,
            "docPublicationName" => Property::DocPublicationName,
            "docReleasePoint" => Property::DocReleasePoint,
            _ => panic!("Unkown Property: {:#?}", s),
        };
        Ok(item)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
pub enum Title {
    DocTitle,
    LongTitle,
    ShortTitle,
}

impl FromStr for Title {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "docTitle" => Title::DocTitle,
            "longTitle" => Title::LongTitle,
            "shortTitle" => Title::ShortTitle,
            _ => panic!("Unkown Title: {:#?}", s),
        };
        Ok(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    Preliminary,
    Title,
    Subtitle,
    Chapter,
    Subchapter,
    Part,
    Subpart,
    Division,
    Subdivision,
    Article,
    Subarticle,
    Section,
    Subsection,
    Paragraph,
    Subparagraph,
    Clause,
    Subclause,
    Item,
    Subitem,
    Subsubitem,
    CompiledAct,
    CourtRules,
    CourtRule,
    ReorganizationPlans,
    ReorganizationPlan,
}

impl FromStr for Level {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "preliminary" => Level::Preliminary,
            "title" => Level::Title,
            "subtitle" => Level::Subtitle,
            "chapter" => Level::Chapter,
            "subchapter" => Level::Subchapter,
            "part" => Level::Part,
            "subpart" => Level::Subpart,
            "division" => Level::Division,
            "subdivision" => Level::Subdivision,
            "article" => Level::Article,
            "subarticle" => Level::Subarticle,
            "section" => Level::Section,
            "subsection" => Level::Subsection,
            "paragraph" => Level::Paragraph,
            "subparagraph" => Level::Subparagraph,
            "clause" => Level::Clause,
            "subclause" => Level::Subclause,
            "item" => Level::Item,
            "subitem" => Level::Subitem,
            "subsubitem" => Level::Subsubitem,
            "compiledAct" => Level::CompiledAct,
            "courtRules" => Level::CourtRules,
            "courtRule" => Level::CourtRule,
            "reorganizationPlans" => Level::ReorganizationPlans,
            "reorganizationPlan" => Level::ReorganizationPlan,
            _ => panic!("Unkown Level: {:#?}", s),
        };
        Ok(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Other {
    Def,
    Term,
    Chapeau,
    Continuation,
    Proviso,
}

impl FromStr for Other {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "def" => Other::Def,
            "term" => Other::Term,
            "chapeau" => Other::Chapeau,
            "continuation" => Other::Continuation,
            "proviso" => Other::Proviso,
            _ => panic!("Unkown Other: {:#?}", s),
        };
        Ok(item)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
pub enum Note {
    SourceCredit,
    StatutoryNote,
    EditorialNote,
    ChangeNote,
}

impl FromStr for Note {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "sourceCredit" => Note::SourceCredit,
            "statutoryNote" => Note::StatutoryNote,
            "editorialNote" => Note::EditorialNote,
            "changeNote" => Note::ChangeNote,
            _ => panic!("Unkown Note: {:#?}", s),
        };
        Ok(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Signature {
    Made,
    Approved,
}

impl FromStr for Signature {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "made" => Signature::Made,
            "approved" => Signature::Approved,
            _ => panic!("Unkown Signature: {:#?}", s),
        };
        Ok(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Appendix {
    Schedule,
}

impl FromStr for Appendix {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "schedule" => Appendix::Schedule,
            _ => panic!("Unkown Appendix: {:#?}", s),
        };
        Ok(item)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Meta {
    // Dublin Core Metadata Elements
    Dc(Dc),
    DocStage,
    DocPublicationName,
    DocReleasePoint,
    CitableAs,
    CurrentChamber,
    ProcessedBy,
    ProcessedDate,
    Congress,
    Session,
    RelatedDocument,
    PublicPrivate,
    Img,
}

impl FromStr for Meta {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "dc" => Meta::Dc(Dc::from_str(s)?),
            "citableAs" => Meta::CitableAs,
            "docStage" => Meta::DocStage,
            "currentChamber" => Meta::CurrentChamber,
            "processedBy" => Meta::ProcessedBy,
            "processedDate" => Meta::ProcessedDate,
            "congress" => Meta::Congress,
            "session" => Meta::Session,
            "relatedDocument" => Meta::RelatedDocument,
            "publicPrivate" => Meta::PublicPrivate,
            "img" => Meta::Img,
            _ => panic!("Unkown Meta: {:#?}", s,),
        };
        Ok(item)
    }
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
        let item = match s {
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
            _ => panic!("Unknown Dublin Core: {}", s),
        };
        Ok(item)
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
pub enum Table {
    Table,
    Th,
    Tr,
}

impl FromStr for Table {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item = match s {
            "table" => Table::Table,
            "th" => Table::Th,
            "tr" => Table::Tr,
            _ => panic!("Unkown Meta: {:#?}", s,),
        };
        Ok(item)
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
    Ok(TagType::Meta(Meta::Dc(Dc::from_str(s)?)))
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

    use crate::tags;

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
                tag_type: TagType::Doc(Doc::Bill),
                attributes: vec![],
                content: None,
                children: vec![
                 Tag {
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
                tag_type: TagType::Meta(Meta::RelatedDocument),
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
                tag_type: TagType::Core(Core::Property),
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
                tag_type: TagType::Core(Core::Meta),
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
                tag_type: TagType::Core(Core::Meta),
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
        assert_eq!(output, TagType::Core(Core::Property));
    }

    #[test]
    fn test_parse_close_tag() {
        let mut input = "</property>";

        let output = closing_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Core(Core::Property));
    }

    #[test]
    fn test_dc() {
        let mut input = ":title";

        let output = dc(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, TagType::Meta(Meta::Dc(Dc::Title)),);
    }

    #[test]
    fn test_dc_tag() {
        let mut input = "<dc:title></dc:title>";

        let output = tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            Tag {
                tag_type: TagType::Meta(Meta::Dc(Dc::Title)),
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
                tag_type: TagType::Meta(Meta::Dc(Dc::Title)),
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
                tag_type: TagType::Meta(Meta::Dc(Dc::Title)),
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

    #[test]
    fn test_toc_columns() {
        let mut input = r#"<column>1.</column>               <column leaders=".">General Provisions</column>
        <column>101</column>"#;

        let output = parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![
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
        );
    }

    #[test]
    fn test_toc_item() {
        let mut input = r#"<tocItem></tocItem>"#;

        let output = parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Core(Core::TocItem),
                attributes: vec![],
                content: None,
                children: vec![],
            },]
        );
    }

    #[test]
    fn test_toc_item_attribute() {
        let mut input = r#"<tocItem title="Chapter 1"></tocItem>"#;

        let output = parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
                tag_type: TagType::Core(Core::TocItem),
                attributes: vec![Attribute::Title("Chapter 1")],
                content: None,
                children: vec![],
            },]
        );
    }

    #[test]
    fn test_toc_item_with_attribute() {
        let mut input = r#"<tocItem title="Chapter 1">               <column>1.</column>               <column leaders=".">General Provisions</column>
        <column>101</column>            </tocItem>"#;

        let output = parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![Tag {
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
        );
    }

    #[test]
    fn test_law_doc() {
        let mut input = r#"<lawDoc xmlns="http://xml.house.gov/schemas/uslm/1.0" xsi:schemaLocation="http://xml.house.gov/schemas/uslm/1.0" xml:base="http://resolver.mydomain.com" identifier="/us/usc/t5">
</lawDoc>"#;

        let _output = parse(&mut input).unwrap();
        assert_eq!(input, "");
    }
}

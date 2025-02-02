use std::{error::Error, str::FromStr};

use mime::Mime;
use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute<'s> {
    Version(Version),
    Encoding(Encoding),
    Name(&'s str),
    Type(Mime),
    Style(&'s str),
    StyleType(&'s str),
    Href(&'s str),
    Xmlns(Url),
    XmlBase(Url),
    XmlnsDc(Url),
    XmlnsHtml(Url),
    XmlnsiUslm(Url),
    XmlnsiXsi(Url),
    XsiSchemaLocation(Url),
    XmlLang(&'s str),
    Id(&'s str),
    Role(&'s str),
    Value(&'s str),
    StartValue(&'s str),
    EndValue(&'s str),
    Display(&'s str),
    Class(&'s str),
    Identifier(&'s str),
    SenateId(&'s str),
    Leaders(&'s str),
    Title(&'s str),
    Status(&'s str),
    TemporalId(&'s str),
    Pos(&'s str),
    PosText(&'s str),
    PosCount(&'s str),
    Idref(&'s str),
    Src(&'s str),
    Note(&'s str),
    Alt(&'s str),
    Meta(&'s str),
    Misc(&'s str),
    DraftingTip(&'s str),
    CodificationTip(&'s str),
    Brief(&'s str),
    SortOrder(&'s str),
    Portion(&'s str),
    Occurrence(&'s str),
    CommencementDate(&'s str),
    Date(&'s str),
    BeginDate(&'s str),
    EndDate(&'s str),
    StartPeriod(&'s str),
    EndPeriod(&'s str),
    Partial(&'s str),
    ColSpan(&'s str),
    RowSpan(&'s str),
    Topic(&'s str),
    Orientation(&'s str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    One,
}

impl FromStr for Version {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.0" => Ok(Version::One),
            _ => panic!("Unrecognized xml version"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
}

impl FromStr for Encoding {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UTF-8" => Ok(Encoding::Utf8),
            _ => panic!("Unrecognized Encoding"),
        }
    }
}

pub(super) trait VecExt<'s> {
    fn into_attributes(self) -> Vec<Attribute<'s>>;
}

impl<'s> VecExt<'s> for Vec<(&'s str, &'s str)> {
    fn into_attributes(self) -> Vec<Attribute<'s>> {
        self.into_iter()
            .flat_map(|(k, v)| -> Result<Attribute<'_>, Box<dyn Error>> {
                let attribute = match k {
                    "version" => Attribute::Version(Version::from_str(v)?),
                    "encoding" => Attribute::Encoding(Encoding::from_str(v)?),
                    "name" => Attribute::Name(v),
                    "type" => Attribute::Type(Mime::from_str(v).unwrap()),
                    "style" => Attribute::Style(v),
                    "styleType" => Attribute::StyleType(v),
                    "href" => Attribute::Href(v),
                    "xmlns" => Attribute::Xmlns(Url::from_str(v)?),
                    "xml:base" => Attribute::XmlBase(Url::from_str(v)?),
                    "xmlns:dc" => Attribute::XmlnsDc(Url::from_str(v)?),
                    "xmlns:html" => Attribute::XmlnsHtml(Url::from_str(v)?),
                    "xmlns:uslm" => Attribute::XmlnsiUslm(Url::from_str(v)?),
                    "xmlns:xsi" => Attribute::XmlnsiXsi(Url::from_str(v)?),
                    "xsi:schemaLocation" => Attribute::XsiSchemaLocation(Url::from_str(v)?),
                    "xml:lang" => Attribute::XmlLang(v),
                    "id" => Attribute::Id(v),
                    "role" => Attribute::Role(v),
                    "value" => Attribute::Value(v),
                    "startValue" => Attribute::StartValue(v),
                    "endValue" => Attribute::EndValue(v),
                    "display" => Attribute::Display(v),
                    "class" => Attribute::Class(v),
                    "identifier" => Attribute::Identifier(v),
                    "senateId" => Attribute::SenateId(v),
                    "leaders" => Attribute::Leaders(v),
                    "title" => Attribute::Title(v),
                    "status" => Attribute::Status(v),
                    "temporalId" => Attribute::TemporalId(v),
                    "pos" => Attribute::Pos(v),
                    "posText" => Attribute::PosText(v),
                    "posCount" => Attribute::PosCount(v),
                    "idref" => Attribute::Idref(v),
                    "src" => Attribute::Src(v),
                    "note" => Attribute::Note(v),
                    "alt" => Attribute::Alt(v),
                    "meta" => Attribute::Meta(v),
                    "misc" => Attribute::Misc(v),
                    "draftingTip" => Attribute::DraftingTip(v),
                    "codificationTip" => Attribute::CodificationTip(v),
                    "brief" => Attribute::Brief(v),
                    "sortOrder" => Attribute::SortOrder(v),
                    "portion" => Attribute::Portion(v),
                    "occurrence" => Attribute::Occurrence(v),
                    "commencementDate" => Attribute::CommencementDate(v),
                    "date" => Attribute::Date(v),
                    "beginDate" => Attribute::BeginDate(v),
                    "endDate" => Attribute::EndDate(v),
                    "startPeriod" => Attribute::StartPeriod(v),
                    "endPeriod" => Attribute::EndPeriod(v),
                    "partial" => Attribute::Partial(v),
                    "colspan" => Attribute::ColSpan(v),
                    "rowspan" => Attribute::RowSpan(v),
                    "topic" => Attribute::Topic(v),
                    "orientation" => Attribute::Orientation(v),
                    _ => panic!("Unrecognized doc attribute variant: {:#?}", v),
                };
                Ok(attribute)
            })
            .collect()
    }
}

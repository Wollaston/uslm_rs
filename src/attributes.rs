use std::{error::Error, str::FromStr};

use mime::Mime;
use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute<'s> {
    Version(Version),
    Encoding(Encoding),
    Name(&'s str),
    StyleType(Mime),
    Href(&'s str),
    Xmlns(Url),
    XmlnsDc(Url),
    XmlnsHtml(Url),
    XmlnsiUslm(Url),
    XmlnsiXsi(Url),
    XsiSchemaLocation(Url),
    XmlLang(&'s str),
    Id(&'s str),
    Role(&'s str),
    Value(&'s str),
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
                    "type" => Attribute::StyleType(Mime::from_str(v)?),
                    "href" => Attribute::Href(v),
                    "xmlns" => Attribute::Xmlns(Url::from_str(v)?),
                    "xmlns:dc" => Attribute::XmlnsDc(Url::from_str(v)?),
                    "xmlns:html" => Attribute::XmlnsHtml(Url::from_str(v)?),
                    "xmlns:uslm" => Attribute::XmlnsiUslm(Url::from_str(v)?),
                    "xmlns:xsi" => Attribute::XmlnsiXsi(Url::from_str(v)?),
                    "xsi:schemaLocation" => Attribute::XsiSchemaLocation(Url::from_str(v)?),
                    "xml:lang" => Attribute::XmlLang(v),
                    "id" => Attribute::Id(v),
                    "role" => Attribute::Role(v),
                    "value" => Attribute::Value(v),
                    _ => panic!("Unrecognized doc attribute variant: {:#?}", v),
                };
                Ok(attribute)
            })
            .collect()
    }
}

use std::{error::Error, str::FromStr};

use mime::Mime;
use url::Url;
use winnow::{
    combinator::delimited,
    stream::AsChar,
    token::{literal, take_till},
    PResult, Parser,
};

use crate::common::parse_attribute_kvs;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Header<'s> {
    tags: Vec<HeaderTag<'s>>,
}

#[derive(Debug, PartialEq, Eq)]
enum HeaderTag<'s> {
    Xml { attributes: Vec<Attribute<'s>> },
    XmlStyleSheet { attributes: Vec<Attribute<'s>> },
    Bill { attributes: Vec<Attribute<'s>> },
}

impl<'s> Header<'s> {
    pub(super) fn parse(input: &mut &str) -> PResult<Self> {
        let (doc_format, doc_attributes) = parse_header(input)?;

        let format = Format::from_str(doc_format).unwrap();

        Ok(Self { format, attributes })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Format {
    Xml,
}

impl FromStr for Format {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?xml" => Ok(Format::Xml),
            _ => panic!("Unrecognized xml version"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Attribute<'s> {
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
}

#[derive(Debug, PartialEq, Eq)]
enum Version {
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
enum Encoding {
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

fn parse_header<'s>(input: &mut &'s str) -> PResult<(&'s str, Vec<(&'s str, &'s str)>)> {
    delimited("<?", parse_header_tag, "?>").parse_next(input)
}

fn parse_header_tag<'s, T>(input: &mut &'s str) -> PResult<HeaderTag>
where
    T: HeaderTag,
{
    literal("xml ").parse_next(input)?;
    let attributes = parse_attribute_kvs(input)?;

    Ok((xml, attributes))
}

trait VecExt<'s> {
    fn into_attributes(self) -> Vec<Attribute<'s>>;
}

impl<'s> VecExt<'s> for Vec<(&'s str, &'s str)> {
    fn into_attributes(self) -> Vec<Attribute<'s>> {
        self.into_iter()
            .map(|(k, v)| match k {
                "version" => Attribute::Version(Version::from_str(v).unwrap()),
                "encoding" => Attribute::Encoding(Encoding::from_str(v).unwrap()),
                "name" => Attribute::Name(v),
                "type" => Attribute::StyleType(Mime::from_str(v).unwrap()),
                "href" => Attribute::Href(v),
                "xmlns" => Attribute::Xmlns(Url::from_str(v).unwrap()),
                "xmlns:dc" => Attribute::XmlnsDc(Url::from_str(v).unwrap()),
                "xmlns:html" => Attribute::XmlnsHtml(Url::from_str(v).unwrap()),
                "xmlns:uslm" => Attribute::XmlnsiUslm(Url::from_str(v).unwrap()),
                "xmlns:xsi" => Attribute::XmlnsiXsi(Url::from_str(v).unwrap()),
                "xsi:schemaLocation" => Attribute::XsiSchemaLocation(Url::from_str(v).unwrap()),
                "xml:lang" => Attribute::XmlLang(v),
                "id" => Attribute::Id(v),
                _ => panic!("Unrecognized doc attribute variant: {:#?}", v),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Uslm;

    use super::*;

    #[test]
    fn test_xml_literal() {
        let mut input = "?xml";

        let output = parse_xml(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, "?xml");
    }

    #[test]
    fn test_xml_literal_extended() {
        let mut input = "?xml_test_me";

        let output = parse_xml(&mut input).unwrap();

        assert_eq!(input, "_test_me");
        assert_eq!(output, "?xml");
    }

    #[test]
    fn test_parse_header_tags() {
        let mut input = r#"?xml version="1.0" encoding="UTF-8""#;

        let output = parse_header_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            ("?xml", vec![("version", "1.0"), ("encoding", r#"UTF-8"#)])
        );
    }

    #[test]
    fn test_parse_header() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

        let output = parse_header(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            ("?xml", vec![("version", "1.0"), ("encoding", r#"UTF-8"#)])
        );
    }

    #[test]
    fn test_parse_header_struct() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

        let output = Header::parse(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            Header {
                format: Format::Xml,
                attributes: vec![
                    Attribute::Version(Version::One),
                    Attribute::Encoding(Encoding::Utf8)
                ]
            },
            output
        );
    }

    #[test]
    fn test_parse_uslm() {
        let mut input = r#"<?xml version="1.0" encoding="UTF-8?">

<lawDoc       
     xmlns=http://xml.house.gov/schemas/uslm/1.0
	xsi:schemaLocation"http://xml.house.gov/schemas/uslm/1.0
       ./USLM-1.0.xsd"
	 xml:base="http://resolver.mydomain.com"
     identifier="/us/usc/t5">
   <meta>
      <property name=&quot;docTitle&quot;>…</property>
      …
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
            ...
         </content>
      </level>
   </main>
</lawDoc>"#;

        let output = Uslm::parse(&mut input).unwrap();

        assert_eq!(
            Uslm {
                header: Header {
                    format: Format::Xml,
                    attributes: vec![
                        Attribute::Version(Version::One),
                        Attribute::Encoding(Encoding::Utf8)
                    ]
                },
            },
            output
        );
    }
}

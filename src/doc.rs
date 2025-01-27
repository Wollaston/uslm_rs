use std::str::FromStr;

use winnow::{
    ascii::alpha1,
    combinator::{dispatch, fail, peek},
    stream::AsChar,
    token::take_while,
    PResult, Parser,
};

fn parse_doc_tag<'s>(input: &mut &'s str) -> PResult<DocTag> {
    "<".parse_next(input)?;
    dispatch!(peek(alpha1);
        "lawDoc" => parse_law_doc,
    _ => fail
    )
    .parse_next(input)
}

fn parse_law_doc<'s>(input: &mut &'s str) -> PResult<DocTag> {
    let output = take_while(0.., AsChar::is_alpha).parse_next(input)?;
    DocTag::from_str(output)
}

#[derive(Debug, PartialEq, Eq)]
enum DocTag {
    LawDoc,
}

impl FromStr for DocTag {
    type Err = winnow::error::ErrMode<winnow::error::ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lawDoc" => Ok(DocTag::LawDoc),
            _ => panic!("Unkown DocTag: {:#?}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_doc_tag_law_doc() {
        let mut input = r#"<lawDoc"#;

        let output = parse_doc_tag(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, DocTag::LawDoc);
    }

    #[test]
    fn test_parse_law_doc() {
        let mut input = r#"lawDoc"#;

        let output = parse_law_doc(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, DocTag::LawDoc);
    }
}

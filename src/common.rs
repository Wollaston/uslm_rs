use winnow::{
    combinator::{alt, delimited, opt, separated, separated_pair},
    token::take_while,
    ModalResult, Parser,
};

pub(super) fn content<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_while(
        1..,
        (
            'a'..='z',
            'A'..='Z',
            '0'..='9',
            '-',
            '=',
            '+',
            '.',
            ',',
            '&',
            ';',
            '\n',
            '\r',
            '\t',
            ' ',
            ':',
            '/',
            '[',
            ']',
            '–',
        ),
    )
    .parse_next(input)
}

pub(super) fn key<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    inner.parse_next(input)
}

pub(super) fn value<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alt((with_quotes, inner)).parse_next(input)
}

fn with_quotes<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    delimited('"', inner, '"').parse_next(input)
}

pub(super) fn inner<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_while(
        1..,
        (
            'a'..='z',
            'A'..='Z',
            '0'..='9',
            '-',
            '.',
            '&',
            ';',
            '/',
            ':',
        ),
    )
    .parse_next(input)
}

pub(super) fn kv<'s>(input: &mut &'s str) -> ModalResult<(&'s str, &'s str)> {
    separated_pair(key, '=', value).parse_next(input)
}

pub(super) fn kvs<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    opt(' ').parse_next(input)?;
    separated(0.., kv, ' ').parse_next(input)
}

pub(crate) fn ws<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_while(0.., WS).parse_next(input)
}

const WS: &[char] = &[' ', '\t', '\r', '\n'];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attribute_key() {
        let mut input = r#"encoding="UTF-8""#;

        let output = key(&mut input).unwrap();

        assert_eq!(input, r#"="UTF-8""#);
        assert_eq!(output, "encoding");
    }

    #[test]
    fn test_parse_attribute_key_name() {
        let mut input = r#"name="UTF-8""#;

        let output = key(&mut input).unwrap();

        assert_eq!(input, r#"="UTF-8""#);
        assert_eq!(output, "name");
    }

    #[test]
    fn test_parse_attribute_value() {
        let mut input = r#""UTF-8""#;

        let output = value(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, r#"UTF-8"#);
    }

    #[test]
    fn test_parse_attribute_value_escaped() {
        let mut input = r#"&quot;docTitle&quot;"#;

        let output = value(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, r#"&quot;docTitle&quot;"#);
    }

    #[test]
    fn test_parse_attribute_kv() {
        let mut input = r#"encoding="UTF-8""#;

        let output = kv(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, ("encoding", r#"UTF-8"#));
    }

    #[test]
    fn test_parse_attribute_kvs_single() {
        let mut input = r#"version="1.0""#;

        let output = kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("version", "1.0")]);
    }

    #[test]
    fn test_parse_attribute_kvs_escaped() {
        let mut input = "name=&quot;docTitle&quot;";

        let output = kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("name", "&quot;docTitle&quot;")]);
    }

    #[test]
    fn test_parse_attribute_kvs_multiple() {
        let mut input = r#"version="1.0" encoding="UTF-8""#;

        let output = kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("version", "1.0"), ("encoding", r#"UTF-8"#)]);
    }

    #[test]
    fn test_example_content() {
        let mut input = "110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes.";

        let output = content.parse_next(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, "110 S 2062 RIS: To amend the Native American Housing Assistance and Self-Determination Act of 1996 to reauthorize that Act, and for other purposes.");
    }

    #[test]
    fn test_bracketed_content() {
        let mut input = "[Report No. 110–238]";

        let output = content.parse_next(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, "[Report No. 110–238]");
    }

    #[test]
    fn test_metadata_kvs() {
        let mut input = r#"role="report" href="/us/srpt/110/238" value="CRPT-110srpt238""#;

        let output = kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(
            output,
            vec![
                ("role", "report"),
                ("href", r#"/us/srpt/110/238"#),
                ("value", r#"CRPT-110srpt238"#)
            ]
        );
    }
}

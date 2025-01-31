use winnow::{
    combinator::{alt, delimited, opt, separated, separated_pair},
    token::take_while,
    ModalResult, Parser,
};

pub(super) fn parse_content<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    take_while(
        1..,
        (
            'a'..='z',
            'A'..='Z',
            '0'..='9',
            '-',
            '=',
            '.',
            '&',
            ';',
            '\n',
            '\r',
            '\t',
            ' ',
        ),
    )
    .parse_next(input)
}

pub(super) fn parse_attribute_key<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    inner.parse_next(input)
}

pub(super) fn parse_attribute_value<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alt((parse_with_quotes, inner)).parse_next(input)
}

fn parse_with_quotes<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
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

pub(super) fn parse_attribute_kv<'s>(input: &mut &'s str) -> ModalResult<(&'s str, &'s str)> {
    separated_pair(parse_attribute_key, '=', parse_attribute_value).parse_next(input)
}

pub(super) fn parse_attribute_kvs<'s>(input: &mut &'s str) -> ModalResult<Vec<(&'s str, &'s str)>> {
    opt(' ').parse_next(input)?;
    separated(0.., parse_attribute_kv, ' ').parse_next(input)
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

        let output = parse_attribute_key(&mut input).unwrap();

        assert_eq!(input, r#"="UTF-8""#);
        assert_eq!(output, "encoding");
    }

    #[test]
    fn test_parse_attribute_key_name() {
        let mut input = r#"name="UTF-8""#;

        let output = parse_attribute_key(&mut input).unwrap();

        assert_eq!(input, r#"="UTF-8""#);
        assert_eq!(output, "name");
    }

    #[test]
    fn test_parse_attribute_value() {
        let mut input = r#""UTF-8""#;

        let output = parse_attribute_value(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, r#"UTF-8"#);
    }

    #[test]
    fn test_parse_attribute_value_escaped() {
        let mut input = r#"&quot;docTitle&quot;"#;

        let output = parse_attribute_value(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, r#"&quot;docTitle&quot;"#);
    }

    #[test]
    fn test_parse_attribute_kv() {
        let mut input = r#"encoding="UTF-8""#;

        let output = parse_attribute_kv(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, ("encoding", r#"UTF-8"#));
    }

    #[test]
    fn test_parse_attribute_kvs_single() {
        let mut input = r#"version="1.0""#;

        let output = parse_attribute_kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("version", "1.0")]);
    }

    #[test]
    fn test_parse_attribute_kvs_escaped() {
        let mut input = "name=&quot;docTitle&quot;";

        let output = parse_attribute_kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("name", "&quot;docTitle&quot;")]);
    }

    #[test]
    fn test_parse_attribute_kvs_multiple() {
        let mut input = r#"version="1.0" encoding="UTF-8""#;

        let output = parse_attribute_kvs(&mut input).unwrap();

        assert_eq!(input, "");
        assert_eq!(output, vec![("version", "1.0"), ("encoding", r#"UTF-8"#)]);
    }
}

use std::ops::RangeInclusive;
use winnow::combinator::alt;
use winnow::combinator::eof;
use winnow::combinator::opt;
use winnow::combinator::repeat;
use winnow::combinator::terminated;
use winnow::prelude::*;
use winnow::token::one_of;
use winnow::token::take_while;
use crate::parser::prelude::*;
pub(crate) unsafe fn from_utf8_unchecked<'b>(
    bytes: &'b [u8],
    safety_justification: &'static str,
) -> &'b str {
    if cfg!(debug_assertions) {
        std::str::from_utf8(bytes).expect(safety_justification)
    } else {
        std::str::from_utf8_unchecked(bytes)
    }
}
pub(crate) const WSCHAR: (u8, u8) = (b' ', b'\t');
pub(crate) fn ws(input: Input<'_>) -> IResult<Input<'_>, &str, ParserError<'_>> {
    take_while(0.., WSCHAR)
        .map(|b| unsafe { from_utf8_unchecked(b, "`is_wschar` filters out on-ASCII") })
        .parse_next(input)
}
pub(crate) const NON_ASCII: RangeInclusive<u8> = 0x80..=0xff;
pub(crate) const NON_EOL: (u8, RangeInclusive<u8>, RangeInclusive<u8>) = (
    0x09,
    0x20..=0x7E,
    NON_ASCII,
);
pub(crate) const COMMENT_START_SYMBOL: u8 = b'#';
pub(crate) fn comment(input: Input<'_>) -> IResult<Input<'_>, &[u8], ParserError<'_>> {
    (COMMENT_START_SYMBOL, take_while(0.., NON_EOL)).recognize().parse_next(input)
}
pub(crate) fn newline(input: Input<'_>) -> IResult<Input<'_>, u8, ParserError<'_>> {
    alt((one_of(LF).value(b'\n'), (one_of(CR), one_of(LF)).value(b'\n')))
        .parse_next(input)
}
pub(crate) const LF: u8 = b'\n';
pub(crate) const CR: u8 = b'\r';
pub(crate) fn ws_newline(input: Input<'_>) -> IResult<Input<'_>, &str, ParserError<'_>> {
    repeat(0.., alt((newline.value(&b"\n"[..]), take_while(1.., WSCHAR))))
        .map(|()| ())
        .recognize()
        .map(|b| unsafe {
            from_utf8_unchecked(b, "`is_wschar` and `newline` filters out on-ASCII")
        })
        .parse_next(input)
}
pub(crate) fn ws_newlines(
    input: Input<'_>,
) -> IResult<Input<'_>, &str, ParserError<'_>> {
    (newline, ws_newline)
        .recognize()
        .map(|b| unsafe {
            from_utf8_unchecked(b, "`is_wschar` and `newline` filters out on-ASCII")
        })
        .parse_next(input)
}
pub(crate) fn ws_comment_newline(
    input: Input<'_>,
) -> IResult<Input<'_>, &[u8], ParserError<'_>> {
    repeat(
            0..,
            alt((
                repeat(1.., alt((take_while(1.., WSCHAR), newline.value(&b"\n"[..]))))
                    .map(|()| ()),
                comment.value(()),
            )),
        )
        .map(|()| ())
        .recognize()
        .parse_next(input)
}
pub(crate) fn line_ending(
    input: Input<'_>,
) -> IResult<Input<'_>, &str, ParserError<'_>> {
    alt((newline.value("\n"), eof.value(""))).parse_next(input)
}
pub(crate) fn line_trailing(
    input: Input<'_>,
) -> IResult<Input<'_>, std::ops::Range<usize>, ParserError<'_>> {
    terminated((ws, opt(comment)).span(), line_ending).parse_next(input)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn trivia() {
        let inputs = [
            "",
            r#" "#,
            r#"
"#,
            r#"
# comment

# comment2


"#,
            r#"
        "#,
            r#"# comment
# comment2


   "#,
        ];
        for input in inputs {
            dbg!(input);
            let parsed = ws_comment_newline.parse(new_input(input));
            assert!(parsed.is_ok(), "{:?}", parsed);
            let parsed = parsed.unwrap();
            assert_eq!(parsed, input.as_bytes());
        }
    }
}
#[cfg(test)]
mod tests_llm_16_421_llm_16_421 {
    use crate::parser::trivia::from_utf8_unchecked;
    #[test]
    fn test_from_utf8_unchecked_valid() {
        let _rug_st_tests_llm_16_421_llm_16_421_rrrruuuugggg_test_from_utf8_unchecked_valid = 0;
        let rug_fuzz_0 = b"hello";
        let rug_fuzz_1 = "valid UTF-8";
        let bytes = rug_fuzz_0;
        let safety_justification = rug_fuzz_1;
        unsafe {
            let result = from_utf8_unchecked(bytes, safety_justification);
            debug_assert_eq!(result, "hello");
        }
        let _rug_ed_tests_llm_16_421_llm_16_421_rrrruuuugggg_test_from_utf8_unchecked_valid = 0;
    }
    #[test]
    #[should_panic(expected = "valid UTF-8")]
    fn test_from_utf8_unchecked_invalid() {
        let _rug_st_tests_llm_16_421_llm_16_421_rrrruuuugggg_test_from_utf8_unchecked_invalid = 0;
        let rug_fuzz_0 = b"\xffhello";
        let rug_fuzz_1 = "valid UTF-8";
        let bytes = rug_fuzz_0;
        let safety_justification = rug_fuzz_1;
        unsafe {
            let _result = from_utf8_unchecked(bytes, safety_justification);
        }
        let _rug_ed_tests_llm_16_421_llm_16_421_rrrruuuugggg_test_from_utf8_unchecked_invalid = 0;
    }
}

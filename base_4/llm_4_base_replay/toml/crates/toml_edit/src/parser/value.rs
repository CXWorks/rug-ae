use winnow::branch::alt;
use winnow::bytes::any;
use winnow::combinator::fail;
use winnow::combinator::peek;
use crate::parser::array::array;
use crate::parser::datetime::date_time;
use crate::parser::inline_table::inline_table;
use crate::parser::numbers::{float, integer};
use crate::parser::prelude::*;
use crate::parser::strings::string;
use crate::repr::{Formatted, Repr};
use crate::value as v;
use crate::RawString;
use crate::Value;
pub(crate) fn value(
    check: RecursionCheck,
) -> impl FnMut(Input<'_>) -> IResult<Input<'_>, v::Value, ParserError<'_>> {
    move |input| {
        dispatch! {
            peek(any); crate ::parser::strings::QUOTATION_MARK | crate
            ::parser::strings::APOSTROPHE => string.map(| s | {
            v::Value::String(Formatted::new(s.into_owned())) }), crate
            ::parser::array::ARRAY_OPEN => array(check).map(v::Value::Array), crate
            ::parser::inline_table::INLINE_TABLE_OPEN => inline_table(check)
            .map(v::Value::InlineTable), b'+' | b'-' | b'0'..= b'9' => { alt((date_time
            .map(v::Value::from), float.map(v::Value::from), integer
            .map(v::Value::from),)) }, b'_' => { integer.map(v::Value::from)
            .context(Context::Expected(ParserValue::Description("leading digit"))) },
            b'.' => { float.map(v::Value::from)
            .context(Context::Expected(ParserValue::Description("leading digit"))) },
            b't' => { crate ::parser::numbers::true_.map(v::Value::from)
            .context(Context::Expression("string"))
            .context(Context::Expected(ParserValue::CharLiteral('"')))
            .context(Context::Expected(ParserValue::CharLiteral('\''))) }, b'f' => {
            crate ::parser::numbers::false_.map(v::Value::from)
            .context(Context::Expression("string"))
            .context(Context::Expected(ParserValue::CharLiteral('"')))
            .context(Context::Expected(ParserValue::CharLiteral('\''))) }, b'i' => {
            crate ::parser::numbers::inf.map(v::Value::from)
            .context(Context::Expression("string"))
            .context(Context::Expected(ParserValue::CharLiteral('"')))
            .context(Context::Expected(ParserValue::CharLiteral('\''))) }, b'n' => {
            crate ::parser::numbers::nan.map(v::Value::from)
            .context(Context::Expression("string"))
            .context(Context::Expected(ParserValue::CharLiteral('"')))
            .context(Context::Expected(ParserValue::CharLiteral('\''))) }, _ => { fail
            .context(Context::Expression("string"))
            .context(Context::Expected(ParserValue::CharLiteral('"')))
            .context(Context::Expected(ParserValue::CharLiteral('\''))) },
        }
            .with_span()
            .try_map(|(value, span)| apply_raw(value, span))
            .parse_next(input)
    }
}
fn apply_raw(
    mut val: Value,
    span: std::ops::Range<usize>,
) -> Result<Value, std::str::Utf8Error> {
    match val {
        Value::String(ref mut f) => {
            let raw = RawString::with_span(span);
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Integer(ref mut f) => {
            let raw = RawString::with_span(span);
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Float(ref mut f) => {
            let raw = RawString::with_span(span);
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Boolean(ref mut f) => {
            let raw = RawString::with_span(span);
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Datetime(ref mut f) => {
            let raw = RawString::with_span(span);
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Array(ref mut arr) => {
            arr.span = Some(span);
        }
        Value::InlineTable(ref mut table) => {
            table.span = Some(span);
        }
    };
    val.decorate("", "");
    Ok(val)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn values() {
        let inputs = [
            "1979-05-27T00:32:00.999999",
            "-239",
            "1e200",
            "9_224_617.445_991_228_313",
            r#"'''I [dw]on't need \d{2} apples'''"#,
            r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"#,
            r#""Jos\u00E9\n""#,
            r#""\\\"\b/\f\n\r\t\u00E9\U000A0000""#,
            r#"{ hello = "world", a = 1}"#,
            r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        ];
        for input in inputs {
            dbg!(input);
            let mut parsed = value(Default::default()).parse(new_input(input));
            if let Ok(parsed) = &mut parsed {
                parsed.despan(input);
            }
            assert_eq!(parsed.map(| a | a.to_string()), Ok(input.to_owned()));
        }
    }
}
#[cfg(test)]
mod tests_llm_16_429 {
    use super::*;
    use crate::*;
    use crate::parser::value::apply_raw;
    use crate::Value;
    use std::str::FromStr;
    #[test]
    fn test_apply_raw_string() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_value = Value::String(Formatted::new(String::from(rug_fuzz_0)));
        let span = rug_fuzz_1..rug_fuzz_2;
        let applied = apply_raw(raw_value, span.clone()).expect(rug_fuzz_3);
        if let Value::String(s) = applied {
            debug_assert_eq!(span, s.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::String");
        }
             }
}
}
}    }
    #[test]
    fn test_apply_raw_integer() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(i64, usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_value = Value::Integer(Formatted::new(rug_fuzz_0));
        let span = rug_fuzz_1..rug_fuzz_2;
        let applied = apply_raw(raw_value, span.clone()).expect(rug_fuzz_3);
        if let Value::Integer(i) = applied {
            debug_assert_eq!(span, i.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Integer");
        }
             }
}
}
}    }
    #[test]
    fn test_apply_raw_float() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(f64, usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_value = Value::Float(Formatted::new(rug_fuzz_0));
        let span = rug_fuzz_1..rug_fuzz_2;
        let applied = apply_raw(raw_value, span.clone()).expect(rug_fuzz_3);
        if let Value::Float(f) = applied {
            debug_assert_eq!(span, f.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Float");
        }
             }
}
}
}    }
    #[test]
    fn test_apply_raw_boolean() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(bool, usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_value = Value::Boolean(Formatted::new(rug_fuzz_0));
        let span = rug_fuzz_1..rug_fuzz_2;
        let applied = apply_raw(raw_value, span.clone()).expect(rug_fuzz_3);
        if let Value::Boolean(b) = applied {
            debug_assert_eq!(span, b.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Boolean");
        }
             }
}
}
}    }
    #[test]
    fn test_apply_raw_datetime() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_value = Value::Datetime(
            Formatted::new(Datetime::from_str(rug_fuzz_0).unwrap()),
        );
        let span = rug_fuzz_1..rug_fuzz_2;
        let applied = apply_raw(raw_value, span.clone()).expect(rug_fuzz_3);
        if let Value::Datetime(dt) = applied {
            debug_assert_eq!(span, dt.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Datetime");
        }
             }
}
}
}    }
    #[test]
    fn test_apply_raw_array() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_value = Value::Array(Array::new());
        let span = rug_fuzz_0..rug_fuzz_1;
        let applied = apply_raw(raw_value, span.clone()).expect(rug_fuzz_2);
        if let Value::Array(arr) = applied {
            debug_assert_eq!(Some(span), arr.span());
        } else {
            panic!("apply_raw did not return a Value::Array");
        }
             }
}
}
}    }
    #[test]
    fn test_apply_raw_inline_table() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let raw_value = Value::InlineTable(InlineTable::new());
        let span = rug_fuzz_0..rug_fuzz_1;
        let applied = apply_raw(raw_value, span.clone()).expect(rug_fuzz_2);
        if let Value::InlineTable(it) = applied {
            debug_assert_eq!(Some(span), it.span());
        } else {
            panic!("apply_raw did not return a Value::InlineTable");
        }
             }
}
}
}    }
}

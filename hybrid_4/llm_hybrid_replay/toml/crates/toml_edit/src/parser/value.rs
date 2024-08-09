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

// val = string / boolean / array / inline-table / date-time / float / integer
pub(crate) fn value(
    check: RecursionCheck,
) -> impl FnMut(Input<'_>) -> IResult<Input<'_>, v::Value, ParserError<'_>> {
    move |input| {
        dispatch!{peek(any);
            crate::parser::strings::QUOTATION_MARK |
            crate::parser::strings::APOSTROPHE => string.map(|s| {
                v::Value::String(Formatted::new(
                    s.into_owned()
                ))
            }),
            crate::parser::array::ARRAY_OPEN => array(check).map(v::Value::Array),
            crate::parser::inline_table::INLINE_TABLE_OPEN => inline_table(check).map(v::Value::InlineTable),
            // Date/number starts
            b'+' | b'-' | b'0'..=b'9' => {
                // Uncommon enough not to be worth optimizing at this time
                alt((
                    date_time
                        .map(v::Value::from),
                    float
                        .map(v::Value::from),
                    integer
                        .map(v::Value::from),
                ))
            },
            // Report as if they were numbers because its most likely a typo
            b'_' => {
                    integer
                        .map(v::Value::from)
                .context(Context::Expected(ParserValue::Description("leading digit")))
            },
            // Report as if they were numbers because its most likely a typo
            b'.' =>  {
                    float
                        .map(v::Value::from)
                .context(Context::Expected(ParserValue::Description("leading digit")))
            },
            b't' => {
                crate::parser::numbers::true_.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            b'f' => {
                crate::parser::numbers::false_.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            b'i' => {
                crate::parser::numbers::inf.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            b'n' => {
                crate::parser::numbers::nan.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            _ => {
                fail
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
    }
        .with_span()
        .try_map(|(value, span)| apply_raw(value, span))
        .parse_next(input)
    }
}

fn apply_raw(mut val: Value, span: std::ops::Range<usize>) -> Result<Value, std::str::Utf8Error> {
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
            assert_eq!(parsed.map(|a| a.to_string()), Ok(input.to_owned()));
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
        let raw_value = Value::String(Formatted::new(String::from("test")));
        let span = 0..4;
        let applied = apply_raw(raw_value, span.clone()).expect("apply_raw failed");
        if let Value::String(s) = applied {
            assert_eq!(span, s.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::String");
        }
    }

    #[test]
    fn test_apply_raw_integer() {
        let raw_value = Value::Integer(Formatted::new(42));
        let span = 5..7;
        let applied = apply_raw(raw_value, span.clone()).expect("apply_raw failed");
        if let Value::Integer(i) = applied {
            assert_eq!(span, i.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Integer");
        }
    }

    #[test]
    fn test_apply_raw_float() {
        let raw_value = Value::Float(Formatted::new(3.14));
        let span = 8..12;
        let applied = apply_raw(raw_value, span.clone()).expect("apply_raw failed");
        if let Value::Float(f) = applied {
            assert_eq!(span, f.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Float");
        }
    }

    #[test]
    fn test_apply_raw_boolean() {
        let raw_value = Value::Boolean(Formatted::new(true));
        let span = 13..17;
        let applied = apply_raw(raw_value, span.clone()).expect("apply_raw failed");
        if let Value::Boolean(b) = applied {
            assert_eq!(span, b.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Boolean");
        }
    }

    #[test]
    fn test_apply_raw_datetime() {
        let raw_value = Value::Datetime(Formatted::new(Datetime::from_str("2020-09-09T12:34:56Z").unwrap()));
        let span = 18..38;
        let applied = apply_raw(raw_value, span.clone()).expect("apply_raw failed");
        if let Value::Datetime(dt) = applied {
            assert_eq!(span, dt.span().expect("Span was not applied"));
        } else {
            panic!("apply_raw did not return a Value::Datetime");
        }
    }

    #[test]
    fn test_apply_raw_array() {
        let raw_value = Value::Array(Array::new());
        let span = 39..42;
        let applied = apply_raw(raw_value, span.clone()).expect("apply_raw failed");
        if let Value::Array(arr) = applied {
            assert_eq!(Some(span), arr.span());
        } else {
            panic!("apply_raw did not return a Value::Array");
        }
    }

    #[test]
    fn test_apply_raw_inline_table() {
        let raw_value = Value::InlineTable(InlineTable::new());
        let span = 43..46;
        let applied = apply_raw(raw_value, span.clone()).expect("apply_raw failed");
        if let Value::InlineTable(it) = applied {
            assert_eq!(Some(span), it.span());
        } else {
            panic!("apply_raw did not return a Value::InlineTable");
        }
    }
}
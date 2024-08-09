//! Combinators applying their child parser multiple times
#[cfg(test)]
mod tests;
use crate::error::ErrorKind;
use crate::error::ParseError;
use crate::internal::{Err, IResult, Needed, Parser};
use crate::lib::std::num::NonZeroUsize;
#[cfg(feature = "alloc")]
use crate::lib::std::vec::Vec;
use crate::Input;
use crate::{
    traits::{InputLength, ToUsize},
    NomRange,
};
/// Don't pre-allocate more than 64KiB when calling `Vec::with_capacity`.
///
/// Pre-allocating memory is a nice optimization but count fields can't
/// always be trusted. We should clamp initial capacities to some reasonable
/// amount. This reduces the risk of a bogus count value triggering a panic
/// due to an OOM error.
///
/// This does not affect correctness. Nom will always read the full number
/// of elements regardless of the capacity cap.
#[cfg(feature = "alloc")]
const MAX_INITIAL_CAPACITY_BYTES: usize = 65536;
/// Repeats the embedded parser, gathering the results in a `Vec`.
///
/// This stops on [`Err::Error`] and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will
/// return an error, to prevent going into an infinite loop
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::many0;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many0(tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many0<I, F>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<<F as Parser<I>>::Output>, <F as Parser<I>>::Error>
where
    I: Clone + InputLength,
    F: Parser<I>,
{
    move |mut i: I| {
        let mut acc = crate::lib::std::vec::Vec::with_capacity(4);
        loop {
            let len = i.input_len();
            match f.parse(i.clone()) {
                Err(Err::Error(_)) => return Ok((i, acc)),
                Err(e) => return Err(e),
                Ok((i1, o)) => {
                    if i1.input_len() == len {
                        return Err(
                            Err::Error(
                                <F as Parser<
                                    I,
                                >>::Error::from_error_kind(i, ErrorKind::Many0),
                            ),
                        );
                    }
                    i = i1;
                    acc.push(o);
                }
            }
        }
    }
}
/// Runs the embedded parser, gathering the results in a `Vec`.
///
/// This stops on [`Err::Error`] if there is at least one result,  and returns the results that were accumulated. To instead chain an error up,
/// see [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::multi::many1;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many1(tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many1<I, F>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<<F as Parser<I>>::Output>, <F as Parser<I>>::Error>
where
    I: Clone + InputLength,
    F: Parser<I>,
{
    move |mut i: I| match f.parse(i.clone()) {
        Err(Err::Error(err)) => {
            Err(Err::Error(<F as Parser<I>>::Error::append(i, ErrorKind::Many1, err)))
        }
        Err(e) => Err(e),
        Ok((i1, o)) => {
            let mut acc = crate::lib::std::vec::Vec::with_capacity(4);
            acc.push(o);
            i = i1;
            loop {
                let len = i.input_len();
                match f.parse(i.clone()) {
                    Err(Err::Error(_)) => return Ok((i, acc)),
                    Err(e) => return Err(e),
                    Ok((i1, o)) => {
                        if i1.input_len() == len {
                            return Err(
                                Err::Error(
                                    <F as Parser<
                                        I,
                                    >>::Error::from_error_kind(i, ErrorKind::Many1),
                                ),
                            );
                        }
                        i = i1;
                        acc.push(o);
                    }
                }
            }
        }
    }
}
/// Applies the parser `f` until the parser `g` produces a result.
///
/// Returns a tuple of the results of `f` in a `Vec` and the result of `g`.
///
/// `f` keeps going so long as `g` produces [`Err::Error`]. To instead chain an error up, see [`cut`][crate::combinator::cut].
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::multi::many_till;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
///   many_till(tag("abc"), tag("end"))(s)
/// };
///
/// assert_eq!(parser("abcabcend"), Ok(("", (vec!["abc", "abc"], "end"))));
/// assert_eq!(parser("abc123end"), Err(Err::Error(Error::new("123end", ErrorKind::Tag))));
/// assert_eq!(parser("123123end"), Err(Err::Error(Error::new("123123end", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("abcendefg"), Ok(("efg", (vec!["abc"], "end"))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many_till<I, E, F, G>(
    mut f: F,
    mut g: G,
) -> impl FnMut(
    I,
) -> IResult<I, (Vec<<F as Parser<I>>::Output>, <G as Parser<I>>::Output), E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    G: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut res = crate::lib::std::vec::Vec::new();
        loop {
            let len = i.input_len();
            match g.parse(i.clone()) {
                Ok((i1, o)) => return Ok((i1, (res, o))),
                Err(Err::Error(_)) => {
                    match f.parse(i.clone()) {
                        Err(Err::Error(err)) => {
                            return Err(
                                Err::Error(E::append(i, ErrorKind::ManyTill, err)),
                            );
                        }
                        Err(e) => return Err(e),
                        Ok((i1, o)) => {
                            if i1.input_len() == len {
                                return Err(
                                    Err::Error(E::from_error_kind(i1, ErrorKind::ManyTill)),
                                );
                            }
                            res.push(o);
                            i = i1;
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}
/// Alternates between two parsers to produce a list of elements.
///
/// This stops when either parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `sep` Parses the separator between list elements.
/// * `f` Parses the elements of the list.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::separated_list0;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   separated_list0(tag("|"), tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abc|abc|abc"), Ok(("", vec!["abc", "abc", "abc"])));
/// assert_eq!(parser("abc123abc"), Ok(("123abc", vec!["abc"])));
/// assert_eq!(parser("abc|def"), Ok(("|def", vec!["abc"])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("def|abc"), Ok(("def|abc", vec![])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn separated_list0<I, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<<F as Parser<I>>::Output>, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    G: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut res = Vec::new();
        match f.parse(i.clone()) {
            Err(Err::Error(_)) => return Ok((i, res)),
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }
        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    if i1.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)),
                        );
                    }
                    match f.parse(i1.clone()) {
                        Err(Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}
/// Alternates between two parsers to produce a list of elements until [`Err::Error`].
///
/// Fails if the element parser does not produce at least one element.$
///
/// This stops when either parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `sep` Parses the separator between list elements.
/// * `f` Parses the elements of the list.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::multi::separated_list1;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   separated_list1(tag("|"), tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abc|abc|abc"), Ok(("", vec!["abc", "abc", "abc"])));
/// assert_eq!(parser("abc123abc"), Ok(("123abc", vec!["abc"])));
/// assert_eq!(parser("abc|def"), Ok(("|def", vec!["abc"])));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("def|abc"), Err(Err::Error(Error::new("def|abc", ErrorKind::Tag))));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn separated_list1<I, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<<F as Parser<I>>::Output>, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    G: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut res = Vec::new();
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }
        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    if i1.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)),
                        );
                    }
                    match f.parse(i1.clone()) {
                        Err(Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}
/// Repeats the embedded parser `m..=n` times
///
/// This stops before `n` when the parser returns [`Err::Error`]  and returns the results that were accumulated. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `m` The minimum number of iterations.
/// * `n` The maximum number of iterations.
/// * `f` The parser to apply.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::many_m_n;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many_m_n(0, 2, tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many_m_n<I, E, F>(
    min: usize,
    max: usize,
    mut parse: F,
) -> impl FnMut(I) -> IResult<I, Vec<<F as Parser<I>>::Output>, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |mut input: I| {
        if min > max {
            return Err(Err::Failure(E::from_error_kind(input, ErrorKind::ManyMN)));
        }
        let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES
            / crate::lib::std::mem::size_of::<<F as Parser<I>>::Output>().max(1);
        let mut res = crate::lib::std::vec::Vec::with_capacity(
            min.min(max_initial_capacity),
        );
        for count in 0..max {
            let len = input.input_len();
            match parse.parse(input.clone()) {
                Ok((tail, value)) => {
                    if tail.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(input, ErrorKind::ManyMN)),
                        );
                    }
                    res.push(value);
                    input = tail;
                }
                Err(Err::Error(e)) => {
                    if count < min {
                        return Err(Err::Error(E::append(input, ErrorKind::ManyMN, e)));
                    } else {
                        return Ok((input, res));
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((input, res))
    }
}
/// Repeats the embedded parser, counting the results
///
/// This stops on [`Err::Error`]. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will
/// return an error, to prevent going into an infinite loop
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::many0_count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, usize> {
///   many0_count(tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", 2)));
/// assert_eq!(parser("abc123"), Ok(("123", 1)));
/// assert_eq!(parser("123123"), Ok(("123123", 0)));
/// assert_eq!(parser(""), Ok(("", 0)));
/// ```
pub fn many0_count<I, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, usize, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |i: I| {
        let mut input = i;
        let mut count = 0;
        loop {
            let input_ = input.clone();
            let len = input.input_len();
            match f.parse(input_) {
                Ok((i, _)) => {
                    if i.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(input, ErrorKind::Many0Count)),
                        );
                    }
                    input = i;
                    count += 1;
                }
                Err(Err::Error(_)) => return Ok((input, count)),
                Err(e) => return Err(e),
            }
        }
    }
}
/// Runs the embedded parser, counting the results.
///
/// This stops on [`Err::Error`] if there is at least one result. To instead chain an error up,
/// see [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::multi::many1_count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, usize> {
///   many1_count(tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", 2)));
/// assert_eq!(parser("abc123"), Ok(("123", 1)));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Many1Count))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Many1Count))));
/// ```
pub fn many1_count<I, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, usize, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |i: I| {
        let i_ = i.clone();
        match f.parse(i_) {
            Err(Err::Error(_)) => {
                Err(Err::Error(E::from_error_kind(i, ErrorKind::Many1Count)))
            }
            Err(i) => Err(i),
            Ok((i1, _)) => {
                let mut count = 1;
                let mut input = i1;
                loop {
                    let len = input.input_len();
                    let input_ = input.clone();
                    match f.parse(input_) {
                        Err(Err::Error(_)) => return Ok((input, count)),
                        Err(e) => return Err(e),
                        Ok((i, _)) => {
                            if i.input_len() == len {
                                return Err(
                                    Err::Error(E::from_error_kind(i, ErrorKind::Many1Count)),
                                );
                            }
                            count += 1;
                            input = i;
                        }
                    }
                }
            }
        }
    }
}
/// Runs the embedded parser `count` times, gathering the results in a `Vec`
///
/// # Arguments
/// * `f` The parser to apply.
/// * `count` How often to apply the parser.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::multi::count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   count(tag("abc"), 2)(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Err(Err::Error(Error::new("123", ErrorKind::Tag))));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn count<I, E, F>(
    mut f: F,
    count: usize,
) -> impl FnMut(I) -> IResult<I, Vec<<F as Parser<I>>::Output>, E>
where
    I: Clone + PartialEq,
    F: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |i: I| {
        let mut input = i.clone();
        let max_initial_capacity = MAX_INITIAL_CAPACITY_BYTES
            / crate::lib::std::mem::size_of::<<F as Parser<I>>::Output>().max(1);
        let mut res = crate::lib::std::vec::Vec::with_capacity(
            count.min(max_initial_capacity),
        );
        for _ in 0..count {
            let input_ = input.clone();
            match f.parse(input_) {
                Ok((i, o)) => {
                    res.push(o);
                    input = i;
                }
                Err(Err::Error(e)) => {
                    return Err(Err::Error(E::append(i, ErrorKind::Count, e)));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((input, res))
    }
}
/// Runs the embedded parser repeatedly, filling the given slice with results.
///
/// This parser fails if the input runs out before the given slice is full.
///
/// # Arguments
/// * `f` The parser to apply.
/// * `buf` The slice to fill
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::multi::fill;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, [&str; 2]> {
///   let mut buf = ["", ""];
///   let (rest, ()) = fill(tag("abc"), &mut buf)(s)?;
///   Ok((rest, buf))
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", ["abc", "abc"])));
/// assert_eq!(parser("abc123"), Err(Err::Error(Error::new("123", ErrorKind::Tag))));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", ["abc", "abc"])));
/// ```
pub fn fill<'a, I, E, F>(
    mut f: F,
    buf: &'a mut [<F as Parser<I>>::Output],
) -> impl FnMut(I) -> IResult<I, (), E> + 'a
where
    I: Clone + PartialEq,
    F: Parser<I, Error = E> + 'a,
    E: ParseError<I>,
{
    move |i: I| {
        let mut input = i.clone();
        for elem in buf.iter_mut() {
            let input_ = input.clone();
            match f.parse(input_) {
                Ok((i, o)) => {
                    *elem = o;
                    input = i;
                }
                Err(Err::Error(e)) => {
                    return Err(Err::Error(E::append(i, ErrorKind::Count, e)));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((input, ()))
    }
}
/// Repeats the embedded parser, calling `g` to gather the results.
///
/// This stops on [`Err::Error`]. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
/// * `init` A function returning the initial value.
/// * `g` The function that combines a result of `f` with
///       the current accumulator.
///
/// *Note*: if the parser passed in accepts empty inputs (like `alpha0` or `digit0`), `many0` will
/// return an error, to prevent going into an infinite loop
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::fold_many0;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold_many0(
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   )(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// ```
pub fn fold_many0<I, E, F, G, H, R>(
    mut f: F,
    mut init: H,
    mut g: G,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    G: FnMut(R, <F as Parser<I>>::Output) -> R,
    H: FnMut() -> R,
    E: ParseError<I>,
{
    move |i: I| {
        let mut res = init();
        let mut input = i;
        loop {
            let i_ = input.clone();
            let len = input.input_len();
            match f.parse(i_) {
                Ok((i, o)) => {
                    if i.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(input, ErrorKind::Many0)),
                        );
                    }
                    res = g(res, o);
                    input = i;
                }
                Err(Err::Error(_)) => {
                    return Ok((input, res));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
/// Repeats the embedded parser, calling `g` to gather the results.
///
/// This stops on [`Err::Error`] if there is at least one result. To instead chain an error up,
/// see [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `f` The parser to apply.
/// * `init` A function returning the initial value.
/// * `g` The function that combines a result of `f` with
///       the current accumulator.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::multi::fold_many1;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold_many1(
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   )(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Err(Err::Error(Error::new("123123", ErrorKind::Many1))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Many1))));
/// ```
pub fn fold_many1<I, E, F, G, H, R>(
    mut f: F,
    mut init: H,
    mut g: G,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    G: FnMut(R, <F as Parser<I>>::Output) -> R,
    H: FnMut() -> R,
    E: ParseError<I>,
{
    move |i: I| {
        let _i = i.clone();
        let init = init();
        match f.parse(_i) {
            Err(Err::Error(_)) => {
                Err(Err::Error(E::from_error_kind(i, ErrorKind::Many1)))
            }
            Err(e) => Err(e),
            Ok((i1, o1)) => {
                let mut acc = g(init, o1);
                let mut input = i1;
                loop {
                    let _input = input.clone();
                    let len = input.input_len();
                    match f.parse(_input) {
                        Err(Err::Error(_)) => {
                            break;
                        }
                        Err(e) => return Err(e),
                        Ok((i, o)) => {
                            if i.input_len() == len {
                                return Err(
                                    Err::Failure(E::from_error_kind(i, ErrorKind::Many1)),
                                );
                            }
                            acc = g(acc, o);
                            input = i;
                        }
                    }
                }
                Ok((input, acc))
            }
        }
    }
}
/// Repeats the embedded parser `m..=n` times, calling `g` to gather the results
///
/// This stops before `n` when the parser returns [`Err::Error`]. To instead chain an error up, see
/// [`cut`][crate::combinator::cut].
///
/// # Arguments
/// * `m` The minimum number of iterations.
/// * `n` The maximum number of iterations.
/// * `f` The parser to apply.
/// * `init` A function returning the initial value.
/// * `g` The function that combines a result of `f` with
///       the current accumulator.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::fold_many_m_n;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold_many_m_n(
///     0,
///     2,
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   )(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
pub fn fold_many_m_n<I, E, F, G, H, R>(
    min: usize,
    max: usize,
    mut parse: F,
    mut init: H,
    mut fold: G,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    G: FnMut(R, <F as Parser<I>>::Output) -> R,
    H: FnMut() -> R,
    E: ParseError<I>,
{
    move |mut input: I| {
        if min > max {
            return Err(Err::Failure(E::from_error_kind(input, ErrorKind::ManyMN)));
        }
        let mut acc = init();
        for count in 0..max {
            let len = input.input_len();
            match parse.parse(input.clone()) {
                Ok((tail, value)) => {
                    if tail.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(tail, ErrorKind::ManyMN)),
                        );
                    }
                    acc = fold(acc, value);
                    input = tail;
                }
                Err(Err::Error(err)) => {
                    if count < min {
                        return Err(Err::Error(E::append(input, ErrorKind::ManyMN, err)));
                    } else {
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok((input, acc))
    }
}
/// Gets a number from the parser and returns a
/// subslice of the input of that size.
/// If the parser returns `Incomplete`,
/// `length_data` will return an error.
/// # Arguments
/// * `f` The parser to apply.
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::number::complete::be_u16;
/// use nom::multi::length_data;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   length_data(be_u16)(s)
/// }
///
/// assert_eq!(parser(b"\x00\x03abcefg"), Ok((&b"efg"[..], &b"abc"[..])));
/// assert_eq!(parser(b"\x00\x03a"), Err(Err::Incomplete(Needed::new(2))));
/// ```
pub fn length_data<I, E, F>(mut f: F) -> impl FnMut(I) -> IResult<I, I, E>
where
    I: Input,
    <F as Parser<I>>::Output: ToUsize,
    F: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |i: I| {
        let (i, length) = f.parse(i)?;
        let length: usize = length.to_usize();
        if let Some(needed)
            = length.checked_sub(i.input_len()).and_then(NonZeroUsize::new)
        {
            Err(Err::Incomplete(Needed::Size(needed)))
        } else {
            Ok(i.take_split(length))
        }
    }
}
/// Gets a number from the first parser,
/// takes a subslice of the input of that size,
/// then applies the second parser on that subslice.
/// If the second parser returns `Incomplete`,
/// `length_value` will return an error.
/// # Arguments
/// * `f` The parser to apply.
/// * `g` The parser to apply on the subslice.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::number::complete::be_u16;
/// use nom::multi::length_value;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
///   length_value(be_u16, tag("abc"))(s)
/// }
///
/// assert_eq!(parser(b"\x00\x03abcefg"), Ok((&b"efg"[..], &b"abc"[..])));
/// assert_eq!(parser(b"\x00\x03123123"), Err(Err::Error(Error::new(&b"123"[..], ErrorKind::Tag))));
/// assert_eq!(parser(b"\x00\x03a"), Err(Err::Incomplete(Needed::new(2))));
/// ```
pub fn length_value<I, E, F, G>(
    mut f: F,
    mut g: G,
) -> impl FnMut(I) -> IResult<I, <G as Parser<I>>::Output, E>
where
    I: Clone + Input,
    <F as Parser<I>>::Output: ToUsize,
    F: Parser<I, Error = E>,
    G: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |i: I| {
        let (i, length) = f.parse(i)?;
        let length: usize = length.to_usize();
        if let Some(needed)
            = length.checked_sub(i.input_len()).and_then(NonZeroUsize::new)
        {
            Err(Err::Incomplete(Needed::Size(needed)))
        } else {
            let (rest, i) = i.take_split(length);
            match g.parse(i.clone()) {
                Err(Err::Incomplete(_)) => {
                    Err(Err::Error(E::from_error_kind(i, ErrorKind::Complete)))
                }
                Err(e) => Err(e),
                Ok((_, o)) => Ok((rest, o)),
            }
        }
    }
}
/// Gets a number from the first parser,
/// then applies the second parser that many times.
/// # Arguments
/// * `f` The parser to apply to obtain the count.
/// * `g` The parser to apply repeatedly.
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::number::complete::u8;
/// use nom::multi::length_count;
/// use nom::bytes::complete::tag;
/// use nom::combinator::map;
///
/// fn parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
///   length_count(map(u8, |i| {
///      println!("got number: {}", i);
///      i
///   }), tag("abc"))(s)
/// }
///
/// assert_eq!(parser(&b"\x02abcabcabc"[..]), Ok(((&b"abc"[..], vec![&b"abc"[..], &b"abc"[..]]))));
/// assert_eq!(parser(b"\x03123123123"), Err(Err::Error(Error::new(&b"123123123"[..], ErrorKind::Tag))));
/// ```
#[cfg(feature = "alloc")]
pub fn length_count<I, E, F, G>(
    mut f: F,
    mut g: G,
) -> impl FnMut(I) -> IResult<I, Vec<<G as Parser<I>>::Output>, E>
where
    I: Clone,
    <F as Parser<I>>::Output: ToUsize,
    F: Parser<I, Error = E>,
    G: Parser<I, Error = E>,
    E: ParseError<I>,
{
    move |i: I| {
        let (i, count) = f.parse(i)?;
        let mut input = i.clone();
        let mut res = Vec::new();
        for _ in 0..count.to_usize() {
            let input_ = input.clone();
            match g.parse(input_) {
                Ok((i, o)) => {
                    res.push(o);
                    input = i;
                }
                Err(Err::Error(e)) => {
                    return Err(Err::Error(E::append(i, ErrorKind::Count, e)));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((input, res))
    }
}
/// Repeats the embedded parser and collects the results in a type implementing `Extend + Default`.
/// Fails if the amount of time the embedded parser is run is not
/// within the specified range.
/// # Arguments
/// * `range` Constrains the number of iterations.
///   * A range without an upper bound `a..` is equivalent to a range of `a..=usize::MAX`.
///   * A single `usize` value is equivalent to `value..=value`.
///   * An empty range is invalid.
/// * `parse` The parser to apply.
///
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::many;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   many(0..=2, tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
///
/// This is not limited to `Vec`, other collections like `HashMap`
/// can be used:
///
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::many;
/// use nom::bytes::complete::{tag, take_while};
/// use nom::sequence::{separated_pair, terminated};
/// use nom::AsChar;
///
/// use std::collections::HashMap;
///
/// fn key_value(s: &str) -> IResult<&str, HashMap<&str, &str>> {
///   many(0.., terminated(
///     separated_pair(
///       take_while(AsChar::is_alpha),
///       tag("="),
///       take_while(AsChar::is_alpha)
///     ),
///     tag(";")
///   ))(s)
/// }
///
/// assert_eq!(
///   key_value("a=b;c=d;"),
///   Ok(("", HashMap::from([("a", "b"), ("c", "d")])))
/// );
/// ```
///
/// If more control is needed on the default value, [fold] can
/// be used instead:
///
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::fold;
/// use nom::bytes::complete::tag;
///
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold(
///     0..=4,
///     tag("abc"),
///     // preallocates a vector of the max size
///     || Vec::with_capacity(4),
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   )(s)
/// }
///
///
/// assert_eq!(parser("abcabcabcabc"), Ok(("", vec!["abc", "abc", "abc", "abc"])));
/// ```
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "alloc")))]
pub fn many<I, E, Collection, F, G>(
    range: G,
    mut parse: F,
) -> impl FnMut(I) -> IResult<I, Collection, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    Collection: Extend<<F as Parser<I>>::Output> + Default,
    E: ParseError<I>,
    G: NomRange<usize>,
{
    move |mut input: I| {
        if range.is_inverted() {
            return Err(Err::Failure(E::from_error_kind(input, ErrorKind::Many)));
        }
        let mut res = Collection::default();
        for count in range.bounded_iter() {
            let len = input.input_len();
            match parse.parse(input.clone()) {
                Ok((tail, value)) => {
                    if tail.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(input, ErrorKind::Many)),
                        );
                    }
                    res.extend(Some(value));
                    input = tail;
                }
                Err(Err::Error(e)) => {
                    if !range.contains(&count) {
                        return Err(Err::Error(E::append(input, ErrorKind::Many, e)));
                    } else {
                        return Ok((input, res));
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((input, res))
    }
}
/// Applies a parser and accumulates the results using a given
/// function and initial value.
/// Fails if the amount of time the embedded parser is run is not
/// within the specified range.
///
/// # Arguments
/// * `range` Constrains the number of iterations.
///   * A range without an upper bound `a..` allows the parser to run until it fails.
///   * A single `usize` value is equivalent to `value..=value`.
///   * An empty range is invalid.
/// * `parse` The parser to apply.
/// * `init` A function returning the initial value.
/// * `fold` The function that combines a result of `f` with
///       the current accumulator.
/// ```rust
/// # #[macro_use] extern crate nom;
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use nom::multi::fold;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, Vec<&str>> {
///   fold(
///     0..=2,
///     tag("abc"),
///     Vec::new,
///     |mut acc: Vec<_>, item| {
///       acc.push(item);
///       acc
///     }
///   )(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
/// assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
/// assert_eq!(parser("123123"), Ok(("123123", vec![])));
/// assert_eq!(parser(""), Ok(("", vec![])));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
/// ```
pub fn fold<I, E, F, G, H, J, R>(
    range: J,
    mut parse: F,
    mut init: H,
    mut fold: G,
) -> impl FnMut(I) -> IResult<I, R, E>
where
    I: Clone + InputLength,
    F: Parser<I, Error = E>,
    G: FnMut(R, <F as Parser<I>>::Output) -> R,
    H: FnMut() -> R,
    E: ParseError<I>,
    J: NomRange<usize>,
{
    move |mut input: I| {
        if range.is_inverted() {
            return Err(Err::Failure(E::from_error_kind(input, ErrorKind::Fold)));
        }
        let mut acc = init();
        for count in range.saturating_iter() {
            let len = input.input_len();
            match parse.parse(input.clone()) {
                Ok((tail, value)) => {
                    if tail.input_len() == len {
                        return Err(
                            Err::Error(E::from_error_kind(tail, ErrorKind::Fold)),
                        );
                    }
                    acc = fold(acc, value);
                    input = tail;
                }
                Err(Err::Error(err)) => {
                    if !range.contains(&count) {
                        return Err(Err::Error(E::append(input, ErrorKind::Fold, err)));
                    } else {
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok((input, acc))
    }
}
#[cfg(test)]
mod tests_llm_16_442 {
    use crate::{
        bytes::complete::tag, error::{Error, ErrorKind},
        multi::count, IResult, Err,
    };
    fn parse_abc(input: &str) -> IResult<&str, &str> {
        tag("abc")(input)
    }
    #[test]
    fn count_success() {
        let result = count(parse_abc, 3)("abcabcabc");
        assert_eq!(result, Ok(("", vec!["abc", "abc", "abc"])));
    }
    #[test]
    fn count_incomplete() {
        let result = count(parse_abc, 3)("abcabc");
        assert!(result.is_err());
        assert_eq!(result, Err(Err::Error(Error::new("abc", ErrorKind::Count))));
    }
    #[test]
    fn count_no_match() {
        let result = count(parse_abc, 3)("xyzxyzxyz");
        assert!(result.is_err());
        assert_eq!(result, Err(Err::Error(Error::new("xyzxyzxyz", ErrorKind::Count))));
    }
    #[test]
    fn count_partial_match() {
        let result = count(parse_abc, 3)("abcabcxyz");
        assert!(result.is_err());
        assert_eq!(result, Err(Err::Error(Error::new("xyz", ErrorKind::Count))));
    }
    #[test]
    fn count_zero_times() {
        let result = count(parse_abc, 0)("abcabcabc");
        assert_eq!(result, Ok(("abcabcabc", vec![])));
    }
    #[test]
    fn count_excess_input() {
        let result = count(parse_abc, 2)("abcabcabc");
        assert_eq!(result, Ok(("abc", vec!["abc", "abc"])));
    }
}
#[cfg(test)]
mod tests_llm_16_443_llm_16_443 {
    use crate::{
        error::{Error, ErrorKind},
        multi::fill, IResult, Err,
    };
    use crate::bytes::complete::tag;
    #[test]
    fn fill_success() {
        fn parser(s: &str) -> IResult<&str, [&str; 3]> {
            let mut buf = ["", "", ""];
            let (rest, ()) = fill(tag("abc"), &mut buf)(s)?;
            Ok((rest, buf))
        }
        assert_eq!(parser("abcabcabc"), Ok(("", ["abc", "abc", "abc"])));
    }
    #[test]
    fn fill_partial_input() {
        fn parser(s: &str) -> IResult<&str, [&str; 4]> {
            let mut buf = ["", "", "", ""];
            let (rest, ()) = fill(tag("abc"), &mut buf)(s)?;
            Ok((rest, buf))
        }
        assert_eq!(
            parser("abcabc"), Err(Err::Error(Error::new("abc", ErrorKind::Count)))
        );
    }
    #[test]
    fn fill_incomplete_input() {
        fn parser(s: &str) -> IResult<&str, [&str; 3]> {
            let mut buf = ["", "", ""];
            let (rest, ()) = fill(tag("abc"), &mut buf)(s)?;
            Ok((rest, buf))
        }
        assert_eq!(parser("abcab"), Err(Err::Error(Error::new("ab", ErrorKind::Count))));
    }
    #[test]
    fn fill_no_input() {
        fn parser(s: &str) -> IResult<&str, [&str; 3]> {
            let mut buf = ["", "", ""];
            let (rest, ()) = fill(tag("abc"), &mut buf)(s)?;
            Ok((rest, buf))
        }
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Count))));
    }
    #[test]
    fn fill_incorrect_input() {
        fn parser(s: &str) -> IResult<&str, [&str; 3]> {
            let mut buf = ["", "", ""];
            let (rest, ()) = fill(tag("abc"), &mut buf)(s)?;
            Ok((rest, buf))
        }
        assert_eq!(
            parser("abxabcabc"), Err(Err::Error(Error::new("abxabcabc", ErrorKind::Tag)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_444 {
    use super::*;
    use crate::*;
    use crate::{
        error::{ErrorKind, ParseError},
        Err, IResult,
    };
    fn match_a(input: &str) -> IResult<&str, &str, crate::error::Error<&str>> {
        if input.starts_with('a') {
            Ok((&input[1..], &input[0..1]))
        } else {
            Err(Err::Error(crate::error::Error::new(input, ErrorKind::Tag)))
        }
    }
    #[test]
    fn fold_success_with_in_range() {
        fn init() -> Vec<&'static str> {
            vec![]
        }
        let mut parser = fold(
            0..=2,
            match_a,
            init,
            |mut acc, item| {
                acc.push(item);
                acc
            },
        );
        let input = "aaa";
        let expected = Ok(("", vec!["a", "a"]));
        assert_eq!(parser(input), expected);
    }
    #[test]
    fn fold_success_with_zero_matches() {
        fn init() -> Vec<&'static str> {
            vec![]
        }
        let mut parser = fold(
            0..=2,
            match_a,
            init,
            |mut acc, item| {
                acc.push(item);
                acc
            },
        );
        let input = "bbb";
        let expected = Ok(("bbb", vec![]));
        assert_eq!(parser(input), expected);
    }
    #[test]
    fn fold_failure_with_incomplete() {
        fn init() -> Vec<&'static str> {
            vec![]
        }
        let mut parser = fold(
            2..=2,
            match_a,
            init,
            |mut acc, item| {
                acc.push(item);
                acc
            },
        );
        let input = "a";
        assert!(matches!(parser(input), Err(Err::Error(_))));
    }
    #[test]
    fn fold_success_with_exact() {
        fn init() -> Vec<&'static str> {
            vec![]
        }
        let mut parser = fold(
            2..2,
            match_a,
            init,
            |mut acc, item| {
                acc.push(item);
                acc
            },
        );
        let input = "a";
        let expected = Ok(("a", vec![]));
        assert_eq!(parser(input), expected);
    }
    #[test]
    fn fold_failure_with_invalid_range() {
        fn init() -> Vec<&'static str> {
            vec![]
        }
        let mut parser = fold(
            2..1,
            match_a,
            init,
            |mut acc, item| {
                acc.push(item);
                acc
            },
        );
        let input = "aaa";
        assert!(matches!(parser(input), Err(Err::Failure(_))));
    }
}
#[cfg(test)]
mod tests_llm_16_445 {
    use super::*;
    use crate::*;
    use crate::{
        error::ErrorKind, error::ParseError, multi::fold_many0, bytes::complete::tag,
        IResult,
    };
    #[test]
    fn fold_many0_test() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many0(
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("abcabcabc"), Ok(("", vec!["abc", "abc", "abc"])));
        assert_eq!(parser("abc"), Ok(("", vec!["abc"])));
        assert_eq!(parser("abcabcxxx"), Ok(("xxx", vec!["abc", "abc"])));
        assert_eq!(parser("xxxabcabc"), Ok(("xxxabcabc", vec![])));
        assert_eq!(parser(""), Ok(("", vec![])));
        fn failing_parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many0(
                tag(""),
                Vec::new,
                |mut acc: Vec<&str>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        let res = failing_parser("abc");
        assert!(res.is_err());
        if let Err(crate::Err::Error(err)) = res {
            assert!(matches!(err.code, ErrorKind::Many0));
        } else {
            panic!("Error case did not match for failing_parser with input 'abc'");
        }
    }
}
#[cfg(test)]
mod tests_llm_16_446 {
    use crate::{Err, error::ErrorKind, IResult};
    use crate::multi::fold_many1;
    use crate::bytes::complete::tag;
    use crate::error::Error;
    use crate::traits::InputLength;
    use crate::internal::Parser;
    #[test]
    fn fold_many1_success_multiple() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many1(
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("abcabcabc"), Ok(("", vec!["abc", "abc", "abc"])));
    }
    #[test]
    fn fold_many1_success_single() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many1(
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("abc"), Ok(("", vec!["abc"])));
    }
    #[test]
    fn fold_many1_incomplete() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many1(
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("abcabcx"), Ok(("x", vec!["abc", "abc"])));
    }
    #[test]
    fn fold_many1_error() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many1(
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("x"), Err(Err::Error(Error::new("x", ErrorKind::Many1))));
    }
    #[test]
    fn fold_many1_infinite_loop() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many1(
                tag(""),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert!(parser("abc").is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_447 {
    use super::*;
    use crate::*;
    use crate::{
        error::{Error, ErrorKind},
        multi::fold_many_m_n, combinator::cut, bytes::complete::tag, IResult,
    };
    use crate::error::ParseError;
    #[test]
    fn fold_many_m_n_min_less_than_max() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many_m_n(
                1,
                3,
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("abcabcabc"), Ok(("", vec!["abc", "abc", "abc"])));
        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc"), Ok(("", vec!["abc"])));
        assert_eq!(parser("abcabcabcd"), Ok(("d", vec!["abc", "abc", "abc"])));
        assert_eq!(parser("123"), Err(Err::Error(Error::new("123", ErrorKind::ManyMN))));
    }
    #[test]
    fn fold_many_m_n_min_equals_max() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many_m_n(
                2,
                2,
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
        assert_eq!(parser("abc"), Err(Err::Error(Error::new("abc", ErrorKind::ManyMN))));
        assert_eq!(
            parser("abc123"), Err(Err::Error(Error::new("abc123", ErrorKind::ManyMN)))
        );
        assert_eq!(parser("123"), Err(Err::Error(Error::new("123", ErrorKind::ManyMN))));
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::ManyMN))));
    }
    #[test]
    fn fold_many_m_n_min_greater_than_max() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many_m_n(
                3,
                2,
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(
            parser("abcabcabc"), Err(Err::Error(Error::new("abcabcabc",
            ErrorKind::ManyMN)))
        );
    }
    #[test]
    fn fold_many_m_n_zero_min() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many_m_n(
                0,
                2,
                tag("abc"),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(parser("123123"), Ok(("123123", vec![])));
        assert_eq!(parser(""), Ok(("", vec![])));
        assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
    }
    #[test]
    fn fold_many_m_n_consumes_nothing() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many_m_n(
                1,
                3,
                cut(tag("")),
                Vec::new,
                |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                },
            )(s)
        }
        assert!(parser("abc").is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_448 {
    use crate::{
        error::{Error, ErrorKind},
        IResult, Parser,
    };
    use crate::multi::length_count;
    use crate::bytes::complete::tag;
    use crate::number::complete::u8;
    #[test]
    fn test_length_count_exact() {
        fn count_parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
            length_count(u8, tag("abc"))(s)
        }
        assert_eq!(
            count_parser(& b"\x02abcabcabc"[..]), Ok((& b"abc"[..], vec![& b"abc"[..], &
            b"abc"[..]]))
        );
    }
    #[test]
    fn test_length_count_incomplete() {
        fn count_parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
            length_count(u8, tag("abc"))(s)
        }
        assert_eq!(
            count_parser(& b"\x03abcabc"[..]), Err(crate ::Err::Error(Error::new(&
            b"abc"[..], ErrorKind::Count)))
        );
    }
    #[test]
    fn test_length_count_no_count() {
        fn count_parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
            length_count(u8, tag("abc"))(s)
        }
        assert_eq!(count_parser(& b"\x00abcabc"[..]), Ok((& b"\x00abcabc"[..], vec![])));
    }
    #[test]
    fn test_length_count_invalid() {
        fn count_parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
            length_count(u8, tag("abc"))(s)
        }
        assert_eq!(
            count_parser(& b"\x02ab"[..]), Err(crate ::Err::Error(Error::new(& b"ab"[..],
            ErrorKind::Count)))
        );
    }
    #[test]
    fn test_length_count_extra_data() {
        fn count_parser(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
            length_count(u8, tag("abc"))(s)
        }
        assert_eq!(
            count_parser(& b"\x01abcxyz"[..]), Ok((& b"xyz"[..], vec![& b"abc"[..]]))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_450_llm_16_450 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult, Needed,
    };
    use crate::number::complete::be_u16;
    use crate::multi::length_value;
    use crate::bytes::complete::tag;
    use std::num::NonZeroUsize;
    fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
        length_value(be_u16, tag("abc"))(s)
    }
    #[test]
    fn length_value_success() {
        let input = b"\x00\x03abcefg";
        assert_eq!(parser(input), Ok((& b"efg"[..], & b"abc"[..])));
    }
    #[test]
    fn length_value_tag_error() {
        let input = b"\x00\x03123123";
        assert_eq!(
            parser(input), Err(Err::Error(Error::new(& b"123"[..], ErrorKind::Tag)))
        );
    }
    #[test]
    fn length_value_incomplete() {
        let input = b"\x00\x03a";
        assert_eq!(parser(input), Err(Err::Incomplete(Needed::new(2))));
    }
    #[test]
    fn length_value_zero_length() {
        let input = b"\x00\x00abcdefg";
        assert_eq!(parser(input), Ok((& b"abcdefg"[..], & b""[..])));
    }
    #[test]
    fn length_value_incomplete_length() {
        let input = b"\x00";
        assert_eq!(parser(input), Err(Err::Incomplete(Needed::new(1))));
    }
    #[test]
    fn length_value_too_large_length() {
        let input = b"\xFF\xFFabc";
        assert_eq!(
            parser(input), Err(Err::Error(Error::new(& input[..],
            ErrorKind::LengthValue)))
        );
    }
    #[test]
    fn length_value_empty_input() {
        let input = b"";
        assert_eq!(parser(input), Err(Err::Incomplete(Needed::new(1))));
    }
}
#[cfg(test)]
mod tests_llm_16_451 {
    use super::*;
    use crate::*;
    use crate::{
        error::{Error, ErrorKind, ParseError},
        multi::many, IResult,
    };
    #[derive(Debug, PartialEq)]
    struct TestError<I>(I, ErrorKind);
    impl<I> ParseError<I> for TestError<I> {
        fn from_error_kind(input: I, kind: ErrorKind) -> Self {
            TestError(input, kind)
        }
        fn append(_: I, _: ErrorKind, other: Self) -> Self {
            other
        }
    }
    fn tag<'a>(
        s: &'a str,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, TestError<&'a str>> {
        move |input: &'a str| {
            if input.starts_with(s) {
                Ok((&input[s.len()..], &input[..s.len()]))
            } else {
                Err(crate::Err::Error(TestError(input, ErrorKind::Tag)))
            }
        }
    }
    #[test]
    fn many_test() {
        let mut parser = many(0..3, tag("abc"));
        assert_eq!(parser("abcabcabc").unwrap(), ("abc", vec!["abc", "abc"]));
        assert_eq!(parser("abcabc123").unwrap(), ("123", vec!["abc", "abc"]));
        assert_eq!(parser("abc").unwrap(), ("", vec!["abc"]));
        assert_eq!(parser("123").unwrap(), ("123", vec![]));
        assert_eq!(parser("").unwrap(), ("", vec![]));
        assert!(parser("abcabcabcabc").is_err());
    }
}
#[cfg(test)]
mod tests_llm_16_452_llm_16_452 {
    use super::*;
    use crate::*;
    use crate::{
        error::{Error, ErrorKind},
        IResult, Parser,
    };
    use crate::multi::many0;
    use crate::bytes::complete::tag;
    #[test]
    fn many0_empty_input() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parser = many0(tag(rug_fuzz_0));
        let input = rug_fuzz_1;
        let expected: IResult<&str, Vec<&str>> = Ok((rug_fuzz_2, vec![]));
        debug_assert_eq!(parser(input), expected);
             }
});    }
    #[test]
    fn many0_single_match() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parser = many0(tag(rug_fuzz_0));
        let input = rug_fuzz_1;
        let expected: IResult<&str, Vec<&str>> = Ok((rug_fuzz_2, vec![rug_fuzz_3]));
        debug_assert_eq!(parser(input), expected);
             }
});    }
    #[test]
    fn many0_multiple_matches() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parser = many0(tag(rug_fuzz_0));
        let input = rug_fuzz_1;
        let expected: IResult<&str, Vec<&str>> = Ok((
            rug_fuzz_2,
            vec![rug_fuzz_3, "abc", "abc"],
        ));
        debug_assert_eq!(parser(input), expected);
             }
});    }
    #[test]
    fn many0_no_match_at_beginning() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parser = many0(tag(rug_fuzz_0));
        let input = rug_fuzz_1;
        let expected: IResult<&str, Vec<&str>> = Ok((rug_fuzz_2, vec![]));
        debug_assert_eq!(parser(input), expected);
             }
});    }
    #[test]
    fn many0_incomplete() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parser = many0(tag(rug_fuzz_0));
        let input = rug_fuzz_1;
        let expected: IResult<&str, Vec<&str>> = Ok((
            rug_fuzz_2,
            vec![rug_fuzz_3, "abc"],
        ));
        debug_assert_eq!(parser(input), expected);
             }
});    }
    #[test]
    fn many0_error() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, u32, char, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        use crate::combinator::map_res;
        use crate::sequence::terminated;
        use crate::bytes::complete::take_while_m_n;
        use crate::character::complete::char;
        let mut parser = many0(
            terminated(
                map_res(
                    take_while_m_n(
                        rug_fuzz_0,
                        rug_fuzz_1,
                        |c: char| c.is_digit(rug_fuzz_2),
                    ),
                    |s: &str| s.parse::<u8>(),
                ),
                char(rug_fuzz_3),
            ),
        );
        let input = rug_fuzz_4;
        let expected = Err(crate::Err::Error(Error::new(rug_fuzz_5, ErrorKind::MapRes)));
        debug_assert_eq!(parser(input), expected);
             }
});    }
    #[test]
    fn many0_infinite_loop_detected() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut parser = many0(tag(rug_fuzz_0));
        let input = rug_fuzz_1;
        let expected = Err(crate::Err::Error(Error::new(input, ErrorKind::Many0)));
        debug_assert_eq!(parser(input), expected);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_454 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::multi::many1;
    use crate::bytes::complete::tag;
    #[test]
    fn many1_success_multiple() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many1(tag("abc"))(s)
        }
        assert_eq!(parser("abcabcabc"), Ok(("", vec!["abc", "abc", "abc"])));
    }
    #[test]
    fn many1_success_single() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many1(tag("abc"))(s)
        }
        assert_eq!(parser("abc"), Ok(("", vec!["abc"])));
    }
    #[test]
    fn many1_incomplete() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many1(tag("abc"))(s)
        }
        assert_eq!(parser("ab"), Err(Err::Error(Error::new("ab", ErrorKind::Tag))));
    }
    #[test]
    fn many1_error() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many1(tag("abc"))(s)
        }
        assert_eq!(parser("xyz"), Err(Err::Error(Error::new("xyz", ErrorKind::Tag))));
    }
    #[test]
    fn many1_partial() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many1(tag("abc"))(s)
        }
        assert_eq!(parser("abcxyz"), Ok(("xyz", vec!["abc"])));
    }
    #[test]
    fn many1_empty_input() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many1(tag("abc"))(s)
        }
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
    }
}
#[cfg(test)]
mod tests_llm_16_455 {
    use super::*;
    use crate::*;
    use crate::{IResult, bytes::complete::tag, error::{Error, ErrorKind}};
    #[test]
    fn many1_count_success() {
        fn parser(s: &str) -> IResult<&str, usize> {
            many1_count(tag("abc"))(s)
        }
        assert_eq!(parser("abcabc"), Ok(("", 2)));
        assert_eq!(parser("abc123"), Ok(("123", 1)));
    }
    #[test]
    fn many1_count_error() {
        fn parser(s: &str) -> IResult<&str, usize> {
            many1_count(tag("abc"))(s)
        }
        assert_eq!(
            parser("123123"), Err(Err::Error(Error::new("123123",
            ErrorKind::Many1Count)))
        );
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Many1Count))));
    }
    #[test]
    fn many1_count_incomplete() {
        fn parser(s: &str) -> IResult<&str, usize> {
            many1_count(tag("abc"))(s)
        }
        assert_eq!(
            parser("ab"), Err(Err::Error(Error::new("ab", ErrorKind::Many1Count)))
        );
    }
    #[test]
    fn many1_count_empty_parser() {
        fn parser(s: &str) -> IResult<&str, usize> {
            many1_count(tag(""))(s)
        }
        assert_eq!(
            parser("abc"), Err(Err::Error(Error::new("abc", ErrorKind::Many1Count)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_456 {
    use crate::{
        error::{Error, ErrorKind},
        Err, IResult,
    };
    use crate::multi::many_m_n;
    use crate::bytes::complete::tag;
    #[test]
    fn test_many_m_n_success() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many_m_n(0, 2, tag("abc"))(s)
        }
        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(parser("123123"), Ok(("123123", vec![])));
        assert_eq!(parser(""), Ok(("", vec![])));
    }
    #[test]
    fn test_many_m_n_upper_limit() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many_m_n(0, 2, tag("abc"))(s)
        }
        assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
    }
    #[test]
    fn test_many_m_n_min_error() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many_m_n(2, 3, tag("abc"))(s)
        }
        assert_eq!(parser("abc"), Err(Err::Error(Error::new("abc", ErrorKind::ManyMN))));
    }
    #[test]
    fn test_many_m_n_incomplete() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many_m_n(3, 5, tag("abc"))(s)
        }
        assert_eq!(
            parser("abcabc"), Err(Err::Error(Error::new("abcabc", ErrorKind::ManyMN)))
        );
    }
    #[test]
    fn test_many_m_n_infinite_loop() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many_m_n(0, 3, tag(""))(s)
        }
        assert_eq!(parser("abc"), Err(Err::Error(Error::new("abc", ErrorKind::ManyMN))));
    }
    #[test]
    fn test_many_m_n_invalid_range() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many_m_n(3, 2, tag("abc"))(s)
        }
        assert_eq!(
            parser("abcabcabc"), Err(Err::Failure(Error::new("abcabcabc",
            ErrorKind::ManyMN)))
        );
    }
}
#[cfg(test)]
mod tests_llm_16_457 {
    use crate::multi::many_till;
    use crate::bytes::complete::tag;
    use crate::error::{Error, ErrorKind};
    use crate::IResult;
    use crate::traits::InputLength;
    #[test]
    fn many_till_success() {
        fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
            many_till(tag("abc"), tag("end"))(s)
        }
        let expected = Ok(("", (vec!["abc", "abc"], "end")));
        assert_eq!(parser("abcabcend"), expected);
    }
    #[test]
    fn many_till_incomplete() {
        fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
            many_till(tag("abc"), tag("end"))(s)
        }
        let expected = Err(crate::Err::Error(Error::new("123end", ErrorKind::Tag)));
        assert_eq!(parser("abc123end"), expected);
    }
    #[test]
    fn many_till_no_match() {
        fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
            many_till(tag("abc"), tag("end"))(s)
        }
        let expected = Err(crate::Err::Error(Error::new("123123end", ErrorKind::Tag)));
        assert_eq!(parser("123123end"), expected);
    }
    #[test]
    fn many_till_empty_input() {
        fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
            many_till(tag("abc"), tag("end"))(s)
        }
        let expected = Err(crate::Err::Error(Error::new("", ErrorKind::Tag)));
        assert_eq!(parser(""), expected);
    }
    #[test]
    fn many_till_partial_match() {
        fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
            many_till(tag("abc"), tag("end"))(s)
        }
        let expected = Ok(("efg", (vec!["abc"], "end")));
        assert_eq!(parser("abcendefg"), expected);
    }
}
#[cfg(test)]
mod tests_llm_16_458 {
    use super::*;
    use crate::*;
    use crate::{
        error::{ErrorKind, ParseError},
        IResult,
    };
    use crate::bytes::complete::tag;
    use crate::combinator::cut;
    use crate::sequence::terminated;
    #[test]
    fn test_separated_list0_empty() {
        fn parser(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(","), tag("abc"))(input)
        }
        let result = parser("");
        assert_eq!(result, Ok(("", Vec::new())));
    }
    #[test]
    fn test_separated_list0_single() {
        fn parser(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(","), tag("abc"))(input)
        }
        let result = parser("abc");
        assert_eq!(result, Ok(("", vec!["abc"])));
    }
    #[test]
    fn test_separated_list0_multiple() {
        fn parser(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(","), tag("abc"))(input)
        }
        let result = parser("abc,abc,abc");
        assert_eq!(result, Ok(("", vec!["abc", "abc", "abc"])));
    }
    #[test]
    fn test_separated_list0_with_incomplete() {
        fn parser(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(","), tag("abc"))(input)
        }
        let result = parser("abc,abc,xyz");
        assert_eq!(result, Ok((",xyz", vec!["abc", "abc"])));
    }
    #[test]
    fn test_separated_list0_with_cut() {
        fn parser(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(","), cut(terminated(tag("abc"), tag("!"))))(input)
        }
        let result = parser("abc!,abc!");
        assert_eq!(result, Ok(("", vec!["abc", "abc"])));
    }
    #[test]
    fn test_separated_list0_with_error() {
        fn parser(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(","), cut(terminated(tag("abc"), tag("!"))))(input)
        }
        let result = parser("abc!abc");
        assert!(result.is_err());
        assert!(matches!(result, Err(crate ::Err::Error(_))));
    }
    #[test]
    fn test_separated_list0_with_cut_error() {
        fn parser(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(","), cut(terminated(tag("abc"), tag("!"))))(input)
        }
        let result = parser("abc!,abc");
        assert!(result.is_err());
        assert!(matches!(result, Err(crate ::Err::Failure(_))));
    }
}
#[cfg(test)]
mod tests_llm_16_459 {
    use crate::{
        error::{Error, ErrorKind, ParseError},
        multi::separated_list1, IResult, Parser,
    };
    use crate::bytes::complete::tag;
    #[test]
    fn test_separated_list1_success() {
        fn parse_separated_list1(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list1(tag("|"), tag("abc"))(input)
        }
        assert_eq!(
            parse_separated_list1("abc|abc|abc"), Ok(("", vec!["abc", "abc", "abc"]))
        );
        assert_eq!(parse_separated_list1("abc123abc"), Ok(("123abc", vec!["abc"])));
        assert_eq!(parse_separated_list1("abc|def"), Ok(("|def", vec!["abc"])));
    }
    #[test]
    fn test_separated_list1_incomplete() {
        fn parse_separated_list1(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list1(tag("|"), tag("abc"))(input)
        }
        assert_eq!(
            parse_separated_list1(""), Err(crate ::Err::Error(Error::new("",
            ErrorKind::Tag)))
        );
    }
    #[test]
    fn test_separated_list1_failure() {
        fn parse_separated_list1(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list1(tag("|"), tag("abc"))(input)
        }
        assert_eq!(
            parse_separated_list1("def"), Err(crate ::Err::Error(Error::new("def",
            ErrorKind::Tag)))
        );
    }
    #[test]
    fn test_separated_list1_early_termination() {
        fn parse_separated_list1(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list1(tag("|"), tag("abc"))(input)
        }
        assert_eq!(
            parse_separated_list1("abc|abc|def"), Ok(("|def", vec!["abc", "abc"]))
        );
    }
    #[test]
    fn test_separated_list1_no_separator() {
        fn parse_separated_list1(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list1(tag("|"), tag("abc"))(input)
        }
        assert_eq!(parse_separated_list1("abc"), Ok(("", vec!["abc"])));
    }
}

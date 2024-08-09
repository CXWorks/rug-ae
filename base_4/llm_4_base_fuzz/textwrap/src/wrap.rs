//! Functions for wrapping text.
use std::borrow::Cow;
use crate::core::{break_words, display_width, Word};
use crate::word_splitters::split_words;
use crate::Options;
/// Wrap a line of text at a given width.
///
/// The result is a vector of lines, each line is of type [`Cow<'_,
/// str>`](Cow), which means that the line will borrow from the input
/// `&str` if possible. The lines do not have trailing whitespace,
/// including a final `'\n'`. Please use [`fill()`](crate::fill()) if
/// you need a [`String`] instead.
///
/// The easiest way to use this function is to pass an integer for
/// `width_or_options`:
///
/// ```
/// use textwrap::wrap;
///
/// let lines = wrap("Memory safety without garbage collection.", 15);
/// assert_eq!(lines, &[
///     "Memory safety",
///     "without garbage",
///     "collection.",
/// ]);
/// ```
///
/// If you need to customize the wrapping, you can pass an [`Options`]
/// instead of an `usize`:
///
/// ```
/// use textwrap::{wrap, Options};
///
/// let options = Options::new(15)
///     .initial_indent("- ")
///     .subsequent_indent("  ");
/// let lines = wrap("Memory safety without garbage collection.", &options);
/// assert_eq!(lines, &[
///     "- Memory safety",
///     "  without",
///     "  garbage",
///     "  collection.",
/// ]);
/// ```
///
/// # Optimal-Fit Wrapping
///
/// By default, `wrap` will try to ensure an even right margin by
/// finding breaks which avoid short lines. We call this an
/// “optimal-fit algorithm” since the line breaks are computed by
/// considering all possible line breaks. The alternative is a
/// “first-fit algorithm” which simply accumulates words until they no
/// longer fit on the line.
///
/// As an example, using the first-fit algorithm to wrap the famous
/// Hamlet quote “To be, or not to be: that is the question” in a
/// narrow column with room for only 10 characters looks like this:
///
/// ```
/// # use textwrap::{WrapAlgorithm::FirstFit, Options, wrap};
/// #
/// # let lines = wrap("To be, or not to be: that is the question",
/// #                  Options::new(10).wrap_algorithm(FirstFit));
/// # assert_eq!(lines.join("\n") + "\n", "\
/// To be, or
/// not to be:
/// that is
/// the
/// question
/// # ");
/// ```
///
/// Notice how the second to last line is quite narrow because
/// “question” was too large to fit? The greedy first-fit algorithm
/// doesn’t look ahead, so it has no other option than to put
/// “question” onto its own line.
///
/// With the optimal-fit wrapping algorithm, the previous lines are
/// shortened slightly in order to make the word “is” go into the
/// second last line:
///
/// ```
/// # #[cfg(feature = "smawk")] {
/// # use textwrap::{Options, WrapAlgorithm, wrap};
/// #
/// # let lines = wrap(
/// #     "To be, or not to be: that is the question",
/// #     Options::new(10).wrap_algorithm(WrapAlgorithm::new_optimal_fit())
/// # );
/// # assert_eq!(lines.join("\n") + "\n", "\
/// To be,
/// or not to
/// be: that
/// is the
/// question
/// # "); }
/// ```
///
/// Please see [`WrapAlgorithm`](crate::WrapAlgorithm) for details on
/// the choices.
///
/// # Examples
///
/// The returned iterator yields lines of type `Cow<'_, str>`. If
/// possible, the wrapped lines will borrow from the input string. As
/// an example, a hanging indentation, the first line can borrow from
/// the input, but the subsequent lines become owned strings:
///
/// ```
/// use std::borrow::Cow::{Borrowed, Owned};
/// use textwrap::{wrap, Options};
///
/// let options = Options::new(15).subsequent_indent("....");
/// let lines = wrap("Wrapping text all day long.", &options);
/// let annotated = lines
///     .iter()
///     .map(|line| match line {
///         Borrowed(text) => format!("[Borrowed] {}", text),
///         Owned(text) => format!("[Owned]    {}", text),
///     })
///     .collect::<Vec<_>>();
/// assert_eq!(
///     annotated,
///     &[
///         "[Borrowed] Wrapping text",
///         "[Owned]    ....all day",
///         "[Owned]    ....long.",
///     ]
/// );
/// ```
///
/// ## Leading and Trailing Whitespace
///
/// As a rule, leading whitespace (indentation) is preserved and
/// trailing whitespace is discarded.
///
/// In more details, when wrapping words into lines, words are found
/// by splitting the input text on space characters. One or more
/// spaces (shown here as “␣”) are attached to the end of each word:
///
/// ```text
/// "Foo␣␣␣bar␣baz" -> ["Foo␣␣␣", "bar␣", "baz"]
/// ```
///
/// These words are then put into lines. The interword whitespace is
/// preserved, unless the lines are wrapped so that the `"Foo␣␣␣"`
/// word falls at the end of a line:
///
/// ```
/// use textwrap::wrap;
///
/// assert_eq!(wrap("Foo   bar baz", 10), vec!["Foo   bar", "baz"]);
/// assert_eq!(wrap("Foo   bar baz", 8), vec!["Foo", "bar baz"]);
/// ```
///
/// Notice how the trailing whitespace is removed in both case: in the
/// first example, `"bar␣"` becomes `"bar"` and in the second case
/// `"Foo␣␣␣"` becomes `"Foo"`.
///
/// Leading whitespace is preserved when the following word fits on
/// the first line. To understand this, consider how words are found
/// in a text with leading spaces:
///
/// ```text
/// "␣␣foo␣bar" -> ["␣␣", "foo␣", "bar"]
/// ```
///
/// When put into lines, the indentation is preserved if `"foo"` fits
/// on the first line, otherwise you end up with an empty line:
///
/// ```
/// use textwrap::wrap;
///
/// assert_eq!(wrap("  foo bar", 8), vec!["  foo", "bar"]);
/// assert_eq!(wrap("  foo bar", 4), vec!["", "foo", "bar"]);
/// ```
pub fn wrap<'a, Opt>(text: &str, width_or_options: Opt) -> Vec<Cow<'_, str>>
where
    Opt: Into<Options<'a>>,
{
    let options: Options = width_or_options.into();
    let line_ending_str = options.line_ending.as_str();
    let mut lines = Vec::new();
    for line in text.split(line_ending_str) {
        wrap_single_line(line, &options, &mut lines);
    }
    lines
}
pub(crate) fn wrap_single_line<'a>(
    line: &'a str,
    options: &Options<'_>,
    lines: &mut Vec<Cow<'a, str>>,
) {
    let indent = if lines.is_empty() {
        options.initial_indent
    } else {
        options.subsequent_indent
    };
    if line.len() < options.width && indent.is_empty() {
        lines.push(Cow::from(line.trim_end_matches(' ')));
    } else {
        wrap_single_line_slow_path(line, options, lines)
    }
}
/// Wrap a single line of text.
///
/// This is taken when `line` is longer than `options.width`.
pub(crate) fn wrap_single_line_slow_path<'a>(
    line: &'a str,
    options: &Options<'_>,
    lines: &mut Vec<Cow<'a, str>>,
) {
    let initial_width = options
        .width
        .saturating_sub(display_width(options.initial_indent));
    let subsequent_width = options
        .width
        .saturating_sub(display_width(options.subsequent_indent));
    let line_widths = [initial_width, subsequent_width];
    let words = options.word_separator.find_words(line);
    let split_words = split_words(words, &options.word_splitter);
    let broken_words = if options.break_words {
        let mut broken_words = break_words(split_words, line_widths[1]);
        if !options.initial_indent.is_empty() {
            broken_words.insert(0, Word::from(""));
        }
        broken_words
    } else {
        split_words.collect::<Vec<_>>()
    };
    let wrapped_words = options.wrap_algorithm.wrap(&broken_words, &line_widths);
    let mut idx = 0;
    for words in wrapped_words {
        let last_word = match words.last() {
            None => {
                lines.push(Cow::from(""));
                continue;
            }
            Some(word) => word,
        };
        let len = words
            .iter()
            .map(|word| word.len() + word.whitespace.len())
            .sum::<usize>() - last_word.whitespace.len();
        let mut result = if lines.is_empty() && !options.initial_indent.is_empty() {
            Cow::Owned(options.initial_indent.to_owned())
        } else if !lines.is_empty() && !options.subsequent_indent.is_empty() {
            Cow::Owned(options.subsequent_indent.to_owned())
        } else {
            Cow::from("")
        };
        result += &line[idx..idx + len];
        if !last_word.penalty.is_empty() {
            result.to_mut().push_str(last_word.penalty);
        }
        lines.push(result);
        idx += len + last_word.whitespace.len();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WordSeparator, WordSplitter, WrapAlgorithm};
    #[cfg(feature = "hyphenation")]
    use hyphenation::{Language, Load, Standard};
    #[test]
    fn no_wrap() {
        assert_eq!(wrap("foo", 10), vec!["foo"]);
    }
    #[test]
    fn wrap_simple() {
        assert_eq!(wrap("foo bar baz", 5), vec!["foo", "bar", "baz"]);
    }
    #[test]
    fn to_be_or_not() {
        assert_eq!(
            wrap("To be, or not to be, that is the question.", Options::new(10)
            .wrap_algorithm(WrapAlgorithm::FirstFit)), vec!["To be, or", "not to be,",
            "that is", "the", "question."]
        );
    }
    #[test]
    fn multiple_words_on_first_line() {
        assert_eq!(wrap("foo bar baz", 10), vec!["foo bar", "baz"]);
    }
    #[test]
    fn long_word() {
        assert_eq!(wrap("foo", 0), vec!["f", "o", "o"]);
    }
    #[test]
    fn long_words() {
        assert_eq!(wrap("foo bar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }
    #[test]
    fn max_width() {
        assert_eq!(wrap("foo bar", usize::MAX), vec!["foo bar"]);
        let text = "Hello there! This is some English text. \
                    It should not be wrapped given the extents below.";
        assert_eq!(wrap(text, usize::MAX), vec![text]);
    }
    #[test]
    fn leading_whitespace() {
        assert_eq!(wrap("  foo bar", 6), vec!["  foo", "bar"]);
    }
    #[test]
    fn leading_whitespace_empty_first_line() {
        assert_eq!(wrap(" foobar baz", 6), vec!["", "foobar", "baz"]);
    }
    #[test]
    fn trailing_whitespace() {
        assert_eq!(wrap("foo     bar     baz  ", 5), vec!["foo", "bar", "baz"]);
    }
    #[test]
    fn issue_99() {
        assert_eq!(
            wrap("aaabbbccc x yyyzzzwww", 9), vec!["aaabbbccc", "x", "yyyzzzwww"]
        );
    }
    #[test]
    fn issue_129() {
        let options = Options::new(1).word_separator(WordSeparator::AsciiSpace);
        assert_eq!(wrap("x – x", options), vec!["x", "–", "x"]);
    }
    #[test]
    fn wide_character_handling() {
        assert_eq!(wrap("Hello, World!", 15), vec!["Hello, World!"]);
        assert_eq!(
            wrap("Ｈｅｌｌｏ, Ｗｏｒｌｄ!", Options::new(15)
            .word_separator(WordSeparator::AsciiSpace)), vec!["Ｈｅｌｌｏ,",
            "Ｗｏｒｌｄ!"]
        );
        #[cfg(feature = "unicode-linebreak")]
        assert_eq!(
            wrap("Ｈｅｌｌｏ, Ｗｏｒｌｄ!", Options::new(15)
            .word_separator(WordSeparator::UnicodeBreakProperties),),
            vec!["Ｈｅｌｌｏ, Ｗ", "ｏｒｌｄ!"]
        );
    }
    #[test]
    fn indent_empty_line() {
        let options = Options::new(10).initial_indent("!!!");
        assert_eq!(wrap("", & options), vec!["!!!"]);
    }
    #[test]
    fn indent_single_line() {
        let options = Options::new(10).initial_indent(">>>");
        assert_eq!(wrap("foo", & options), vec![">>>foo"]);
    }
    #[test]
    fn indent_first_emoji() {
        let options = Options::new(10).initial_indent("👉👉");
        assert_eq!(
            wrap("x x x x x x x x x x x x x", & options), vec!["👉👉x x x",
            "x x x x x", "x x x x x"]
        );
    }
    #[test]
    fn indent_multiple_lines() {
        let options = Options::new(6).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(wrap("foo bar baz", & options), vec!["* foo", "  bar", "  baz"]);
    }
    #[test]
    fn only_initial_indent_multiple_lines() {
        let options = Options::new(10).initial_indent("  ");
        assert_eq!(wrap("foo\nbar\nbaz", & options), vec!["  foo", "bar", "baz"]);
    }
    #[test]
    fn only_subsequent_indent_multiple_lines() {
        let options = Options::new(10).subsequent_indent("  ");
        assert_eq!(wrap("foo\nbar\nbaz", & options), vec!["foo", "  bar", "  baz"]);
    }
    #[test]
    fn indent_break_words() {
        let options = Options::new(5).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(wrap("foobarbaz", & options), vec!["* foo", "  bar", "  baz"]);
    }
    #[test]
    fn initial_indent_break_words() {
        let options = Options::new(5).initial_indent("-->");
        assert_eq!(wrap("foobarbaz", & options), vec!["-->", "fooba", "rbaz"]);
    }
    #[test]
    fn hyphens() {
        assert_eq!(wrap("foo-bar", 5), vec!["foo-", "bar"]);
    }
    #[test]
    fn trailing_hyphen() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foobar-", & options), vec!["foobar-"]);
    }
    #[test]
    fn multiple_hyphens() {
        assert_eq!(wrap("foo-bar-baz", 5), vec!["foo-", "bar-", "baz"]);
    }
    #[test]
    fn hyphens_flag() {
        let options = Options::new(5).break_words(false);
        assert_eq!(
            wrap("The --foo-bar flag.", & options), vec!["The", "--foo-", "bar", "flag."]
        );
    }
    #[test]
    fn repeated_hyphens() {
        let options = Options::new(4).break_words(false);
        assert_eq!(wrap("foo--bar", & options), vec!["foo--bar"]);
    }
    #[test]
    fn hyphens_alphanumeric() {
        assert_eq!(wrap("Na2-CH4", 5), vec!["Na2-", "CH4"]);
    }
    #[test]
    fn hyphens_non_alphanumeric() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foo(-)bar", & options), vec!["foo(-)bar"]);
    }
    #[test]
    fn multiple_splits() {
        assert_eq!(wrap("foo-bar-baz", 9), vec!["foo-bar-", "baz"]);
    }
    #[test]
    fn forced_split() {
        let options = Options::new(5).break_words(false);
        assert_eq!(wrap("foobar-baz", & options), vec!["foobar-", "baz"]);
    }
    #[test]
    fn multiple_unbroken_words_issue_193() {
        let options = Options::new(3).break_words(false);
        assert_eq!(wrap("small large tiny", & options), vec!["small", "large", "tiny"]);
        assert_eq!(
            wrap("small  large   tiny", & options), vec!["small", "large", "tiny"]
        );
    }
    #[test]
    fn very_narrow_lines_issue_193() {
        let options = Options::new(1).break_words(false);
        assert_eq!(wrap("fooo x y", & options), vec!["fooo", "x", "y"]);
        assert_eq!(wrap("fooo   x     y", & options), vec!["fooo", "x", "y"]);
    }
    #[test]
    fn simple_hyphens() {
        let options = Options::new(8).word_splitter(WordSplitter::HyphenSplitter);
        assert_eq!(wrap("foo bar-baz", & options), vec!["foo bar-", "baz"]);
    }
    #[test]
    fn no_hyphenation() {
        let options = Options::new(8).word_splitter(WordSplitter::NoHyphenation);
        assert_eq!(wrap("foo bar-baz", & options), vec!["foo", "bar-baz"]);
    }
    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_double_hyphenation() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            wrap("Internationalization", & options), vec!["Internatio", "nalization"]
        );
        let options = Options::new(10)
            .word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            wrap("Internationalization", & options), vec!["Interna-", "tionaliza-",
            "tion"]
        );
    }
    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_issue_158() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            wrap("participation is the key to success", & options), vec!["participat",
            "ion is", "the key to", "success"]
        );
        let options = Options::new(10)
            .word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            wrap("participation is the key to success", & options), vec!["partici-",
            "pation is", "the key to", "success"]
        );
    }
    #[test]
    #[cfg(feature = "hyphenation")]
    fn split_len_hyphenation() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(15)
            .word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            wrap("garbage   collection", & options), vec!["garbage   col-", "lection"]
        );
    }
    #[test]
    #[cfg(feature = "hyphenation")]
    fn borrowed_lines() {
        use std::borrow::Cow::{Borrowed, Owned};
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10)
            .word_splitter(WordSplitter::Hyphenation(dictionary));
        let lines = wrap("Internationalization", &options);
        assert_eq!(lines, vec!["Interna-", "tionaliza-", "tion"]);
        if let Borrowed(s) = lines[0] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Borrowed(s) = lines[1] {
            assert!(false, "should not have been borrowed: {:?}", s);
        }
        if let Owned(ref s) = lines[2] {
            assert!(false, "should not have been owned: {:?}", s);
        }
    }
    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_with_hyphen() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(8).break_words(false);
        assert_eq!(wrap("over-caffinated", & options), vec!["over-", "caffinated"]);
        let options = options.word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(wrap("over-caffinated", & options), vec!["over-", "caffi-", "nated"]);
    }
    #[test]
    fn break_words() {
        assert_eq!(wrap("foobarbaz", 3), vec!["foo", "bar", "baz"]);
    }
    #[test]
    fn break_words_wide_characters() {
        let options = Options::new(5).word_separator(WordSeparator::AsciiSpace);
        assert_eq!(wrap("Ｈｅｌｌｏ", options), vec!["Ｈｅ", "ｌｌ", "ｏ"]);
    }
    #[test]
    fn break_words_zero_width() {
        assert_eq!(wrap("foobar", 0), vec!["f", "o", "o", "b", "a", "r"]);
    }
    #[test]
    fn break_long_first_word() {
        assert_eq!(wrap("testx y", 4), vec!["test", "x y"]);
    }
    #[test]
    fn wrap_preserves_line_breaks_trims_whitespace() {
        assert_eq!(wrap("  ", 80), vec![""]);
        assert_eq!(wrap("  \n  ", 80), vec!["", ""]);
        assert_eq!(wrap("  \n \n  \n ", 80), vec!["", "", "", ""]);
    }
    #[test]
    fn wrap_colored_text() {
        let green_hello = "\u{1b}[0m\u{1b}[32mHello\u{1b}[0m";
        let blue_world = "\u{1b}[0m\u{1b}[34mWorld!\u{1b}[0m";
        assert_eq!(
            wrap(& format!("{} {}", green_hello, blue_world), 6), vec![green_hello,
            blue_world],
        );
    }
}
#[cfg(test)]
mod tests_llm_16_43_llm_16_43 {
    use crate::wrap;
    use crate::Options;
    use std::borrow::Cow::{Borrowed, Owned};
    #[test]
    fn test_wrap_simple() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let text = rug_fuzz_0;
        let width = rug_fuzz_1;
        let lines = wrap(text, width);
        debug_assert_eq!(
            lines, vec![Borrowed("Memory safety"), Borrowed("without garbage"),
            Borrowed("collection."),]
        );
             }
});    }
    #[test]
    fn test_wrap_with_options() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let options = Options::new(rug_fuzz_0)
            .initial_indent(rug_fuzz_1)
            .subsequent_indent(rug_fuzz_2);
        let text = rug_fuzz_3;
        let lines = wrap(text, &options);
        debug_assert_eq!(
            lines, vec![Borrowed("- Memory safety"), Owned("  without".to_string()),
            Owned("  garbage".to_string()), Owned("  collection.".to_string()),]
        );
             }
});    }
    #[test]
    fn test_wrap_with_indentation() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let options = Options::new(rug_fuzz_0).subsequent_indent(rug_fuzz_1);
        let text = rug_fuzz_2;
        let lines = wrap(text, &options);
        let annotated = lines
            .iter()
            .map(|line| match line {
                Borrowed(text) => format!("[Borrowed] {}", text),
                Owned(text) => format!("[Owned]    {}", text),
            })
            .collect::<Vec<_>>();
        debug_assert_eq!(
            annotated, vec!["[Borrowed] Wrapping text", "[Owned]    ....all day long.",]
        );
             }
});    }
    #[test]
    fn test_wrap_preserve_leading_whitespace() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let text = rug_fuzz_0;
        let width = rug_fuzz_1;
        let lines = wrap(text, width);
        debug_assert_eq!(lines, vec![Borrowed("  Foo"), Borrowed("bar baz")]);
             }
});    }
    #[test]
    fn test_wrap_discard_trailing_whitespace() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let text = rug_fuzz_0;
        let lines = wrap(text, rug_fuzz_1);
        debug_assert_eq!(lines, vec![Borrowed("Foo"), Borrowed("bar baz")]);
             }
});    }
    #[test]
    fn test_wrap_discard_extra_trailing_whitespace() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let text = rug_fuzz_0;
        let lines = wrap(text, rug_fuzz_1);
        debug_assert_eq!(lines, vec![Borrowed("Foo   bar"), Borrowed("baz")]);
             }
});    }
    #[test]
    fn test_wrap_leading_whitespace_causing_empty_line() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let text = rug_fuzz_0;
        let width = rug_fuzz_1;
        let lines = wrap(text, width);
        debug_assert_eq!(lines, vec![Borrowed(""), Borrowed("foo"), Borrowed("bar")]);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_44 {
    use super::*;
    use crate::*;
    use crate::wrap::wrap_single_line;
    use crate::options::Options;
    use crate::line_ending::LineEnding;
    use std::borrow::Cow;
    #[test]
    fn test_wrap_single_line_empty_indent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, usize, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let mut lines = Vec::new();
        let options = Options {
            width: rug_fuzz_1,
            line_ending: LineEnding::LF,
            initial_indent: rug_fuzz_2,
            subsequent_indent: rug_fuzz_3,
            break_words: rug_fuzz_4,
            wrap_algorithm: WrapAlgorithm::FirstFit,
            word_separator: WordSeparator::new(),
            word_splitter: WordSplitter::HyphenSplitter,
        };
        wrap_single_line(line, &options, &mut lines);
        debug_assert_eq!(lines, vec![Cow::from(line)]);
             }
});    }
    #[test]
    fn test_wrap_single_line_with_initial_indent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, usize, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let mut lines = Vec::new();
        let options = Options {
            width: rug_fuzz_1,
            line_ending: LineEnding::LF,
            initial_indent: rug_fuzz_2,
            subsequent_indent: rug_fuzz_3,
            break_words: rug_fuzz_4,
            wrap_algorithm: WrapAlgorithm::FirstFit,
            word_separator: WordSeparator::new(),
            word_splitter: WordSplitter::HyphenSplitter,
        };
        wrap_single_line(line, &options, &mut lines);
        debug_assert_eq!(lines, vec![Cow::from(">>This is a test line.")]);
             }
});    }
    #[test]
    fn test_wrap_single_line_with_subsequent_indent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, usize, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let mut lines = vec![Cow::from(rug_fuzz_1)];
        let options = Options {
            width: rug_fuzz_2,
            line_ending: LineEnding::LF,
            initial_indent: rug_fuzz_3,
            subsequent_indent: rug_fuzz_4,
            break_words: rug_fuzz_5,
            wrap_algorithm: WrapAlgorithm::FirstFit,
            word_separator: WordSeparator::new(),
            word_splitter: WordSplitter::HyphenSplitter,
        };
        wrap_single_line(line, &options, &mut lines);
        debug_assert_eq!(
            lines, vec![Cow::from("First line."), Cow::from("++This is a test line.")]
        );
             }
});    }
    #[test]
    fn test_wrap_single_line_with_indent_and_trim() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, usize, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let mut lines = Vec::new();
        let options = Options {
            width: rug_fuzz_1,
            line_ending: LineEnding::LF,
            initial_indent: rug_fuzz_2,
            subsequent_indent: rug_fuzz_3,
            break_words: rug_fuzz_4,
            wrap_algorithm: WrapAlgorithm::FirstFit,
            word_separator: WordSeparator::new(),
            word_splitter: WordSplitter::HyphenSplitter,
        };
        wrap_single_line(line, &options, &mut lines);
        debug_assert_eq!(lines, vec![Cow::from("##This is a test line")]);
             }
});    }
    #[test]
    fn test_wrap_single_line_slow_path() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, usize, &str, &str, bool, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let mut lines = Vec::new();
        let options = Options {
            width: rug_fuzz_1,
            line_ending: LineEnding::LF,
            initial_indent: rug_fuzz_2,
            subsequent_indent: rug_fuzz_3,
            break_words: rug_fuzz_4,
            wrap_algorithm: WrapAlgorithm::FirstFit,
            word_separator: WordSeparator::new(),
            word_splitter: WordSplitter::HyphenSplitter,
        };
        wrap_single_line(line, &options, &mut lines);
        debug_assert!(lines.len() > rug_fuzz_5);
             }
});    }
}
#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;
    use crate::*;
    use crate::core::Word;
    use crate::options::Options;
    use crate::wrap_algorithms::WrapAlgorithm;
    use crate::line_ending::LineEnding;
    use crate::word_separators::WordSeparator;
    use crate::word_splitters::WordSplitter;
    use std::borrow::Cow;
    #[test]
    fn test_wrap_single_line_slow_path_empty_line() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let options = Options::new(rug_fuzz_1);
        let mut lines = Vec::new();
        wrap_single_line_slow_path(line, &options, &mut lines);
        debug_assert_eq!(lines, Vec:: < Cow < str > > ::new());
             }
});    }
    #[test]
    fn test_wrap_single_line_slow_path_no_wrap() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let options = Options::new(rug_fuzz_1);
        let mut lines = Vec::new();
        wrap_single_line_slow_path(line, &options, &mut lines);
        debug_assert_eq!(lines, vec![Cow::from("small line")]);
             }
});    }
    #[test]
    fn test_wrap_single_line_slow_path_with_wrap() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let options = Options::new(rug_fuzz_1)
            .word_separator(WordSeparator::AsciiSpace)
            .wrap_algorithm(WrapAlgorithm::FirstFit);
        let mut lines = Vec::new();
        wrap_single_line_slow_path(line, &options, &mut lines);
        debug_assert_eq!(
            lines, vec![Cow::from("Wrap this "), Cow::from("line "), Cow::from("please")]
        );
             }
});    }
    #[test]
    fn test_wrap_single_line_slow_path_with_long_word() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let options = Options::new(rug_fuzz_1)
            .word_separator(WordSeparator::AsciiSpace)
            .wrap_algorithm(WrapAlgorithm::FirstFit)
            .break_words(rug_fuzz_2)
            .word_splitter(WordSplitter::HyphenSplitter);
        let mut lines = Vec::new();
        wrap_single_line_slow_path(line, &options, &mut lines);
        debug_assert_eq!(
            lines, vec![Cow::from("Wrap this "), Cow::from("supercalif"),
            Cow::from("ragilistic"), Cow::from("expialido"), Cow::from("cious line"),
            Cow::from("please")]
        );
             }
});    }
    #[test]
    fn test_wrap_single_line_slow_path_initial_indent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let options = Options::new(rug_fuzz_1)
            .word_separator(WordSeparator::AsciiSpace)
            .wrap_algorithm(WrapAlgorithm::FirstFit)
            .initial_indent(rug_fuzz_2);
        let mut lines = Vec::new();
        wrap_single_line_slow_path(line, &options, &mut lines);
        debug_assert_eq!(
            lines, vec![Cow::from(">> Wrap this"), Cow::from("line with"),
            Cow::from("initial"), Cow::from("indent")]
        );
             }
});    }
    #[test]
    fn test_wrap_single_line_slow_path_subsequent_indent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let options = Options::new(rug_fuzz_1)
            .word_separator(WordSeparator::AsciiSpace)
            .wrap_algorithm(WrapAlgorithm::FirstFit)
            .subsequent_indent(rug_fuzz_2);
        let mut lines = Vec::new();
        wrap_single_line_slow_path(line, &options, &mut lines);
        debug_assert_eq!(
            lines, vec![Cow::from("Wrap this line"), Cow::from("... with"),
            Cow::from("... subsequent"), Cow::from("... indent")]
        );
             }
});    }
    #[test]
    fn test_wrap_single_line_slow_path_initial_and_subsequent_indent() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, usize, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let line = rug_fuzz_0;
        let options = Options::new(rug_fuzz_1)
            .word_separator(WordSeparator::AsciiSpace)
            .wrap_algorithm(WrapAlgorithm::FirstFit)
            .initial_indent(rug_fuzz_2)
            .subsequent_indent(rug_fuzz_3);
        let mut lines = Vec::new();
        wrap_single_line_slow_path(line, &options, &mut lines);
        debug_assert_eq!(
            lines, vec![Cow::from(">>Wrap this line"), Cow::from("...with both"),
            Cow::from("...indents")]
        );
             }
});    }
}

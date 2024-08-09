//! Building blocks for advanced wrapping functionality.
//!
//! The functions and structs in this module can be used to implement
//! advanced wrapping functionality when [`wrap()`](crate::wrap())
//! [`fill()`](crate::fill()) don't do what you want.
//!
//! In general, you want to follow these steps when wrapping
//! something:
//!
//! 1. Split your input into [`Fragment`]s. These are abstract blocks
//!    of text or content which can be wrapped into lines. See
//!    [`WordSeparator`](crate::word_separators::WordSeparator) for
//!    how to do this for text.
//!
//! 2. Potentially split your fragments into smaller pieces. This
//!    allows you to implement things like hyphenation. If you use the
//!    `Word` type, you can use [`WordSplitter`](crate::WordSplitter)
//!    enum for this.
//!
//! 3. Potentially break apart fragments that are still too large to
//!    fit on a single line. This is implemented in [`break_words`].
//!
//! 4. Finally take your fragments and put them into lines. There are
//!    two algorithms for this in the
//!    [`wrap_algorithms`](crate::wrap_algorithms) module:
//!    [`wrap_optimal_fit`](crate::wrap_algorithms::wrap_optimal_fit)
//!    and [`wrap_first_fit`](crate::wrap_algorithms::wrap_first_fit).
//!    The former produces better line breaks, the latter is faster.
//!
//! 5. Iterate through the slices returned by the wrapping functions
//!    and construct your lines of output.
//!
//! Please [open an issue](https://github.com/mgeisler/textwrap/) if
//! the functionality here is not sufficient or if you have ideas for
//! improving it. We would love to hear from you!
/// The CSI or “Control Sequence Introducer” introduces an ANSI escape
/// sequence. This is typically used for colored text and will be
/// ignored when computing the text width.
const CSI: (char, char) = ('\x1b', '[');
/// The final bytes of an ANSI escape sequence must be in this range.
const ANSI_FINAL_BYTE: std::ops::RangeInclusive<char> = '\x40'..='\x7e';
/// Skip ANSI escape sequences. The `ch` is the current `char`, the
/// `chars` provide the following characters. The `chars` will be
/// modified if `ch` is the start of an ANSI escape sequence.
#[inline]
pub(crate) fn skip_ansi_escape_sequence<I: Iterator<Item = char>>(
    ch: char,
    chars: &mut I,
) -> bool {
    if ch == CSI.0 && chars.next() == Some(CSI.1) {
        for ch in chars {
            if ANSI_FINAL_BYTE.contains(&ch) {
                return true;
            }
        }
    }
    false
}
#[cfg(feature = "unicode-width")]
#[inline]
fn ch_width(ch: char) -> usize {
    unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0)
}
/// First character which [`ch_width`] will classify as double-width.
/// Please see [`display_width`].
#[cfg(not(feature = "unicode-width"))]
const DOUBLE_WIDTH_CUTOFF: char = '\u{1100}';
#[cfg(not(feature = "unicode-width"))]
#[inline]
fn ch_width(ch: char) -> usize {
    if ch < DOUBLE_WIDTH_CUTOFF { 1 } else { 2 }
}
/// Compute the display width of `text` while skipping over ANSI
/// escape sequences.
///
/// # Examples
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!(display_width("Café Plain"), 10);
/// assert_eq!(display_width("\u{1b}[31mCafé Rouge\u{1b}[0m"), 10);
/// ```
///
/// **Note:** When the `unicode-width` Cargo feature is disabled, the
/// width of a `char` is determined by a crude approximation which
/// simply counts chars below U+1100 as 1 column wide, and all other
/// characters as 2 columns wide. With the feature enabled, function
/// will correctly deal with [combining characters] in their
/// decomposed form (see [Unicode equivalence]).
///
/// An example of a decomposed character is “é”, which can be
/// decomposed into: “e” followed by a combining acute accent: “◌́”.
/// Without the `unicode-width` Cargo feature, every `char` below
/// U+1100 has a width of 1. This includes the combining accent:
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!(display_width("Cafe Plain"), 10);
/// #[cfg(feature = "unicode-width")]
/// assert_eq!(display_width("Cafe\u{301} Plain"), 10);
/// #[cfg(not(feature = "unicode-width"))]
/// assert_eq!(display_width("Cafe\u{301} Plain"), 11);
/// ```
///
/// ## Emojis and CJK Characters
///
/// Characters such as emojis and [CJK characters] used in the
/// Chinese, Japanese, and Korean languages are seen as double-width,
/// even if the `unicode-width` feature is disabled:
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!(display_width("😂😭🥺🤣✨😍🙏🥰😊🔥"), 20);
/// assert_eq!(display_width("你好"), 4);  // “Nǐ hǎo” or “Hello” in Chinese
/// ```
///
/// # Limitations
///
/// The displayed width of a string cannot always be computed from the
/// string alone. This is because the width depends on the rendering
/// engine used. This is particularly visible with [emoji modifier
/// sequences] where a base emoji is modified with, e.g., skin tone or
/// hair color modifiers. It is up to the rendering engine to detect
/// this and to produce a suitable emoji.
///
/// A simple example is “❤️”, which consists of “❤” (U+2764: Black
/// Heart Symbol) followed by U+FE0F (Variation Selector-16). By
/// itself, “❤” is a black heart, but if you follow it with the
/// variant selector, you may get a wider red heart.
///
/// A more complex example would be “👨‍🦰” which should depict a man
/// with red hair. Here the computed width is too large — and the
/// width differs depending on the use of the `unicode-width` feature:
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!("👨‍🦰".chars().collect::<Vec<char>>(), ['\u{1f468}', '\u{200d}', '\u{1f9b0}']);
/// #[cfg(feature = "unicode-width")]
/// assert_eq!(display_width("👨‍🦰"), 4);
/// #[cfg(not(feature = "unicode-width"))]
/// assert_eq!(display_width("👨‍🦰"), 6);
/// ```
///
/// This happens because the grapheme consists of three code points:
/// “👨” (U+1F468: Man), Zero Width Joiner (U+200D), and “🦰”
/// (U+1F9B0: Red Hair). You can see them above in the test. With
/// `unicode-width` enabled, the ZWJ is correctly seen as having zero
/// width, without it is counted as a double-width character.
///
/// ## Terminal Support
///
/// Modern browsers typically do a great job at combining characters
/// as shown above, but terminals often struggle more. As an example,
/// Gnome Terminal version 3.38.1, shows “❤️” as a big red heart, but
/// shows "👨‍🦰" as “👨🦰”.
///
/// [combining characters]: https://en.wikipedia.org/wiki/Combining_character
/// [Unicode equivalence]: https://en.wikipedia.org/wiki/Unicode_equivalence
/// [CJK characters]: https://en.wikipedia.org/wiki/CJK_characters
/// [emoji modifier sequences]: https://unicode.org/emoji/charts/full-emoji-modifiers.html
pub fn display_width(text: &str) -> usize {
    let mut chars = text.chars();
    let mut width = 0;
    while let Some(ch) = chars.next() {
        if skip_ansi_escape_sequence(ch, &mut chars) {
            continue;
        }
        width += ch_width(ch);
    }
    width
}
/// A (text) fragment denotes the unit which we wrap into lines.
///
/// Fragments represent an abstract _word_ plus the _whitespace_
/// following the word. In case the word falls at the end of the line,
/// the whitespace is dropped and a so-called _penalty_ is inserted
/// instead (typically `"-"` if the word was hyphenated).
///
/// For wrapping purposes, the precise content of the word, the
/// whitespace, and the penalty is irrelevant. All we need to know is
/// the displayed width of each part, which this trait provides.
pub trait Fragment: std::fmt::Debug {
    /// Displayed width of word represented by this fragment.
    fn width(&self) -> f64;
    /// Displayed width of the whitespace that must follow the word
    /// when the word is not at the end of a line.
    fn whitespace_width(&self) -> f64;
    /// Displayed width of the penalty that must be inserted if the
    /// word falls at the end of a line.
    fn penalty_width(&self) -> f64;
}
/// A piece of wrappable text, including any trailing whitespace.
///
/// A `Word` is an example of a [`Fragment`], so it has a width,
/// trailing whitespace, and potentially a penalty item.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    /// Word content.
    pub word: &'a str,
    /// Whitespace to insert if the word does not fall at the end of a line.
    pub whitespace: &'a str,
    /// Penalty string to insert if the word falls at the end of a line.
    pub penalty: &'a str,
    pub(crate) width: usize,
}
impl std::ops::Deref for Word<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.word
    }
}
impl<'a> Word<'a> {
    /// Construct a `Word` from a string.
    ///
    /// A trailing stretch of `' '` is automatically taken to be the
    /// whitespace part of the word.
    pub fn from(word: &str) -> Word<'_> {
        let trimmed = word.trim_end_matches(' ');
        Word {
            word: trimmed,
            width: display_width(trimmed),
            whitespace: &word[trimmed.len()..],
            penalty: "",
        }
    }
    /// Break this word into smaller words with a width of at most
    /// `line_width`. The whitespace and penalty from this `Word` is
    /// added to the last piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::core::Word;
    /// assert_eq!(
    ///     Word::from("Hello!  ").break_apart(3).collect::<Vec<_>>(),
    ///     vec![Word::from("Hel"), Word::from("lo!  ")]
    /// );
    /// ```
    pub fn break_apart<'b>(
        &'b self,
        line_width: usize,
    ) -> impl Iterator<Item = Word<'a>> + 'b {
        let mut char_indices = self.word.char_indices();
        let mut offset = 0;
        let mut width = 0;
        std::iter::from_fn(move || {
            while let Some((idx, ch)) = char_indices.next() {
                if skip_ansi_escape_sequence(
                    ch,
                    &mut char_indices.by_ref().map(|(_, ch)| ch),
                ) {
                    continue;
                }
                if width > 0 && width + ch_width(ch) > line_width {
                    let word = Word {
                        word: &self.word[offset..idx],
                        width: width,
                        whitespace: "",
                        penalty: "",
                    };
                    offset = idx;
                    width = ch_width(ch);
                    return Some(word);
                }
                width += ch_width(ch);
            }
            if offset < self.word.len() {
                let word = Word {
                    word: &self.word[offset..],
                    width: width,
                    whitespace: self.whitespace,
                    penalty: self.penalty,
                };
                offset = self.word.len();
                return Some(word);
            }
            None
        })
    }
}
impl Fragment for Word<'_> {
    #[inline]
    fn width(&self) -> f64 {
        self.width as f64
    }
    #[inline]
    fn whitespace_width(&self) -> f64 {
        self.whitespace.len() as f64
    }
    #[inline]
    fn penalty_width(&self) -> f64 {
        self.penalty.len() as f64
    }
}
/// Forcibly break words wider than `line_width` into smaller words.
///
/// This simply calls [`Word::break_apart`] on words that are too
/// wide. This means that no extra `'-'` is inserted, the word is
/// simply broken into smaller pieces.
pub fn break_words<'a, I>(words: I, line_width: usize) -> Vec<Word<'a>>
where
    I: IntoIterator<Item = Word<'a>>,
{
    let mut shortened_words = Vec::new();
    for word in words {
        if word.width() > line_width as f64 {
            shortened_words.extend(word.break_apart(line_width));
        } else {
            shortened_words.push(word);
        }
    }
    shortened_words
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "unicode-width")]
    use unicode_width::UnicodeWidthChar;
    #[test]
    fn skip_ansi_escape_sequence_works() {
        let blue_text = "\u{1b}[34mHello\u{1b}[0m";
        let mut chars = blue_text.chars();
        let ch = chars.next().unwrap();
        assert!(skip_ansi_escape_sequence(ch, & mut chars));
        assert_eq!(chars.next(), Some('H'));
    }
    #[test]
    fn emojis_have_correct_width() {
        use unic_emoji_char::is_emoji;
        for ch in '\u{1}'..'\u{FF}' {
            if is_emoji(ch) {
                let desc = format!("{:?} U+{:04X}", ch, ch as u32);
                #[cfg(feature = "unicode-width")]
                assert_eq!(ch.width().unwrap(), 1, "char: {}", desc);
                #[cfg(not(feature = "unicode-width"))]
                assert_eq!(ch_width(ch), 1, "char: {}", desc);
            }
        }
        for ch in '\u{FF}'..'\u{2FFFF}' {
            if is_emoji(ch) {
                let desc = format!("{:?} U+{:04X}", ch, ch as u32);
                #[cfg(feature = "unicode-width")]
                assert!(ch.width().unwrap() <= 2, "char: {}", desc);
                #[cfg(not(feature = "unicode-width"))]
                assert_eq!(ch_width(ch), 2, "char: {}", desc);
            }
        }
    }
    #[test]
    fn display_width_works() {
        assert_eq!("Café Plain".len(), 11);
        assert_eq!(display_width("Café Plain"), 10);
        assert_eq!(display_width("\u{1b}[31mCafé Rouge\u{1b}[0m"), 10);
    }
    #[test]
    fn display_width_narrow_emojis() {
        #[cfg(feature = "unicode-width")] assert_eq!(display_width("⁉"), 1);
        #[cfg(not(feature = "unicode-width"))] assert_eq!(display_width("⁉"), 2);
    }
    #[test]
    fn display_width_narrow_emojis_variant_selector() {
        #[cfg(feature = "unicode-width")] assert_eq!(display_width("⁉\u{fe0f}"), 1);
        #[cfg(not(feature = "unicode-width"))]
        assert_eq!(display_width("⁉\u{fe0f}"), 4);
    }
    #[test]
    fn display_width_emojis() {
        assert_eq!(display_width("😂😭🥺🤣✨😍🙏🥰😊🔥"), 20);
    }
}
#[cfg(test)]
mod tests_llm_16_1 {
    use super::*;
    use crate::*;
    #[test]
    fn penalty_width_empty_penalty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let word = Word {
            word: rug_fuzz_0,
            whitespace: rug_fuzz_1,
            penalty: rug_fuzz_2,
            width: rug_fuzz_3,
        };
        debug_assert_eq!(word.penalty_width(), 0.0);
             }
}
}
}    }
    #[test]
    fn penalty_width_non_empty_penalty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let word = Word {
            word: rug_fuzz_0,
            whitespace: rug_fuzz_1,
            penalty: rug_fuzz_2,
            width: rug_fuzz_3,
        };
        debug_assert_eq!(word.penalty_width(), 1.0);
             }
}
}
}    }
    #[test]
    fn penalty_width_with_multiple_chars_penalty() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let word = Word {
            word: rug_fuzz_0,
            whitespace: rug_fuzz_1,
            penalty: rug_fuzz_2,
            width: rug_fuzz_3,
        };
        debug_assert_eq!(word.penalty_width(), 2.0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_2 {
    use super::*;
    use crate::*;
    #[test]
    fn whitespace_width_test() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11)) = <(&str, &str, &str, usize, &str, &str, &str, usize, &str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let word_with_space = Word {
            word: rug_fuzz_0,
            whitespace: rug_fuzz_1,
            penalty: rug_fuzz_2,
            width: rug_fuzz_3,
        };
        debug_assert_eq!(word_with_space.whitespace_width(), 5.0);
        let word_with_no_space = Word {
            word: rug_fuzz_4,
            whitespace: rug_fuzz_5,
            penalty: rug_fuzz_6,
            width: rug_fuzz_7,
        };
        debug_assert_eq!(word_with_no_space.whitespace_width(), 0.0);
        let word_with_mixed_space = Word {
            word: rug_fuzz_8,
            whitespace: rug_fuzz_9,
            penalty: rug_fuzz_10,
            width: rug_fuzz_11,
        };
        debug_assert_eq!(word_with_mixed_space.whitespace_width(), 3.0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_3 {
    use super::*;
    use crate::*;
    #[test]
    fn test_word_width() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let word = Word {
            word: rug_fuzz_0,
            whitespace: rug_fuzz_1,
            penalty: rug_fuzz_2,
            width: rug_fuzz_3,
        };
        debug_assert_eq!(word.width(), 4.0);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_4 {
    use super::*;
    use crate::*;
    #[test]
    fn word_deref() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let word = Word::from(rug_fuzz_0);
        let word_str: &str = &word;
        debug_assert_eq!(word_str, "hello");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_14 {
    use super::*;
    use crate::*;
    #[test]
    fn test_break_apart() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(&str, usize, &str, usize, &str, &str, &str, usize, usize, &str, usize, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let word = Word::from(rug_fuzz_0);
        let broken_words: Vec<Word> = word.break_apart(rug_fuzz_1).collect();
        debug_assert_eq!(broken_words, vec![Word::from("Hello"), Word::from("World")]);
        let word = Word::from(rug_fuzz_2);
        let broken_words: Vec<Word> = word.break_apart(rug_fuzz_3).collect();
        debug_assert_eq!(broken_words, vec![Word::from("Hel"), Word::from("lo  ")]);
        let word = Word {
            word: rug_fuzz_4,
            whitespace: rug_fuzz_5,
            penalty: rug_fuzz_6,
            width: rug_fuzz_7,
        };
        let broken_words: Vec<Word> = word.break_apart(rug_fuzz_8).collect();
        debug_assert_eq!(
            broken_words, vec![Word::from("Hello"), Word { word : "-", whitespace : "",
            penalty : "-", width : 1, }]
        );
        let word = Word::from(rug_fuzz_9);
        let broken_words: Vec<Word> = word.break_apart(rug_fuzz_10).collect();
        debug_assert_eq!(broken_words, vec![Word::from("Hey")]);
        let word = Word::from(rug_fuzz_11);
        let broken_words: Vec<Word> = word.break_apart(rug_fuzz_12).collect();
        debug_assert_eq!(broken_words, vec![]);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_15 {
    use super::*;
    use crate::*;
    #[test]
    fn test_word_from_empty_string() {
        let input = "";
        let word = Word::from(input);
        assert_eq!(word.word, "");
        assert_eq!(word.whitespace, "");
        assert_eq!(word.penalty, "");
        assert_eq!(word.width, 0);
    }
    #[test]
    fn test_word_from_non_empty_string() {
        let input = "hello";
        let word = Word::from(input);
        assert_eq!(word.word, "hello");
        assert_eq!(word.whitespace, "");
        assert_eq!(word.penalty, "");
        assert_eq!(word.width, display_width("hello"));
    }
    #[test]
    fn test_word_from_string_with_trailing_whitespace() {
        let input = "hello   ";
        let word = Word::from(input);
        assert_eq!(word.word, "hello");
        assert_eq!(word.whitespace, "   ");
        assert_eq!(word.penalty, "");
        assert_eq!(word.width, display_width("hello"));
    }
    #[test]
    fn test_word_from_string_with_only_whitespace() {
        let input = "     ";
        let word = Word::from(input);
        assert_eq!(word.word, "");
        assert_eq!(word.whitespace, "     ");
        assert_eq!(word.penalty, "");
        assert_eq!(word.width, 0);
    }
    #[test]
    fn test_word_from_string_with_internal_whitespace() {
        let input = "he llo  ";
        let word = Word::from(input);
        assert_eq!(word.word, "he llo");
        assert_eq!(word.whitespace, "  ");
        assert_eq!(word.penalty, "");
        assert_eq!(word.width, display_width("he llo"));
    }
    fn display_width(s: &str) -> usize {
        s.chars().map(|ch| ch_width(ch)).sum()
    }
    fn ch_width(ch: char) -> usize {
        1
    }
}
#[cfg(test)]
mod tests_llm_16_19_llm_16_19 {
    use crate::core::skip_ansi_escape_sequence;
    use crate::line_ending::NonEmptyLines;
    use std::iter::Iterator;
    const CSI: (char, char) = ('\x1B', '[');
    const ANSI_FINAL_BYTE: &[char] = &[
        '\x40',
        '\x41',
        '\x42',
        '\x43',
        '\x44',
        '\x45',
        '\x46',
        '\x47',
        '\x48',
        '\x49',
        '\x4A',
        '\x4B',
        '\x4C',
        '\x4D',
        '\x4E',
        '\x4F',
        '\x50',
        '\x51',
        '\x52',
        '\x53',
        '\x54',
        '\x55',
        '\x56',
        '\x57',
        '\x58',
        '\x59',
        '\x5A',
        '\x5B',
        '\x5C',
        '\x5D',
        '\x5E',
        '\x5F',
        '\x60',
        '\x61',
        '\x62',
        '\x63',
        '\x64',
        '\x65',
        '\x66',
        '\x67',
        '\x68',
        '\x69',
        '\x6A',
        '\x6B',
        '\x6C',
        '\x6D',
        '\x6E',
        '\x6F',
        '\x70',
        '\x71',
        '\x72',
        '\x73',
        '\x74',
        '\x75',
        '\x76',
        '\x77',
        '\x78',
        '\x79',
        '\x7A',
        '\x7B',
        '\x7C',
        '\x7D',
        '\x7E',
    ];
    #[test]
    fn test_skip_ansi_escape_sequence() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, i32, i32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let csi = CSI.0;
        let seq1 = rug_fuzz_0;
        let mut chars1 = seq1.chars();
        debug_assert!(skip_ansi_escape_sequence(csi, & mut chars1));
        debug_assert_eq!(chars1.collect:: < String > (), "Hello World\x1B[0m");
        let seq2 = rug_fuzz_1;
        let mut chars2 = seq2.chars();
        (rug_fuzz_2..rug_fuzz_3)
            .for_each(|_| {
                chars2.next();
            });
        debug_assert!(skip_ansi_escape_sequence(csi, & mut chars2));
        debug_assert_eq!(chars2.collect:: < String > (), "World\x1B[0m");
        let seq3 = rug_fuzz_4;
        let mut chars3 = seq3.chars();
        debug_assert!(! skip_ansi_escape_sequence(csi, & mut chars3));
        debug_assert_eq!(chars3.collect:: < String > (), "Hello World");
        let seq4 = rug_fuzz_5;
        let mut chars4 = seq4.chars();
        debug_assert!(! skip_ansi_escape_sequence(csi, & mut chars4));
        debug_assert_eq!(chars4.collect:: < String > (), "Hello World");
             }
}
}
}    }
}

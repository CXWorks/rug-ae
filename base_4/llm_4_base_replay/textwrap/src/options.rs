//! Options for wrapping text.
use crate::{LineEnding, WordSeparator, WordSplitter, WrapAlgorithm};
/// Holds configuration options for wrapping and filling text.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Options<'a> {
    /// The width in columns at which the text will be wrapped.
    pub width: usize,
    /// Line ending used for breaking lines.
    pub line_ending: LineEnding,
    /// Indentation used for the first line of output. See the
    /// [`Options::initial_indent`] method.
    pub initial_indent: &'a str,
    /// Indentation used for subsequent lines of output. See the
    /// [`Options::subsequent_indent`] method.
    pub subsequent_indent: &'a str,
    /// Allow long words to be broken if they cannot fit on a line.
    /// When set to `false`, some lines may be longer than
    /// `self.width`. See the [`Options::break_words`] method.
    pub break_words: bool,
    /// Wrapping algorithm to use, see the implementations of the
    /// [`WrapAlgorithm`] trait for details.
    pub wrap_algorithm: WrapAlgorithm,
    /// The line breaking algorithm to use, see the [`WordSeparator`]
    /// trait for an overview and possible implementations.
    pub word_separator: WordSeparator,
    /// The method for splitting words. This can be used to prohibit
    /// splitting words on hyphens, or it can be used to implement
    /// language-aware machine hyphenation.
    pub word_splitter: WordSplitter,
}
impl<'a> From<&'a Options<'a>> for Options<'a> {
    fn from(options: &'a Options<'a>) -> Self {
        Self {
            width: options.width,
            line_ending: options.line_ending,
            initial_indent: options.initial_indent,
            subsequent_indent: options.subsequent_indent,
            break_words: options.break_words,
            word_separator: options.word_separator,
            wrap_algorithm: options.wrap_algorithm,
            word_splitter: options.word_splitter.clone(),
        }
    }
}
impl<'a> From<usize> for Options<'a> {
    fn from(width: usize) -> Self {
        Options::new(width)
    }
}
impl<'a> Options<'a> {
    /// Creates a new [`Options`] with the specified width.
    ///
    /// The other fields are given default values as follows:
    ///
    /// ```
    /// # use textwrap::{LineEnding, Options, WordSplitter, WordSeparator, WrapAlgorithm};
    /// # let width = 80;
    /// let options = Options::new(width);
    /// assert_eq!(options.line_ending, LineEnding::LF);
    /// assert_eq!(options.initial_indent, "");
    /// assert_eq!(options.subsequent_indent, "");
    /// assert_eq!(options.break_words, true);
    ///
    /// #[cfg(feature = "unicode-linebreak")]
    /// assert_eq!(options.word_separator, WordSeparator::UnicodeBreakProperties);
    /// #[cfg(not(feature = "unicode-linebreak"))]
    /// assert_eq!(options.word_separator, WordSeparator::AsciiSpace);
    ///
    /// #[cfg(feature = "smawk")]
    /// assert_eq!(options.wrap_algorithm, WrapAlgorithm::new_optimal_fit());
    /// #[cfg(not(feature = "smawk"))]
    /// assert_eq!(options.wrap_algorithm, WrapAlgorithm::FirstFit);
    ///
    /// assert_eq!(options.word_splitter, WordSplitter::HyphenSplitter);
    /// ```
    ///
    /// Note that the default word separator and wrap algorithms
    /// changes based on the available Cargo features. The best
    /// available algorithms are used by default.
    pub const fn new(width: usize) -> Self {
        Options {
            width,
            line_ending: LineEnding::LF,
            initial_indent: "",
            subsequent_indent: "",
            break_words: true,
            word_separator: WordSeparator::new(),
            wrap_algorithm: WrapAlgorithm::new(),
            word_splitter: WordSplitter::HyphenSplitter,
        }
    }
    /// Change [`self.line_ending`]. This specifies which of the
    /// supported line endings should be used to break the lines of the
    /// input text.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::{refill, LineEnding, Options};
    ///
    /// let options = Options::new(15).line_ending(LineEnding::CRLF);
    /// assert_eq!(refill("This is a little example.", options),
    ///            "This is a\r\nlittle example.");
    /// ```
    ///
    /// [`self.line_ending`]: #structfield.line_ending
    pub fn line_ending(self, line_ending: LineEnding) -> Self {
        Options { line_ending, ..self }
    }
    /// Change [`self.initial_indent`]. The initial indentation is
    /// used on the very first line of output.
    ///
    /// # Examples
    ///
    /// Classic paragraph indentation can be achieved by specifying an
    /// initial indentation and wrapping each paragraph by itself:
    ///
    /// ```
    /// use textwrap::{wrap, Options};
    ///
    /// let options = Options::new(16).initial_indent("    ");
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["    This is a",
    ///                 "little example."]);
    /// ```
    ///
    /// [`self.initial_indent`]: #structfield.initial_indent
    pub fn initial_indent(self, indent: &'a str) -> Self {
        Options {
            initial_indent: indent,
            ..self
        }
    }
    /// Change [`self.subsequent_indent`]. The subsequent indentation
    /// is used on lines following the first line of output.
    ///
    /// # Examples
    ///
    /// Combining initial and subsequent indentation lets you format a
    /// single paragraph as a bullet list:
    ///
    /// ```
    /// use textwrap::{wrap, Options};
    ///
    /// let options = Options::new(12)
    ///     .initial_indent("* ")
    ///     .subsequent_indent("  ");
    /// #[cfg(feature = "smawk")]
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["* This is",
    ///                 "  a little",
    ///                 "  example."]);
    ///
    /// // Without the `smawk` feature, the wrapping is a little different:
    /// #[cfg(not(feature = "smawk"))]
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["* This is a",
    ///                 "  little",
    ///                 "  example."]);
    /// ```
    ///
    /// [`self.subsequent_indent`]: #structfield.subsequent_indent
    pub fn subsequent_indent(self, indent: &'a str) -> Self {
        Options {
            subsequent_indent: indent,
            ..self
        }
    }
    /// Change [`self.break_words`]. This controls if words longer
    /// than `self.width` can be broken, or if they will be left
    /// sticking out into the right margin.
    ///
    /// See [`Options::word_splitter`] instead if you want to control
    /// hyphenation.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::{wrap, Options};
    ///
    /// let options = Options::new(4).break_words(true);
    /// assert_eq!(wrap("This is a little example.", options),
    ///            vec!["This",
    ///                 "is a",
    ///                 "litt",
    ///                 "le",
    ///                 "exam",
    ///                 "ple."]);
    /// ```
    ///
    /// [`self.break_words`]: #structfield.break_words
    pub fn break_words(self, setting: bool) -> Self {
        Options {
            break_words: setting,
            ..self
        }
    }
    /// Change [`self.word_separator`].
    ///
    /// See the [`WordSeparator`] trait for details on the choices.
    ///
    /// [`self.word_separator`]: #structfield.word_separator
    pub fn word_separator(self, word_separator: WordSeparator) -> Options<'a> {
        Options {
            width: self.width,
            line_ending: self.line_ending,
            initial_indent: self.initial_indent,
            subsequent_indent: self.subsequent_indent,
            break_words: self.break_words,
            word_separator: word_separator,
            wrap_algorithm: self.wrap_algorithm,
            word_splitter: self.word_splitter,
        }
    }
    /// Change [`self.wrap_algorithm`].
    ///
    /// See the [`WrapAlgorithm`] trait for details on the choices.
    ///
    /// [`self.wrap_algorithm`]: #structfield.wrap_algorithm
    pub fn wrap_algorithm(self, wrap_algorithm: WrapAlgorithm) -> Options<'a> {
        Options {
            width: self.width,
            line_ending: self.line_ending,
            initial_indent: self.initial_indent,
            subsequent_indent: self.subsequent_indent,
            break_words: self.break_words,
            word_separator: self.word_separator,
            wrap_algorithm: wrap_algorithm,
            word_splitter: self.word_splitter,
        }
    }
    /// Change [`self.word_splitter`]. The [`WordSplitter`] is used to
    /// fit part of a word into the current line when wrapping text.
    ///
    /// See [`Options::break_words`] instead if you want to control the
    /// handling of words longer than the line width.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::{wrap, Options, WordSplitter};
    ///
    /// // The default is WordSplitter::HyphenSplitter.
    /// let options = Options::new(5);
    /// assert_eq!(wrap("foo-bar-baz", &options),
    ///            vec!["foo-", "bar-", "baz"]);
    ///
    /// // The word is now so long that break_words kick in:
    /// let options = Options::new(5)
    ///     .word_splitter(WordSplitter::NoHyphenation);
    /// assert_eq!(wrap("foo-bar-baz", &options),
    ///            vec!["foo-b", "ar-ba", "z"]);
    ///
    /// // If you want to breaks at all, disable both:
    /// let options = Options::new(5)
    ///     .break_words(false)
    ///     .word_splitter(WordSplitter::NoHyphenation);
    /// assert_eq!(wrap("foo-bar-baz", &options),
    ///            vec!["foo-bar-baz"]);
    /// ```
    ///
    /// [`self.word_splitter`]: #structfield.word_splitter
    pub fn word_splitter(self, word_splitter: WordSplitter) -> Options<'a> {
        Options {
            width: self.width,
            line_ending: self.line_ending,
            initial_indent: self.initial_indent,
            subsequent_indent: self.subsequent_indent,
            break_words: self.break_words,
            word_separator: self.word_separator,
            wrap_algorithm: self.wrap_algorithm,
            word_splitter,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn options_agree_with_usize() {
        let opt_usize = Options::from(42_usize);
        let opt_options = Options::new(42);
        assert_eq!(opt_usize.width, opt_options.width);
        assert_eq!(opt_usize.initial_indent, opt_options.initial_indent);
        assert_eq!(opt_usize.subsequent_indent, opt_options.subsequent_indent);
        assert_eq!(opt_usize.break_words, opt_options.break_words);
        assert_eq!(
            opt_usize.word_splitter.split_points("hello-world"), opt_options
            .word_splitter.split_points("hello-world")
        );
    }
}
#[cfg(test)]
mod tests_llm_16_6 {
    use super::*;
    use crate::*;
    use crate::wrap_algorithms::WrapAlgorithm;
    use crate::line_ending::LineEnding;
    use crate::word_separators::WordSeparator;
    use crate::word_splitters::WordSplitter;
    #[test]
    fn test_from_options() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let src_options = Options::new(rug_fuzz_0)
            .line_ending(LineEnding::CRLF)
            .initial_indent(rug_fuzz_1)
            .subsequent_indent(rug_fuzz_2)
            .break_words(rug_fuzz_3)
            .word_separator(WordSeparator::AsciiSpace)
            .wrap_algorithm(WrapAlgorithm::FirstFit)
            .word_splitter(WordSplitter::HyphenSplitter);
        let new_options = Options::from(&src_options);
        debug_assert_eq!(new_options.width, 20);
        debug_assert_eq!(new_options.line_ending, LineEnding::CRLF);
        debug_assert_eq!(new_options.initial_indent, "->");
        debug_assert_eq!(new_options.subsequent_indent, "--");
        debug_assert_eq!(new_options.break_words, false);
        debug_assert_eq!(new_options.word_separator, WordSeparator::AsciiSpace);
        debug_assert_eq!(new_options.wrap_algorithm, WrapAlgorithm::FirstFit);
        debug_assert_eq!(new_options.word_splitter, WordSplitter::HyphenSplitter);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;
    use crate::*;
    #[test]
    fn options_from_usize() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let width = rug_fuzz_0;
        let options = Options::from(width);
        debug_assert_eq!(options.width, width);
        debug_assert_eq!(options.line_ending, LineEnding::LF);
        debug_assert_eq!(options.initial_indent, "");
        debug_assert_eq!(options.subsequent_indent, "");
        debug_assert_eq!(options.break_words, true);
        #[cfg(feature = "unicode-linebreak")]
        debug_assert_eq!(options.word_separator, WordSeparator::UnicodeBreakProperties);
        #[cfg(not(feature = "unicode-linebreak"))]
        debug_assert_eq!(options.word_separator, WordSeparator::AsciiSpace);
        #[cfg(feature = "smawk")]
        debug_assert_eq!(options.wrap_algorithm, WrapAlgorithm::new_optimal_fit());
        #[cfg(not(feature = "smawk"))]
        debug_assert_eq!(options.wrap_algorithm, WrapAlgorithm::FirstFit);
        debug_assert_eq!(options.word_splitter, WordSplitter::HyphenSplitter);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_26 {
    use super::*;
    use crate::*;
    #[test]
    fn test_break_words() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, bool, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut options = Options::new(rug_fuzz_0);
        options = options.break_words(rug_fuzz_1);
        debug_assert!(options.break_words);
        options = options.break_words(rug_fuzz_2);
        debug_assert!(! options.break_words);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_27 {
    use super::*;
    use crate::*;
    use crate::wrap_algorithms::WrapAlgorithm;
    use crate::line_ending::LineEnding;
    use crate::word_separators::WordSeparator;
    use crate::word_splitters::WordSplitter;
    #[test]
    fn test_initial_indent() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, &str, &str, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let opt = Options::new(rug_fuzz_0).initial_indent(rug_fuzz_1);
        debug_assert_eq!(opt.initial_indent, "* ");
        let opt = opt.initial_indent(rug_fuzz_2);
        debug_assert_eq!(opt.initial_indent, "");
        let opt = Options::new(rug_fuzz_3).initial_indent(rug_fuzz_4);
        debug_assert_eq!(opt.initial_indent, ">> ");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_28_llm_16_28 {
    use super::*;
    use crate::*;
    use crate::Options;
    use crate::core::Word;
    use crate::line_ending::LineEnding;
    use crate::wrap_algorithms::WrapAlgorithm;
    use crate::wrap_algorithms::WrapAlgorithm::OptimalFit;
    use crate::word_splitters::WordSplitter;
    use crate::word_separators::WordSeparator;
    #[test]
    fn test_line_ending_crlf() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let options = Options::new(rug_fuzz_0).line_ending(LineEnding::CRLF);
        debug_assert_eq!(options.line_ending, LineEnding::CRLF);
             }
}
}
}    }
    #[test]
    fn test_line_ending_lf() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let options = Options::new(rug_fuzz_0).line_ending(LineEnding::LF);
        debug_assert_eq!(options.line_ending, LineEnding::LF);
             }
}
}
}    }
    #[test]
    fn test_line_ending_retains_other_fields() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, &str, &str, bool) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let options = Options {
            width: rug_fuzz_0,
            line_ending: LineEnding::LF,
            initial_indent: rug_fuzz_1,
            subsequent_indent: rug_fuzz_2,
            break_words: rug_fuzz_3,
            word_separator: WordSeparator::AsciiSpace,
            wrap_algorithm: WrapAlgorithm::new(),
            word_splitter: WordSplitter::NoHyphenation,
        };
        let new_options = options.line_ending(LineEnding::CRLF);
        debug_assert_eq!(new_options.line_ending, LineEnding::CRLF);
        debug_assert_eq!(new_options.width, 42);
        debug_assert_eq!(new_options.initial_indent, ">> ");
        debug_assert_eq!(new_options.subsequent_indent, "|| ");
        debug_assert_eq!(new_options.break_words, false);
        debug_assert_eq!(new_options.word_separator, WordSeparator::AsciiSpace);
        debug_assert_eq!(new_options.wrap_algorithm, WrapAlgorithm::new());
        debug_assert_eq!(new_options.word_splitter, WordSplitter::NoHyphenation);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_29 {
    use super::*;
    use crate::*;
    #[test]
    fn options_new_default_values() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let width = rug_fuzz_0;
        let options = Options::new(width);
        debug_assert_eq!(options.width, width);
        debug_assert_eq!(options.line_ending, LineEnding::LF);
        debug_assert_eq!(options.initial_indent, "");
        debug_assert_eq!(options.subsequent_indent, "");
        debug_assert_eq!(options.break_words, true);
        #[cfg(feature = "unicode-linebreak")]
        debug_assert_eq!(options.word_separator, WordSeparator::UnicodeBreakProperties);
        #[cfg(not(feature = "unicode-linebreak"))]
        debug_assert_eq!(options.word_separator, WordSeparator::AsciiSpace);
        #[cfg(feature = "smawk")]
        debug_assert_eq!(options.wrap_algorithm, WrapAlgorithm::new_optimal_fit());
        #[cfg(not(feature = "smawk"))]
        debug_assert_eq!(options.wrap_algorithm, WrapAlgorithm::FirstFit);
        debug_assert_eq!(options.word_splitter, WordSplitter::HyphenSplitter);
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_30 {
    use super::*;
    use crate::*;
    use crate::options::Options;
    #[test]
    fn test_subsequent_indent() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let options = Options::new(rug_fuzz_0);
        debug_assert_eq!(options.subsequent_indent, "");
        let options = options.subsequent_indent(rug_fuzz_1);
        debug_assert_eq!(options.subsequent_indent, "-> ");
        let options = options.subsequent_indent(rug_fuzz_2);
        debug_assert_eq!(options.subsequent_indent, "");
        let options = options
            .subsequent_indent(rug_fuzz_3)
            .subsequent_indent(rug_fuzz_4)
            .subsequent_indent(rug_fuzz_5);
        debug_assert_eq!(options.subsequent_indent, "");
             }
}
}
}    }
}
#[cfg(test)]
mod tests_llm_16_31 {
    use super::*;
    use crate::*;
    use crate::core::Word;
    use crate::line_ending::LineEnding;
    use crate::word_separators::WordSeparator;
    use crate::word_splitters::WordSplitter;
    use crate::wrap_algorithms::WrapAlgorithm;
    #[test]
    fn test_word_separator_ascii_space() {
        let options = Options::new(80);
        let new_options = options.word_separator(WordSeparator::AsciiSpace);
        assert_eq!(new_options.word_separator, WordSeparator::AsciiSpace);
    }
    #[cfg(feature = "unicode-linebreak")]
    #[test]
    fn test_word_separator_unicode_break_properties() {
        let options = Options::new(80);
        let new_options = options.word_separator(WordSeparator::UnicodeBreakProperties);
        assert_eq!(new_options.word_separator, WordSeparator::UnicodeBreakProperties);
    }
    #[test]
    fn test_word_separator_custom() {
        fn custom_separator(_line: &str) -> Box<dyn Iterator<Item = Word<'_>> + '_> {
            Box::new(std::iter::empty())
        }
        let options = Options::new(80);
        let new_options = options
            .word_separator(WordSeparator::Custom(custom_separator));
        match new_options.word_separator {
            WordSeparator::Custom(_) => {}
            _ => panic!("Expected WordSeparator::Custom"),
        }
    }
}
#[cfg(test)]
mod tests_llm_16_32_llm_16_32 {
    use super::*;
    use crate::*;
    use crate::wrap_algorithms::WrapAlgorithm;
    use crate::word_splitters::WordSplitter;
    use crate::word_separators::WordSeparator;
    #[test]
    fn test_options_word_splitter() {

    extern crate arbitrary;
    if let Ok(folder) = std::env::var("FUZZ_CORPUS"){
                for f in std::fs::read_dir(folder).unwrap(){
                    if let Ok(corpus) = f{
                        let rug_data: &[u8] = &std::fs::read(corpus.path()).unwrap();
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut options = Options::new(rug_fuzz_0)
            .word_splitter(WordSplitter::HyphenSplitter)
            .word_separator(WordSeparator::AsciiSpace)
            .wrap_algorithm(WrapAlgorithm::FirstFit);
        let original_options = options.clone();
        options = options.word_splitter(WordSplitter::NoHyphenation);
        debug_assert_eq!(options.word_splitter, WordSplitter::NoHyphenation);
        debug_assert_eq!(options.width, original_options.width);
        debug_assert_eq!(options.line_ending, original_options.line_ending);
        debug_assert_eq!(options.initial_indent, original_options.initial_indent);
        debug_assert_eq!(options.subsequent_indent, original_options.subsequent_indent);
        debug_assert_eq!(options.break_words, original_options.break_words);
        debug_assert_eq!(options.word_separator, original_options.word_separator);
        debug_assert_eq!(options.wrap_algorithm, original_options.wrap_algorithm);
             }
}
}
}    }
}

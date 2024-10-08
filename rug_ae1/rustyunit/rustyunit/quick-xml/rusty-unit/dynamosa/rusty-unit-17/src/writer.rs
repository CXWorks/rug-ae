//! A module to handle `Writer`

use crate::errors::{Error, Result};
use crate::events::{attributes::Attribute, BytesCData, BytesStart, BytesText, Event};
use std::io::Write;

/// XML writer.
///
/// Writes XML `Event`s to a `Write` implementor.
///
/// # Examples
///
/// ```rust
/// # use pretty_assertions::assert_eq;
/// use quick_xml::{Reader, Writer};
/// use quick_xml::events::{Event, BytesEnd, BytesStart};
/// use std::io::Cursor;
///
/// let xml = r#"<this_tag k1="v1" k2="v2"><child>text</child></this_tag>"#;
/// let mut reader = Reader::from_str(xml);
/// reader.trim_text(true);
/// let mut writer = Writer::new(Cursor::new(Vec::new()));
/// let mut buf = Vec::new();
/// loop {
///     match reader.read_event(&mut buf) {
///         Ok(Event::Start(ref e)) if e.name() == b"this_tag" => {
///
///             // crates a new element ... alternatively we could reuse `e` by calling
///             // `e.into_owned()`
///             let mut elem = BytesStart::owned(b"my_elem".to_vec(), "my_elem".len());
///
///             // collect existing attributes
///             elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
///
///             // copy existing attributes, adds a new my-key="some value" attribute
///             elem.push_attribute(("my-key", "some value"));
///
///             // writes the event to the writer
///             assert!(writer.write_event(Event::Start(elem)).is_ok());
///         },
///         Ok(Event::End(ref e)) if e.name() == b"this_tag" => {
///             assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"my_elem"))).is_ok());
///         },
///         Ok(Event::Eof) => break,
///         // we can either move or borrow the event to write, depending on your use-case
///         Ok(e) => assert!(writer.write_event(&e).is_ok()),
///         Err(e) => panic!("{}", e),
///     }
///     buf.clear();
/// }
///
/// let result = writer.into_inner().into_inner();
/// let expected = r#"<my_elem k1="v1" k2="v2" my-key="some value"><child>text</child></my_elem>"#;
/// assert_eq!(result, expected.as_bytes());
/// ```
#[derive(Clone)]
pub struct Writer<W: Write> {
    /// underlying writer
    writer: W,
    indent: Option<Indentation>,
}

impl<W: Write> Writer<W> {
    /// Creates a Writer from a generic Write
    pub fn new(inner: W) -> Writer<W> {
        Writer {
            writer: inner,
            indent: None,
        }
    }

    /// Creates a Writer with configured whitespace indents from a generic Write
    pub fn new_with_indent(inner: W, indent_char: u8, indent_size: usize) -> Writer<W> {
        Writer {
            writer: inner,
            indent: Some(Indentation::new(indent_char, indent_size)),
        }
    }

    /// Consumes this `Writer`, returning the underlying writer.
    pub fn into_inner(self) -> W {
        self.writer
    }

    /// Get inner writer, keeping ownership
    pub fn inner(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Writes the given event to the underlying writer.
    pub fn write_event<'a, E: AsRef<Event<'a>>>(&mut self, event: E) -> Result<()> {
        let mut next_should_line_break = true;
        let result = match *event.as_ref() {
            Event::Start(ref e) => {
                let result = self.write_wrapped(b"<", e, b">");
                if let Some(i) = self.indent.as_mut() {
                    i.grow();
                }
                result
            }
            Event::End(ref e) => {
                if let Some(i) = self.indent.as_mut() {
                    i.shrink();
                }
                self.write_wrapped(b"</", e, b">")
            }
            Event::Empty(ref e) => self.write_wrapped(b"<", e, b"/>"),
            Event::Text(ref e) => {
                next_should_line_break = false;
                self.write(&e.escaped())
            }
            Event::Comment(ref e) => self.write_wrapped(b"<!--", e, b"-->"),
            Event::CData(ref e) => {
                next_should_line_break = false;
                self.write(b"<![CDATA[")?;
                self.write(e)?;
                self.write(b"]]>")
            }
            Event::Decl(ref e) => self.write_wrapped(b"<?", e, b"?>"),
            Event::PI(ref e) => self.write_wrapped(b"<?", e, b"?>"),
            Event::DocType(ref e) => self.write_wrapped(b"<!DOCTYPE ", e, b">"),
            Event::Eof => Ok(()),
        };
        if let Some(i) = self.indent.as_mut() {
            i.should_line_break = next_should_line_break;
        }
        result
    }

    /// Writes bytes

    pub fn write(&mut self, value: &[u8]) -> Result<()> {
        self.writer.write_all(value).map_err(Error::Io)
    }


    fn write_wrapped(&mut self, before: &[u8], value: &[u8], after: &[u8]) -> Result<()> {
        if let Some(ref i) = self.indent {
            if i.should_line_break {
                self.writer.write_all(b"\n").map_err(Error::Io)?;
                self.writer
                    .write_all(&i.indents[..i.indents_len])
                    .map_err(Error::Io)?;
            }
        }
        self.write(before)?;
        self.write(value)?;
        self.write(after)?;
        Ok(())
    }

    /// Manually write a newline and indentation at the proper level.
    ///
    /// This can be used when the heuristic to line break and indent after any [Event] apart
    /// from [Text] fails such as when a [Start] occurs directly after [Text].
    /// This method will do nothing if `Writer` was not constructed with `new_with_indent`.
    ///
    /// [Event]: events/enum.Event.html
    /// [Text]: events/enum.Event.html#variant.Text
    /// [Start]: events/enum.Event.html#variant.Start
    pub fn write_indent(&mut self) -> Result<()> {
        if let Some(ref i) = self.indent {
            self.writer.write_all(b"\n").map_err(Error::Io)?;
            self.writer
                .write_all(&i.indents[..i.indents_len])
                .map_err(Error::Io)?;
        }
        Ok(())
    }

    /// Provides a simple, high-level API for writing XML elements.
    ///
    /// Returns an [ElementWriter] that simplifies setting attributes and writing content inside the element.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use quick_xml::Result;
    /// # fn main() -> Result<()> {
    /// use quick_xml::{Error, Writer};
    /// use quick_xml::events::{BytesStart, BytesText, Event};
    /// use std::io::Cursor;
    ///
    /// let mut writer = Writer::new(Cursor::new(Vec::new()));
    ///
    /// // writes <tag attr1="value1"/>
    /// writer.create_element("tag")
    ///     .with_attribute(("attr1", "value1"))  // chain `with_attribute()` calls to add many attributes
    ///     .write_empty()?;
    ///
    /// // writes <tag attr1="value1" attr2="value2">with some text inside</tag>
    /// writer.create_element("tag")
    ///     .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter())  // or add attributes from an iterator
    ///     .write_text_content(BytesText::from_plain_str("with some text inside"))?;
    ///
    /// // writes <tag><fruit quantity="0">apple</fruit><fruit quantity="1">orange</fruit></tag>
    /// writer.create_element("tag")
    ///     .write_inner_content(|writer| {
    ///         let fruits = ["apple", "orange"];
    ///         for (quant, item) in fruits.iter().enumerate() {
    ///             writer
    ///                 .create_element("fruit")
    ///                 .with_attribute(("quantity", quant.to_string().as_str()))
    ///                 .write_text_content(BytesText::from_plain_str(item))?;
    ///         }
    ///         Ok(())
    ///     })?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn create_element<'a, N>(&'a mut self, name: &'a N) -> ElementWriter<W>
    where
        N: 'a + AsRef<[u8]> + ?Sized,
    {
        ElementWriter {
            writer: self,
            start_tag: BytesStart::borrowed_name(name.as_ref()),
        }
    }
}

/// A struct to write an element. Contains methods to add attributes and inner
/// elements to the element
pub struct ElementWriter<'a, W: Write> {
    writer: &'a mut Writer<W>,
    start_tag: BytesStart<'a>,
}

impl<'a, W: Write> ElementWriter<'a, W> {
    /// Adds an attribute to this element.
    pub fn with_attribute<'b, I>(mut self, attr: I) -> Self
    where
        I: Into<Attribute<'b>>,
    {
        self.start_tag.push_attribute(attr);
        self
    }

    /// Add additional attributes to this element using an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn with_attributes<'b, I>(mut self, attributes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        self.start_tag.extend_attributes(attributes);
        self
    }

    /// Write some text inside the current element.
    pub fn write_text_content(self, text: BytesText) -> Result<&'a mut Writer<W>> {
        self.writer
            .write_event(Event::Start(self.start_tag.to_borrowed()))?;
        self.writer.write_event(Event::Text(text))?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }

    /// Write a CData event `<![CDATA[...]]>` inside the current element.
    pub fn write_cdata_content(self, text: BytesCData) -> Result<&'a mut Writer<W>> {
        self.writer
            .write_event(Event::Start(self.start_tag.to_borrowed()))?;
        self.writer.write_event(Event::CData(text))?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }

    /// Write a processing instruction `<?...?>` inside the current element.
    pub fn write_pi_content(self, text: BytesText) -> Result<&'a mut Writer<W>> {
        self.writer
            .write_event(Event::Start(self.start_tag.to_borrowed()))?;
        self.writer.write_event(Event::PI(text))?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }

    /// Write an empty (self-closing) tag.
    pub fn write_empty(self) -> Result<&'a mut Writer<W>> {
        self.writer.write_event(Event::Empty(self.start_tag))?;
        Ok(self.writer)
    }

    /// Create a new scope for writing XML inside the current element.
    pub fn write_inner_content<F>(mut self, closure: F) -> Result<&'a mut Writer<W>>
    where
        F: Fn(&mut Writer<W>) -> Result<()>,
    {
        self.writer
            .write_event(Event::Start(self.start_tag.to_borrowed()))?;
        closure(&mut self.writer)?;
        self.writer
            .write_event(Event::End(self.start_tag.to_end()))?;
        Ok(self.writer)
    }
}

#[derive(Clone)]
struct Indentation {
    should_line_break: bool,
    indent_char: u8,
    indent_size: usize,
    indents: Vec<u8>,
    indents_len: usize,
}

impl Indentation {
    fn new(indent_char: u8, indent_size: usize) -> Indentation {
        Indentation {
            should_line_break: false,
            indent_char,
            indent_size,
            indents: vec![indent_char; 128],
            indents_len: 0,
        }
    }

    fn grow(&mut self) {
        self.indents_len += self.indent_size;
        if self.indents_len > self.indents.len() {
            self.indents.resize(self.indents_len, self.indent_char);
        }
    }

    fn shrink(&mut self) {
        self.indents_len = match self.indents_len.checked_sub(self.indent_size) {
            Some(result) => result,
            None => 0,
        };
    }
}

#[cfg(test)]
mod indentation {
    use super::*;
    use crate::events::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn self_closed() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let tag = BytesStart::borrowed_name(b"self-closed")
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        writer
            .write_event(Event::Empty(tag))
            .expect("write tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<self-closed attr1="value1" attr2="value2"/>"#
        );
    }

    #[test]
    fn empty_paired() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let name = b"paired";
        let start = BytesStart::borrowed_name(name)
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = BytesEnd::borrowed(name);
        writer
            .write_event(Event::Start(start))
            .expect("write start tag failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
</paired>"#
        );
    }

    #[test]
    fn paired_with_inner() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let name = b"paired";
        let start = BytesStart::borrowed_name(name)
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = BytesEnd::borrowed(name);
        let inner = BytesStart::borrowed_name(b"inner");

        writer
            .write_event(Event::Start(start))
            .expect("write start tag failed");
        writer
            .write_event(Event::Empty(inner))
            .expect("write inner tag failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
    <inner/>
</paired>"#
        );
    }

    #[test]
    fn paired_with_text() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let name = b"paired";
        let start = BytesStart::borrowed_name(name)
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = BytesEnd::borrowed(name);
        let text = BytesText::from_plain(b"text");

        writer
            .write_event(Event::Start(start))
            .expect("write start tag failed");
        writer
            .write_event(Event::Text(text))
            .expect("write text failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text</paired>"#
        );
    }

    #[test]
    fn mixed_content() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let name = b"paired";
        let start = BytesStart::borrowed_name(name)
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = BytesEnd::borrowed(name);
        let text = BytesText::from_plain(b"text");
        let inner = BytesStart::borrowed_name(b"inner");

        writer
            .write_event(Event::Start(start))
            .expect("write start tag failed");
        writer
            .write_event(Event::Text(text))
            .expect("write text failed");
        writer
            .write_event(Event::Empty(inner))
            .expect("write inner tag failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text<inner/>
</paired>"#
        );
    }

    #[test]
    fn nested() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        let name = b"paired";
        let start = BytesStart::borrowed_name(name)
            .with_attributes(vec![("attr1", "value1"), ("attr2", "value2")].into_iter());
        let end = BytesEnd::borrowed(name);
        let inner = BytesStart::borrowed_name(b"inner");

        writer
            .write_event(Event::Start(start.clone()))
            .expect("write start 1 tag failed");
        writer
            .write_event(Event::Start(start))
            .expect("write start 2 tag failed");
        writer
            .write_event(Event::Empty(inner))
            .expect("write inner tag failed");
        writer
            .write_event(Event::End(end.clone()))
            .expect("write end tag 2 failed");
        writer
            .write_event(Event::End(end))
            .expect("write end tag 1 failed");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">
    <paired attr1="value1" attr2="value2">
        <inner/>
    </paired>
</paired>"#
        );
    }
    #[test]
    fn element_writer_empty() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer
            .create_element(b"empty")
            .with_attribute(("attr1", "value1"))
            .with_attribute(("attr2", "value2"))
            .write_empty()
            .expect("failure");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<empty attr1="value1" attr2="value2"/>"#
        );
    }

    #[test]
    fn element_writer_text() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer
            .create_element("paired")
            .with_attribute(("attr1", "value1"))
            .with_attribute(("attr2", "value2"))
            .write_text_content(BytesText::from_plain_str("text"))
            .expect("failure");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<paired attr1="value1" attr2="value2">text</paired>"#
        );
    }

    #[test]
    fn element_writer_nested() {
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        writer
            .create_element("outer")
            .with_attribute(("attr1", "value1"))
            .with_attribute(("attr2", "value2"))
            .write_inner_content(|writer| {
                let fruits = ["apple", "orange", "banana"];
                for (quant, item) in fruits.iter().enumerate() {
                    writer
                        .create_element("fruit")
                        .with_attribute(("quantity", quant.to_string().as_str()))
                        .write_text_content(BytesText::from_plain_str(item))?;
                }
                writer
                    .create_element("inner")
                    .write_inner_content(|writer| {
                        writer.create_element("empty").write_empty()?;
                        Ok(())
                    })?;

                Ok(())
            })
            .expect("failure");

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            r#"<outer attr1="value1" attr2="value2">
    <fruit quantity="0">apple</fruit>
    <fruit quantity="1">orange</fruit>
    <fruit quantity="2">banana</fruit>
    <inner>
        <empty/>
    </inner>
</outer>"#
        );
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6796() {
    rusty_monitor::set_test_id(6796);
    let mut str_0: &str = "y9UgHS6G";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut str_1: &str = "rfv9VV6B9ArLXFGlNmX";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut usize_0: usize = 9079usize;
    let mut u8_0: u8 = 74u8;
    let mut indentation_0: crate::writer::Indentation = crate::writer::Indentation::new(u8_0, usize_0);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut usize_1: usize = 4678usize;
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_1, bool_0);
    let mut iterstate_0_ref_0: &crate::events::attributes::IterState = &mut iterstate_0;
    let mut usize_2: usize = 4177usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_2);
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut event_1: events::Event = crate::events::Event::DocType(bytestext_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2087() {
    rusty_monitor::set_test_id(2087);
    let mut u8_0: u8 = 96u8;
    let mut usize_0: usize = 5134usize;
    let mut u8_1: u8 = 20u8;
    let mut str_0: &str = "rfv9VV6B9ArLXFGlNmX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut usize_1: usize = 9079usize;
    let mut u8_2: u8 = 74u8;
    let mut indentation_0: crate::writer::Indentation = crate::writer::Indentation::new(u8_2, usize_1);
    let mut indentation_0_ref_0: &mut crate::writer::Indentation = &mut indentation_0;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut usize_2: usize = 4678usize;
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_2, bool_0);
    let mut iterstate_0_ref_0: &crate::events::attributes::IterState = &mut iterstate_0;
    let mut usize_3: usize = 4177usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_3);
    crate::writer::Indentation::shrink(indentation_0_ref_0);
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut indentation_1: crate::writer::Indentation = crate::writer::Indentation::new(u8_1, usize_0);
    let mut bool_1: bool = crate::reader::is_whitespace(u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8253() {
    rusty_monitor::set_test_id(8253);
    let mut str_0: &str = "rfv9VV6B9ArLXFGlNmX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut usize_0: usize = 9079usize;
    let mut u8_0: u8 = 74u8;
    let mut indentation_0: crate::writer::Indentation = crate::writer::Indentation::new(u8_0, usize_0);
    let mut indentation_0_ref_0: &mut crate::writer::Indentation = &mut indentation_0;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut bool_0: bool = false;
    let mut usize_1: usize = 4678usize;
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_1, bool_0);
    let mut iterstate_0_ref_0: &crate::events::attributes::IterState = &mut iterstate_0;
    crate::writer::Indentation::shrink(indentation_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4496() {
    rusty_monitor::set_test_id(4496);
    let mut str_0: &str = "HoBbyGOD3JaLEuw1e";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 3246usize;
    let mut usize_1: usize = 5362usize;
    let mut isize_0: isize = -9108isize;
    let mut isize_1: isize = -11161isize;
    let mut str_1: &str = "bT1ruego41zNvu";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut isize_2: isize = 1191isize;
    let mut usize_2: usize = 2361usize;
    let mut u8_0: u8 = 59u8;
    let mut indentation_0: crate::writer::Indentation = crate::writer::Indentation::new(u8_0, usize_2);
    let mut indentation_0_ref_0: &mut crate::writer::Indentation = &mut indentation_0;
    crate::writer::Indentation::grow(indentation_0_ref_0);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::Empty(isize_2);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_1, isize_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_1, usize_0);
    panic!("From RustyUnit with love");
}
}
//! Defines zero-copy XML events used throughout this library.
//!
//! A XML event often represents part of a XML element.
//! They occur both during reading and writing and are
//! usually used with the stream-oriented API.
//!
//! For example, the XML element
//! ```xml
//! <name attr="value">Inner text</name>
//! ```
//! consists of the three events `Start`, `Text` and `End`.
//! They can also represent other parts in an XML document like the
//! XML declaration. Each Event usually contains further information,
//! like the tag name, the attribute or the inner text.
//!
//! See [`Event`] for a list of all possible events.
//!
//! # Reading
//! When reading a XML stream, the events are emitted by
//! [`Reader::read_event`]. You must listen
//! for the different types of events you are interested in.
//!
//! See [`Reader`] for further information.
//!
//! # Writing
//! When writing the XML document, you must create the XML element
//! by constructing the events it consists of and pass them to the writer
//! sequentially.
//!
//! See [`Writer`] for further information.
//!
//! [`Reader::read_event`]: ../reader/struct.Reader.html#method.read_event
//! [`Reader`]: ../reader/struct.Reader.html
//! [`Writer`]: ../writer/struct.Writer.html
//! [`Event`]: enum.Event.html

pub mod attributes;

#[cfg(feature = "encoding_rs")]
use encoding_rs::Encoding;
use std::{borrow::Cow, collections::HashMap, io::BufRead, ops::Deref, str::from_utf8};

use crate::escape::{do_unescape, escape, partial_escape};
use crate::utils::write_cow_string;
use crate::{errors::Error, errors::Result, reader::Reader};
use attributes::{Attribute, Attributes};

#[cfg(feature = "serialize")]
use crate::escape::EscapeError;

use memchr;

/// Opening tag data (`Event::Start`), with optional attributes.
///
/// `<name attr="value">`.
///
/// The name can be accessed using the [`name`], [`local_name`] or [`unescaped`] methods. An
/// iterator over the attributes is returned by the [`attributes`] method.
///
/// [`name`]: #method.name
/// [`local_name`]: #method.local_name
/// [`unescaped`]: #method.unescaped
/// [`attributes`]: #method.attributes
#[derive(Clone, Eq, PartialEq)]
pub struct BytesStart<'a> {
    /// content of the element, before any utf8 conversion
    buf: Cow<'a, [u8]>,
    /// end of the element name, the name starts at that the start of `buf`
    name_len: usize,
}

impl<'a> BytesStart<'a> {
    /// Creates a new `BytesStart` from the given content (name + attributes).
    ///
    /// # Warning
    ///
    /// `&content[..name_len]` is not checked to be a valid name

    pub fn borrowed(content: &'a [u8], name_len: usize) -> Self {
        BytesStart {
            buf: Cow::Borrowed(content),
            name_len,
        }
    }

    /// Creates a new `BytesStart` from the given name.
    ///
    /// # Warning
    ///
    /// `&content` is not checked to be a valid name

    pub fn borrowed_name(name: &'a [u8]) -> BytesStart<'a> {
        Self::borrowed(name, name.len())
    }

    /// Creates a new `BytesStart` from the given content (name + attributes)
    ///
    /// Owns its contents.

    pub fn owned<C: Into<Vec<u8>>>(content: C, name_len: usize) -> BytesStart<'static> {
        BytesStart {
            buf: Cow::Owned(content.into()),
            name_len,
        }
    }

    /// Creates a new `BytesStart` from the given name
    ///
    /// Owns its contents.

    pub fn owned_name<C: Into<Vec<u8>>>(name: C) -> BytesStart<'static> {
        let content = name.into();
        BytesStart {
            name_len: content.len(),
            buf: Cow::Owned(content),
        }
    }

    /// Converts the event into an owned event.
    pub fn into_owned(self) -> BytesStart<'static> {
        Self::owned(self.buf.into_owned(), self.name_len)
    }

    /// Converts the event into an owned event without taking ownership of Event
    pub fn to_owned(&self) -> BytesStart<'static> {
        Self::owned(self.buf.to_owned(), self.name_len)
    }

    /// Converts the event into a borrowed event. Most useful when paired with [`to_end`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use quick_xml::{Error, Writer};
    /// use quick_xml::events::{BytesStart, Event};
    ///
    /// struct SomeStruct<'a> {
    ///     attrs: BytesStart<'a>,
    ///     // ...
    /// }
    /// # impl<'a> SomeStruct<'a> {
    /// # fn example(&self) -> Result<(), Error> {
    /// # let mut writer = Writer::new(Vec::new());
    ///
    /// writer.write_event(Event::Start(self.attrs.to_borrowed()))?;
    /// // ...
    /// writer.write_event(Event::End(self.attrs.to_end()))?;
    /// # Ok(())
    /// # }}
    /// ```
    ///
    /// [`to_end`]: #method.to_end
    pub fn to_borrowed(&self) -> BytesStart {
        BytesStart::borrowed(&self.buf, self.name_len)
    }

    /// Creates new paired close tag
    pub fn to_end(&self) -> BytesEnd {
        BytesEnd::borrowed(self.name())
    }

    /// Gets the undecoded raw tag name as a `&[u8]`.

    pub fn name(&self) -> &[u8] {
        &self.buf[..self.name_len]
    }

    /// Gets the undecoded raw local tag name (excluding namespace) as a `&[u8]`.
    ///
    /// All content up to and including the first `:` character is removed from the tag name.

    pub fn local_name(&self) -> &[u8] {
        let name = self.name();
        memchr::memchr(b':', name).map_or(name, |i| &name[i + 1..])
    }

    /// Gets the unescaped tag name.
    ///
    /// XML escape sequences like "`&lt;`" will be replaced by their unescaped characters like
    /// "`<`".
    ///
    /// See also [`unescaped_with_custom_entities()`](#method.unescaped_with_custom_entities)

    pub fn unescaped(&self) -> Result<Cow<[u8]>> {
        self.make_unescaped(None)
    }

    /// Gets the unescaped tag name, using custom entities.
    ///
    /// XML escape sequences like "`&lt;`" will be replaced by their unescaped characters like
    /// "`<`".
    /// Additional entities can be provided in `custom_entities`.
    ///
    /// # Pre-condition
    ///
    /// The keys and values of `custom_entities`, if any, must be valid UTF-8.
    ///
    /// See also [`unescaped()`](#method.unescaped)

    pub fn unescaped_with_custom_entities<'s>(
        &'s self,
        custom_entities: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<Cow<'s, [u8]>> {
        self.make_unescaped(Some(custom_entities))
    }


    fn make_unescaped<'s>(
        &'s self,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<Cow<'s, [u8]>> {
        do_unescape(&*self.buf, custom_entities).map_err(Error::EscapeError)
    }

    /// Returns the unescaped and decoded string value.
    ///
    /// This allocates a `String` in all cases. For performance reasons it might be a better idea to
    /// instead use one of:
    ///
    /// * [`unescaped()`], as it doesn't allocate when no escape sequences are used.
    /// * [`Reader::decode()`], as it only allocates when the decoding can't be performed otherwise.
    ///
    /// [`unescaped()`]: #method.unescaped
    /// [`Reader::decode()`]: ../reader/struct.Reader.html#method.decode

    pub fn unescape_and_decode<B: BufRead>(&self, reader: &Reader<B>) -> Result<String> {
        self.do_unescape_and_decode_with_custom_entities(reader, None)
    }

    /// Returns the unescaped and decoded string value with custom entities.
    ///
    /// This allocates a `String` in all cases. For performance reasons it might be a better idea to
    /// instead use one of:
    ///
    /// * [`unescaped_with_custom_entities()`], as it doesn't allocate when no escape sequences are used.
    /// * [`Reader::decode()`], as it only allocates when the decoding can't be performed otherwise.
    ///
    /// [`unescaped_with_custom_entities()`]: #method.unescaped_with_custom_entities
    /// [`Reader::decode()`]: ../reader/struct.Reader.html#method.decode
    ///
    /// # Pre-condition
    ///
    /// The keys and values of `custom_entities`, if any, must be valid UTF-8.

    pub fn unescape_and_decode_with_custom_entities<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<String> {
        self.do_unescape_and_decode_with_custom_entities(reader, Some(custom_entities))
    }

    #[cfg(feature = "encoding")]

    fn do_unescape_and_decode_with_custom_entities<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<String> {
        let decoded = reader.decode(&*self);
        let unescaped =
            do_unescape(decoded.as_bytes(), custom_entities).map_err(Error::EscapeError)?;
        String::from_utf8(unescaped.into_owned()).map_err(|e| Error::Utf8(e.utf8_error()))
    }

    #[cfg(not(feature = "encoding"))]

    fn do_unescape_and_decode_with_custom_entities<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<String> {
        let decoded = reader.decode(&*self)?;
        let unescaped =
            do_unescape(decoded.as_bytes(), custom_entities).map_err(Error::EscapeError)?;
        String::from_utf8(unescaped.into_owned()).map_err(|e| Error::Utf8(e.utf8_error()))
    }

    /// Edit the name of the BytesStart in-place
    ///
    /// # Warning
    ///
    /// `name` is not checked to be a valid name
    pub fn set_name(&mut self, name: &[u8]) -> &mut BytesStart<'a> {
        let bytes = self.buf.to_mut();
        bytes.splice(..self.name_len, name.iter().cloned());
        self.name_len = name.len();
        self
    }
}

/// Attribute-related methods
impl<'a> BytesStart<'a> {
    /// Consumes `self` and yield a new `BytesStart` with additional attributes from an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn with_attributes<'b, I>(mut self, attributes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        self.extend_attributes(attributes);
        self
    }

    /// Add additional attributes to this tag using an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn extend_attributes<'b, I>(&mut self, attributes: I) -> &mut BytesStart<'a>
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        for attr in attributes {
            self.push_attribute(attr);
        }
        self
    }

    /// Adds an attribute to this element.
    pub fn push_attribute<'b, A>(&mut self, attr: A)
    where
        A: Into<Attribute<'b>>,
    {
        let a = attr.into();
        let bytes = self.buf.to_mut();
        bytes.push(b' ');
        bytes.extend_from_slice(a.key);
        bytes.extend_from_slice(b"=\"");
        bytes.extend_from_slice(&*a.value);
        bytes.push(b'"');
    }

    /// Remove all attributes from the ByteStart
    pub fn clear_attributes(&mut self) -> &mut BytesStart<'a> {
        self.buf.to_mut().truncate(self.name_len);
        self
    }

    /// Returns an iterator over the attributes of this tag.
    pub fn attributes(&self) -> Attributes {
        Attributes::new(&self.buf, self.name_len)
    }

    /// Returns an iterator over the HTML-like attributes of this tag (no mandatory quotes or `=`).
    pub fn html_attributes(&self) -> Attributes {
        Attributes::html(self, self.name_len)
    }

    /// Gets the undecoded raw string with the attributes of this tag as a `&[u8]`,
    /// including the whitespace after the tag name if there is any.

    pub fn attributes_raw(&self) -> &[u8] {
        &self.buf[self.name_len..]
    }

    /// Try to get an attribute
    pub fn try_get_attribute<N: AsRef<[u8]> + Sized>(
        &'a self,
        attr_name: N,
    ) -> Result<Option<Attribute<'a>>> {
        for a in self.attributes() {
            let a = a?;
            if a.key == attr_name.as_ref() {
                return Ok(Some(a));
            }
        }
        Ok(None)
    }
}

impl<'a> std::fmt::Debug for BytesStart<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BytesStart {{ buf: ")?;
        write_cow_string(f, &self.buf)?;
        write!(f, ", name_len: {} }}", self.name_len)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// An XML declaration (`Event::Decl`).
///
/// [W3C XML 1.1 Prolog and Document Type Declaration](http://w3.org/TR/xml11/#sec-prolog-dtd)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BytesDecl<'a> {
    element: BytesStart<'a>,
}

impl<'a> BytesDecl<'a> {
    /// Creates a `BytesDecl` from a `BytesStart`
    pub fn from_start(start: BytesStart<'a>) -> BytesDecl<'a> {
        BytesDecl { element: start }
    }

    /// Gets xml version, excluding quotes (`'` or `"`).
    ///
    /// According to the [grammar], the version *must* be the first thing in the declaration.
    /// This method tries to extract the first thing in the declaration and return it.
    /// In case of multiple attributes value of the first one is returned.
    ///
    /// If version is missed in the declaration, or the first thing is not a version,
    /// [`Error::XmlDeclWithoutVersion`] will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use quick_xml::Error;
    /// use quick_xml::events::{BytesDecl, BytesStart};
    ///
    /// // <?xml version='1.1'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" version='1.1'", 0));
    /// assert_eq!(
    ///     decl.version().unwrap(),
    ///     Cow::Borrowed(b"1.1".as_ref())
    /// );
    ///
    /// // <?xml version='1.0' version='1.1'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" version='1.0' version='1.1'", 0));
    /// assert_eq!(
    ///     decl.version().unwrap(),
    ///     Cow::Borrowed(b"1.0".as_ref())
    /// );
    ///
    /// // <?xml encoding='utf-8'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" encoding='utf-8'", 0));
    /// match decl.version() {
    ///     Err(Error::XmlDeclWithoutVersion(Some(key))) => assert_eq!(key, "encoding".to_string()),
    ///     _ => assert!(false),
    /// }
    ///
    /// // <?xml encoding='utf-8' version='1.1'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" encoding='utf-8' version='1.1'", 0));
    /// match decl.version() {
    ///     Err(Error::XmlDeclWithoutVersion(Some(key))) => assert_eq!(key, "encoding".to_string()),
    ///     _ => assert!(false),
    /// }
    ///
    /// // <?xml?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b"", 0));
    /// match decl.version() {
    ///     Err(Error::XmlDeclWithoutVersion(None)) => {},
    ///     _ => assert!(false),
    /// }
    /// ```
    ///
    /// [grammar]: https://www.w3.org/TR/xml11/#NT-XMLDecl
    pub fn version(&self) -> Result<Cow<[u8]>> {
        // The version *must* be the first thing in the declaration.
        match self.element.attributes().with_checks(false).next() {
            Some(Ok(a)) if a.key == b"version" => Ok(a.value),
            // first attribute was not "version"
            Some(Ok(a)) => {
                let found = from_utf8(a.key).map_err(Error::Utf8)?.to_string();
                Err(Error::XmlDeclWithoutVersion(Some(found)))
            }
            // error parsing attributes
            Some(Err(e)) => Err(e.into()),
            // no attributes
            None => Err(Error::XmlDeclWithoutVersion(None)),
        }
    }

    /// Gets xml encoding, excluding quotes (`'` or `"`).
    ///
    /// Although according to the [grammar] encoding must appear before `"standalone"`
    /// and after `"version"`, this method does not check that. The first occurrence
    /// of the attribute will be returned even if there are several. Also, method does
    /// not restrict symbols that can forming the encoding, so the returned encoding
    /// name may not correspond to the grammar.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use quick_xml::Error;
    /// use quick_xml::events::{BytesDecl, BytesStart};
    ///
    /// // <?xml version='1.1'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" version='1.1'", 0));
    /// assert!(decl.encoding().is_none());
    ///
    /// // <?xml encoding='utf-8'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" encoding='utf-8'", 0));
    /// match decl.encoding() {
    ///     Some(Ok(Cow::Borrowed(encoding))) => assert_eq!(encoding, b"utf-8"),
    ///     _ => assert!(false),
    /// }
    ///
    /// // <?xml encoding='something_WRONG' encoding='utf-8'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" encoding='something_WRONG' encoding='utf-8'", 0));
    /// match decl.encoding() {
    ///     Some(Ok(Cow::Borrowed(encoding))) => assert_eq!(encoding, b"something_WRONG"),
    ///     _ => assert!(false),
    /// }
    /// ```
    ///
    /// [grammar]: https://www.w3.org/TR/xml11/#NT-XMLDecl
    pub fn encoding(&self) -> Option<Result<Cow<[u8]>>> {
        self.element
            .try_get_attribute("encoding")
            .map(|a| a.map(|a| a.value))
            .transpose()
    }

    /// Gets xml standalone, excluding quotes (`'` or `"`).
    ///
    /// Although according to the [grammar] standalone flag must appear after `"version"`
    /// and `"encoding"`, this method does not check that. The first occurrence of the
    /// attribute will be returned even if there are several. Also, method does not
    /// restrict symbols that can forming the value, so the returned flag name may not
    /// correspond to the grammar.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use quick_xml::Error;
    /// use quick_xml::events::{BytesDecl, BytesStart};
    ///
    /// // <?xml version='1.1'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" version='1.1'", 0));
    /// assert!(decl.standalone().is_none());
    ///
    /// // <?xml standalone='yes'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" standalone='yes'", 0));
    /// match decl.standalone() {
    ///     Some(Ok(Cow::Borrowed(encoding))) => assert_eq!(encoding, b"yes"),
    ///     _ => assert!(false),
    /// }
    ///
    /// // <?xml standalone='something_WRONG' encoding='utf-8'?>
    /// let decl = BytesDecl::from_start(BytesStart::borrowed(b" standalone='something_WRONG' encoding='utf-8'", 0));
    /// match decl.standalone() {
    ///     Some(Ok(Cow::Borrowed(flag))) => assert_eq!(flag, b"something_WRONG"),
    ///     _ => assert!(false),
    /// }
    /// ```
    ///
    /// [grammar]: https://www.w3.org/TR/xml11/#NT-XMLDecl
    pub fn standalone(&self) -> Option<Result<Cow<[u8]>>> {
        self.element
            .try_get_attribute("standalone")
            .map(|a| a.map(|a| a.value))
            .transpose()
    }

    /// Constructs a new `XmlDecl` from the (mandatory) _version_ (should be `1.0` or `1.1`),
    /// the optional _encoding_ (e.g., `UTF-8`) and the optional _standalone_ (`yes` or `no`)
    /// attribute.
    ///
    /// Does not escape any of its inputs. Always uses double quotes to wrap the attribute values.
    /// The caller is responsible for escaping attribute values. Shouldn't usually be relevant since
    /// the double quote character is not allowed in any of the attribute values.
    pub fn new(
        version: &[u8],
        encoding: Option<&[u8]>,
        standalone: Option<&[u8]>,
    ) -> BytesDecl<'static> {
        // Compute length of the buffer based on supplied attributes
        // ' encoding=""'   => 12
        let encoding_attr_len = if let Some(xs) = encoding {
            12 + xs.len()
        } else {
            0
        };
        // ' standalone=""' => 14
        let standalone_attr_len = if let Some(xs) = standalone {
            14 + xs.len()
        } else {
            0
        };
        // 'xml version=""' => 14
        let mut buf = Vec::with_capacity(14 + encoding_attr_len + standalone_attr_len);

        buf.extend_from_slice(b"xml version=\"");
        buf.extend_from_slice(version);

        if let Some(encoding_val) = encoding {
            buf.extend_from_slice(b"\" encoding=\"");
            buf.extend_from_slice(encoding_val);
        }

        if let Some(standalone_val) = standalone {
            buf.extend_from_slice(b"\" standalone=\"");
            buf.extend_from_slice(standalone_val);
        }
        buf.push(b'"');

        BytesDecl {
            element: BytesStart::owned(buf, 3),
        }
    }

    /// Gets the decoder struct
    #[cfg(feature = "encoding_rs")]
    pub fn encoder(&self) -> Option<&'static Encoding> {
        self.encoding()
            .and_then(|e| e.ok())
            .and_then(|e| Encoding::for_label(&*e))
    }

    /// Converts the event into an owned event.
    pub fn into_owned(self) -> BytesDecl<'static> {
        BytesDecl {
            element: self.element.into_owned(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// A struct to manage `Event::End` events
#[derive(Clone, Eq, PartialEq)]
pub struct BytesEnd<'a> {
    name: Cow<'a, [u8]>,
}

impl<'a> BytesEnd<'a> {
    /// Creates a new `BytesEnd` borrowing a slice

    pub fn borrowed(name: &'a [u8]) -> BytesEnd<'a> {
        BytesEnd {
            name: Cow::Borrowed(name),
        }
    }

    /// Creates a new `BytesEnd` owning its name

    pub fn owned(name: Vec<u8>) -> BytesEnd<'static> {
        BytesEnd {
            name: Cow::Owned(name),
        }
    }

    /// Converts the event into an owned event.
    pub fn into_owned(self) -> BytesEnd<'static> {
        BytesEnd {
            name: Cow::Owned(self.name.into_owned()),
        }
    }

    /// Gets `BytesEnd` event name

    pub fn name(&self) -> &[u8] {
        &*self.name
    }

    /// local name (excluding namespace) as &[u8] (without eventual attributes)
    /// returns the name() with any leading namespace removed (all content up to
    /// and including the first ':' character)

    pub fn local_name(&self) -> &[u8] {
        if let Some(i) = self.name().iter().position(|b| *b == b':') {
            &self.name()[i + 1..]
        } else {
            self.name()
        }
    }
}

impl<'a> std::fmt::Debug for BytesEnd<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BytesEnd {{ name: ")?;
        write_cow_string(f, &self.name)?;
        write!(f, " }}")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Data from various events (most notably, `Event::Text`) that stored in XML
/// in escaped form. Internally data is stored in escaped form
#[derive(Clone, Eq, PartialEq)]
pub struct BytesText<'a> {
    // Invariant: The content is always escaped.
    content: Cow<'a, [u8]>,
}

impl<'a> BytesText<'a> {
    /// Creates a new `BytesText` from an escaped byte sequence.

    pub fn from_escaped<C: Into<Cow<'a, [u8]>>>(content: C) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Creates a new `BytesText` from a byte sequence. The byte sequence is
    /// expected not to be escaped.

    pub fn from_plain(content: &'a [u8]) -> Self {
        Self {
            content: escape(content),
        }
    }

    /// Creates a new `BytesText` from an escaped string.

    pub fn from_escaped_str<C: Into<Cow<'a, str>>>(content: C) -> Self {
        Self::from_escaped(match content.into() {
            Cow::Owned(o) => Cow::Owned(o.into_bytes()),
            Cow::Borrowed(b) => Cow::Borrowed(b.as_bytes()),
        })
    }

    /// Creates a new `BytesText` from a string. The string is expected not to
    /// be escaped.

    pub fn from_plain_str(content: &'a str) -> Self {
        Self::from_plain(content.as_bytes())
    }

    /// Ensures that all data is owned to extend the object's lifetime if
    /// necessary.

    pub fn into_owned(self) -> BytesText<'static> {
        BytesText {
            content: self.content.into_owned().into(),
        }
    }

    /// Extracts the inner `Cow` from the `BytesText` event container.

    pub fn into_inner(self) -> Cow<'a, [u8]> {
        self.content
    }

    /// Returns unescaped version of the text content, that can be written
    /// as CDATA in XML
    #[cfg(feature = "serialize")]
    pub(crate) fn unescape(self) -> std::result::Result<BytesCData<'a>, EscapeError> {
        //TODO: need to think about better API instead of dozens similar functions
        // Maybe use builder pattern. After that expose function as public API
        //FIXME: need to take into account entities defined in the document
        Ok(BytesCData::new(match do_unescape(&self.content, None)? {
            Cow::Borrowed(_) => self.content,
            Cow::Owned(unescaped) => Cow::Owned(unescaped),
        }))
    }

    /// gets escaped content
    ///
    /// Searches for '&' into content and try to escape the coded character if possible
    /// returns Malformed error with index within element if '&' is not followed by ';'
    ///
    /// See also [`unescaped_with_custom_entities()`](#method.unescaped_with_custom_entities)
    pub fn unescaped(&self) -> Result<Cow<[u8]>> {
        self.make_unescaped(None)
    }

    /// gets escaped content with custom entities
    ///
    /// Searches for '&' into content and try to escape the coded character if possible
    /// returns Malformed error with index within element if '&' is not followed by ';'
    /// Additional entities can be provided in `custom_entities`.
    ///
    /// # Pre-condition
    ///
    /// The keys and values of `custom_entities`, if any, must be valid UTF-8.
    ///
    /// See also [`unescaped()`](#method.unescaped)
    pub fn unescaped_with_custom_entities<'s>(
        &'s self,
        custom_entities: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<Cow<'s, [u8]>> {
        self.make_unescaped(Some(custom_entities))
    }

    fn make_unescaped<'s>(
        &'s self,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<Cow<'s, [u8]>> {
        do_unescape(self, custom_entities).map_err(Error::EscapeError)
    }

    /// helper method to unescape then decode self using the reader encoding
    /// but without BOM (Byte order mark)
    ///
    /// for performance reasons (could avoid allocating a `String`),
    /// it might be wiser to manually use
    /// 1. BytesText::unescaped()
    /// 2. Reader::decode(...)
    #[cfg(feature = "encoding")]
    pub fn unescape_and_decode_without_bom<B: BufRead>(
        &self,
        reader: &mut Reader<B>,
    ) -> Result<String> {
        self.do_unescape_and_decode_without_bom(reader, None)
    }

    /// helper method to unescape then decode self using the reader encoding
    /// but without BOM (Byte order mark)
    ///
    /// for performance reasons (could avoid allocating a `String`),
    /// it might be wiser to manually use
    /// 1. BytesText::unescaped()
    /// 2. Reader::decode(...)
    #[cfg(not(feature = "encoding"))]
    pub fn unescape_and_decode_without_bom<B: BufRead>(
        &self,
        reader: &Reader<B>,
    ) -> Result<String> {
        self.do_unescape_and_decode_without_bom(reader, None)
    }

    /// helper method to unescape then decode self using the reader encoding with custom entities
    /// but without BOM (Byte order mark)
    ///
    /// for performance reasons (could avoid allocating a `String`),
    /// it might be wiser to manually use
    /// 1. BytesText::unescaped()
    /// 2. Reader::decode(...)
    ///
    /// # Pre-condition
    ///
    /// The keys and values of `custom_entities`, if any, must be valid UTF-8.
    #[cfg(feature = "encoding")]
    pub fn unescape_and_decode_without_bom_with_custom_entities<B: BufRead>(
        &self,
        reader: &mut Reader<B>,
        custom_entities: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<String> {
        self.do_unescape_and_decode_without_bom(reader, Some(custom_entities))
    }

    /// helper method to unescape then decode self using the reader encoding with custom entities
    /// but without BOM (Byte order mark)
    ///
    /// for performance reasons (could avoid allocating a `String`),
    /// it might be wiser to manually use
    /// 1. BytesText::unescaped()
    /// 2. Reader::decode(...)
    ///
    /// # Pre-condition
    ///
    /// The keys and values of `custom_entities`, if any, must be valid UTF-8.
    #[cfg(not(feature = "encoding"))]
    pub fn unescape_and_decode_without_bom_with_custom_entities<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<String> {
        self.do_unescape_and_decode_without_bom(reader, Some(custom_entities))
    }

    #[cfg(feature = "encoding")]
    fn do_unescape_and_decode_without_bom<B: BufRead>(
        &self,
        reader: &mut Reader<B>,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<String> {
        let decoded = reader.decode_without_bom(&*self);
        let unescaped =
            do_unescape(decoded.as_bytes(), custom_entities).map_err(Error::EscapeError)?;
        String::from_utf8(unescaped.into_owned()).map_err(|e| Error::Utf8(e.utf8_error()))
    }

    #[cfg(not(feature = "encoding"))]
    fn do_unescape_and_decode_without_bom<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<String> {
        let decoded = reader.decode_without_bom(&*self)?;
        let unescaped =
            do_unescape(decoded.as_bytes(), custom_entities).map_err(Error::EscapeError)?;
        String::from_utf8(unescaped.into_owned()).map_err(|e| Error::Utf8(e.utf8_error()))
    }

    /// helper method to unescape then decode self using the reader encoding
    ///
    /// for performance reasons (could avoid allocating a `String`),
    /// it might be wiser to manually use
    /// 1. BytesText::unescaped()
    /// 2. Reader::decode(...)
    pub fn unescape_and_decode<B: BufRead>(&self, reader: &Reader<B>) -> Result<String> {
        self.do_unescape_and_decode_with_custom_entities(reader, None)
    }

    /// helper method to unescape then decode self using the reader encoding with custom entities
    ///
    /// for performance reasons (could avoid allocating a `String`),
    /// it might be wiser to manually use
    /// 1. BytesText::unescaped()
    /// 2. Reader::decode(...)
    ///
    /// # Pre-condition
    ///
    /// The keys and values of `custom_entities`, if any, must be valid UTF-8.
    pub fn unescape_and_decode_with_custom_entities<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: &HashMap<Vec<u8>, Vec<u8>>,
    ) -> Result<String> {
        self.do_unescape_and_decode_with_custom_entities(reader, Some(custom_entities))
    }

    #[cfg(feature = "encoding")]
    fn do_unescape_and_decode_with_custom_entities<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<String> {
        let decoded = reader.decode(&*self);
        let unescaped =
            do_unescape(decoded.as_bytes(), custom_entities).map_err(Error::EscapeError)?;
        String::from_utf8(unescaped.into_owned()).map_err(|e| Error::Utf8(e.utf8_error()))
    }

    #[cfg(not(feature = "encoding"))]
    fn do_unescape_and_decode_with_custom_entities<B: BufRead>(
        &self,
        reader: &Reader<B>,
        custom_entities: Option<&HashMap<Vec<u8>, Vec<u8>>>,
    ) -> Result<String> {
        let decoded = reader.decode(&*self)?;
        let unescaped =
            do_unescape(decoded.as_bytes(), custom_entities).map_err(Error::EscapeError)?;
        String::from_utf8(unescaped.into_owned()).map_err(|e| Error::Utf8(e.utf8_error()))
    }

    /// Gets escaped content.
    pub fn escaped(&self) -> &[u8] {
        self.content.as_ref()
    }
}

impl<'a> std::fmt::Debug for BytesText<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BytesText {{ content: ")?;
        write_cow_string(f, &self.content)?;
        write!(f, " }}")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// CDATA content contains unescaped data from the reader. If you want to write them as a text,
/// [convert](Self::escape) it to [`BytesText`]
#[derive(Clone, Eq, PartialEq)]
pub struct BytesCData<'a> {
    content: Cow<'a, [u8]>,
}

impl<'a> BytesCData<'a> {
    /// Creates a new `BytesCData` from a byte sequence.

    pub fn new<C: Into<Cow<'a, [u8]>>>(content: C) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Creates a new `BytesCData` from a string

    pub fn from_str(content: &'a str) -> Self {
        Self::new(content.as_bytes())
    }

    /// Ensures that all data is owned to extend the object's lifetime if
    /// necessary.

    pub fn into_owned(self) -> BytesCData<'static> {
        BytesCData {
            content: self.content.into_owned().into(),
        }
    }

    /// Extracts the inner `Cow` from the `BytesCData` event container.

    pub fn into_inner(self) -> Cow<'a, [u8]> {
        self.content
    }

    /// Converts this CDATA content to an escaped version, that can be written
    /// as an usual text in XML.
    ///
    /// This function performs following replacements:
    ///
    /// | Character | Replacement
    /// |-----------|------------
    /// | `<`       | `&lt;`
    /// | `>`       | `&gt;`
    /// | `&`       | `&amp;`
    /// | `'`       | `&apos;`
    /// | `"`       | `&quot;`
    pub fn escape(self) -> BytesText<'a> {
        BytesText::from_escaped(match escape(&self.content) {
            Cow::Borrowed(_) => self.content,
            Cow::Owned(escaped) => Cow::Owned(escaped),
        })
    }

    /// Converts this CDATA content to an escaped version, that can be written
    /// as an usual text in XML.
    ///
    /// In XML text content, it is allowed (though not recommended) to leave
    /// the quote special characters `"` and `'` unescaped.
    ///
    /// This function performs following replacements:
    ///
    /// | Character | Replacement
    /// |-----------|------------
    /// | `<`       | `&lt;`
    /// | `>`       | `&gt;`
    /// | `&`       | `&amp;`
    pub fn partial_escape(self) -> BytesText<'a> {
        BytesText::from_escaped(match partial_escape(&self.content) {
            Cow::Borrowed(_) => self.content,
            Cow::Owned(escaped) => Cow::Owned(escaped),
        })
    }

    /// Gets content of this text buffer in the specified encoding
    #[cfg(feature = "serialize")]
    pub(crate) fn decode(&self, decoder: crate::reader::Decoder) -> Result<Cow<'a, str>> {
        Ok(match &self.content {
            Cow::Borrowed(bytes) => {
                #[cfg(feature = "encoding")]
                {
                    decoder.decode(bytes)
                }
                #[cfg(not(feature = "encoding"))]
                {
                    decoder.decode(bytes)?.into()
                }
            }
            Cow::Owned(bytes) => {
                #[cfg(feature = "encoding")]
                let decoded = decoder.decode(bytes).into_owned();

                #[cfg(not(feature = "encoding"))]
                let decoded = decoder.decode(bytes)?.to_string();

                decoded.into()
            }
        })
    }
}

impl<'a> std::fmt::Debug for BytesCData<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BytesCData {{ content: ")?;
        write_cow_string(f, &self.content)?;
        write!(f, " }}")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Event emitted by [`Reader::read_event`].
///
/// [`Reader::read_event`]: ../reader/struct.Reader.html#method.read_event
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event<'a> {
    /// Start tag (with attributes) `<tag attr="value">`.
    Start(BytesStart<'a>),
    /// End tag `</tag>`.
    End(BytesEnd<'a>),
    /// Empty element tag (with attributes) `<tag attr="value" />`.
    Empty(BytesStart<'a>),
    /// Character data between `Start` and `End` element.
    Text(BytesText<'a>),
    /// Comment `<!-- ... -->`.
    Comment(BytesText<'a>),
    /// CData `<![CDATA[...]]>`.
    CData(BytesCData<'a>),
    /// XML declaration `<?xml ...?>`.
    Decl(BytesDecl<'a>),
    /// Processing instruction `<?...?>`.
    PI(BytesText<'a>),
    /// Doctype `<!DOCTYPE ...>`.
    DocType(BytesText<'a>),
    /// End of XML document.
    Eof,
}

impl<'a> Event<'a> {
    /// Converts the event to an owned version, untied to the lifetime of
    /// buffer used when reading but incurring a new, separate allocation.
    pub fn into_owned(self) -> Event<'static> {
        match self {
            Event::Start(e) => Event::Start(e.into_owned()),
            Event::End(e) => Event::End(e.into_owned()),
            Event::Empty(e) => Event::Empty(e.into_owned()),
            Event::Text(e) => Event::Text(e.into_owned()),
            Event::Comment(e) => Event::Comment(e.into_owned()),
            Event::CData(e) => Event::CData(e.into_owned()),
            Event::Decl(e) => Event::Decl(e.into_owned()),
            Event::PI(e) => Event::PI(e.into_owned()),
            Event::DocType(e) => Event::DocType(e.into_owned()),
            Event::Eof => Event::Eof,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl<'a> Deref for BytesStart<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &*self.buf
    }
}

impl<'a> Deref for BytesDecl<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &*self.element
    }
}

impl<'a> Deref for BytesEnd<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &*self.name
    }
}

impl<'a> Deref for BytesText<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &*self.content
    }
}

impl<'a> Deref for BytesCData<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &*self.content
    }
}

impl<'a> Deref for Event<'a> {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match *self {
            Event::Start(ref e) | Event::Empty(ref e) => &*e,
            Event::End(ref e) => &*e,
            Event::Text(ref e) => &*e,
            Event::Decl(ref e) => &*e,
            Event::PI(ref e) => &*e,
            Event::CData(ref e) => &*e,
            Event::Comment(ref e) => &*e,
            Event::DocType(ref e) => &*e,
            Event::Eof => &[],
        }
    }
}

impl<'a> AsRef<Event<'a>> for Event<'a> {
    fn as_ref(&self) -> &Event<'a> {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn local_name() {
        use std::str::from_utf8;
        let xml = r#"
            <foo:bus attr='bar'>foobusbar</foo:bus>
            <foo: attr='bar'>foobusbar</foo:>
            <:foo attr='bar'>foobusbar</:foo>
            <foo:bus:baz attr='bar'>foobusbar</foo:bus:baz>
            "#;
        let mut rdr = Reader::from_str(xml);
        let mut buf = Vec::new();
        let mut parsed_local_names = Vec::new();
        loop {
            match rdr.read_event(&mut buf).expect("unable to read xml event") {
                Event::Start(ref e) => parsed_local_names.push(
                    from_utf8(e.local_name())
                        .expect("unable to build str from local_name")
                        .to_string(),
                ),
                Event::End(ref e) => parsed_local_names.push(
                    from_utf8(e.local_name())
                        .expect("unable to build str from local_name")
                        .to_string(),
                ),
                Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(parsed_local_names[0], "bus".to_string());
        assert_eq!(parsed_local_names[1], "bus".to_string());
        assert_eq!(parsed_local_names[2], "".to_string());
        assert_eq!(parsed_local_names[3], "".to_string());
        assert_eq!(parsed_local_names[4], "foo".to_string());
        assert_eq!(parsed_local_names[5], "foo".to_string());
        assert_eq!(parsed_local_names[6], "bus:baz".to_string());
        assert_eq!(parsed_local_names[7], "bus:baz".to_string());
    }

    #[test]
    fn bytestart_create() {
        let b = BytesStart::owned_name("test");
        assert_eq!(b.len(), 4);
        assert_eq!(b.name(), b"test");
    }

    #[test]
    fn bytestart_set_name() {
        let mut b = BytesStart::owned_name("test");
        assert_eq!(b.len(), 4);
        assert_eq!(b.name(), b"test");
        assert_eq!(b.attributes_raw(), b"");
        b.push_attribute(("x", "a"));
        assert_eq!(b.len(), 10);
        assert_eq!(b.attributes_raw(), b" x=\"a\"");
        b.set_name(b"g");
        assert_eq!(b.len(), 7);
        assert_eq!(b.name(), b"g");
    }

    #[test]
    fn bytestart_clear_attributes() {
        let mut b = BytesStart::owned_name("test");
        b.push_attribute(("x", "y\"z"));
        b.push_attribute(("x", "y\"z"));
        b.clear_attributes();
        assert!(b.attributes().next().is_none());
        assert_eq!(b.len(), 4);
        assert_eq!(b.name(), b"test");
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_239() {
    rusty_monitor::set_test_id(239);
    let mut usize_0: usize = 3804usize;
    let mut str_0: &str = "2JW7mP6BPUAHEvIn";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_1: usize = 6528usize;
    let mut usize_2: usize = 7520usize;
    let mut str_1: &str = "P";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_2};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl {element: bytesstart_1};
    let mut i32_0: i32 = 5206i32;
    let mut usize_3: usize = 8094usize;
    let mut usize_4: usize = 5987usize;
    let mut usize_5: usize = 3231usize;
    let mut bool_0: bool = true;
    let mut i32_1: i32 = 12340i32;
    let mut bool_1: bool = true;
    let mut i32_2: i32 = -13772i32;
    let mut i32_3: i32 = 17016i32;
    let mut usize_6: usize = 611usize;
    let mut usize_7: usize = 8728usize;
    let mut usize_8: usize = 8364usize;
    let mut isize_0: isize = 4437isize;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::Empty(isize_0);
    let mut bytesdecl_1: crate::events::BytesDecl = crate::events::BytesDecl::into_owned(bytesdecl_0);
    crate::events::BytesStart::unescaped(bytesstart_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3129() {
    rusty_monitor::set_test_id(3129);
    let mut usize_0: usize = 974usize;
    let mut usize_1: usize = 6497usize;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut usize_2: usize = 5340usize;
    let mut str_0: &str = "MEdNotnmq4CkepjVkNS";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_2};
    let mut bytesstart_0_ref_0: &mut crate::events::BytesStart = &mut bytesstart_0;
    let mut bool_0: bool = false;
    let mut usize_3: usize = 4259usize;
    let mut str_1: &str = "1FlUUCE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_3};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut attributes_0: crate::events::attributes::Attributes = crate::events::BytesStart::attributes(bytesstart_1_ref_0);
    let mut attributes_0_ref_0: &mut crate::events::attributes::Attributes = &mut attributes_0;
    let mut str_2: &str = "vbtOdmD";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut attributes_1: &mut crate::events::attributes::Attributes = crate::events::attributes::Attributes::with_checks(attributes_0_ref_0, bool_0);
    let mut error_0: errors::Error = crate::errors::Error::TextNotFound;
    let mut bytesstart_2: &mut crate::events::BytesStart = crate::events::BytesStart::clear_attributes(bytesstart_0_ref_0);
    let mut error_1: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_1, usize_0);
    let mut error_2: errors::Error = crate::errors::Error::TextNotFound;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_337() {
    rusty_monitor::set_test_id(337);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_0: &str = "xd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "fhcKl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData {content: cow_0};
    let mut usize_0: usize = 6usize;
    let mut str_2: &str = "wtqVwYKrn7OL7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesStart::to_end(bytesstart_0_ref_0);
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut usize_1: usize = 9510usize;
    let mut usize_2: usize = 1732usize;
    let mut str_3: &str = "sj";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_2};
    let mut usize_3: usize = 6048usize;
    let mut usize_4: usize = 9982usize;
    let mut usize_5: usize = 4747usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_4);
    let mut bytesstart_2: crate::events::BytesStart = crate::events::BytesStart::into_owned(bytesstart_1);
    let mut bytesstart_2_ref_0: &crate::events::BytesStart = &mut bytesstart_2;
    let mut attributes_0: crate::events::attributes::Attributes = crate::events::BytesStart::attributes(bytesstart_2_ref_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    crate::events::BytesText::make_unescaped(bytestext_0_ref_0, option_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2800() {
    rusty_monitor::set_test_id(2800);
    let mut u8_0: u8 = 40u8;
    let mut bool_0: bool = false;
    let mut str_0: &str = "W2H";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText {content: cow_0};
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "NcQR8bcVzAPAmQJf";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_0: isize = 412isize;
    let mut isize_1: isize = 18364isize;
    let mut usize_0: usize = 7082usize;
    let mut usize_1: usize = 80usize;
    let mut str_2: &str = "SmnwOhyELdn";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_1};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut attributes_0: crate::events::attributes::Attributes = crate::events::BytesStart::attributes(bytesstart_0_ref_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_0);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_1, isize_0);
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    crate::events::BytesText::unescaped(bytestext_0_ref_0);
    let mut attributes_0_ref_0: &mut crate::events::attributes::Attributes = &mut attributes_0;
    let mut attributes_1: &mut crate::events::attributes::Attributes = crate::events::attributes::Attributes::with_checks(attributes_0_ref_0, bool_0);
    let mut vec_0_ref_0: &mut std::vec::Vec<u8> = &mut vec_0;
    std::vec::Vec::push(vec_0_ref_0, u8_0);
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::into_owned(bytescdata_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4673() {
    rusty_monitor::set_test_id(4673);
    let mut str_0: &str = "36lK";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut str_1: &str = "Nj3sRKvQ8MOQkt22j";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_0: char = ':';
    let mut str_2: &str = "zybkinfK1jQt044kWuG";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_3: &str = "Pa5h";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    let mut usize_0: usize = 8321usize;
    let mut str_4: &str = "5VUMbc";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_4_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut usize_1: usize = 4174usize;
    let mut usize_2: usize = 7357usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_2, usize_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    crate::events::BytesText::make_unescaped(bytestext_1_ref_0, option_0);
    let mut event_0: events::Event = crate::events::Event::DocType(bytestext_0);
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart::to_borrowed(bytesstart_0_ref_0);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_0);
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4740() {
    rusty_monitor::set_test_id(4740);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut usize_0: usize = 4031usize;
    let mut str_1: &str = "dqOVSXcp5QTC55s";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_1: usize = 4985usize;
    let mut usize_2: usize = 8224usize;
    let mut str_2: &str = "PEnSYkfF6nQh8";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_2};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesStart::to_end(bytesstart_1_ref_0);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesend_1: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_0);
    let mut bytesend_2: crate::events::BytesEnd = crate::events::BytesEnd::into_owned(bytesend_1);
    let mut event_0: events::Event = crate::events::Event::End(bytesend_0);
    let mut bytesend_2_ref_0: &crate::events::BytesEnd = &mut bytesend_2;
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_2_ref_0);
    let mut bytesend_3: crate::events::BytesEnd = crate::events::BytesStart::to_end(bytesstart_0_ref_0);
    let mut bytesend_3_ref_0: &crate::events::BytesEnd = &mut bytesend_3;
    let mut u8_slice_1: &[u8] = crate::events::BytesEnd::name(bytesend_3_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2031() {
    rusty_monitor::set_test_id(2031);
    let mut str_0: &str = "uEv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut cow_0_ref_0: &std::borrow::Cow<[u8]> = &mut cow_0;
    let mut usize_0: usize = 3999usize;
    let mut str_1: &str = "TRvbQdnP0S3L2pMb5";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_0};
    let mut bool_0: bool = true;
    let mut usize_1: usize = 2272usize;
    let mut str_2: &str = "1XR9Wzs";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_1};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut attributes_0: crate::events::attributes::Attributes = crate::events::BytesStart::html_attributes(bytesstart_1_ref_0);
    let mut attributes_0_ref_0: &mut crate::events::attributes::Attributes = &mut attributes_0;
    let mut usize_2: usize = 5088usize;
    let mut usize_3: usize = 9061usize;
    let mut str_3: &str = "CiSQ6FsbCuPuVlpc";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_2: crate::events::BytesStart = crate::events::BytesStart {buf: cow_3, name_len: usize_3};
    let mut bytesstart_2_ref_0: &crate::events::BytesStart = &mut bytesstart_2;
    let mut usize_4: usize = 4335usize;
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::attributes_raw(bytesstart_2_ref_0);
    let mut attributes_1: &mut crate::events::attributes::Attributes = crate::events::attributes::Attributes::with_checks(attributes_0_ref_0, bool_0);
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5033() {
    rusty_monitor::set_test_id(5033);
    let mut str_0: &str = "QD61s2FiuA4w5jd";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "JDXdBTnkm4VEi";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut usize_0: usize = 40usize;
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut u8_0: u8 = 117u8;
    let mut usize_1: usize = 4507usize;
    let mut str_2: &str = "t";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut usize_2: usize = 5246usize;
    let mut str_3: &str = "bs67vEgF7KIq2";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_2};
    let mut bytesstart_0_ref_0: &mut crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_3: usize = 135usize;
    let mut bytesstart_1: &mut crate::events::BytesStart = crate::events::BytesStart::clear_attributes(bytesstart_0_ref_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_1, u8_0);
    let mut u8_slice_0: &[u8] = std::option::Option::unwrap(option_1);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_0);
    let mut event_0: events::Event = crate::events::Event::Text(bytestext_1);
    let mut u8_slice_1: &[u8] = std::option::Option::unwrap(option_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1612() {
    rusty_monitor::set_test_id(1612);
    let mut usize_0: usize = 1992usize;
    let mut str_0: &str = "YsmYlnDSm8eM9wpF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut char_0: char = '~';
    let mut usize_1: usize = 7953usize;
    let mut str_2: &str = "OFvZqRw91zj8BD3";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut str_3: &str = "lG1B3Uu";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut bytesend_1: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_2};
    let mut event_0: events::Event = crate::events::Event::End(bytesend_1);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_1);
    let mut event_3: events::Event = crate::events::Event::into_owned(event_2);
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart::to_borrowed(bytesstart_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_527() {
    rusty_monitor::set_test_id(527);
    let mut str_0: &str = "rS50A";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bool_0: bool = true;
    let mut usize_0: usize = 3159usize;
    let mut u8_0: u8 = 18u8;
    let mut str_1: &str = "qx3EaonGMnHUEl2";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut usize_1: usize = 9851usize;
    let mut str_2: &str = "6ZHGVMHywYlH";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut str_3: &str = "TOWoAANMideTd";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_2: usize = 6391usize;
    let mut str_4: &str = "6NNdSh";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_2};
    let mut event_0: events::Event = crate::events::Event::Start(bytesstart_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_1);
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_3_ref_0);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_1};
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::into_owned(bytestext_0);
    let mut bool_1: bool = crate::reader::is_whitespace(u8_0);
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_0, bool_0);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    let mut bytesstart_1_ref_0: &mut crate::events::BytesStart = &mut bytesstart_1;
    let mut event_3: events::Event = crate::events::Event::CData(bytescdata_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2456() {
    rusty_monitor::set_test_id(2456);
    let mut str_0: &str = "N3wveViY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData {content: cow_0};
    let mut u8_0: u8 = 23u8;
    let mut usize_0: usize = 5761usize;
    let mut isize_0: isize = 5935isize;
    let mut isize_1: isize = -11929isize;
    let mut usize_1: usize = 6945usize;
    let mut str_1: &str = "enOkpYq4";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_1};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut isize_2: isize = 7387isize;
    let mut isize_3: isize = -5902isize;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut usize_2: usize = 3640usize;
    let mut str_2: &str = "VyIw8FkwFqL";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_2};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    crate::events::BytesStart::make_unescaped(bytesstart_1_ref_0, option_0);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_3, isize_2);
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::local_name(bytesstart_0_ref_0);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_1, isize_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_0, u8_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1762() {
    rusty_monitor::set_test_id(1762);
    let mut u8_0: u8 = 36u8;
    let mut isize_0: isize = 762isize;
    let mut usize_0: usize = 191usize;
    let mut str_0: &str = "cP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl {element: bytesstart_0};
    let mut usize_1: usize = 5647usize;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_1: &str = "dw6P8SkaZCYL";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut str_2: &str = "3eAvGcaX";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut usize_2: usize = 1685usize;
    let mut usize_3: usize = 3187usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_3, usize_2);
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData {content: cow_1};
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_2);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    crate::events::BytesText::make_unescaped(bytestext_2_ref_0, option_0);
    let mut bytesdecl_1: crate::events::BytesDecl = crate::events::BytesDecl::into_owned(bytesdecl_0);
    let mut bytesdecl_1_ref_0: &crate::events::BytesDecl = &mut bytesdecl_1;
    crate::events::BytesDecl::version(bytesdecl_1_ref_0);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::Empty(isize_0);
    let mut bool_0: bool = crate::reader::is_whitespace(u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3854() {
    rusty_monitor::set_test_id(3854);
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut str_0: &str = "PzBXO";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut str_1: &str = "62wVAvhixswTyE0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut str_2: &str = "T1DtHwHF4WLs8";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "9bCUHlgDlFln";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut usize_0: usize = 5294usize;
    let mut usize_1: usize = 4448usize;
    let mut str_4: &str = "X22d19Uw47lUtZiRqFt";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_1};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_0);
    let mut usize_2: usize = 5125usize;
    let mut event_1: events::Event = crate::events::Event::Decl(bytesdecl_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_1);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
    let mut error_0: errors::Error = crate::errors::Error::TextNotFound;
    let mut event_3: events::Event = crate::events::Event::into_owned(event_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_2_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2927() {
    rusty_monitor::set_test_id(2927);
    let mut usize_0: usize = 4186usize;
    let mut usize_1: usize = 5467usize;
    let mut str_0: &str = "27FfwSS";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_1};
    let mut usize_2: usize = 6842usize;
    let mut usize_3: usize = 3587usize;
    let mut str_1: &str = "Q06FBUbRoDSK0Xn";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_3};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut usize_4: usize = 6420usize;
    let mut usize_5: usize = 7575usize;
    let mut str_2: &str = "H8II1";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText {content: cow_2};
    let mut str_3: &str = "hT5GMuKefB4i254";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut event_0: events::Event = crate::events::Event::PI(bytestext_1);
    let mut bytesstart_2: crate::events::BytesStart = crate::events::BytesStart::to_borrowed(bytesstart_1_ref_0);
    let mut bytesstart_3: crate::events::BytesStart = crate::events::BytesStart::into_owned(bytesstart_0);
    let mut bytesstart_2_ref_0: &crate::events::BytesStart = &mut bytesstart_2;
    let mut bytesstart_4: crate::events::BytesStart = crate::events::BytesStart::to_owned(bytesstart_2_ref_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2184() {
    rusty_monitor::set_test_id(2184);
    let mut usize_0: usize = 3460usize;
    let mut usize_1: usize = 4137usize;
    let mut usize_2: usize = 7212usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_2);
    let mut str_0: &str = "PQHL34SvoSyxW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "9vmiRWdP";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText {content: cow_0};
    let mut usize_3: usize = 7600usize;
    let mut usize_4: usize = 1400usize;
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_4);
    let mut str_2: &str = "TwtH";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData {content: cow_1};
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd::owned(vec_0);
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::into_owned(bytescdata_2);
    let mut error_0: errors::Error = crate::errors::Error::InvalidAttr(attrerror_1);
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut error_1: errors::Error = crate::errors::Error::InvalidAttr(attrerror_0);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_1, usize_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4746() {
    rusty_monitor::set_test_id(4746);
    let mut usize_0: usize = 7245usize;
    let mut str_0: &str = "HqJA5";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut str_1: &str = "CO";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut str_2: &str = "tPdZ5YTOF";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText {content: cow_2};
    let mut str_3: &str = "";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut usize_1: usize = 6857usize;
    let mut str_4: &str = "TvUxroDeua";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_4);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_3, name_len: usize_1};
    let mut bytesstart_1_ref_0: &mut crate::events::BytesStart = &mut bytesstart_1;
    let mut usize_2: usize = 5153usize;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_3);
    let mut event_0: events::Event = crate::events::Event::PI(bytestext_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut event_1: events::Event = crate::events::Event::Empty(bytesstart_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3139() {
    rusty_monitor::set_test_id(3139);
    let mut usize_0: usize = 5372usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_0);
    let mut str_0: &str = "OBG0tE47";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut str_1: &str = "Tiv0";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut usize_1: usize = 4461usize;
    let mut str_3: &str = "b";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_3, name_len: usize_1};
    let mut usize_2: usize = 2162usize;
    let mut usize_3: usize = 1073usize;
    let mut str_4: &str = "S8FLrBEUrJsOTaHbJ";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_4_ref_0);
    let mut cow_4: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesText {content: cow_4};
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_3, usize_2);
    let mut event_0: events::Event = crate::events::Event::Empty(bytesstart_0);
    let mut bytestext_3_ref_0: &crate::events::BytesText = &mut bytestext_3;
    let mut u8_slice_0: &[u8] = crate::events::BytesText::escaped(bytestext_3_ref_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData {content: cow_0};
    let mut error_0: errors::Error = crate::errors::Error::InvalidAttr(attrerror_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2713() {
    rusty_monitor::set_test_id(2713);
    let mut usize_0: usize = 9562usize;
    let mut str_0: &str = "SN5cLZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut str_1: &str = "dc";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut char_0: char = '}';
    let mut str_2: &str = "cvDAYnW3z";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut str_3: &str = "UZKxK8a";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut usize_1: usize = 200usize;
    let mut str_4: &str = "3790D";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_1};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut usize_2: usize = 1858usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_2);
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::attributes_raw(bytesstart_1_ref_0);
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_3);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_0);
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4191() {
    rusty_monitor::set_test_id(4191);
    let mut str_0: &str = "0wZHAIH6vyX80gY9d";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut usize_0: usize = 2578usize;
    let mut str_1: &str = "iB";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_0};
    let mut usize_1: usize = 9310usize;
    let mut char_0: char = 'P';
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut option_1: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut usize_2: usize = 6299usize;
    let mut str_2: &str = "rusK5sXeLQ3";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_2};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_1);
    let mut usize_3: usize = 7012usize;
    let mut event_0: events::Event = crate::events::Event::Decl(bytesdecl_0);
    let mut hashmap_0: &std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>> = std::option::Option::unwrap(option_1);
    let mut error_0: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_0);
    let mut event_2: events::Event = crate::events::Event::Empty(bytesstart_0);
    let mut event_3: events::Event = crate::events::Event::into_owned(event_2);
    let mut event_4: events::Event = crate::events::Event::into_owned(event_1);
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData {content: cow_0};
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::into_owned(bytescdata_2);
    panic!("From RustyUnit with love");
}
}
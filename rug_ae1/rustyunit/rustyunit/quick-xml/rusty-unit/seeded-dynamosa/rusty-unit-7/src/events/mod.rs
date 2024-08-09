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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8751() {
//    rusty_monitor::set_test_id(8751);
    let mut usize_0: usize = 4usize;
    let mut str_0: &str = "Start";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut event_0: events::Event = crate::events::Event::Empty(bytesstart_0);
    let mut str_1: &str = "EntityWithNull";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut usize_1: usize = 1usize;
    let mut str_2: &str = "ljRR";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_1};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_1);
    let mut bytesdecl_0_ref_0: &crate::events::BytesDecl = &mut bytesdecl_0;
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut usize_2: usize = 7175usize;
    let mut str_3: &str = "Start";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut bytesstart_2: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_2};
    let mut bytesstart_2_ref_0: &mut crate::events::BytesStart = &mut bytesstart_2;
    let mut option_2: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_3: std::option::Option<&[u8]> = std::option::Option::None;
    let mut str_4: &str = "Next";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_4);
    let mut option_4: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut event_1: events::Event = crate::events::Event::DocType(bytestext_1);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_124() {
//    rusty_monitor::set_test_id(124);
    let mut usize_0: usize = 9235usize;
    let mut str_0: &str = "Attr::Unquoted";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_1: usize = 2usize;
    let mut bool_0: bool = false;
    let mut usize_2: usize = 7738usize;
    let mut str_1: &str = "GqRXv3j";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_2};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut attributes_0: crate::events::attributes::Attributes = crate::events::BytesStart::html_attributes(bytesstart_1_ref_0);
    let mut attributes_0_ref_0: &mut crate::events::attributes::Attributes = &mut attributes_0;
    let mut usize_3: usize = 6529usize;
    let mut usize_4: usize = 9173usize;
    let mut str_2: &str = "bindings";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut event_0: events::Event = crate::events::Event::DocType(bytestext_1);
    let mut attributes_1: &mut crate::events::attributes::Attributes = crate::events::attributes::Attributes::with_checks(attributes_0_ref_0, bool_0);
    let mut bytesstart_2: crate::events::BytesStart = crate::events::BytesStart::to_owned(bytesstart_0_ref_0);
    let mut bytesstart_2_ref_0: &crate::events::BytesStart = &mut bytesstart_2;
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::name(bytesstart_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4638() {
//    rusty_monitor::set_test_id(4638);
    let mut u8_0: u8 = 61u8;
    let mut usize_0: usize = 14usize;
    let mut str_0: &str = "initialized";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut str_1: &str = "K77lEBenWgqSuFU";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData {content: cow_1};
    let mut event_0: events::Event = crate::events::Event::Start(bytesstart_0);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut bool_0: bool = crate::reader::is_whitespace(u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_711() {
//    rusty_monitor::set_test_id(711);
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut event_2: events::Event = crate::events::Event::Eof;
    let mut event_3: events::Event = crate::events::Event::into_owned(event_2);
    let mut usize_0: usize = 6286usize;
    let mut str_0: &str = "check_duplicates";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_0);
    let mut event_4: events::Event = crate::events::Event::Decl(bytesdecl_0);
    let mut usize_1: usize = 1270usize;
    let mut str_1: &str = "value_len";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_1};
    let mut bytesdecl_1: crate::events::BytesDecl = crate::events::BytesDecl {element: bytesstart_1};
    let mut event_5: events::Event = crate::events::Event::Decl(bytesdecl_1);
    let mut str_2: &str = "WdsCdq4LHkS8";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_2);
    let mut event_6: events::Event = crate::events::Event::Comment(bytestext_1);
    let mut event_7: events::Event = crate::events::Event::into_owned(event_6);
    let mut event_8: events::Event = crate::events::Event::into_owned(event_5);
    let mut event_9: events::Event = crate::events::Event::into_owned(event_4);
    let mut event_10: events::Event = crate::events::Event::into_owned(event_3);
    let mut event_11: events::Event = crate::events::Event::into_owned(event_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2718() {
//    rusty_monitor::set_test_id(2718);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_0: &str = "&";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText {content: cow_0};
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "Empty";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText {content: cow_1};
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    let mut str_2: &str = "CYjMttHbzrQYwzpUAk";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 12usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_1, usize_0);
    let mut str_3: &str = "unzeiHJsPxw4BsB";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_2};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut str_4: &str = "7a8mjEvZsVmirIXn";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_4);
    let mut str_5: &str = "End";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_5: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut option_1: std::option::Option<std::string::String> = std::option::Option::None;
    let mut usize_2: usize = 12usize;
    let mut str_6: &str = "UnquotedValue";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytescdata_6: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_6);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_4);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_3, name_len: usize_2};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut option_2: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    crate::events::BytesText::unescaped(bytestext_1_ref_0);
    crate::events::BytesText::make_unescaped(bytestext_0_ref_0, option_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_330() {
//    rusty_monitor::set_test_id(330);
    let mut str_0: &str = "UnexpectedToken";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut str_1: &str = "q01mLUx4";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData {content: cow_0};
    let mut str_2: &str = "SEvgrl72fag";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut str_3: &str = "eC47u53rg3mP";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData {content: cow_1};
    let mut str_4: &str = "aBCMFJPQ";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_4_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData {content: cow_2};
    let mut str_5: &str = "'";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_5_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut bytescdata_5: crate::events::BytesCData = crate::events::BytesCData {content: cow_3};
    let mut cow_4: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_5);
    let mut cow_5: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_4);
    let mut cow_6: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut cow_7: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut cow_8: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut cow_9: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4587() {
//    rusty_monitor::set_test_id(4587);
    let mut str_0: &str = "SkipEqValue";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText {content: cow_0};
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut usize_0: usize = 7316usize;
    let mut str_1: &str = "XmlDecl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_0};
    let mut bytesstart_0_ref_0: &mut crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_1: usize = 5usize;
    let mut str_2: &str = "InvalidCodepoint";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_1};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_1);
    let mut bytesdecl_0_ref_0: &crate::events::BytesDecl = &mut bytesdecl_0;
    let mut u8_slice_0: &[u8] = crate::events::BytesText::escaped(bytestext_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_881() {
//    rusty_monitor::set_test_id(881);
    let mut usize_0: usize = 1usize;
    let mut str_0: &str = "UnexpectedBang";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesStart::to_end(bytesstart_0_ref_0);
    let mut event_0: events::Event = crate::events::Event::End(bytesend_0);
    let mut str_1: &str = "expected";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "PI";
    let mut str_3: &str = "\"";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut str_4: &str = "7DphQezZKdI";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut str_2_ref_0: &str = &mut str_2;
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1004() {
//    rusty_monitor::set_test_id(1004);
    let mut usize_0: usize = 4679usize;
    let mut str_0: &str = "Attr::Empty";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl {element: bytesstart_0};
    let mut str_1: &str = "YV";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_2: &str = "bindings";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_1: usize = 0usize;
    let mut str_3: &str = "pdizTZ9gEIrdKof4jp";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_1};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::local_name(bytesstart_1_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesText::into_owned(bytestext_2);
    let mut bytestext_3_ref_0: &crate::events::BytesText = &mut bytestext_3;
    crate::events::BytesText::make_unescaped(bytestext_3_ref_0, option_0);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut u8_slice_1: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut bytesdecl_1: crate::events::BytesDecl = crate::events::BytesDecl::into_owned(bytesdecl_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_839() {
//    rusty_monitor::set_test_id(839);
    let mut str_0: &str = "iNqOOVTOS";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut usize_0: usize = 1usize;
    let mut str_1: &str = "UnexpectedBang";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesStart::to_end(bytesstart_0_ref_0);
    let mut event_0: events::Event = crate::events::Event::End(bytesend_0);
    let mut str_2: &str = "expected";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "\"";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut event_1: events::Event = crate::events::Event::Eof;
    let mut event_2: events::Event = crate::events::Event::into_owned(event_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_853() {
//    rusty_monitor::set_test_id(853);
    let mut usize_0: usize = 4679usize;
    let mut str_0: &str = "Attr::Empty";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl {element: bytesstart_0};
    let mut str_1: &str = "YV";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_2: &str = "bindings";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "pdizTZ9gEIrdKof4jp";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::into_owned(bytestext_1);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    crate::events::BytesText::make_unescaped(bytestext_2_ref_0, option_0);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut bytesdecl_1: crate::events::BytesDecl = crate::events::BytesDecl::into_owned(bytesdecl_0);
    let mut bytesdecl_2: crate::events::BytesDecl = crate::events::BytesDecl::into_owned(bytesdecl_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1159() {
//    rusty_monitor::set_test_id(1159);
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_1: &str = "7a8mjEvZsVmirIXn";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut str_2: &str = "End";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut usize_0: usize = 12usize;
    let mut str_3: &str = "UnquotedValue";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut option_1: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_4: &str = "End";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_4_ref_0);
    let mut reader_1_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_1;
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::name(bytesstart_0_ref_0);
    let mut error_0: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_0);
    let mut event_0: events::Event = crate::events::Event::Text(bytestext_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_119() {
//    rusty_monitor::set_test_id(119);
    let mut str_0: &str = "AX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut str_2: &str = "Attr::SingleQ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut usize_0: usize = 7756usize;
    let mut usize_1: usize = 12usize;
    let mut str_3: &str = "oBUqytEQZ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_1};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_2: usize = 1145usize;
    let mut usize_3: usize = 6228usize;
    let mut str_4: &str = "value_len";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_4_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_3};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    crate::events::BytesStart::unescaped(bytesstart_1_ref_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::name(bytesstart_0_ref_0);
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedValue(usize_0);
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesText::into_owned(bytestext_2);
    let mut bytestext_4_ref_0: &crate::events::BytesText = &mut bytestext_4;
    let mut event_0: events::Event = crate::events::Event::DocType(bytestext_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5181() {
//    rusty_monitor::set_test_id(5181);
    let mut str_0: &str = "<";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_1: &str = "SGF";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut str_2: &str = "h9";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut u8_0: u8 = 59u8;
    let mut usize_0: usize = 5usize;
    let mut str_3: &str = "InvalidCodepoint";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_0);
    let mut bytesdecl_0_ref_0: &crate::events::BytesDecl = &mut bytesdecl_0;
    let mut u8_1: u8 = 55u8;
    let mut str_4: &str = "unzeiHJsPxw4BsB";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut str_5: &str = "7a8mjEvZsVmirIXn";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_3);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut usize_1: usize = 12usize;
    let mut str_6: &str = "UnquotedValue";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_6_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_4);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_3);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_1};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut option_1: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_7: &str = "End";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = ">";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut bytestext_4: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_8_ref_0);
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_7_ref_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    crate::events::BytesText::make_unescaped(bytestext_0_ref_0, option_1);
    let mut reader_1_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_1;
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bool_0: bool = crate::reader::is_whitespace(u8_1);
    crate::events::BytesDecl::version(bytesdecl_0_ref_0);
    let mut event_0: events::Event = crate::events::Event::Eof;
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut bool_1: bool = crate::reader::is_whitespace(u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3331() {
//    rusty_monitor::set_test_id(3331);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 5usize;
    let mut usize_1: usize = 12usize;
    let mut str_0: &str = "UnexpectedToken";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_1};
    let mut event_0: events::Event = crate::events::Event::Start(bytesstart_0);
    let mut str_1: &str = "InvalidDecimal";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_2: &str = "expected";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    let mut str_3: &str = "&";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_3);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    let mut vec_0: std::vec::Vec<std::ops::Range<usize>> = std::vec::Vec::new();
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
    let mut iterstate_0: crate::events::attributes::IterState = crate::events::attributes::IterState::new(usize_0, bool_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_67() {
//    rusty_monitor::set_test_id(67);
    let mut str_0: &str = "ycCpp5V";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText {content: cow_0};
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "Duplicated";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut usize_0: usize = 4049usize;
    let mut str_2: &str = "Duplicated";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_2);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_1: usize = 9590usize;
    let mut usize_2: usize = 5usize;
    let mut usize_3: usize = 2442usize;
    let mut usize_4: usize = 6371usize;
    let mut usize_5: usize = 7091usize;
    let mut str_3: &str = "UnrecognizedSymbol";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData {content: cow_2};
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedEq(usize_5);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_4, usize_3);
    let mut u8_slice_0: &[u8] = crate::events::BytesStart::attributes_raw(bytesstart_0_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut event_0: events::Event = crate::events::Event::CData(bytescdata_1);
    crate::events::BytesText::unescaped(bytestext_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_519() {
//    rusty_monitor::set_test_id(519);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_0_ref_0: &mut std::vec::Vec<u8> = &mut vec_0;
    let mut usize_0: usize = 4usize;
    let mut str_0: &str = "V7QsytFfGqdkbvrZO1t";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut bool_0: bool = false;
    let mut i32_0: i32 = -15997i32;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5941() {
//    rusty_monitor::set_test_id(5941);
    let mut usize_0: usize = 12usize;
    let mut str_0: &str = "keys";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut attributes_0: crate::events::attributes::Attributes = crate::events::BytesStart::html_attributes(bytesstart_0_ref_0);
    let mut attributes_0_ref_0: &mut crate::events::attributes::Attributes = &mut attributes_0;
    let mut str_1: &str = "value_len";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut usize_1: usize = 5usize;
    let mut str_2: &str = "InvalidCodepoint";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_1};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_1);
    let mut bytesdecl_0_ref_0: &crate::events::BytesDecl = &mut bytesdecl_0;
    let mut str_3: &str = "unzeiHJsPxw4BsB";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_3: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4006() {
//    rusty_monitor::set_test_id(4006);
    let mut str_0: &str = "ExpectedEq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText {content: cow_0};
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    let mut str_1: &str = "Attr::DoubleQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut u8_0: u8 = 55u8;
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 12usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_1, usize_0);
    let mut str_2: &str = "unzeiHJsPxw4BsB";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut str_3: &str = "7a8mjEvZsVmirIXn";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut str_4: &str = "End";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut usize_2: usize = 12usize;
    let mut str_5: &str = "UnquotedValue";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_5_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_4);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_2};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_6: &str = "End";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = ">";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_6_ref_0);
    let mut bytestext_1_ref_0: &crate::events::BytesText = &mut bytestext_1;
    crate::events::BytesText::make_unescaped(bytestext_0_ref_0, option_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut bool_0: bool = crate::reader::is_whitespace(u8_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_3);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1031() {
//    rusty_monitor::set_test_id(1031);
    let mut str_0: &str = "YV";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut str_1: &str = "bindings";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "pdizTZ9gEIrdKof4jp";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::into_owned(bytestext_1);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_0};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_126() {
//    rusty_monitor::set_test_id(126);
    let mut usize_0: usize = 5usize;
    let mut str_0: &str = "LRCzsHjUT8TI3rx";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut str_1: &str = "KxXMkxEHQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_1};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut char_0: char = 'P';
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidHexadecimal(char_0);
    let mut char_1: char = 'J';
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut isize_0: isize = 2isize;
    let mut isize_1: isize = -5662isize;
    let mut isize_2: isize = -4435isize;
    let mut str_2: &str = "QVDx8MSqI1EPWx";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData {content: cow_2};
    let mut event_0: events::Event = crate::events::Event::CData(bytescdata_0);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::Empty(isize_2);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_1, isize_0);
    let mut error_0: errors::Error = crate::errors::Error::XmlDeclWithoutVersion(option_0);
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::InvalidDecimal(char_1);
    let mut error_1: errors::Error = crate::errors::Error::EscapeError(escapeerror_0);
    let mut u8_slice_0: &[u8] = crate::events::BytesEnd::local_name(bytesend_0_ref_0);
    let mut u8_slice_1: &[u8] = crate::events::BytesStart::attributes_raw(bytesstart_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6411() {
//    rusty_monitor::set_test_id(6411);
    let mut u8_0: u8 = 80u8;
    let mut usize_0: usize = 7usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::ExpectedQuote(usize_0, u8_0);
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut usize_1: usize = 6570usize;
    let mut str_0: &str = "SkipEqValue";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_1};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut usize_2: usize = 8usize;
    let mut usize_3: usize = 1831usize;
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_3, usize_2);
    crate::events::BytesStart::make_unescaped(bytesstart_0_ref_0, option_0);
    let mut error_0: errors::Error = crate::errors::Error::InvalidAttr(attrerror_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7562() {
//    rusty_monitor::set_test_id(7562);
    let mut option_0: std::option::Option<&[u8]> = std::option::Option::None;
    let mut option_1: std::option::Option<&[u8]> = std::option::Option::None;
    let mut usize_0: usize = 7175usize;
    let mut str_0: &str = "Start";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &mut crate::events::BytesStart = &mut bytesstart_0;
    let mut option_2: std::option::Option<&[u8]> = std::option::Option::None;
    let mut usize_1: usize = 7usize;
    let mut str_1: &str = "Next";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut isize_0: isize = 3isize;
    let mut isize_1: isize = 5isize;
    let mut option_3: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_2: &str = "Rd3sygI9";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_1);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    crate::events::BytesText::make_unescaped(bytestext_2_ref_0, option_3);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_1, isize_0);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_1};
    let mut bytesstart_2: &mut crate::events::BytesStart = crate::events::BytesStart::clear_attributes(bytesstart_0_ref_0);
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2007() {
//    rusty_monitor::set_test_id(2007);
    let mut u8_0: u8 = 115u8;
    let mut usize_0: usize = 14usize;
    let mut str_0: &str = "initialized";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut str_1: &str = "K77lEBenWgqSuFU";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut str_2: &str = "<";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut bytestext_0_ref_0: &crate::events::BytesText = &mut bytestext_0;
    crate::events::BytesText::unescaped(bytestext_0_ref_0);
    let mut event_0: events::Event = crate::events::Event::Start(bytesstart_0);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut bool_0: bool = crate::reader::is_whitespace(u8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_844() {
//    rusty_monitor::set_test_id(844);
    let mut str_0: &str = "4";
    let mut str_1: &str = "uDPYuM3fBc9DK3ujgku";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_1_ref_0);
    let mut event_0: events::Event = crate::events::Event::DocType(bytestext_0);
    let mut usize_0: usize = 6247usize;
    let mut str_2: &str = "rG63wD";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_1: usize = 5usize;
    let mut str_3: &str = "TooLongDecimal";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_1};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl {element: bytesstart_1};
    let mut bytesdecl_0_ref_0: &crate::events::BytesDecl = &mut bytesdecl_0;
    crate::events::BytesDecl::version(bytesdecl_0_ref_0);
    let mut event_1: events::Event = crate::events::Event::into_owned(event_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7738() {
//    rusty_monitor::set_test_id(7738);
    let mut str_0: &str = "bytes";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut event_0: events::Event = crate::events::Event::Text(bytestext_0);
    let mut str_1: &str = "Attr::Unquoted";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText {content: cow_0};
    let mut usize_0: usize = 5usize;
    let mut str_2: &str = "InvalidCodepoint";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_2);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_2);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_0};
    let mut bytesdecl_0: crate::events::BytesDecl = crate::events::BytesDecl::from_start(bytesstart_0);
    let mut bytesdecl_0_ref_0: &crate::events::BytesDecl = &mut bytesdecl_0;
    let mut str_3: &str = "unzeiHJsPxw4BsB";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut bytescdata_3: crate::events::BytesCData = crate::events::BytesCData::from_str(str_3_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_3);
    let mut bytesend_0: crate::events::BytesEnd = crate::events::BytesEnd {name: cow_2};
    let mut bytesend_0_ref_0: &crate::events::BytesEnd = &mut bytesend_0;
    let mut str_4: &str = "7a8mjEvZsVmirIXn";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut bytescdata_4: crate::events::BytesCData = crate::events::BytesCData::from_str(str_4_ref_0);
    let mut bytestext_3: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_4);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut event_1: events::Event = crate::events::Event::DocType(bytestext_3);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2234() {
//    rusty_monitor::set_test_id(2234);
    let mut usize_0: usize = 4usize;
    let mut usize_1: usize = 12usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_1, usize_0);
    let mut str_0: &str = "7a8mjEvZsVmirIXn";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::escape(bytescdata_0);
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut str_1: &str = "UnquotedValue";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_1);
    let mut option_1: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut str_2: &str = "End";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = ">";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut bytestext_2: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_3_ref_0);
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_2_ref_0);
    let mut bytestext_2_ref_0: &crate::events::BytesText = &mut bytestext_2;
    crate::events::BytesText::make_unescaped(bytestext_2_ref_0, option_1);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_300() {
//    rusty_monitor::set_test_id(300);
    let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut vec_0_ref_0: &mut std::vec::Vec<u8> = &mut vec_0;
    let mut usize_0: usize = 0usize;
    let mut str_0: &str = "Decoder";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut bytesstart_0_ref_0: &crate::events::BytesStart = &mut bytesstart_0;
    let mut bool_0: bool = true;
    let mut i32_0: i32 = 0i32;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_61() {
//    rusty_monitor::set_test_id(61);
    let mut usize_0: usize = 14usize;
    let mut str_0: &str = "Attr::DoubleQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_0);
    let mut bytesstart_0: crate::events::BytesStart = crate::events::BytesStart {buf: cow_0, name_len: usize_0};
    let mut usize_1: usize = 8usize;
    let mut u32_0: u32 = 10u32;
    let mut option_0: std::option::Option<&std::collections::HashMap<std::vec::Vec<u8>, std::vec::Vec<u8>>> = std::option::Option::None;
    let mut usize_2: usize = 1usize;
    let mut str_1: &str = "start";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut cow_1: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_1);
    let mut bytesstart_1: crate::events::BytesStart = crate::events::BytesStart {buf: cow_1, name_len: usize_2};
    let mut bytesstart_1_ref_0: &crate::events::BytesStart = &mut bytesstart_1;
    let mut usize_3: usize = 7usize;
    let mut str_2: &str = "TooLongHexadecimal";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytestext_1: crate::events::BytesText = crate::events::BytesText::from_plain_str(str_2_ref_0);
    let mut cow_2: std::borrow::Cow<[u8]> = crate::events::BytesText::into_inner(bytestext_1);
    let mut usize_4: usize = 5285usize;
    let mut usize_5: usize = 6usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::Duplicated(usize_5, usize_4);
    let mut bytesstart_2: crate::events::BytesStart = crate::events::BytesStart {buf: cow_2, name_len: usize_3};
    crate::events::BytesStart::make_unescaped(bytesstart_1_ref_0, option_0);
    let mut bytesstart_2_ref_0: &crate::events::BytesStart = &mut bytesstart_2;
    let mut bytesstart_3: crate::events::BytesStart = crate::events::BytesStart::to_borrowed(bytesstart_2_ref_0);
    let mut bytesstart_3_ref_0: &mut crate::events::BytesStart = &mut bytesstart_3;
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_0);
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::TooLongDecimal;
    let mut event_0: events::Event = crate::events::Event::Empty(bytesstart_0);
//    panic!("From RustyUnit with love");
}
}
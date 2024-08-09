//! Serializing Rust structures into TOML.
//!
//! This module contains all the Serde support for serializing Rust structures
//! into TOML documents (as strings). Note that some top-level functions here
//! are also provided at the top of the crate.
/// Serialize the given data structure as a String of TOML.
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, if `T` contains a map with non-string keys, or if `T` attempts to
/// serialize an unsupported datatype such as an enum, tuple, or tuple struct.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
///
/// # Examples
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Config {
///     database: Database,
/// }
///
/// #[derive(Serialize)]
/// struct Database {
///     ip: String,
///     port: Vec<u16>,
///     connection_max: u32,
///     enabled: bool,
/// }
///
/// let config = Config {
///     database: Database {
///         ip: "192.168.1.1".to_string(),
///         port: vec![8001, 8002, 8003],
///         connection_max: 5000,
///         enabled: false,
///     },
/// };
///
/// let toml = toml::to_string(&config).unwrap();
/// println!("{}", toml)
/// ```
#[cfg(feature = "display")]
pub fn to_string<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize,
{
    let mut output = String::new();
    let serializer = Serializer::new(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}
/// Serialize the given data structure as a "pretty" String of TOML.
///
/// This is identical to `to_string` except the output string has a more
/// "pretty" output. See `Serializer::pretty` for more details.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
///
/// For greater customization, instead serialize to a
/// [`toml_edit::Document`](https://docs.rs/toml_edit/latest/toml_edit/struct.Document.html).
#[cfg(feature = "display")]
pub fn to_string_pretty<T: ?Sized>(value: &T) -> Result<String, Error>
where
    T: serde::ser::Serialize,
{
    let mut output = String::new();
    let serializer = Serializer::pretty(&mut output);
    value.serialize(serializer)?;
    Ok(output)
}
/// Errors that can occur when serializing a type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub(crate) inner: crate::edit::ser::Error,
}
impl Error {
    pub(crate) fn new(inner: impl std::fmt::Display) -> Self {
        Self {
            inner: crate::edit::ser::Error::Custom(inner.to_string()),
        }
    }
    #[cfg(feature = "display")]
    pub(crate) fn wrap(inner: crate::edit::ser::Error) -> Self {
        Self { inner }
    }
    pub(crate) fn unsupported_type(t: Option<&'static str>) -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedType(t),
        }
    }
    pub(crate) fn unsupported_none() -> Self {
        Self {
            inner: crate::edit::ser::Error::UnsupportedNone,
        }
    }
    pub(crate) fn key_not_string() -> Self {
        Self {
            inner: crate::edit::ser::Error::KeyNotString,
        }
    }
}
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::new(msg)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
impl std::error::Error for Error {}
/// Serialization for TOML documents.
///
/// This structure implements serialization support for TOML to serialize an
/// arbitrary type to TOML. Note that the TOML format does not support all
/// datatypes in Rust, such as enums, tuples, and tuple structs. These types
/// will generate an error when serialized.
///
/// Currently a serializer always writes its output to an in-memory `String`,
/// which is passed in when creating the serializer itself.
///
/// To serialize TOML values, instead of documents, see [`ValueSerializer`].
#[non_exhaustive]
#[cfg(feature = "display")]
pub struct Serializer<'d> {
    dst: &'d mut String,
    settings: crate::fmt::DocumentFormatter,
}
#[cfg(feature = "display")]
impl<'d> Serializer<'d> {
    /// Creates a new serializer which will emit TOML into the buffer provided.
    ///
    /// The serializer can then be used to serialize a type after which the data
    /// will be present in `dst`.
    pub fn new(dst: &'d mut String) -> Self {
        Self {
            dst,
            settings: Default::default(),
        }
    }
    /// Apply a default "pretty" policy to the document
    ///
    /// For greater customization, instead serialize to a
    /// [`toml_edit::Document`](https://docs.rs/toml_edit/latest/toml_edit/struct.Document.html).
    pub fn pretty(dst: &'d mut String) -> Self {
        let mut ser = Serializer::new(dst);
        ser.settings.multiline_array = true;
        ser
    }
}
#[cfg(feature = "display")]
impl<'d> serde::ser::Serializer for Serializer<'d> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeDocumentArray<'d>;
    type SerializeTuple = SerializeDocumentArray<'d>;
    type SerializeTupleStruct = SerializeDocumentArray<'d>;
    type SerializeTupleVariant = SerializeDocumentArray<'d>;
    type SerializeMap = SerializeDocumentTable<'d>;
    type SerializeStruct = SerializeDocumentTable<'d>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_bool(v),
        )
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i8(v),
        )
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i16(v),
        )
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i32(v),
        )
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_i64(v),
        )
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u8(v),
        )
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u16(v),
        )
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u32(v),
        )
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_u64(v),
        )
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_f32(v),
        )
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_f64(v),
        )
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_char(v),
        )
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_str(v),
        )
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_bytes(v),
        )
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_none(),
        )
    }
    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_some(v),
        )
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit(),
        )
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_unit_struct(name),
        )
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new()
                .serialize_unit_variant(name, variant_index, variant),
        )
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_struct(name, v),
        )
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_document(
            self.dst,
            self.settings,
            toml_edit::ser::ValueSerializer::new()
                .serialize_newtype_variant(name, variant_index, variant, value),
        )
    }
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_seq(len)
            .map_err(Error::wrap)?;
        let ser = SerializeDocumentArray::new(self, ser);
        Ok(ser)
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_map(len)
            .map_err(Error::wrap)?;
        let ser = SerializeDocumentTable::new(self, ser);
        Ok(ser)
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::unsupported_type(Some(name)))
    }
}
/// Serialization for TOML [values][crate::Value].
///
/// This structure implements serialization support for TOML to serialize an
/// arbitrary type to TOML. Note that the TOML format does not support all
/// datatypes in Rust, such as enums, tuples, and tuple structs. These types
/// will generate an error when serialized.
///
/// Currently a serializer always writes its output to an in-memory `String`,
/// which is passed in when creating the serializer itself.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Config {
///     database: Database,
/// }
///
/// #[derive(Serialize)]
/// struct Database {
///     ip: String,
///     port: Vec<u16>,
///     connection_max: u32,
///     enabled: bool,
/// }
///
/// let config = Config {
///     database: Database {
///         ip: "192.168.1.1".to_string(),
///         port: vec![8001, 8002, 8003],
///         connection_max: 5000,
///         enabled: false,
///     },
/// };
///
/// let mut value = String::new();
/// serde::Serialize::serialize(
///     &config,
///     toml::ser::ValueSerializer::new(&mut value)
/// ).unwrap();
/// println!("{}", value)
/// ```
#[non_exhaustive]
#[cfg(feature = "display")]
pub struct ValueSerializer<'d> {
    dst: &'d mut String,
}
#[cfg(feature = "display")]
impl<'d> ValueSerializer<'d> {
    /// Creates a new serializer which will emit TOML into the buffer provided.
    ///
    /// The serializer can then be used to serialize a type after which the data
    /// will be present in `dst`.
    pub fn new(dst: &'d mut String) -> Self {
        Self { dst }
    }
}
#[cfg(feature = "display")]
impl<'d> serde::ser::Serializer for ValueSerializer<'d> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeValueArray<'d>;
    type SerializeTuple = SerializeValueArray<'d>;
    type SerializeTupleStruct = SerializeValueArray<'d>;
    type SerializeTupleVariant = SerializeValueArray<'d>;
    type SerializeMap = SerializeValueTable<'d>;
    type SerializeStruct = SerializeValueTable<'d>;
    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_bool(v))
    }
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_i8(v))
    }
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_i16(v))
    }
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_i32(v))
    }
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_i64(v))
    }
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_u8(v))
    }
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_u16(v))
    }
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_u32(v))
    }
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_u64(v))
    }
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_f32(v))
    }
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_f64(v))
    }
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_char(v))
    }
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_str(v))
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_bytes(v))
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_none())
    }
    fn serialize_some<T: ?Sized>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_some(v))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write_value(self.dst, toml_edit::ser::ValueSerializer::new().serialize_unit())
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_unit_struct(name),
        )
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new()
                .serialize_unit_variant(name, variant_index, variant),
        )
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        v: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new().serialize_newtype_struct(name, v),
        )
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        write_value(
            self.dst,
            toml_edit::ser::ValueSerializer::new()
                .serialize_newtype_variant(name, variant_index, variant, value),
        )
    }
    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_seq(len)
            .map_err(Error::wrap)?;
        let ser = SerializeValueArray::new(self, ser);
        Ok(ser)
    }
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }
    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap, Self::Error> {
        let ser = toml_edit::ser::ValueSerializer::new()
            .serialize_map(len)
            .map_err(Error::wrap)?;
        let ser = SerializeValueTable::new(self, ser);
        Ok(ser)
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::unsupported_type(Some(name)))
    }
}
#[cfg(feature = "display")]
use internal::*;
#[cfg(feature = "display")]
mod internal {
    use super::*;
    use crate::fmt::DocumentFormatter;
    type InnerSerializeDocumentSeq = <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;
    #[doc(hidden)]
    pub struct SerializeDocumentArray<'d> {
        inner: InnerSerializeDocumentSeq,
        dst: &'d mut String,
        settings: DocumentFormatter,
    }
    impl<'d> SerializeDocumentArray<'d> {
        pub(crate) fn new(
            ser: Serializer<'d>,
            inner: InnerSerializeDocumentSeq,
        ) -> Self {
            Self {
                inner,
                dst: ser.dst,
                settings: ser.settings,
            }
        }
    }
    impl<'d> serde::ser::SerializeSeq for SerializeDocumentArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeTuple for SerializeDocumentArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeTupleVariant for SerializeDocumentArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeTupleStruct for SerializeDocumentArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }
    type InnerSerializeDocumentTable = <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;
    #[doc(hidden)]
    pub struct SerializeDocumentTable<'d> {
        inner: InnerSerializeDocumentTable,
        dst: &'d mut String,
        settings: DocumentFormatter,
    }
    impl<'d> SerializeDocumentTable<'d> {
        pub(crate) fn new(
            ser: Serializer<'d>,
            inner: InnerSerializeDocumentTable,
        ) -> Self {
            Self {
                inner,
                dst: ser.dst,
                settings: ser.settings,
            }
        }
    }
    impl<'d> serde::ser::SerializeMap for SerializeDocumentTable<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_key(input).map_err(Error::wrap)
        }
        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_value(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeStruct for SerializeDocumentTable<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(key, value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_document(self.dst, self.settings, self.inner.end())
        }
    }
    pub(crate) fn write_document(
        dst: &mut String,
        mut settings: DocumentFormatter,
        value: Result<toml_edit::Value, crate::edit::ser::Error>,
    ) -> Result<(), Error> {
        use std::fmt::Write;
        let value = value.map_err(Error::wrap)?;
        let mut table = match toml_edit::Item::Value(value).into_table() {
            Ok(i) => i,
            Err(_) => {
                return Err(Error::unsupported_type(None));
            }
        };
        use toml_edit::visit_mut::VisitMut as _;
        settings.visit_table_mut(&mut table);
        let doc: toml_edit::Document = table.into();
        write!(dst, "{}", doc).unwrap();
        Ok(())
    }
    type InnerSerializeValueSeq = <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeSeq;
    #[doc(hidden)]
    pub struct SerializeValueArray<'d> {
        inner: InnerSerializeValueSeq,
        dst: &'d mut String,
    }
    impl<'d> SerializeValueArray<'d> {
        pub(crate) fn new(
            ser: ValueSerializer<'d>,
            inner: InnerSerializeValueSeq,
        ) -> Self {
            Self { inner, dst: ser.dst }
        }
    }
    impl<'d> serde::ser::SerializeSeq for SerializeValueArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeTuple for SerializeValueArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_element(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeTupleVariant for SerializeValueArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeTupleStruct for SerializeValueArray<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }
    type InnerSerializeValueTable = <toml_edit::ser::ValueSerializer as serde::Serializer>::SerializeMap;
    #[doc(hidden)]
    pub struct SerializeValueTable<'d> {
        inner: InnerSerializeValueTable,
        dst: &'d mut String,
    }
    impl<'d> SerializeValueTable<'d> {
        pub(crate) fn new(
            ser: ValueSerializer<'d>,
            inner: InnerSerializeValueTable,
        ) -> Self {
            Self { inner, dst: ser.dst }
        }
    }
    impl<'d> serde::ser::SerializeMap for SerializeValueTable<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_key<T: ?Sized>(&mut self, input: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_key(input).map_err(Error::wrap)
        }
        fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_value(value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }
    impl<'d> serde::ser::SerializeStruct for SerializeValueTable<'d> {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized>(
            &mut self,
            key: &'static str,
            value: &T,
        ) -> Result<(), Self::Error>
        where
            T: serde::ser::Serialize,
        {
            self.inner.serialize_field(key, value).map_err(Error::wrap)
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            write_value(self.dst, self.inner.end())
        }
    }
    pub(crate) fn write_value(
        dst: &mut String,
        value: Result<toml_edit::Value, crate::edit::ser::Error>,
    ) -> Result<(), Error> {
        use std::fmt::Write;
        let value = value.map_err(Error::wrap)?;
        write!(dst, "{}", value).unwrap();
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_67 {
    use super::*;
    use crate::*;
    use crate::ser::Serializer;
    use serde::Serializer as _;
    #[test]
    fn test_serialize_bool_true() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_serialize_bool_true = 0;
        let rug_fuzz_0 = true;
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.serialize_bool(rug_fuzz_0).unwrap();
        debug_assert_eq!(buffer, "true");
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_serialize_bool_true = 0;
    }
    #[test]
    fn test_serialize_bool_false() {
        let _rug_st_tests_llm_16_67_rrrruuuugggg_test_serialize_bool_false = 0;
        let rug_fuzz_0 = false;
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.serialize_bool(rug_fuzz_0).unwrap();
        debug_assert_eq!(buffer, "false");
        let _rug_ed_tests_llm_16_67_rrrruuuugggg_test_serialize_bool_false = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_69 {
    use crate::Serializer;
    use serde::Serializer as _;
    #[test]
    fn test_serialize_char() {
        let _rug_st_tests_llm_16_69_rrrruuuugggg_test_serialize_char = 0;
        let rug_fuzz_0 = 'a';
        let mut output = String::new();
        let mut serializer = Serializer::new(&mut output);
        serializer.serialize_char(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, "'a'");
        let _rug_ed_tests_llm_16_69_rrrruuuugggg_test_serialize_char = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_72_llm_16_72 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer as _;
    #[test]
    fn test_serialize_i16() -> Result<(), crate::ser::Error> {
        let value: i16 = 42;
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.serialize_i16(value)?;
        assert_eq!(buffer, "42");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_75 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    use std::string::ToString;
    use crate::ser::Serializer as TomlSerializer;
    use toml_edit::ser::Error;
    #[test]
    fn test_serialize_i8() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_serialize_i8 = 0;
        let rug_fuzz_0 = 42;
        let mut output = String::new();
        let mut serializer = TomlSerializer::new(&mut output);
        serializer.serialize_i8(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, "42");
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_serialize_i8 = 0;
    }
    #[test]
    fn test_serialize_i8_negative() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_negative = 0;
        let rug_fuzz_0 = 42;
        let mut output = String::new();
        let mut serializer = TomlSerializer::new(&mut output);
        serializer.serialize_i8(-rug_fuzz_0).unwrap();
        debug_assert_eq!(output, "-42");
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_negative = 0;
    }
    #[test]
    fn test_serialize_i8_min_value() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_min_value = 0;
        let mut output = String::new();
        let mut serializer = TomlSerializer::new(&mut output);
        serializer.serialize_i8(i8::MIN).unwrap();
        debug_assert_eq!(output, i8::MIN.to_string());
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_min_value = 0;
    }
    #[test]
    fn test_serialize_i8_max_value() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_max_value = 0;
        let mut output = String::new();
        let mut serializer = TomlSerializer::new(&mut output);
        serializer.serialize_i8(i8::MAX).unwrap();
        debug_assert_eq!(output, i8::MAX.to_string());
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_max_value = 0;
    }
    #[test]
    fn test_serialize_i8_zero() {
        let _rug_st_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_zero = 0;
        let rug_fuzz_0 = 0;
        let mut output = String::new();
        let mut serializer = TomlSerializer::new(&mut output);
        serializer.serialize_i8(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, "0");
        let _rug_ed_tests_llm_16_75_rrrruuuugggg_test_serialize_i8_zero = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_76_llm_16_76 {
    use crate::ser::Serializer;
    use crate::ser::Error;
    use serde::ser::{SerializeMap, Serializer as _};
    #[test]
    fn test_serialize_map() {
        let _rug_st_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_serialize_map = 0;
        let rug_fuzz_0 = "key";
        let rug_fuzz_1 = "value";
        let mut dst = String::new();
        let mut serializer = Serializer::new(&mut dst);
        let serialize_map_result = serializer.serialize_map(None);
        debug_assert!(serialize_map_result.is_ok());
        let mut map_serializer = serialize_map_result.unwrap();
        debug_assert!(map_serializer.serialize_key(rug_fuzz_0).is_ok());
        debug_assert!(map_serializer.serialize_value(rug_fuzz_1).is_ok());
        debug_assert!(map_serializer.end().is_ok());
        debug_assert_eq!(dst, "key = \"value\"\n");
        let _rug_ed_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_serialize_map = 0;
    }
    #[test]
    fn test_serialize_map_with_len() {
        let _rug_st_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_serialize_map_with_len = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = "key";
        let rug_fuzz_2 = "value";
        let mut dst = String::new();
        let mut serializer = Serializer::new(&mut dst);
        let serialize_map_result = serializer.serialize_map(Some(rug_fuzz_0));
        debug_assert!(serialize_map_result.is_ok());
        let mut map_serializer = serialize_map_result.unwrap();
        debug_assert!(map_serializer.serialize_key(rug_fuzz_1).is_ok());
        debug_assert!(map_serializer.serialize_value(rug_fuzz_2).is_ok());
        debug_assert!(map_serializer.end().is_ok());
        debug_assert_eq!(dst, "key = \"value\"\n");
        let _rug_ed_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_serialize_map_with_len = 0;
    }
    #[test]
    fn test_serialize_map_error() {
        let _rug_st_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_serialize_map_error = 0;
        let rug_fuzz_0 = "Should have created map serializer";
        let rug_fuzz_1 = "value";
        let mut dst = String::new();
        let serializer = Serializer::new(&mut dst);
        let serialize_map_result: Result<_, Error> = serializer.serialize_map(None);
        let mut map_serializer = serialize_map_result.expect(rug_fuzz_0);
        let res = map_serializer.serialize_value(rug_fuzz_1);
        debug_assert!(res.is_err());
        let _rug_ed_tests_llm_16_76_llm_16_76_rrrruuuugggg_test_serialize_map_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_79 {
    use crate::Serializer;
    use serde::Serializer as _;
    use std::string::ToString;
    use crate::ser::Error;
    #[test]
    fn test_serialize_none() -> Result<(), Error> {
        let mut output = String::new();
        let serializer = Serializer::new(&mut output);
        serializer.serialize_none()?;
        assert!(output.is_empty(), "Expected empty string for None, found: {}", output);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_85_llm_16_85 {
    use super::*;
    use crate::*;
    use crate::ser::Serializer;
    use serde::ser::{SerializeSeq, Serializer as _};
    use toml_edit::ser::Error;
    #[test]
    fn test_serialize_tuple() {
        let _rug_st_tests_llm_16_85_llm_16_85_rrrruuuugggg_test_serialize_tuple = 0;
        let rug_fuzz_0 = 2;
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = 2;
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        let mut tuple_serializer = serializer.serialize_tuple(rug_fuzz_0).unwrap();
        tuple_serializer.serialize_element(&rug_fuzz_1).unwrap();
        tuple_serializer.serialize_element(&rug_fuzz_2).unwrap();
        tuple_serializer.end().unwrap();
        debug_assert_eq!(buffer, "1\n2\n");
        let _rug_ed_tests_llm_16_85_llm_16_85_rrrruuuugggg_test_serialize_tuple = 0;
    }
    #[test]
    fn test_serialize_tuple_error() {
        let _rug_st_tests_llm_16_85_llm_16_85_rrrruuuugggg_test_serialize_tuple_error = 0;
        let rug_fuzz_0 = 2;
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        let res = serializer.serialize_tuple(rug_fuzz_0);
        debug_assert!(matches!(res, Ok(_)));
        let _rug_ed_tests_llm_16_85_llm_16_85_rrrruuuugggg_test_serialize_tuple_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_88_llm_16_88 {
    use crate::Serializer;
    use serde::Serializer as SerdeSerializer;
    #[test]
    fn serialize_u16_test() {
        let _rug_st_tests_llm_16_88_llm_16_88_rrrruuuugggg_serialize_u16_test = 0;
        let rug_fuzz_0 = 42;
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.serialize_u16(rug_fuzz_0).unwrap();
        debug_assert_eq!(buffer, "42");
        let _rug_ed_tests_llm_16_88_llm_16_88_rrrruuuugggg_serialize_u16_test = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_89 {
    use crate::Serializer;
    use serde::Serializer as _;
    fn serialize_u32_helper(value: u32) -> Result<String, crate::ser::Error> {
        let mut buffer = String::new();
        let serializer = Serializer::new(&mut buffer);
        serializer.serialize_u32(value)?;
        Ok(buffer)
    }
    #[test]
    fn serialize_u32_min_value() -> Result<(), crate::ser::Error> {
        let value = u32::MIN;
        let result = serialize_u32_helper(value)?;
        assert_eq!(result, "0");
        Ok(())
    }
    #[test]
    fn serialize_u32_max_value() -> Result<(), crate::ser::Error> {
        let value = u32::MAX;
        let result = serialize_u32_helper(value)?;
        assert_eq!(result, u32::MAX.to_string());
        Ok(())
    }
    #[test]
    fn serialize_u32_arbitrary_value() -> Result<(), crate::ser::Error> {
        let value = 12345;
        let result = serialize_u32_helper(value)?;
        assert_eq!(result, "12345");
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_90 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer as _;
    #[test]
    fn test_serialize_u64() {
        let _rug_st_tests_llm_16_90_rrrruuuugggg_test_serialize_u64 = 0;
        let rug_fuzz_0 = 42;
        let mut output = String::new();
        let mut serializer = Serializer::new(&mut output);
        debug_assert!(serializer.serialize_u64(rug_fuzz_0).is_ok());
        debug_assert_eq!(output, "42");
        let _rug_ed_tests_llm_16_90_rrrruuuugggg_test_serialize_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_91_llm_16_91 {
    use super::*;
    use crate::*;
    use crate::ser::Serializer;
    use serde::ser::Serializer as _;
    use crate::Value;
    use crate::ser::Error;
    #[test]
    fn test_serialize_u8() {
        let _rug_st_tests_llm_16_91_llm_16_91_rrrruuuugggg_test_serialize_u8 = 0;
        let rug_fuzz_0 = 42;
        let mut buf = String::new();
        let mut serializer = Serializer::new(&mut buf);
        let res = serializer.serialize_u8(rug_fuzz_0);
        debug_assert!(res.is_ok());
        debug_assert_eq!(buf, "42");
        let _rug_ed_tests_llm_16_91_llm_16_91_rrrruuuugggg_test_serialize_u8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_92_llm_16_92 {
    use crate::ser::Serializer;
    use serde::ser::Serializer as _;
    #[test]
    fn test_serialize_unit() {
        let _rug_st_tests_llm_16_92_llm_16_92_rrrruuuugggg_test_serialize_unit = 0;
        let mut output = String::new();
        {
            let mut ser = Serializer::new(&mut output);
            let res = ser.serialize_unit();
            debug_assert!(res.is_ok());
        }
        debug_assert_eq!(output, "");
        let _rug_ed_tests_llm_16_92_llm_16_92_rrrruuuugggg_test_serialize_unit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_93 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    #[test]
    fn test_serialize_unit_struct() {
        let _rug_st_tests_llm_16_93_rrrruuuugggg_test_serialize_unit_struct = 0;
        let rug_fuzz_0 = "MyUnitStruct";
        let mut buffer = String::new();
        let serializer = ser::Serializer::new(&mut buffer);
        let result = serializer.serialize_unit_struct(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, "");
        let _rug_ed_tests_llm_16_93_rrrruuuugggg_test_serialize_unit_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_96_llm_16_96 {
    use crate::ser::{Error, Serializer as TomlSerializer, ValueSerializer};
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_bytes() {
        let _rug_st_tests_llm_16_96_llm_16_96_rrrruuuugggg_test_serialize_bytes = 0;
        let rug_fuzz_0 = 0;
        let rug_fuzz_1 = 159;
        let rug_fuzz_2 = 146;
        let rug_fuzz_3 = 150;
        let rug_fuzz_4 = "\"\\u{0}\\u{9f}\\u{92}\\u{96}\"";
        let mut dst = String::new();
        let mut serializer = ValueSerializer::new(&mut dst);
        let test_bytes: &[u8] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        let result = serializer.serialize_bytes(test_bytes);
        debug_assert!(result.is_ok());
        let expected_value = String::from(rug_fuzz_4);
        debug_assert_eq!(expected_value, dst);
        let _rug_ed_tests_llm_16_96_llm_16_96_rrrruuuugggg_test_serialize_bytes = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_97_llm_16_97 {
    use super::*;
    use crate::*;
    use crate::ser::ValueSerializer;
    use serde::Serializer;
    #[test]
    fn test_serialize_char() {
        let _rug_st_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_serialize_char = 0;
        let rug_fuzz_0 = 'a';
        let mut output = String::new();
        let serializer = ValueSerializer::new(&mut output);
        serializer.serialize_char(rug_fuzz_0).unwrap();
        debug_assert_eq!(output, "'a'");
        let _rug_ed_tests_llm_16_97_llm_16_97_rrrruuuugggg_test_serialize_char = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_100 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    #[test]
    fn test_serialize_i16() {
        let _rug_st_tests_llm_16_100_rrrruuuugggg_test_serialize_i16 = 0;
        let rug_fuzz_0 = 123;
        let mut dst = String::new();
        let serializer = ValueSerializer::new(&mut dst);
        let value: i16 = rug_fuzz_0;
        serializer.serialize_i16(value).unwrap();
        debug_assert_eq!(dst, "123");
        let _rug_ed_tests_llm_16_100_rrrruuuugggg_test_serialize_i16 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_101_llm_16_101 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::ser::ValueSerializer;
    #[test]
    fn test_serialize_i32() {
        let _rug_st_tests_llm_16_101_llm_16_101_rrrruuuugggg_test_serialize_i32 = 0;
        let rug_fuzz_0 = 42;
        let mut buffer = String::new();
        let serializer = ValueSerializer::new(&mut buffer);
        let result = serializer.serialize_i32(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, "42");
        let _rug_ed_tests_llm_16_101_llm_16_101_rrrruuuugggg_test_serialize_i32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_102_llm_16_102 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use crate::ser::ValueSerializer;
    use crate::ser::Error;
    #[test]
    fn test_serialize_i64() {
        let _rug_st_tests_llm_16_102_llm_16_102_rrrruuuugggg_test_serialize_i64 = 0;
        let rug_fuzz_0 = 42_i64;
        let mut buffer = String::new();
        let serializer = ValueSerializer::new(&mut buffer);
        let result = serializer.serialize_i64(rug_fuzz_0);
        match result {
            Ok(()) => debug_assert_eq!(buffer, "42"),
            Err(_) => panic!("Expected Ok, got Err"),
        }
        let _rug_ed_tests_llm_16_102_llm_16_102_rrrruuuugggg_test_serialize_i64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_103 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_i8() {
        let _rug_st_tests_llm_16_103_rrrruuuugggg_test_serialize_i8 = 0;
        let rug_fuzz_0 = 42;
        let mut serialized_string = String::new();
        let serializer = ValueSerializer::new(&mut serialized_string);
        let value_to_serialize: i8 = rug_fuzz_0;
        let result = serializer.serialize_i8(value_to_serialize);
        debug_assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        debug_assert_eq!(serialized_string, "42");
        let _rug_ed_tests_llm_16_103_rrrruuuugggg_test_serialize_i8 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_106_llm_16_106 {
    use super::*;
    use crate::*;
    use crate::ser::{Error, ValueSerializer};
    use crate::Value;
    use serde::Serialize;
    use std::collections::BTreeMap;
    use serde::ser::Serializer;
    #[derive(Serialize)]
    struct NewTypeVariantTest {
        data: String,
    }
    #[test]
    fn test_serialize_newtype_variant() {
        let _rug_st_tests_llm_16_106_llm_16_106_rrrruuuugggg_test_serialize_newtype_variant = 0;
        let rug_fuzz_0 = "test_data";
        let rug_fuzz_1 = "TestVariant";
        let rug_fuzz_2 = 0;
        let rug_fuzz_3 = "NewTypeVariantTest";
        let rug_fuzz_4 = r#"TestVariant = "test_data""#;
        let mut dst = String::new();
        let mut serializer = ValueSerializer::new(&mut dst);
        let test_value = NewTypeVariantTest {
            data: rug_fuzz_0.to_string(),
        };
        let ser_result = serializer
            .serialize_newtype_variant(rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, &test_value);
        debug_assert!(ser_result.is_ok());
        let expected_output = rug_fuzz_4;
        debug_assert_eq!(dst, expected_output);
        let _rug_ed_tests_llm_16_106_llm_16_106_rrrruuuugggg_test_serialize_newtype_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_107 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_none() {
        let _rug_st_tests_llm_16_107_rrrruuuugggg_test_serialize_none = 0;
        let mut dst = String::new();
        let serializer = ValueSerializer::new(&mut dst);
        serializer.serialize_none().unwrap();
        debug_assert_eq!(dst, "null");
        let _rug_ed_tests_llm_16_107_rrrruuuugggg_test_serialize_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_109_llm_16_109 {
    use crate::ser::{ValueSerializer, Error};
    use serde::Serializer;
    #[test]
    fn test_serialize_some_with_present_value() {
        let _rug_st_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_present_value = 0;
        let rug_fuzz_0 = 42u32;
        let mut dst = String::new();
        let serializer = ValueSerializer::new(&mut dst);
        let result = serializer.serialize_some(&rug_fuzz_0);
        debug_assert!(result.is_ok(), "Expected Ok but got Err");
        debug_assert_eq!(dst, "42");
        let _rug_ed_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_present_value = 0;
    }
    #[test]
    fn test_serialize_some_with_none() {
        let _rug_st_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_none = 0;
        let mut dst = String::new();
        let serializer = ValueSerializer::new(&mut dst);
        let result: Result<(), Error> = serializer.serialize_some(&Option::<u32>::None);
        debug_assert!(result.is_ok(), "Expected Ok but got Err");
        debug_assert_eq!(dst, "");
        let _rug_ed_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_none = 0;
    }
    #[test]
    fn test_serialize_some_with_error() {
        let _rug_st_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_error = 0;
        let rug_fuzz_0 = "unsupported type";
        let mut dst = String::new();
        let serializer = ValueSerializer::new(&mut dst);
        let result = serializer.serialize_some(&rug_fuzz_0);
        debug_assert!(result.is_err(), "Expected Err but got Ok");
        let _rug_ed_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_error = 0;
    }
    #[test]
    fn test_serialize_some_with_complex_type() {
        let _rug_st_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_complex_type = 0;
        let rug_fuzz_0 = "example";
        let rug_fuzz_1 = 42;
        #[derive(serde::Serialize)]
        struct ComplexType {
            key: String,
            value: u32,
        }
        let complex_value = ComplexType {
            key: rug_fuzz_0.into(),
            value: rug_fuzz_1,
        };
        let mut dst = String::new();
        let serializer = ValueSerializer::new(&mut dst);
        let result = serializer.serialize_some(&complex_value);
        debug_assert!(result.is_ok(), "Expected Ok but got Err");
        debug_assert_eq!(dst, "key = \"example\"\nvalue = 42");
        let _rug_ed_tests_llm_16_109_llm_16_109_rrrruuuugggg_test_serialize_some_with_complex_type = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_110_llm_16_110 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_str() {
        let _rug_st_tests_llm_16_110_llm_16_110_rrrruuuugggg_test_serialize_str = 0;
        let rug_fuzz_0 = "Hello, World!";
        let mut dst = String::new();
        let mut serializer = ValueSerializer::new(&mut dst);
        let result = serializer.serialize_str(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(dst, "\"Hello, World!\"");
        let _rug_ed_tests_llm_16_110_llm_16_110_rrrruuuugggg_test_serialize_str = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_114 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    use crate::ser::Error;
    struct MockSerializeTupleStruct;
    impl serde::ser::SerializeTupleStruct for MockSerializeTupleStruct {
        type Ok = ();
        type Error = Error;
        fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
        where
            T: serde::Serialize,
        {
            Ok(())
        }
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    #[test]
    fn test_serialize_tuple_struct() {
        let _rug_st_tests_llm_16_114_rrrruuuugggg_test_serialize_tuple_struct = 0;
        let rug_fuzz_0 = "MyTupleStruct";
        let rug_fuzz_1 = 3;
        let mut output = String::new();
        let serializer = ValueSerializer::new(&mut output);
        let name = rug_fuzz_0;
        let len = rug_fuzz_1;
        let result = serializer.serialize_tuple_struct(name, len);
        debug_assert!(matches!(result, Ok(_)));
        let _rug_ed_tests_llm_16_114_rrrruuuugggg_test_serialize_tuple_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_116 {
    use serde::Serializer;
    use crate::ser::{ValueSerializer, Error};
    #[test]
    fn test_serialize_u16() {
        let _rug_st_tests_llm_16_116_rrrruuuugggg_test_serialize_u16 = 0;
        let rug_fuzz_0 = 42u16;
        let mut dest = String::new();
        let value_serializer = ValueSerializer::new(&mut dest);
        let result = value_serializer.serialize_u16(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(dest, "42");
        let _rug_ed_tests_llm_16_116_rrrruuuugggg_test_serialize_u16 = 0;
    }
    #[test]
    fn test_serialize_u16_error() {
        let _rug_st_tests_llm_16_116_rrrruuuugggg_test_serialize_u16_error = 0;
        let _rug_ed_tests_llm_16_116_rrrruuuugggg_test_serialize_u16_error = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_117 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    use serde::ser::Error;
    use std::string::String;
    #[test]
    fn test_serialize_u32() {
        let _rug_st_tests_llm_16_117_rrrruuuugggg_test_serialize_u32 = 0;
        let rug_fuzz_0 = 1234_u32;
        let mut dest = String::new();
        let serializer = crate::ser::ValueSerializer::new(&mut dest);
        let result = serializer.serialize_u32(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(dest, "1234");
        let _rug_ed_tests_llm_16_117_rrrruuuugggg_test_serialize_u32 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_118 {
    use serde::Serializer;
    use super::*;
    use crate::*;
    #[test]
    fn test_serialize_u64() {
        let _rug_st_tests_llm_16_118_rrrruuuugggg_test_serialize_u64 = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = "Failed to serialize u64 value";
        let mut buffer = String::new();
        let value_serializer = ValueSerializer::new(&mut buffer);
        let value: u64 = rug_fuzz_0;
        value_serializer.serialize_u64(value).expect(rug_fuzz_1);
        debug_assert_eq!(buffer, "42");
        let _rug_ed_tests_llm_16_118_rrrruuuugggg_test_serialize_u64 = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_120 {
    use super::*;
    use crate::*;
    use serde::Serializer;
    #[test]
    fn test_serialize_unit() {
        let _rug_st_tests_llm_16_120_rrrruuuugggg_test_serialize_unit = 0;
        let mut buf = String::new();
        let serializer = ValueSerializer::new(&mut buf);
        let result = serializer.serialize_unit();
        debug_assert!(result.is_ok());
        debug_assert_eq!(buf, "");
        let _rug_ed_tests_llm_16_120_rrrruuuugggg_test_serialize_unit = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_121 {
    use super::*;
    use crate::*;
    use serde::ser::Serializer;
    #[test]
    fn test_serialize_unit_struct() {
        let _rug_st_tests_llm_16_121_rrrruuuugggg_test_serialize_unit_struct = 0;
        let rug_fuzz_0 = "UnitTestStruct";
        let mut buffer = String::new();
        let serializer = ValueSerializer::new(&mut buffer);
        let result = serializer.serialize_unit_struct(rug_fuzz_0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(buffer, "");
        let _rug_ed_tests_llm_16_121_rrrruuuugggg_test_serialize_unit_struct = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_122_llm_16_122 {
    use crate::ser::{Error, ValueSerializer};
    use serde::Serializer;
    #[test]
    fn serialize_unit_variant_first() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_serialize_unit_variant_first = 0;
        let rug_fuzz_0 = "UnitVariant";
        let rug_fuzz_1 = 0;
        let rug_fuzz_2 = "First";
        let rug_fuzz_3 = "First";
        let mut output = String::new();
        let serializer = ValueSerializer::new(&mut output);
        serializer.serialize_unit_variant(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let expected = rug_fuzz_3;
        debug_assert!(output.contains(expected));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_serialize_unit_variant_first = 0;
    }
    #[test]
    fn serialize_unit_variant_second() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_serialize_unit_variant_second = 0;
        let rug_fuzz_0 = "UnitVariant";
        let rug_fuzz_1 = 1;
        let rug_fuzz_2 = "Second";
        let rug_fuzz_3 = "Second";
        let mut output = String::new();
        let serializer = ValueSerializer::new(&mut output);
        serializer.serialize_unit_variant(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2).unwrap();
        let expected = rug_fuzz_3;
        debug_assert!(output.contains(expected));
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_serialize_unit_variant_second = 0;
    }
    #[test]
    fn serialize_unit_variant_invalid_variant() {
        let _rug_st_tests_llm_16_122_llm_16_122_rrrruuuugggg_serialize_unit_variant_invalid_variant = 0;
        let rug_fuzz_0 = "UnitVariant";
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = "Third";
        let mut output = String::new();
        let serializer = ValueSerializer::new(&mut output);
        let result = serializer
            .serialize_unit_variant(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        debug_assert!(result.is_err());
        let _rug_ed_tests_llm_16_122_llm_16_122_rrrruuuugggg_serialize_unit_variant_invalid_variant = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_304 {
    use super::*;
    use crate::*;
    use std::fmt;
    #[test]
    fn test_key_not_string() {
        let _rug_st_tests_llm_16_304_rrrruuuugggg_test_key_not_string = 0;
        let error = ser::Error::key_not_string();
        match error.inner {
            crate::edit::ser::Error::KeyNotString => {}
            _ => panic!("key_not_string did not create the correct Error variant"),
        }
        let _rug_ed_tests_llm_16_304_rrrruuuugggg_test_key_not_string = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_306_llm_16_306 {
    use crate::ser::Error;
    use serde::ser::Error as SerError;
    use std::fmt::Write;
    #[test]
    fn test_unsupported_none() {
        let _rug_st_tests_llm_16_306_llm_16_306_rrrruuuugggg_test_unsupported_none = 0;
        let rug_fuzz_0 = "UnsupportedNone";
        let err = Error::unsupported_none();
        debug_assert!(format!("{:?}", err) .contains(rug_fuzz_0));
        let _rug_ed_tests_llm_16_306_llm_16_306_rrrruuuugggg_test_unsupported_none = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_307_llm_16_307 {
    use super::*;
    use crate::*;
    use std::error::Error as StdError;
    #[test]
    fn test_unsupported_type_with_none() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_with_none = 0;
        let error = Error::unsupported_type(None);
        debug_assert_eq!(error.to_string(), "unsupported type: none");
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_with_none = 0;
    }
    #[test]
    fn test_unsupported_type_with_some() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_with_some = 0;
        let rug_fuzz_0 = "special_type";
        let error = Error::unsupported_type(Some(rug_fuzz_0));
        debug_assert_eq!(error.to_string(), "unsupported type: special_type");
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_with_some = 0;
    }
    #[test]
    fn test_unsupported_type_implements_error_trait() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_error_trait = 0;
        let rug_fuzz_0 = "test";
        let error = Error::unsupported_type(Some(rug_fuzz_0));
        let error_trait: &dyn StdError = &error;
        debug_assert_eq!(error_trait.to_string(), "unsupported type: test");
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_error_trait = 0;
    }
    #[test]
    fn test_unsupported_type_implements_std_error() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_std_error = 0;
        let rug_fuzz_0 = "test";
        let error = Error::unsupported_type(Some(rug_fuzz_0));
        let source = error.source();
        debug_assert!(source.is_none());
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_std_error = 0;
    }
    #[test]
    fn test_unsupported_type_implements_display() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_display = 0;
        let rug_fuzz_0 = "test";
        let error = Error::unsupported_type(Some(rug_fuzz_0));
        let display = format!("{}", error);
        debug_assert_eq!(display, "unsupported type: test");
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_display = 0;
    }
    #[test]
    fn test_unsupported_type_implements_debug() {
        let _rug_st_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_debug = 0;
        let rug_fuzz_0 = "test";
        let rug_fuzz_1 = "UnsupportedType";
        let error = Error::unsupported_type(Some(rug_fuzz_0));
        let debug = format!("{:?}", error);
        debug_assert!(debug.contains(rug_fuzz_1));
        let _rug_ed_tests_llm_16_307_llm_16_307_rrrruuuugggg_test_unsupported_type_implements_debug = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_308 {
    use super::*;
    use crate::*;
    use crate::edit::ser::Error as EditSerError;
    #[test]
    fn test_wrap() {
        let _rug_st_tests_llm_16_308_rrrruuuugggg_test_wrap = 0;
        let rug_fuzz_0 = "inner error message";
        let edit_error = EditSerError::Custom(rug_fuzz_0.to_string());
        let ser_error = ser::Error::wrap(edit_error.clone());
        debug_assert!(matches!(ser_error.inner, EditSerError::Custom(_)));
        debug_assert_eq!(edit_error.to_string(), ser_error.inner.to_string());
        let _rug_ed_tests_llm_16_308_rrrruuuugggg_test_wrap = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_319_llm_16_319 {
    use super::*;
    use crate::*;
    use crate::Value;
    use serde::Serialize;
    use std::collections::BTreeMap;
    #[derive(Serialize)]
    struct Config {
        title: String,
        owner: Owner,
    }
    #[derive(Serialize)]
    struct Owner {
        name: String,
        dob: String,
    }
    #[test]
    fn test_to_string_pretty() {
        let _rug_st_tests_llm_16_319_llm_16_319_rrrruuuugggg_test_to_string_pretty = 0;
        let rug_fuzz_0 = "TOML Example";
        let rug_fuzz_1 = "Tom Preston-Werner";
        let rug_fuzz_2 = "1979-05-27T07:32:00Z";
        let rug_fuzz_3 = r#"
            title = "TOML Example"

            [owner]
            name = "Tom Preston-Werner"
            dob = "1979-05-27T07:32:00Z"
        "#;
        let config = Config {
            title: rug_fuzz_0.to_string(),
            owner: Owner {
                name: rug_fuzz_1.to_string(),
                dob: rug_fuzz_2.to_string(),
            },
        };
        let pretty_toml = to_string_pretty(&config).unwrap();
        let expected_toml = rug_fuzz_3.trim_start();
        debug_assert_eq!(pretty_toml.trim(), expected_toml);
        let _rug_ed_tests_llm_16_319_llm_16_319_rrrruuuugggg_test_to_string_pretty = 0;
    }
    #[test]
    #[should_panic(expected = "serialize error")]
    fn test_to_string_pretty_error() {
        let _rug_st_tests_llm_16_319_llm_16_319_rrrruuuugggg_test_to_string_pretty_error = 0;
        let val = InvalidValue;
        to_string_pretty(&val).unwrap();
        let _rug_ed_tests_llm_16_319_llm_16_319_rrrruuuugggg_test_to_string_pretty_error = 0;
    }
    struct InvalidValue;
    impl Serialize for InvalidValue {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::Error;
            Err(S::Error::custom("serialize error"))
        }
    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use serde::Serialize;
    use crate::Value;
    #[derive(Serialize)]
    struct Config {
        database: Database,
    }
    #[derive(Serialize)]
    struct Database {
        ip: String,
        port: Vec<u16>,
        connection_max: u32,
        enabled: bool,
    }
    #[test]
    fn test_to_string() {
        let _rug_st_tests_rug_15_rrrruuuugggg_test_to_string = 0;
        let rug_fuzz_0 = "192.168.1.1";
        let rug_fuzz_1 = 8001;
        let rug_fuzz_2 = 5000;
        let rug_fuzz_3 = false;
        let rug_fuzz_4 = 0;
        let config = Config {
            database: Database {
                ip: rug_fuzz_0.to_string(),
                port: vec![rug_fuzz_1, 8002, 8003],
                connection_max: rug_fuzz_2,
                enabled: rug_fuzz_3,
            },
        };
        let toml_string = crate::ser::to_string(&config).unwrap();
        debug_assert!(toml_string.len() > rug_fuzz_4);
        let _rug_ed_tests_rug_15_rrrruuuugggg_test_to_string = 0;
    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::fmt::DocumentFormatter;
    use toml_edit::{Document, Item, Value};
    use toml_edit::ser::Error as TomlEditError;
    use toml_edit::visit_mut::VisitMut as _;
    #[test]
    fn test_write_document() {
        let _rug_st_tests_rug_16_rrrruuuugggg_test_write_document = 0;
        let rug_fuzz_0 = true;
        let rug_fuzz_1 = 42;
        let mut p0: String = String::new();
        let mut p1 = DocumentFormatter {
            multiline_array: rug_fuzz_0,
        };
        let mut p2: Result<Value, TomlEditError> = Ok(Value::from(rug_fuzz_1));
        debug_assert!(crate ::ser::internal::write_document(& mut p0, p1, p2).is_ok());
        debug_assert_eq!(p0.trim(), "42");
        let _rug_ed_tests_rug_16_rrrruuuugggg_test_write_document = 0;
    }
}
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    use toml_edit::{Document, Value};
    use std::result::Result;
    #[test]
    fn test_write_value() {
        let _rug_st_tests_rug_17_rrrruuuugggg_test_write_value = 0;
        let rug_fuzz_0 = 42;
        let mut p0: std::string::String = String::new();
        let mut p1: Result<Value, toml_edit::ser::Error> = Ok(Value::from(rug_fuzz_0));
        let result = crate::ser::internal::write_value(&mut p0, p1);
        debug_assert!(result.is_ok());
        debug_assert_eq!(p0, "42");
        let _rug_ed_tests_rug_17_rrrruuuugggg_test_write_value = 0;
    }
}
#[cfg(test)]
mod tests_rug_19 {
    use crate::ser::Error;
    use crate::Value;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_19_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Sample string value";
        let mut p0: Value = Value::from(rug_fuzz_0);
        <Error as serde::ser::Error>::custom(p0);
        let _rug_ed_tests_rug_19_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    #[test]
    fn test_new() {
        let _rug_st_tests_rug_20_rrrruuuugggg_test_new = 0;
        let rug_fuzz_0 = "initial value";
        let mut p0: String = String::from(rug_fuzz_0);
        let _result = crate::ser::Serializer::new(&mut p0);
        let _rug_ed_tests_rug_20_rrrruuuugggg_test_new = 0;
    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use std::string::String;
    #[test]
    fn test_pretty() {
        let _rug_st_tests_rug_21_rrrruuuugggg_test_pretty = 0;
        let rug_fuzz_0 = "title = 'TOML Example'";
        let mut p0: String = String::from(rug_fuzz_0);
        let serializer = crate::ser::Serializer::<'_>::pretty(&mut p0);
        let _rug_ed_tests_rug_21_rrrruuuugggg_test_pretty = 0;
    }
}
#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    #[test]
    fn test_new_serializer() {
        let _rug_st_tests_rug_37_rrrruuuugggg_test_new_serializer = 0;
        let rug_fuzz_0 = "initial value";
        let mut p0: String = String::from(rug_fuzz_0);
        let serializer = crate::ser::ValueSerializer::new(&mut p0);
        let _rug_ed_tests_rug_37_rrrruuuugggg_test_new_serializer = 0;
    }
}
#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use crate::ser::ValueSerializer;
    use serde::Serializer;
    use std::fmt::Write;
    #[test]
    fn test_serialize_bool() {
        let _rug_st_tests_rug_38_rrrruuuugggg_test_serialize_bool = 0;
        let rug_fuzz_0 = true;
        let mut buf = String::new();
        let mut p0 = ValueSerializer::new(&mut buf);
        let p1: bool = rug_fuzz_0;
        p0.serialize_bool(p1).unwrap();
        debug_assert_eq!(buf, "true");
        let _rug_ed_tests_rug_38_rrrruuuugggg_test_serialize_bool = 0;
    }
}

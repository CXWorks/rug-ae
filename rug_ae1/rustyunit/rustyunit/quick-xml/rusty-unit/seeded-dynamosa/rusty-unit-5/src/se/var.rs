use crate::{
    de::{INNER_VALUE, UNFLATTEN_PREFIX},
    errors::{serialize::DeError, Error},
    events::{BytesEnd, BytesStart, Event},
    se::Serializer,
    writer::Writer,
};
use serde::ser::{self, Serialize};
use serde::Serializer as _;
use std::io::Write;

/// An implementation of `SerializeMap` for serializing to XML.
pub struct Map<'r, 'w, W>
where
    W: 'w + Write,
{
    parent: &'w mut Serializer<'r, W>,
}

impl<'r, 'w, W> Map<'r, 'w, W>
where
    W: 'w + Write,
{
    /// Create a new Map
    pub fn new(parent: &'w mut Serializer<'r, W>) -> Self {
        Map { parent }
    }
}

impl<'r, 'w, W> ser::SerializeMap for Map<'r, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), DeError> {
        /*
        Err(DeError::Unsupported(
            "impossible to serialize the key on its own, please use serialize_entry()",
        ))
        */
        write!(self.parent.writer.inner(), "<enum key=\"").map_err(Error::Io)?;
        key.serialize(&mut *self.parent)?;
        write!(self.parent.writer.inner(), "\"/>").map_err(Error::Io)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), DeError> {
        value.serialize(&mut *self.parent)
    }

    fn end(self) -> Result<Self::Ok, DeError> {
        if let Some(tag) = self.parent.root_tag {
            self.parent
                .writer
                .write_event(Event::End(BytesEnd::borrowed(tag.as_bytes())))?;
        }
        Ok(())
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), DeError> {
        // TODO: Is it possible to ensure our key is never a composite type?
        // Anything which isn't a "primitive" would lead to malformed XML here...
        write!(self.parent.writer.inner(), "<").map_err(Error::Io)?;
        key.serialize(&mut *self.parent)?;
        write!(self.parent.writer.inner(), ">").map_err(Error::Io)?;

        value.serialize(&mut *self.parent)?;

        write!(self.parent.writer.inner(), "</").map_err(Error::Io)?;
        key.serialize(&mut *self.parent)?;
        write!(self.parent.writer.inner(), ">").map_err(Error::Io)?;
        Ok(())
    }
}

/// An implementation of `SerializeStruct` for serializing to XML.
pub struct Struct<'r, 'w, W>
where
    W: 'w + Write,
{
    parent: &'w mut Serializer<'r, W>,
    /// Buffer for holding fields, serialized as attributes. Doesn't allocate
    /// if there are no fields represented as attributes
    attrs: BytesStart<'w>,
    /// Buffer for holding fields, serialized as elements
    children: Vec<u8>,
    /// Buffer for serializing one field. Cleared after serialize each field
    buffer: Vec<u8>,
}

impl<'r, 'w, W> Struct<'r, 'w, W>
where
    W: 'w + Write,
{
    /// Create a new `Struct`
    pub fn new(parent: &'w mut Serializer<'r, W>, name: &'r str) -> Self {
        let name = name.as_bytes();
        Struct {
            parent,
            attrs: BytesStart::borrowed_name(name),
            children: Vec::new(),
            buffer: Vec::new(),
        }
    }
}

impl<'r, 'w, W> ser::SerializeStruct for Struct<'r, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), DeError> {
        // TODO: Inherit indentation state from self.parent.writer
        let writer = Writer::new(&mut self.buffer);
        if key.starts_with(UNFLATTEN_PREFIX) {
            let key = &key[UNFLATTEN_PREFIX.len()..];
            let mut serializer = Serializer::with_root(writer, Some(key));
            serializer.serialize_newtype_struct(key, value)?;
            self.children.append(&mut self.buffer);
        } else {
            let mut serializer = Serializer::with_root(writer, Some(key));
            value.serialize(&mut serializer)?;

            if !self.buffer.is_empty() {
                if self.buffer[0] == b'<' || key == INNER_VALUE {
                    // Drains buffer, moves it to children
                    self.children.append(&mut self.buffer);
                } else {
                    self.attrs
                        .push_attribute((key.as_bytes(), self.buffer.as_ref()));
                    self.buffer.clear();
                }
            }
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, DeError> {
        if self.children.is_empty() {
            self.parent.writer.write_event(Event::Empty(self.attrs))?;
        } else {
            self.parent
                .writer
                .write_event(Event::Start(self.attrs.to_borrowed()))?;
            self.parent.writer.write(&self.children)?;
            self.parent
                .writer
                .write_event(Event::End(self.attrs.to_end()))?;
        }
        Ok(())
    }
}

impl<'r, 'w, W> ser::SerializeStructVariant for Struct<'r, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;


    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        <Self as ser::SerializeStruct>::serialize_field(self, key, value)
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeStruct>::end(self)
    }
}

/// An implementation of `SerializeSeq' for serializing to XML.
pub struct Seq<'r, 'w, W>
where
    W: 'w + Write,
{
    parent: &'w mut Serializer<'r, W>,
}

impl<'r, 'w, W> Seq<'r, 'w, W>
where
    W: 'w + Write,
{
    /// Create a new `Seq`
    pub fn new(parent: &'w mut Serializer<'r, W>) -> Self {
        Seq { parent }
    }
}

impl<'r, 'w, W> ser::SerializeSeq for Seq<'r, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.parent)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// An implementation of `SerializeTuple`, `SerializeTupleStruct` and
/// `SerializeTupleVariant` for serializing to XML.
pub struct Tuple<'r, 'w, W>
where
    W: 'w + Write,
{
    parent: &'w mut Serializer<'r, W>,
    /// Possible qualified name of XML tag surrounding each element
    name: &'r str,
}

impl<'r, 'w, W> Tuple<'r, 'w, W>
where
    W: 'w + Write,
{
    /// Create a new `Tuple`
    pub fn new(parent: &'w mut Serializer<'r, W>, name: &'r str) -> Self {
        Tuple { parent, name }
    }
}

impl<'r, 'w, W> ser::SerializeTuple for Tuple<'r, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        write!(self.parent.writer.inner(), "<{}>", self.name).map_err(Error::Io)?;
        value.serialize(&mut *self.parent)?;
        write!(self.parent.writer.inner(), "</{}>", self.name).map_err(Error::Io)?;
        Ok(())
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'r, 'w, W> ser::SerializeTupleStruct for Tuple<'r, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;


    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        <Self as ser::SerializeTuple>::serialize_element(self, value)
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

impl<'r, 'w, W> ser::SerializeTupleVariant for Tuple<'r, 'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = DeError;


    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        <Self as ser::SerializeTuple>::serialize_element(self, value)
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeTuple>::end(self)
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_315() {
//    rusty_monitor::set_test_id(315);
    let mut str_0: &str = "PI";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "UnexpectedEof";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "Attr::Empty";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "XmlDeclWithoutVersion";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "bindings";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "UnexpectedEof";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "UnrecognizedSymbol";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "Element";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "keys";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "ExpectedEq";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "keys";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut error_0: errors::Error = crate::errors::Error::UnexpectedEof(string_10);
    let mut error_1: errors::Error = crate::errors::Error::UnexpectedEof(string_9);
    let mut error_2: errors::Error = crate::errors::Error::UnexpectedEof(string_8);
    let mut error_3: errors::Error = crate::errors::Error::UnexpectedEof(string_7);
    let mut error_4: errors::Error = crate::errors::Error::UnexpectedEof(string_6);
    let mut error_5: errors::Error = crate::errors::Error::UnexpectedEof(string_5);
    let mut error_6: errors::Error = crate::errors::Error::UnexpectedEof(string_4);
    let mut error_7: errors::Error = crate::errors::Error::UnexpectedEof(string_3);
    let mut error_8: errors::Error = crate::errors::Error::UnexpectedEof(string_2);
    let mut error_9: errors::Error = crate::errors::Error::UnexpectedEof(string_1);
    let mut error_10: errors::Error = crate::errors::Error::UnexpectedEof(string_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_518() {
//    rusty_monitor::set_test_id(518);
    let mut str_0: &str = "bytes";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "UnexpectedToken";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "Start";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "SPXRPqyGa";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "<";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "state";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "element";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut str_7: &str = "SkipValue";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut str_8: &str = "UnexpectedBang";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut str_9: &str = "DOCTYPE";
    let mut str_9_ref_0: &str = &mut str_9;
    let mut str_10: &str = "UnrecognizedSymbol";
    let mut str_10_ref_0: &str = &mut str_10;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_10_ref_0);
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_9_ref_0);
    let mut reader_2: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_8_ref_0);
    let mut reader_3: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_7_ref_0);
    let mut reader_4: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_6_ref_0);
    let mut reader_5: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_5_ref_0);
    let mut reader_6: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_4_ref_0);
    let mut reader_7: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_3_ref_0);
    let mut reader_8: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_2_ref_0);
    let mut reader_9: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    let mut reader_10: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_523() {
//    rusty_monitor::set_test_id(523);
    let mut str_0: &str = "qQpomBKQAVt2MjrmER";
    let mut string_0: std::string::String = std::string::String::from(str_0);
    let mut str_1: &str = "HsB5Au";
    let mut string_1: std::string::String = std::string::String::from(str_1);
    let mut str_2: &str = "InvalidHexadecimal";
    let mut string_2: std::string::String = std::string::String::from(str_2);
    let mut str_3: &str = "InvalidHexadecimal";
    let mut string_3: std::string::String = std::string::String::from(str_3);
    let mut str_4: &str = "Zxs0Vt";
    let mut string_4: std::string::String = std::string::String::from(str_4);
    let mut str_5: &str = "html";
    let mut string_5: std::string::String = std::string::String::from(str_5);
    let mut str_6: &str = "";
    let mut string_6: std::string::String = std::string::String::from(str_6);
    let mut str_7: &str = "PI";
    let mut string_7: std::string::String = std::string::String::from(str_7);
    let mut str_8: &str = "check_duplicates";
    let mut string_8: std::string::String = std::string::String::from(str_8);
    let mut str_9: &str = "bindings";
    let mut string_9: std::string::String = std::string::String::from(str_9);
    let mut str_10: &str = "InvalidHexadecimal";
    let mut string_10: std::string::String = std::string::String::from(str_10);
    let mut str_11: &str = "expected";
    let mut string_11: std::string::String = std::string::String::from(str_11);
    let mut str_12: &str = "XmlDecl";
    let mut string_12: std::string::String = std::string::String::from(str_12);
    let mut str_13: &str = "value_len";
    let mut string_13: std::string::String = std::string::String::from(str_13);
    let mut error_0: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_13, found: string_12};
    let mut error_1: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_11, found: string_10};
    let mut error_2: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_9, found: string_8};
    let mut error_3: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_7, found: string_6};
    let mut error_4: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_5, found: string_4};
    let mut error_5: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_3, found: string_2};
    let mut error_6: errors::Error = crate::errors::Error::EndEventMismatch {expected: string_1, found: string_0};
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_260() {
//    rusty_monitor::set_test_id(260);
    let mut usize_0: usize = 1usize;
    let mut usize_1: usize = 1usize;
    let mut usize_2: usize = 9305usize;
    let mut usize_3: usize = 1usize;
    let mut usize_4: usize = 8usize;
    let mut usize_5: usize = 8usize;
    let mut usize_6: usize = 8usize;
    let mut usize_7: usize = 5usize;
    let mut usize_8: usize = 6usize;
    let mut usize_9: usize = 8usize;
    let mut usize_10: usize = 7usize;
    let mut usize_11: usize = 2803usize;
    let mut usize_12: usize = 5usize;
    let mut usize_13: usize = 8921usize;
    let mut usize_14: usize = 0usize;
    let mut usize_15: usize = 867usize;
    let mut attrerror_0: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_15);
    let mut attrerror_1: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_14);
    let mut attrerror_2: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_13);
    let mut attrerror_3: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_12);
    let mut attrerror_4: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_11);
    let mut attrerror_5: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_10);
    let mut attrerror_6: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_9);
    let mut attrerror_7: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_8);
    let mut attrerror_8: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_7);
    let mut attrerror_9: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_6);
    let mut attrerror_10: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_5);
    let mut attrerror_11: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_4);
    let mut attrerror_12: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_3);
    let mut attrerror_13: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_2);
    let mut attrerror_14: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_1);
    let mut attrerror_15: events::attributes::AttrError = crate::events::attributes::AttrError::UnquotedValue(usize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_450() {
//    rusty_monitor::set_test_id(450);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_1: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_2: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_3: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_4: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_5: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_6: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_7: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_8: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_9: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_10: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_11: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_12: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_13: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_14: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_15: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_16: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_17: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_18: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_19: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_20: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_21: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_22: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_23: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_24: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_25: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_26: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_27: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_28: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
    let mut escapeerror_29: escapei::EscapeError = crate::escapei::EscapeError::TooLongHexadecimal;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_227() {
//    rusty_monitor::set_test_id(227);
    let mut isize_0: isize = 6isize;
    let mut isize_1: isize = 6447isize;
    let mut isize_2: isize = 3isize;
    let mut isize_3: isize = -11691isize;
    let mut isize_4: isize = -9527isize;
    let mut isize_5: isize = 14352isize;
    let mut isize_6: isize = 13202isize;
    let mut isize_7: isize = -7127isize;
    let mut isize_8: isize = -19013isize;
    let mut isize_9: isize = 3isize;
    let mut isize_10: isize = 0isize;
    let mut isize_11: isize = -6002isize;
    let mut isize_12: isize = 9isize;
    let mut isize_13: isize = 3949isize;
    let mut isize_14: isize = 9isize;
    let mut isize_15: isize = 6isize;
    let mut isize_16: isize = -6063isize;
    let mut isize_17: isize = 6isize;
    let mut isize_18: isize = 9isize;
    let mut isize_19: isize = 7isize;
    let mut isize_20: isize = 5isize;
    let mut isize_21: isize = 5isize;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_21, isize_20);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_19, isize_18);
    let mut attr_2: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_17, isize_16);
    let mut attr_3: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_15, isize_14);
    let mut attr_4: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_13, isize_12);
    let mut attr_5: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_11, isize_10);
    let mut attr_6: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_9, isize_8);
    let mut attr_7: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_7, isize_6);
    let mut attr_8: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_5, isize_4);
    let mut attr_9: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_3, isize_2);
    let mut attr_10: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_1, isize_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_344() {
//    rusty_monitor::set_test_id(344);
    let mut str_0: &str = "Empty";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_0_ref_0);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    let mut str_1: &str = "Me2PUIYhJK";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut reader_1: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_1_ref_0);
    let mut reader_1_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_1;
    let mut str_2: &str = "Zp6XmCHq20";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut reader_2: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_2_ref_0);
    let mut reader_2_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_2;
    let mut str_3: &str = "W6Bze";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut reader_3: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_3_ref_0);
    let mut reader_3_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_3;
    let mut str_4: &str = "TooLongHexadecimal";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut reader_4: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_4_ref_0);
    let mut reader_4_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_4;
    let mut str_5: &str = "Attributes";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut reader_5: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_5_ref_0);
    let mut reader_5_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_5;
    let mut str_6: &str = "SfYQ";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut reader_6: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_6_ref_0);
    let mut reader_6_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_6;
    crate::reader::Reader::read_event_unbuffered(reader_6_ref_0);
    crate::reader::Reader::read_event_unbuffered(reader_5_ref_0);
    crate::reader::Reader::read_event_unbuffered(reader_4_ref_0);
    crate::reader::Reader::read_event_unbuffered(reader_3_ref_0);
    crate::reader::Reader::read_event_unbuffered(reader_2_ref_0);
    crate::reader::Reader::read_event_unbuffered(reader_1_ref_0);
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_529() {
//    rusty_monitor::set_test_id(529);
    let mut isize_0: isize = 8isize;
    let mut isize_1: isize = 1isize;
    let mut isize_2: isize = 0isize;
    let mut isize_3: isize = -10195isize;
    let mut isize_4: isize = 8isize;
    let mut isize_5: isize = 5isize;
    let mut isize_6: isize = 7isize;
    let mut isize_7: isize = 4isize;
    let mut isize_8: isize = 2053isize;
    let mut isize_9: isize = 7isize;
    let mut isize_10: isize = 9isize;
    let mut isize_11: isize = -6304isize;
    let mut isize_12: isize = 9197isize;
    let mut isize_13: isize = 3isize;
    let mut isize_14: isize = -711isize;
    let mut isize_15: isize = 6isize;
    let mut isize_16: isize = 3isize;
    let mut isize_17: isize = -12722isize;
    let mut isize_18: isize = 6isize;
    let mut isize_19: isize = 13709isize;
    let mut isize_20: isize = 30453isize;
    let mut isize_21: isize = 9isize;
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_21, isize_20);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_19, isize_18);
    let mut attr_2: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_17, isize_16);
    let mut attr_3: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_15, isize_14);
    let mut attr_4: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_13, isize_12);
    let mut attr_5: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_11, isize_10);
    let mut attr_6: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_9, isize_8);
    let mut attr_7: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_7, isize_6);
    let mut attr_8: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_5, isize_4);
    let mut attr_9: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_3, isize_2);
    let mut attr_10: events::attributes::Attr<isize> = crate::events::attributes::Attr::Unquoted(isize_1, isize_0);
//    panic!("From RustyUnit with love");
}
}
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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4881() {
    rusty_monitor::set_test_id(4881);
    let mut u32_0: u32 = 9669u32;
    let mut isize_0: isize = -4535isize;
    let mut isize_1: isize = -9885isize;
    let mut str_0: &str = "yplNfM1S";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut bytescdata_0: crate::events::BytesCData = crate::events::BytesCData::from_str(str_0_ref_0);
    let mut cow_0: std::borrow::Cow<[u8]> = crate::events::BytesCData::into_inner(bytescdata_0);
    let mut str_1: &str = "ZwNRJHPDhP4lIk4ft";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut bytescdata_1: crate::events::BytesCData = crate::events::BytesCData::from_str(str_1_ref_0);
    let mut event_0: events::Event = crate::events::Event::CData(bytescdata_1);
    let mut usize_0: usize = 2622usize;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut bytescdata_2: crate::events::BytesCData = crate::events::BytesCData::from_str(str_2_ref_0);
    let mut bytestext_0: crate::events::BytesText = crate::events::BytesCData::partial_escape(bytescdata_2);
    let mut str_3: &str = "vV7swT1SFqyCLQcTA";
    let mut string_0: std::string::String = std::string::String::from(str_3);
    let mut result_0: std::result::Result<std::string::String, std::string::FromUtf8Error> = std::result::Result::Ok(string_0);
    let mut isize_2: isize = 15647isize;
    let mut isize_3: isize = 2254isize;
    let mut isize_4: isize = -1260isize;
    let mut isize_5: isize = -3756isize;
    let mut str_4: &str = "7";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut reader_0: crate::reader::Reader<&[u8]> = crate::reader::Reader::from_str(str_4_ref_0);
    let mut attr_0: events::attributes::Attr<isize> = crate::events::attributes::Attr::DoubleQ(isize_5, isize_4);
    let mut attr_1: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_3, isize_2);
    let mut reader_0_ref_0: &mut crate::reader::Reader<&[u8]> = &mut reader_0;
    crate::reader::Reader::read_event_unbuffered(reader_0_ref_0);
    let mut event_1: events::Event = crate::events::Event::PI(bytestext_0);
    let mut event_2: events::Event = crate::events::Event::into_owned(event_0);
    let mut attr_2: events::attributes::Attr<isize> = crate::events::attributes::Attr::SingleQ(isize_1, isize_0);
    let mut event_3: events::Event = crate::events::Event::into_owned(event_2);
    let mut event_4: events::Event = crate::events::Event::into_owned(event_1);
    let mut escapeerror_0: escapei::EscapeError = crate::escapei::EscapeError::InvalidCodepoint(u32_0);
    panic!("From RustyUnit with love");
}
}
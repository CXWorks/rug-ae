use crate::{
    de::{read::Reader, BorrowDecoder, Decode, Decoder},
    enc::{
        self, write::{SizeWriter, Writer},
        Encode, Encoder,
    },
    error::{DecodeError, EncodeError},
    impl_borrow_decode, BorrowDecode, Config,
};
#[cfg(target_has_atomic = "ptr")]
use alloc::sync::Arc;
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box, collections::*, rc::Rc, string::String, vec::Vec,
};
#[derive(Default)]
pub(crate) struct VecWriter {
    inner: Vec<u8>,
}
impl VecWriter {
    /// Create a new vec writer with the given capacity
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
        }
    }
    #[allow(dead_code)]
    pub(crate) fn collect(self) -> Vec<u8> {
        self.inner
    }
}
impl enc::write::Writer for VecWriter {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.inner.extend_from_slice(bytes);
        Ok(())
    }
}
/// Encode the given value into a `Vec<u8>` with the given `Config`. See the [config] module for more information.
///
/// [config]: config/index.html
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn encode_to_vec<E: enc::Encode, C: Config>(
    val: E,
    config: C,
) -> Result<Vec<u8>, EncodeError> {
    let size = {
        let mut size_writer = enc::EncoderImpl::<
            _,
            C,
        >::new(SizeWriter::default(), config);
        val.encode(&mut size_writer)?;
        size_writer.into_writer().bytes_written
    };
    let writer = VecWriter::with_capacity(size);
    let mut encoder = enc::EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(encoder.into_writer().inner)
}
impl<T> Decode for BinaryHeap<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut map = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::decode(decoder)?;
            map.push(key);
        }
        Ok(map)
    }
}
impl<'de, T> BorrowDecode<'de> for BinaryHeap<T>
where
    T: BorrowDecode<'de> + Ord,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut map = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::borrow_decode(decoder)?;
            map.push(key);
        }
        Ok(map)
    }
}
impl<T> Encode for BinaryHeap<T>
where
    T: Encode + Ord,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for val in self.iter() {
            val.encode(encoder)?;
        }
        Ok(())
    }
}
impl<K, V> Decode for BTreeMap<K, V>
where
    K: Decode + Ord,
    V: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());
            let key = K::decode(decoder)?;
            let value = V::decode(decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}
impl<'de, K, V> BorrowDecode<'de> for BTreeMap<K, V>
where
    K: BorrowDecode<'de> + Ord,
    V: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<(K, V)>(len)?;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());
            let key = K::borrow_decode(decoder)?;
            let value = V::borrow_decode(decoder)?;
            map.insert(key, value);
        }
        Ok(map)
    }
}
impl<K, V> Encode for BTreeMap<K, V>
where
    K: Encode + Ord,
    V: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for (key, val) in self.iter() {
            key.encode(encoder)?;
            val.encode(encoder)?;
        }
        Ok(())
    }
}
impl<T> Decode for BTreeSet<T>
where
    T: Decode + Ord,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut map = BTreeSet::new();
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}
impl<'de, T> BorrowDecode<'de> for BTreeSet<T>
where
    T: BorrowDecode<'de> + Ord,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut map = BTreeSet::new();
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::borrow_decode(decoder)?;
            map.insert(key);
        }
        Ok(map)
    }
}
impl<T> Encode for BTreeSet<T>
where
    T: Encode + Ord,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
impl<T> Decode for VecDeque<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut map = VecDeque::with_capacity(len);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::decode(decoder)?;
            map.push_back(key);
        }
        Ok(map)
    }
}
impl<'de, T> BorrowDecode<'de> for VecDeque<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut map = VecDeque::with_capacity(len);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            let key = T::borrow_decode(decoder)?;
            map.push_back(key);
        }
        Ok(map)
    }
}
impl<T> Encode for VecDeque<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
impl<T> Decode for Vec<T>
where
    T: Decode + 'static,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        if core::any::TypeId::of::<T>() == core::any::TypeId::of::<u8>() {
            decoder.claim_container_read::<T>(len)?;
            let mut vec = Vec::new();
            vec.resize(len, 0u8);
            decoder.reader().read(&mut vec)?;
            return Ok(unsafe { core::mem::transmute(vec) });
        }
        decoder.claim_container_read::<T>(len)?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            vec.push(T::decode(decoder)?);
        }
        Ok(vec)
    }
}
impl<'de, T> BorrowDecode<'de> for Vec<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            decoder.unclaim_bytes_read(core::mem::size_of::<T>());
            vec.push(T::borrow_decode(decoder)?);
        }
        Ok(vec)
    }
}
impl<T> Encode for Vec<T>
where
    T: Encode + 'static,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        crate::enc::encode_slice_len(encoder, self.len())?;
        if core::any::TypeId::of::<T>() == core::any::TypeId::of::<u8>() {
            let slice: &[u8] = unsafe { core::mem::transmute(self.as_slice()) };
            encoder.writer().write(slice)?;
            return Ok(());
        }
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}
impl Decode for String {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let bytes = Vec::<u8>::decode(decoder)?;
        String::from_utf8(bytes)
            .map_err(|e| DecodeError::Utf8 {
                inner: e.utf8_error(),
            })
    }
}
impl_borrow_decode!(String);
impl Decode for Box<str> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        String::decode(decoder).map(String::into_boxed_str)
    }
}
impl_borrow_decode!(Box < str >);
impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}
impl<T> Decode for Box<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Box::new(t))
    }
}
impl<'de, T> BorrowDecode<'de> for Box<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Box::new(t))
    }
}
impl<T> Encode for Box<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
impl<T> Decode for Box<[T]>
where
    T: Decode + 'static,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}
impl<'de, T> BorrowDecode<'de> for Box<[T]>
where
    T: BorrowDecode<'de> + 'de,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let vec = Vec::borrow_decode(decoder)?;
        Ok(vec.into_boxed_slice())
    }
}
impl<'cow, T> Decode for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = <T as ToOwned>::Owned::decode(decoder)?;
        Ok(Cow::Owned(t))
    }
}
impl<'cow, T> BorrowDecode<'cow> for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    &'cow T: BorrowDecode<'cow>,
{
    fn borrow_decode<D: BorrowDecoder<'cow>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = <&T>::borrow_decode(decoder)?;
        Ok(Cow::Borrowed(t))
    }
}
impl<'cow, T> Encode for Cow<'cow, T>
where
    T: ToOwned + ?Sized,
    for<'a> &'a T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_ref().encode(encoder)
    }
}
#[test]
fn test_cow_round_trip() {
    let start = Cow::Borrowed("Foo");
    let encoded = crate::encode_to_vec(&start, crate::config::standard()).unwrap();
    let (end, _) = crate::borrow_decode_from_slice::<
        Cow<str>,
        _,
    >(&encoded, crate::config::standard())
        .unwrap();
    assert_eq!(start, end);
    let (end, _) = crate::decode_from_slice::<
        Cow<str>,
        _,
    >(&encoded, crate::config::standard())
        .unwrap();
    assert_eq!(start, end);
}
impl<T> Decode for Rc<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Rc::new(t))
    }
}
impl<'de, T> BorrowDecode<'de> for Rc<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Rc::new(t))
    }
}
impl<T> Encode for Rc<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
impl<T> Decode for Rc<[T]>
where
    T: Decode + 'static,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::decode(decoder)?;
        Ok(vec.into())
    }
}
impl<'de, T> BorrowDecode<'de> for Rc<[T]>
where
    T: BorrowDecode<'de> + 'de,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let vec = Vec::borrow_decode(decoder)?;
        Ok(vec.into())
    }
}
#[cfg(target_has_atomic = "ptr")]
impl<T> Decode for Arc<T>
where
    T: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Arc::new(t))
    }
}
#[cfg(target_has_atomic = "ptr")]
impl Decode for Arc<str> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let decoded = String::decode(decoder)?;
        Ok(decoded.into())
    }
}
#[cfg(target_has_atomic = "ptr")]
impl<'de, T> BorrowDecode<'de> for Arc<T>
where
    T: BorrowDecode<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Arc::new(t))
    }
}
#[cfg(target_has_atomic = "ptr")]
impl<'de> BorrowDecode<'de> for Arc<str> {
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let decoded = String::decode(decoder)?;
        Ok(decoded.into())
    }
}
#[cfg(target_has_atomic = "ptr")]
impl<T> Encode for Arc<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}
#[cfg(target_has_atomic = "ptr")]
impl<T> Decode for Arc<[T]>
where
    T: Decode + 'static,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let vec = Vec::decode(decoder)?;
        Ok(vec.into())
    }
}
#[cfg(target_has_atomic = "ptr")]
impl<'de, T> BorrowDecode<'de> for Arc<[T]>
where
    T: BorrowDecode<'de> + 'de,
{
    fn borrow_decode<D: BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let vec = Vec::borrow_decode(decoder)?;
        Ok(vec.into())
    }
}
#[cfg(test)]
mod tests_llm_16_27 {
    use super::*;
    use crate::*;
    use crate::enc::write::Writer;
    #[test]
    fn vec_writer_writes_bytes_correctly() {
        let _rug_st_tests_llm_16_27_rrrruuuugggg_vec_writer_writes_bytes_correctly = 0;
        let rug_fuzz_0 = b"test data";
        let mut writer = VecWriter::default();
        let data = rug_fuzz_0;
        writer.write(data).unwrap();
        debug_assert_eq!(writer.inner, data);
        let _rug_ed_tests_llm_16_27_rrrruuuugggg_vec_writer_writes_bytes_correctly = 0;
    }
    #[test]
    fn vec_writer_with_capacity_writes_bytes_correctly() {
        let _rug_st_tests_llm_16_27_rrrruuuugggg_vec_writer_with_capacity_writes_bytes_correctly = 0;
        let rug_fuzz_0 = 100;
        let rug_fuzz_1 = b"test data with capacity";
        let mut writer = VecWriter::with_capacity(rug_fuzz_0);
        let data = rug_fuzz_1;
        writer.write(data).unwrap();
        debug_assert_eq!(writer.inner, data);
        let _rug_ed_tests_llm_16_27_rrrruuuugggg_vec_writer_with_capacity_writes_bytes_correctly = 0;
    }
    #[test]
    fn vec_writer_collect_returns_inner_vec() {
        let _rug_st_tests_llm_16_27_rrrruuuugggg_vec_writer_collect_returns_inner_vec = 0;
        let rug_fuzz_0 = 10;
        let rug_fuzz_1 = b"collect test";
        let mut writer = VecWriter::with_capacity(rug_fuzz_0);
        let data = rug_fuzz_1;
        writer.write(data).unwrap();
        let collected = writer.collect();
        debug_assert_eq!(collected, data);
        let _rug_ed_tests_llm_16_27_rrrruuuugggg_vec_writer_collect_returns_inner_vec = 0;
    }
    #[test]
    fn vec_writer_handles_empty_slice() {
        let _rug_st_tests_llm_16_27_rrrruuuugggg_vec_writer_handles_empty_slice = 0;
        let mut writer = VecWriter::default();
        writer.write(&[]).unwrap();
        debug_assert!(writer.inner.is_empty());
        let _rug_ed_tests_llm_16_27_rrrruuuugggg_vec_writer_handles_empty_slice = 0;
    }
    #[test]
    fn vec_writer_appends_data_correctly() {
        let _rug_st_tests_llm_16_27_rrrruuuugggg_vec_writer_appends_data_correctly = 0;
        let rug_fuzz_0 = b"first";
        let rug_fuzz_1 = b"second";
        let mut writer = VecWriter::default();
        let data1 = rug_fuzz_0;
        let data2 = rug_fuzz_1;
        writer.write(data1).unwrap();
        writer.write(data2).unwrap();
        debug_assert_eq!(writer.inner, b"firstsecond");
        let _rug_ed_tests_llm_16_27_rrrruuuugggg_vec_writer_appends_data_correctly = 0;
    }
}
#[cfg(test)]
mod tests_llm_16_294 {
    use super::*;
    use crate::*;
    use std::sync::Arc;
    use crate::de::{Decode, Decoder, DecoderImpl};
    use crate::error::DecodeError;
    use crate::de::read::SliceReader;
    use crate::config::{
        Config, Configuration, InternalIntEncodingConfig, InternalEndianConfig,
        InternalLimitConfig, BigEndian,
    };
    use crate::config::{LittleEndian, Fixint, Varint, Limit, NoLimit};
    use std::marker::PhantomData;
    #[derive(Debug, PartialEq)]
    struct TestStruct(u32);
    impl Decode for TestStruct {
        fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
            let value = u32::decode(decoder)?;
            Ok(TestStruct(value))
        }
    }
    #[test]
    fn decode_arc_test_struct() -> Result<(), DecodeError> {
        let data = [0, 0, 0, 5];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, NoLimit>::default();
        let mut decoder = DecoderImpl::new(&mut reader, config);
        let result = Arc::<TestStruct>::decode(&mut decoder)?;
        let expected = Arc::new(TestStruct(5));
        assert_eq!(result, expected);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_307 {
    use super::*;
    use crate::*;
    use crate::enc::{Encode, Encoder, EncoderImpl};
    use crate::enc::write::{SizeWriter, Writer};
    use crate::config::{
        BigEndian, Configuration, LittleEndian, Varint, NoLimit, InternalEndianConfig,
        InternalIntEncodingConfig, InternalLimitConfig,
    };
    use crate::error::EncodeError;
    use std::marker::PhantomData;
    use std::vec::Vec;
    #[test]
    fn test_encode_vec_of_u8() -> Result<(), EncodeError> {
        let data = vec![1_u8, 2, 3, 4, 5];
        let expected_bytes_written = data.len() + 8;
        let config = Configuration::<LittleEndian, Varint, NoLimit>::default();
        let writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(writer, config);
        data.encode(&mut encoder)?;
        assert_eq!(encoder.into_writer().bytes_written, expected_bytes_written);
        Ok(())
    }
    #[test]
    fn test_encode_vec_of_non_u8() -> Result<(), EncodeError> {
        let data = vec![1_u32, 2, 3, 4, 5];
        let single_element_encoded_size = 4;
        let expected_bytes_written = data.len() * single_element_encoded_size + 8;
        let config = Configuration::<LittleEndian, Varint, NoLimit>::default();
        let writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(writer, config);
        data.encode(&mut encoder)?;
        assert_eq!(encoder.into_writer().bytes_written, expected_bytes_written);
        Ok(())
    }
    fn generate_big_endian_config() -> Configuration<BigEndian, Varint, NoLimit> {
        let config = Configuration::<LittleEndian, Varint, NoLimit>::default()
            .with_big_endian();
        config
    }
    #[test]
    fn test_encode_vec_of_u8_big_endian() -> Result<(), EncodeError> {
        let data = vec![1_u8, 2, 3, 4, 5];
        let expected_bytes_written = data.len() + 8;
        let config = generate_big_endian_config();
        let writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(writer, config);
        data.encode(&mut encoder)?;
        assert_eq!(encoder.into_writer().bytes_written, expected_bytes_written);
        Ok(())
    }
    #[test]
    fn test_encode_vec_of_non_u8_big_endian() -> Result<(), EncodeError> {
        let data = vec![1_u32, 2, 3, 4, 5];
        let single_element_encoded_size = 4;
        let expected_bytes_written = data.len() * single_element_encoded_size + 8;
        let config = generate_big_endian_config();
        let writer = SizeWriter::default();
        let mut encoder = EncoderImpl::new(writer, config);
        data.encode(&mut encoder)?;
        assert_eq!(encoder.into_writer().bytes_written, expected_bytes_written);
        Ok(())
    }
}
#[cfg(test)]
mod tests_llm_16_309 {
    use crate::VecWriter;
    use crate::enc::write::Writer;
    use std::vec::Vec;
    #[test]
    fn vec_writer_with_capacity() {
        let _rug_st_tests_llm_16_309_rrrruuuugggg_vec_writer_with_capacity = 0;
        let rug_fuzz_0 = 10;
        let capacity = rug_fuzz_0;
        let writer = VecWriter::with_capacity(capacity);
        debug_assert_eq!(writer.inner.capacity(), capacity);
        let _rug_ed_tests_llm_16_309_rrrruuuugggg_vec_writer_with_capacity = 0;
    }
    #[test]
    fn vec_writer_write() {
        let _rug_st_tests_llm_16_309_rrrruuuugggg_vec_writer_write = 0;
        let rug_fuzz_0 = 1u8;
        let mut writer = VecWriter::default();
        let data = vec![rug_fuzz_0, 2, 3, 4, 5];
        writer.write(&data).unwrap();
        debug_assert_eq!(writer.inner, data);
        let _rug_ed_tests_llm_16_309_rrrruuuugggg_vec_writer_write = 0;
    }
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    use crate::config::{BigEndian, Fixint, Limit};
    use std::borrow::Cow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_1_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Sample data";
        let mut p0: Cow<'static, str> = Cow::Borrowed(rug_fuzz_0);
        let mut p1 = crate::config::Configuration::<
            BigEndian,
            Fixint,
            Limit<1024>,
        >::default();
        let _ = crate::features::impl_alloc::encode_to_vec(p0, p1).unwrap();
        let _rug_ed_tests_rug_1_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_2 {
    use crate::features::impl_alloc::VecWriter;
    #[test]
    fn test_collect() {
        let _rug_st_tests_rug_2_rrrruuuugggg_test_collect = 0;
        let rug_fuzz_0 = 10;
        let p0 = VecWriter::with_capacity(rug_fuzz_0);
        debug_assert_eq!(p0.collect(), Vec:: < u8 > ::new());
        let _rug_ed_tests_rug_2_rrrruuuugggg_test_collect = 0;
    }
}
#[cfg(test)]
mod tests_rug_3 {
    use crate::de::{self, Decoder, Decode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    use std::collections::BinaryHeap;
    #[test]
    fn test_decode() {
        let _rug_st_tests_rug_3_rrrruuuugggg_test_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        <BinaryHeap<i32>>::decode(&mut decoder).unwrap();
        let _rug_ed_tests_rug_3_rrrruuuugggg_test_decode = 0;
    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    use crate::de::{self, read::SliceReader, BorrowDecoder, BorrowDecode};
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    use std::collections::BinaryHeap;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_4_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <BinaryHeap<i32>>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_4_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::Encode;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::features::VecWriter;
    use std::collections::BinaryHeap;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_5_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 5;
        let rug_fuzz_2 = 2;
        let rug_fuzz_3 = 10;
        let mut p0: BinaryHeap<i32> = BinaryHeap::new();
        p0.push(rug_fuzz_0);
        p0.push(rug_fuzz_1);
        p0.push(rug_fuzz_2);
        let mut config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_3);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        p0.encode(&mut p1).unwrap();
        let _rug_ed_tests_rug_5_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::collections::BTreeMap;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_7_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <BTreeMap<u32, u32>>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_7_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_8 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::{VecWriter, impl_alloc};
    use std::collections::BTreeMap;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_8_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 10;
        let mut p0: BTreeMap<i32, String> = BTreeMap::new();
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let writer = VecWriter::with_capacity(rug_fuzz_0);
        let mut p1 = EncoderImpl::new(writer, config);
        p0.encode(&mut p1).unwrap();
        let _rug_ed_tests_rug_8_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    use std::collections::BTreeSet;
    #[test]
    fn test_decode() {
        let _rug_st_tests_rug_9_rrrruuuugggg_test_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <BTreeSet<u8>>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_9_rrrruuuugggg_test_decode = 0;
    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration};
    #[test]
    fn test_borrow_decode() {
        let _rug_st_tests_rug_10_rrrruuuugggg_test_borrow_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut decoder_impl = DecoderImpl::new(reader, config);
        let result = <std::collections::BTreeSet<i32>>::borrow_decode(&mut decoder_impl);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_10_rrrruuuugggg_test_borrow_decode = 0;
    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use std::collections::BTreeSet;
    use crate::enc::{EncoderImpl, Encode, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_11_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 10;
        let mut p0: BTreeSet<i32> = BTreeSet::new();
        p0.insert(rug_fuzz_0);
        p0.insert(rug_fuzz_1);
        p0.insert(rug_fuzz_2);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_3);
        let mut p1 = EncoderImpl::new(v10, config);
        <BTreeSet<i32>>::encode(&p0, &mut p1).unwrap();
        let _rug_ed_tests_rug_11_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, Decode};
    use crate::de;
    use std::collections::VecDeque;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_decode() {
        let _rug_st_tests_rug_12_rrrruuuugggg_test_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut p0 = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(p0, config);
        let _result = <VecDeque<u8>>::decode(&mut p0).unwrap();
        let _rug_ed_tests_rug_12_rrrruuuugggg_test_decode = 0;
    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::de::{self, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_13_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(v11, config);
        <std::collections::VecDeque<u8>>::borrow_decode(&mut p0).unwrap();
        let _rug_ed_tests_rug_13_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use std::collections::VecDeque;
    use crate::enc::{Encoder, EncoderImpl};
    use crate::enc::write::Writer;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    use crate::features::VecWriter;
    use crate::Encode;
    #[test]
    fn test_encode_vecdeque() {
        let _rug_st_tests_rug_14_rrrruuuugggg_test_encode_vecdeque = 0;
        let rug_fuzz_0 = 1;
        let rug_fuzz_1 = 10;
        let mut v26: VecDeque<i32> = VecDeque::new();
        v26.push_back(rug_fuzz_0);
        let mut v9 = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut v19 = EncoderImpl::new(v10, v9);
        debug_assert!(< VecDeque < i32 > > ::encode(& v26, & mut v19).is_ok());
        let _rug_ed_tests_rug_14_rrrruuuugggg_test_encode_vecdeque = 0;
    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::de::{self, Decoder, Decode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_decode() {
        let _rug_st_tests_rug_15_rrrruuuugggg_test_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(&mut reader, config);
        let result: Result<Vec<u8>, _> = <Vec<u8>>::decode(&mut p0);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), vec![1u8, 2u8, 3u8, 4u8]);
        let _rug_ed_tests_rug_15_rrrruuuugggg_test_decode = 0;
    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    use crate::error::DecodeError;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_16_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::<_, _>::new(reader, config);
        let result = <std::vec::Vec<u8>>::borrow_decode(&mut decoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5]);
        let _rug_ed_tests_rug_16_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_17 {
    use crate::de::{Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::Decode;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_17_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(&mut reader, config);
        let result = <std::string::String>::decode(&mut p0);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_17_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{self, Configuration, BigEndian, Fixint};
    use std::boxed::Box;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_18_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, config::Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = Box::<str>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap(), "something".into());
        let _rug_ed_tests_rug_18_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::Encode;
    use crate::features::VecWriter;
    #[test]
    fn test_encode_string() {
        let _rug_st_tests_rug_19_rrrruuuugggg_test_encode_string = 0;
        let rug_fuzz_0 = "Hello, bincode!";
        let rug_fuzz_1 = 10;
        let p0: std::string::String = rug_fuzz_0.to_string();
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(vec_writer, config);
        let result = <std::string::String as Encode>::encode(&p0, &mut p1);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_19_rrrruuuugggg_test_encode_string = 0;
    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use crate::de::Decoder;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_decode() {
        let _rug_st_tests_rug_20_rrrruuuugggg_test_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <std::boxed::Box<u8>>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_20_rrrruuuugggg_test_decode = 0;
    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::Encode;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_22_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 10;
        let mut p0: std::boxed::Box<i32> = Box::new(rug_fuzz_0);
        let mut p1 = {
            let config = Configuration::<
                crate::config::BigEndian,
                crate::config::Fixint,
                crate::config::Limit<1024>,
            >::default();
            let writer = VecWriter::with_capacity(rug_fuzz_1);
            EncoderImpl::new(writer, config)
        };
        p0.encode(&mut p1).unwrap();
        let _rug_ed_tests_rug_22_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::de::Decoder;
    use crate::config::Configuration;
    use crate::config::{BigEndian, Fixint, Limit};
    #[test]
    fn test_decode_box_slice() {
        let _rug_st_tests_rug_23_rrrruuuugggg_test_decode_box_slice = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        let result = <Box<[u8]>>::decode(&mut decoder);
        debug_assert!(result.is_ok());
        debug_assert_eq!(result.unwrap().as_ref(), & data[..]);
        let _rug_ed_tests_rug_23_rrrruuuugggg_test_decode_box_slice = 0;
    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::de::{DecoderImpl, BorrowDecoder};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_borrow_decode() {
        let _rug_st_tests_rug_24_rrrruuuugggg_test_borrow_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut decoder = DecoderImpl::new(reader, config);
        debug_assert!(
            < std::boxed::Box < [u8] > > ::borrow_decode(& mut decoder).is_ok()
        );
        let _rug_ed_tests_rug_24_rrrruuuugggg_test_borrow_decode = 0;
    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::Decode;
    use crate::de::{self, read::SliceReader, DecoderImpl};
    use crate::config::{self, Configuration};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_25_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = config::Configuration::<
            config::BigEndian,
            config::Fixint,
            config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        <std::borrow::Cow<'_, u8>>::decode(&mut p0).unwrap();
        let _rug_ed_tests_rug_25_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_27 {
    use super::*;
    use crate::Encode;
    use std::borrow::Cow;
    use crate::enc::{EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_27_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 10;
        type T = i32;
        let p0: Cow<'static, T> = Cow::Owned(rug_fuzz_0);
        let mut config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(writer, config);
        <std::borrow::Cow<'_, T> as Encode>::encode(&p0, &mut p1).unwrap();
        let _rug_ed_tests_rug_27_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_28 {
    use super::*;
    use crate::Decode;
    use crate::de::{self, Decoder, DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{BigEndian, Configuration, Fixint, Limit};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_28_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<BigEndian, Fixint, Limit<1024>>::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let result = <std::rc::Rc<u32>>::decode(&mut p0);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_28_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_29 {
    use super::*;
    use crate::de::{self, BorrowDecoder, BorrowDecode};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_29_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut v12 = DecoderImpl::new(v11, config);
        debug_assert!(< std::rc::Rc < u8 > > ::borrow_decode(& mut v12).is_ok());
        let _rug_ed_tests_rug_29_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_30 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::{VecWriter, impl_alloc};
    use std::rc::Rc;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_30_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 42;
        let rug_fuzz_1 = 10;
        let mut v37 = Rc::new(rug_fuzz_0);
        let mut the_config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut the_vec_writer = VecWriter::with_capacity(rug_fuzz_1);
        let mut v19 = EncoderImpl::new(the_vec_writer, the_config);
        <Rc<i32> as Encode>::encode(&v37, &mut v19).unwrap();
        let _rug_ed_tests_rug_30_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_31 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    use crate::de::read::SliceReader;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_31_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(&mut v11, v9);
        let _result = <std::rc::Rc<[u8]>>::decode(&mut p0);
        let _rug_ed_tests_rug_31_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    use crate::Decode;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_33_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut v11 = SliceReader::new(&data);
        let mut v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(&mut v11, v9);
        <std::sync::Arc<str>>::decode(&mut p0).unwrap();
        let _rug_ed_tests_rug_33_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_36 {
    use super::*;
    use crate::Encode;
    use std::sync::Arc;
    use crate::enc::{Encoder, EncoderImpl, write::Writer};
    use crate::config::Configuration;
    use crate::features::VecWriter;
    #[test]
    fn test_encode() {
        let _rug_st_tests_rug_36_rrrruuuugggg_test_encode = 0;
        let rug_fuzz_0 = 5;
        let rug_fuzz_1 = 10;
        let mut p0 = Arc::new(rug_fuzz_0);
        let v9 = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let v10 = VecWriter::with_capacity(rug_fuzz_1);
        let mut p1 = EncoderImpl::new(v10, v9);
        let result = p0.encode(&mut p1);
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_36_rrrruuuugggg_test_encode = 0;
    }
}
#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    use crate::de::{self, Decoder};
    use crate::de::read::SliceReader;
    use crate::config::Configuration;
    use crate::de::DecoderImpl;
    use std::sync::Arc;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_37_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let mut reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut p0 = DecoderImpl::new(reader, config);
        let _result = <Arc<[u8]>>::decode(&mut p0).unwrap();
        let _rug_ed_tests_rug_37_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use crate::de::{self, BorrowDecode, BorrowDecoder};
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::Configuration;
    #[test]
    fn test_borrow_decode() {
        let _rug_st_tests_rug_38_rrrruuuugggg_test_borrow_decode = 0;
        let rug_fuzz_0 = 1u8;
        let rug_fuzz_1 = 2;
        let rug_fuzz_2 = 3;
        let rug_fuzz_3 = 4;
        let rug_fuzz_4 = 5;
        let data: [u8; 5] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3, rug_fuzz_4];
        let reader = SliceReader::new(&data);
        let config = Configuration::<
            crate::config::BigEndian,
            crate::config::Fixint,
            crate::config::Limit<1024>,
        >::default();
        let mut decoder = DecoderImpl::new(reader, config);
        <std::sync::Arc<[u8]>>::borrow_decode(&mut decoder).unwrap();
        let _rug_ed_tests_rug_38_rrrruuuugggg_test_borrow_decode = 0;
    }
}
